use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum Command {
    Shutdown,
    ShutdownMonitor,
    Brightness { value: u8 },
    Volume { value: u8 },
    Netflix,
    VrtNuTvGuide,
    VrtNuLive,
    Spotify,
}
