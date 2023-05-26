use gtk::{gdk::Display, prelude::*, Application};

mod animations;
mod config;
mod helpers;
mod logic;
mod ui;

use helpers::*;

const APP_ID: &str = "ch.bettelini.paolo.LindenmayerGarden";
const LOG_ENV: &str = "LSYS_LOG";
const LOG_ENV_STYLE: &str = "LSYS_LOG_STYLE";

fn main() {
    setup_logger();

    log::info!("Initializing application");
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(move |app| {
        load_css!("style/style.css");

        let default_conf = ""; // Empty editor
        logic::initialize_config(app, default_conf);
    });

    app.run();
}

fn setup_logger() {
    if std::env::var(LOG_ENV).is_err() {
        // Setup default logging value:
        std::env::set_var(LOG_ENV, "info");
    }

    let env = env_logger::Env::new()
        .filter(LOG_ENV)
        .write_style(LOG_ENV_STYLE);

    env_logger::init_from_env(env);
}
