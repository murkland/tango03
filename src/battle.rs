use crate::input;
use crate::mgba;
use crate::signor;

pub struct Init {
    input_delay: u32,
    marshaled: [u8; 0x100],
}

struct BattleHolder {
    number: u32,
    battle: Option<Battle>,
}

pub struct Match {
    dc: Option<std::sync::Arc<webrtc::data_channel::RTCDataChannel>>,
    negotiation_result: parking_lot::Mutex<anyhow::Result<bool>>,
    session_id: String,
    match_type: u16,
    game_title: String,
    game_crc32: u32,
    won_last_battle: bool,
    battle_holder: parking_lot::Mutex<BattleHolder>,
    aborted: std::sync::atomic::AtomicBool,
}

impl Match {
    pub fn new(session_id: String, match_type: u16, game_title: String, game_crc32: u32) -> Self {
        Match {
            dc: None,
            negotiation_result: parking_lot::Mutex::new(Ok(false)),
            session_id,
            match_type,
            game_title,
            game_crc32,
            won_last_battle: false,
            battle_holder: parking_lot::Mutex::new(BattleHolder {
                number: 0,
                battle: None,
            }),
            aborted: false.into(),
        }
    }

    pub fn abort(&mut self) {
        self.aborted
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn aborted(&mut self) -> bool {
        self.aborted.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn lock_battle(&self) -> parking_lot::MappedMutexGuard<Option<Battle>> {
        parking_lot::MutexGuard::map(self.battle_holder.lock(), |battle_holder| {
            &mut battle_holder.battle
        })
    }

    #[tokio::main(flavor = "current_thread")]
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut sc = signor::Client::new("localhost:12345").await?;

        let api = webrtc::api::APIBuilder::new().build();
        let (peer_conn, dc, side) = sc
            .connect(
                || async {
                    let peer_conn = api
                        .new_peer_connection(
                            webrtc::peer_connection::configuration::RTCConfiguration {
                                ..Default::default()
                            },
                        )
                        .await?;
                    let dc = peer_conn
                        .create_data_channel(
                            "tango",
                            Some(
                                webrtc::data_channel::data_channel_init::RTCDataChannelInit {
                                    id: Some(1),
                                    negotiated: Some(true),
                                    ordered: Some(true),
                                    ..Default::default()
                                },
                            ),
                        )
                        .await?;
                    Ok((peer_conn, dc))
                },
                &self.session_id,
            )
            .await?;
        self.dc = Some(dc);

        // TODO: Other negotiation stuff.

        Ok(())
    }

    pub fn poll_for_ready(&self) -> anyhow::Result<bool> {
        match &*self.negotiation_result.lock() {
            Ok(ready) => Ok(*ready),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }
}

pub struct Battle {
    is_p2: bool,
    iq: input::Queue,
    local_pending_turn_wait_ticks_left: i32,
    local_pending_turn: Option<[u8; 0x100]>,
    remote_delay: u32,
    is_accepting_input: bool,
    is_over: bool,
    last_committed_remote_input: input::Input,
    last_input: Option<[input::Input; 2]>,
    state_committed: (), // TODO: what type should this be?
    committed_state: Option<mgba::state::State>,
}

impl Battle {
    pub fn local_player_index(&self) -> u8 {
        if self.is_p2 {
            1
        } else {
            0
        }
    }

    pub fn remote_player_index(&self) -> u8 {
        1 - self.local_player_index()
    }
}
