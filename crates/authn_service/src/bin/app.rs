use async_nats::{ConnectOptions, jetstream};
use authn_service::{AppConf, AppState, models::*, routes};
use mimalloc::MiMalloc;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "info,tower_http=warn,axum=warn,toasty=warn,tokio_postgres=warn".into()
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json().with_target(false).with_current_span(false))
        .init();

    let conf = AppConf::init();

    let db = toasty::Db::builder()
        .models(toasty::models!(
            OauthChallenge,
            RefreshToken,
            Registration,
            SecurityAuditLog,
            User,
            UserDevice,
            UserIdentity,
            UserPasskey,
            UserPassword,
            WebauthnChallenge
        ))
        .connect(&conf.db.to_database_url())
        .await?;

    let cache_client = redis::Client::open(conf.cache.to_database_url())?;
    let cache = cache_client.get_multiplexed_async_connection().await?;

    let nats_options =
        ConnectOptions::with_nkey(conf.nats.user_private_key.expose_str().to_owned());
    let nats = async_nats::connect_with_options(conf.nats.to_addr(), nats_options).await?;
    let js = jetstream::new(nats);

    let addr = conf.server.to_addr();
    let state = AppState {
        db,
        cache,
        js,
        server_conf: conf.server,
        secrets_conf: conf.secrets,
        jwt_token_conf: conf.jwt,
        cookies_conf: conf.cookies,
    };

    info!(addr = %addr, "Starting server");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let app = routes::init(state);
    axum::serve(listener, app).await?;

    Ok(())
}
