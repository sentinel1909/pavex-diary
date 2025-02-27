pub mod index;
pub mod ping;

use pavex::blueprint::{Blueprint, router::GET};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/", f!(self::index::get))
        .error_handler(f!(crate::routes::index::tera_error2response));
    bp.route(GET, "/api/ping", f!(self::ping::get));
}
