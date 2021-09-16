mod config;
mod error;
mod methods;
mod options;
mod sentry;
mod start;

#[macro_use]
extern crate rocket;

use config::CoreConfig;
use methods::auth_attr_shim;
use options::{all_session_options, session_options};
use rocket::{fairing::AdHoc, Build};
use start::{session_start, session_start_jwt};

#[launch]
fn boot() -> _ {
    log::set_boxed_logger(Box::new(sentry::SentryLogger::new(Box::new(
        env_logger::builder().parse_default_env().build(),
    ))))
    .expect("failure to setup loggin");

    let base = setup_routes(rocket::build());
    let config = base.figment().extract::<CoreConfig>().unwrap_or_else(|_| {
        // Ignore error value, as it could contain private keys
        log::error!("Failure to parse configuration");
        panic!("Failure to parse configuration")
    });
    match config.sentry_dsn() {
        Some(dsn) => base.attach(sentry::SentryFairing::new(dsn)),
        None => base,
    }
}

fn setup_routes(base: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    base.mount(
        "/",
        routes![
            all_session_options,
            session_options,
            session_start,
            session_start_jwt,
            auth_attr_shim,
        ],
    )
    .attach(AdHoc::config::<CoreConfig>())
}
