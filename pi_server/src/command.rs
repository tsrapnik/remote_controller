use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Command {
    Shutdown,
    Brightness { value: u8 },
    ShutdownMonitor,
    Netflix,
    VrtNuTvGuide,
    VrtNuLive,
}
