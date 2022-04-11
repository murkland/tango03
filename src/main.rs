#[macro_use]
extern crate lazy_static;

mod audio;
mod battle;
mod bn6;
mod config;
mod current_input;
mod datachannel;
mod fastforwarder;
mod game;
mod gui;
mod input;
mod locales;
mod mgba;
mod protocol;
mod signor;
mod tps;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    mgba::log::init();
    log::info!(
        "welcome to tango v{}-{}!",
        env!("CARGO_PKG_VERSION"),
        git_version::git_version!()
    );
    let config = match config::load_config() {
        Ok(config) => config,
        Err(e) => {
            log::warn!("failed to load config, will load default instead: {}", e);
            let config = config::Config::default();
            config::save_config(&config)?;
            config
        }
    };
    log::info!("current config: {:?}", config);
    let g = game::Game::new(config)?;
    g.run();
    Ok(())
}
