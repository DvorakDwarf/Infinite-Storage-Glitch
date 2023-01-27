mod ui;
mod etcher;
mod settings;
mod embedsource;

use settings::{Data, Settings};

//Make it calculate how much data is jammed in 1 frame for user
fn main() -> anyhow::Result<()> {
    // ui::summon_gooey();
    let bytes = etcher::rip_bytes("src/tests/Baby.wav")?;
    let binary = etcher::rip_binary(bytes)?;
    let data = Data::from_binary(binary);

    let settings = Settings::new(1, 30, 640, 360);

    etcher::etch("src/out/output.avi", data, settings)?;

    return Ok(());
}
