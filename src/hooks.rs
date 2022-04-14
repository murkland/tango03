use crate::{config, fastforwarder, gui, loaded};

pub trait Hooks {
    fn install_fastforwarder_hooks(
        &self,
        core: mgba::core::CoreMutRef,
        ff_state: fastforwarder::State,
    ) -> mgba::trapper::Trapper;

    fn install_main_hooks(
        &self,
        config: std::sync::Arc<parking_lot::Mutex<config::Config>>,
        core: mgba::core::CoreMutRef,
        handle: tokio::runtime::Handle,
        match_state: std::sync::Arc<tokio::sync::Mutex<loaded::MatchState>>,
        joyflags: std::sync::Arc<std::sync::atomic::AtomicU32>,
        gui_state: std::sync::Weak<gui::State>,
        fastforwarder: fastforwarder::Fastforwarder,
    ) -> mgba::trapper::Trapper;

    fn set_init(&self, core: mgba::core::CoreMutRef, player_index: u8, init: &[u8]);

    fn prepare_for_fastforward(&self, core: mgba::core::CoreMutRef);

    fn in_battle_time(&self, core: mgba::core::CoreMutRef) -> u32;
}
