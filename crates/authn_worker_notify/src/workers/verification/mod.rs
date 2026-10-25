use std::sync::LazyLock;

use anyhow::anyhow;
use async_nats::jetstream::{consumer::PullConsumer, stream::Stream};
use futures_util::StreamExt;
use jiff::{Timestamp, tz::TimeZone};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
};
use minijinja::{Environment, context};
use nats_messages::authn::{ArchivedVerification, Verification};
use tokio_util::sync::CancellationToken;
use tracing::info;

static JINJA_ENV: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.add_template("email", include_str!("template.html"))
        .expect("Failed to add local verification template");
    env
});

pub async fn init(
    stream: Stream,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    cancellation_token: CancellationToken,
) -> anyhow::Result<()> {
    let consumer: PullConsumer =
        stream.get_consumer(Verification::CONSUMER).await.map_err(|e| anyhow!(e))?;
    let template = JINJA_ENV.get_template("email")?;

    let mut messages = consumer.messages().await?;

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => {
                info!("Shutting down verification loop");
                break
            },
            next_msg = messages.next() => {
                let Some(Ok(message)) = next_msg else { continue };
            let data =
                rkyv::access::<ArchivedVerification, rkyv::rancor::Failure>(&message.payload)?;

            let expires = Timestamp::from_second(data.expires_at.to_native())?
                .to_zoned(TimeZone::system())
                .to_string();

            let email_body =
                template.render(context! {code => data.code.as_str(), expires => &expires})?;

            let m = Message::builder()
                .from("NoBody <nobody@domain.tld>".parse()?)
                .to(data.email.as_str().parse()?)
                .subject("Welcome")
                .header(ContentType::TEXT_HTML)
                .body(email_body)?;

            mailer.send(m).await?;

            message.ack().await.map_err(|e| anyhow!(e))?;
        }
        }
    }

    Ok(())
}
