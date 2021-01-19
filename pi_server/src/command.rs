use serde::Deserialize;

#[derive(Deserialize)]
pub enum Command {
    Shutdown,
    Brightness {value: u8},
    ShutdownMonitor,
    Netflix,
    VrtNuTvGuide,
    VrtNuLive,
}
