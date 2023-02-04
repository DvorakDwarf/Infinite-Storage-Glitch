mod ui;
mod etcher;
mod settings;
mod embedsource;
mod timer;

use settings::{Data, Settings};

//Make RGB a struct
//Make it calculate how much data is jammed in 1 frame for user
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::summon_gooey().await?;
    // let bytes = etcher::rip_bytes("src/tests/Baby.wav")?;
    // let binary = etcher::rip_binary(bytes)?;

    // let data = Data::from_binary(binary);
    // let settings = Settings::new(1, 30, 640, 360);

    // etcher::etch("src/out/output.avi", data, settings)?;

    // let out_data = etcher::read("src/out/output.avi")?;

    // etcher::write_bytes("src/out/Baby2.wav", out_data)?;

    return Ok(());
}
