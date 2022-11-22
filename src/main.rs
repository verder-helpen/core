mod config;
mod error;
mod methods;
mod options;
mod start;

#[macro_use]
extern crate rocket;

use config::CoreConfig;
use methods::auth_attr_shim;
use options::{all_session_options, session_options};
use rocket::{fairing::AdHoc, Build};
use start::{session_start, session_start_form, session_start_jwt};

#[launch]
fn boot() -> _ {
    #[cfg(feature = "sentry")]
    verder_helpen_sentry::SentryLogger::init();

    #[allow(unused_mut)]
    let mut base = setup_routes(rocket::build());

    #[allow(unused_variables)]
    let config = base.figment().extract::<CoreConfig>().unwrap_or_else(|_| {
        // Ignore error value, as it could contain private keys
        log::error!("Failure to parse configuration");
        panic!("Failure to parse configuration")
    });

    #[cfg(feature = "sentry")]
    if let Some(dsn) = config.sentry_dsn() {
        base = base.attach(verder_helpen_sentry::SentryFairing::new(dsn, "core"));
    }

    base
}

fn setup_routes(base: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    base.mount(
        "/",
        routes![
            all_session_options,
            session_options,
            session_start,
            session_start_form,
            session_start_jwt,
            auth_attr_shim,
        ],
    )
    .attach(AdHoc::config::<CoreConfig>())
}
