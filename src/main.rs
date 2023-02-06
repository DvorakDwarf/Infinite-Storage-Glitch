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
    println!("Welcome to ISG (Infinite Storage Glitch)");
    println!("This tool allows you to turn any file into a compression-resistant video that can be uploaded to YouTube for Infinite Storage:tm:");
    println!("\nHow to use:");
    println!("1. Zip all the files you will be uploading");
    println!("2. Use the embed option on the archive (THE VIDEO WILL BE SEVERAL TIMES LARGER THAN THE FILE: original size * 8 * block size^2 = new size)");
    println!("3. Upload the video to your YouTube channel. You probably want to keep it up as unlisted");
    println!("4. Use the download option to get the video back");
    println!("5. Use the dislodge option to get your files back");
    println!("6. PROFIT. Enjoy being a leech on a huge corporation's servers");

    println!("\nI coudln't figure out a weird bug where if you set the size to something that isn't a factor of the height");
    println!("If you don't want the files you put in to come out as the audio/visual equivalent of a pipe bomb, account for the above bug\n");

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
