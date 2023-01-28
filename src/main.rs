mod ui;
mod etcher;
mod settings;
mod embedsource;

use settings::{Data, Settings};

//Make RGB a struct
//Make it calculate how much data is jammed in 1 frame for user
fn main() -> anyhow::Result<()> {
    // ui::summon_gooey();
    let bytes = etcher::rip_bytes("src/tests/Baby.wav")?;

    dbg!(bytes.len());
    // for i in 0..10000 {
    //     if bytes[i] != 0 {
    //         dbg!(bytes[i]);
    //     }
    // }

    // let binary = etcher::rip_binary(bytes)?;

    let data = Data::from_color(bytes);
    let settings = Settings::new(1, 30, 640, 360);

    etcher::etch("src/out/output.avi", data, settings)?;

    let out_data = etcher::read("src/out/output.avi")?;
    
    //WHO ATE THE BYTES ???
    dbg!(out_data.len());
    // for i in 0..10000 {
    //     if out_data[i] != 0 {
    //         dbg!(out_data[i]);
    //     }
    // }

    etcher::write_bytes("src/out/imbaby.wav", out_data)?;

    return Ok(());
}
