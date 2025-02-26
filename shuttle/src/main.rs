// shuttle/src/main.rs

// dependencies
use pavex::server::Server;
use server::configuration::{ApplicationProfile, Config};
use shuttle_pavex::PavexService;

// module declarations
mod shuttle_pavex;

#[shuttle_runtime::main]
async fn pavex() -> shuttle_pavex::ShuttlePavex {
    let app_profile = Config::load(Some(ApplicationProfile::Prod))?;

    let server = Server::new();

    let shuttle_px = PavexService(server, app_profile);

    Ok(shuttle_px)
}
