use std::time::Duration;

use async_nats::{ConnectOptions, jetstream};
use authn_worker_notify::{AppConf, workers::verification};
use lettre::{AsyncSmtpTransport, Tokio1Executor, transport::smtp::PoolConfig};
use mimalloc::MiMalloc;
use tokio::signal;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,async-nats=warn".into());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json().with_target(false).with_current_span(false))
        .init();

    let conf = AppConf::init();

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::from_url(&conf.smtp.to_addr())?
            .pool_config(
                PoolConfig::new()
                    .max_size(conf.smtp.max_connection)
                    .min_idle(conf.smtp.min_connection),
            )
            .timeout(Duration::try_from(conf.smtp.timeout.0).ok())
            .build();

    let nats_options =
        ConnectOptions::with_nkey(conf.nats.user_private_key.expose_str().to_owned());
    let nats = async_nats::connect_with_options(conf.nats.to_addr(), nats_options).await?;
    let js = jetstream::new(nats);
    let stream = js.get_stream("authn").await?;

    info!("Starting workers");
    let cancellation_token = tokio_util::sync::CancellationToken::new();

    tokio::spawn(verification::init(stream.clone(), mailer.clone(), cancellation_token.clone()));

    tokio::select! {
            _ = signal::ctrl_c() => {},
            _ = async {
                if cfg!(unix) {
                signal::unix::signal(signal::unix::SignalKind::terminate())
                        .expect("failed to install SIGTERM handler")
                        .recv()
                        .await;
            } else {
                std::future::pending::<()>().await;
            }
            } => {},
    }

    info!("Terminating workers");

    cancellation_token.cancel();
    tokio::time::sleep(Duration::from_secs(10)).await;

    info!("Termination successful");
    Ok(())
}
