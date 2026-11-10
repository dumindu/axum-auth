use std::time::Duration;

use anyhow::anyhow;
use async_nats::{ConnectOptions, jetstream, jetstream::consumer::PullConsumer};
use authn_worker_notify::{AppConf, VerificationEmailWorker};
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor, transport::smtp::PoolConfig};
use mimalloc::MiMalloc;
use tokio::{select, task::JoinSet};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tokio=warn,async_nats=warn,lettre=warn".into());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json().with_target(false).with_current_span(false))
        .init();

    let conf = AppConf::init();

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::from_url(&conf.smtp.to_addr())?
            .pool_config(
                PoolConfig::new()
                    .min_idle(conf.smtp.min_connections)
                    .max_size(conf.smtp.max_connections)
                    .idle_timeout(Duration::from_secs(conf.smtp.idle_timeout_in_seconds)),
            )
            .timeout(Some(Duration::from_secs(conf.smtp.timeout_in_seconds)))
            .build();

    mailer.test_connection().await?;

    let nats_options =
        ConnectOptions::with_nkey(conf.nats.user_private_key.expose_str().to_owned());
    let nats = async_nats::connect_with_options(conf.nats.to_addr(), nats_options).await?;
    let stream = jetstream::new(nats).get_stream("authn").await?;

    let cancel_token = CancellationToken::new();

    let signal_token = cancel_token.clone();
    let shutdown_handle = tokio::spawn(async move {
        shutdown_signal().await;
        info!("Shutdown signal received");
        signal_token.cancel();
    });

    info!("Starting workers");

    let mut workers: JoinSet<Result<&str, anyhow::Error>> = JoinSet::new();

    // --- consumers ---
    {
        let c_name = &conf.workers.verification_email.consumer_name;
        let consumer: PullConsumer = stream.get_consumer(c_name).await.map_err(|e| anyhow!(e))?;
        let mailer = mailer.clone();
        let max = conf.workers.verification_email.max_concurrency;
        let child_token = cancel_token.child_token();
        workers.spawn({
            async move {
                VerificationEmailWorker::new(consumer, mailer, max, child_token).run().await?;
                Ok("verification_email")
            }
        });
    }

    select! {
        _ = cancel_token.cancelled() => {
            info!("Shutdown signal received, cancelling workers");
        }
        // early fails
        Some(result) = workers.join_next() => {
            match result {
                Ok(Ok(name)) => error!("Failed to start worker: {}", name),
                Ok(Err(err)) => error!("Failed to start with error: {}", err),
                Err(err) => error!("Panicked with: {}", err)
            }
            cancel_token.cancel();
        }
    }

    info!("Cancelling workers");

    shutdown_handle.abort();

    // Give some time to shut down gracefully
    tokio::time::sleep(Duration::from_secs(5)).await;

    mailer.shutdown().await;

    while let Some(result) = workers.join_next().await {
        match result {
            Ok(Ok(name)) => info!("Worker cancelled: {}", name),
            Ok(Err(err)) => error!("Failed to cancel with error: {}", err),
            Err(err) => error!("Failed to cancel with a panic: {}", err),
        }
    }

    info!("All workers cancelled");
    Ok(())
}

#[cfg(unix)]
async fn shutdown_signal() {
    use tokio::signal::unix::{SignalKind, signal};
    let mut terminate = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");

    select! {
        result = tokio::signal::ctrl_c() => {
            result.expect("Failed to install CTRL+C handler");
        },
        _ = terminate.recv() => {},
    }
}

#[cfg(not(unix))]
async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("Failed to install CTRL+C handler");
}
