// shuttle/src/shuttle_pavex.rs

// dependencies
use app::configuration::{AppConfig, AssetsConfig, TemplateConfig};
use pavex::server::Server;
use server_sdk::{build_application_state, run};
use shuttle_runtime::{CustomError, Error, Service};
use std::borrow::Cow;
use std::net::SocketAddr;

// type declarations
pub type ShuttlePavex = Result<PavexService, Error>;

// A wrapper type for a Pavex Server, so we can implement shuttle_runtime::Service for it.
pub struct PavexService(pub Server);

#[shuttle_runtime::async_trait]
impl Service for PavexService {
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let templates_dir = TemplateConfig {
            dir: Cow::Owned("templates".to_string()),
        };
        let assets_dir = AssetsConfig {
            dir: Cow::Owned("static".to_string()),
        };
        let app_config = AppConfig {
            templates: templates_dir,
            assets: assets_dir,
        };
        let application_state = build_application_state(app_config).await;

        let server = self
            .0
            .bind(addr)
            .await
            .expect("Failed to bind the server TCP listener");

        tracing::info!("Starting to listen for incoming requests at {}", addr);

        run(
            server,
            application_state
                .map_err(|e| CustomError::new(e).context("Unable to create Tera templates"))?,
        )
        .await;

        Ok(())
    }
}

impl From<Server> for PavexService {
    fn from(router: Server) -> Self {
        Self(router)
    }
}
