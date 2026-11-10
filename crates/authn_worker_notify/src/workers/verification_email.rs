use std::sync::{Arc, LazyLock};

use async_nats::jetstream::consumer::PullConsumer;
use futures_util::StreamExt;
use jiff::{Timestamp, tz::TimeZone};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
};
use minijinja::{Environment, Template, context};
use nats_messages::authn::VerificationEmail;
use rkyv::{from_bytes, rancor::Error as RkyvError};
use tokio::{select, sync::Semaphore};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, instrument};

static NAME: &str = "verification_email";
static SENDER: &str = "NoBody <nobody@domain.tld>";
static SUBJECT: &str = "Welcome";

static JINJA_ENV: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.add_template(NAME, include_str!("../../templates/verification_email.html"))
        .expect("Failed to add verification_email template");
    env
});

pub struct Worker {
    consumer: PullConsumer,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    template: Template<'static, 'static>,
    max_concurrency: usize,
    child_token: CancellationToken,
}

impl Worker {
    pub fn new(
        consumer: PullConsumer,
        mailer: AsyncSmtpTransport<Tokio1Executor>,
        max_concurrency: usize,
        child_token: CancellationToken,
    ) -> Self {
        let template = JINJA_ENV.get_template(NAME).unwrap();
        Self { consumer, mailer, template, max_concurrency, child_token }
    }

    #[instrument(name = "worker", skip_all, fields(worker_name = %NAME))]
    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Starting worker");

        let mut messages = self.consumer.messages().await?;
        let semaphore = Arc::new(Semaphore::new(self.max_concurrency));

        loop {
            select! {
                _ = self.child_token.cancelled() => {
                    info!("Cancelling worker");
                    semaphore.close();
                    break;
                }

                permit = semaphore.clone().acquire_owned() => {
                    let _permit = permit?;

                    match messages.next().await {
                        Some(Ok(message)) => {
                            let Ok(data) = from_bytes::<VerificationEmail, RkyvError>(&message.payload)
                                .inspect_err(|error| error!(%error, "Failed to process message")) else {
                                continue;
                            };

                            info!("Building email");

                             let expires = Timestamp::from_second(data.expires_at)?
                                .to_zoned(TimeZone::system())
                                .to_string();

                            let template_str = self.template
                                .render(context! { code => data.code.as_str(), expires => &expires })?;

                            let lettre_message = Message::builder()
                                .from(SENDER.parse()?)
                                .to(data.email.parse()?)
                                .subject(SUBJECT)
                                .header(ContentType::TEXT_HTML)
                                .body(template_str)?;

                            info!("Sending email");

                            let Ok(_) = self.mailer.send(lettre_message).await
                                .inspect_err(|error| error!(%error, "Failed to send email")) else {
                                continue
                            };

                            info!("Email sent");

                            let _ack = message.double_ack().await.inspect_err(|error| error!(%error, "Failed to double ack"));
                        }
                        Some(Err(error)) => {
                            error!(%error, "Failed to receive message");
                        }
                        None => {}
                    }

                }
            }
        }

        Ok(())
    }
}
