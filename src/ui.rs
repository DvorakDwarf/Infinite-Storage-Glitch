use anyhow;
use inquire::{
    CustomType, min_length, Confirm, MultiSelect, Password, Select, Text,
};
use youtube_dl::{download_yt_dlp, YoutubeDl};

use crate::settings::{Settings, OutputMode, Data};
use crate::etcher;

pub async fn summon_gooey() -> anyhow::Result<()> {
    let options = vec![
        "Embed",
        "Download",
        "Dislodge"
    ];

    let modes = Select::new("Pick what you want to do with the program", options)
        .with_help_message("Embed: Create a video from files,\n Download: Download files stored on YouTube,\n Dislodge: Return files from an embedded video")
        .prompt()
        .unwrap();

    match modes {
        "Embed" => return embed_path(),
        "Download" => return download_path().await,
        "Dislodge" => return dislodge_path(),
        _ => {panic!("Something weird happened when selecting modes");}
    }
}


fn embed_path()  -> anyhow::Result<()> {

    //Should use enums
    let presets = vec![
        "Optimal compression resistance",
        "Paranoid compression resistance",
        "Maximum efficiency",
        "Custom"
    ];

    let out_modes = vec![
        "Colored",
        "B/W (Binary)",
    ];

    let resolutions = vec![
        "144p",
        "240p",
        "360p",
        "480p",
        "720p",
    ];

    let path = Text::new("What is the path to your file ?")
    .with_default("src/tests/test.txt")
    .prompt().unwrap();

    let preset = Select::new("You can use one of the existing presets or custom settings", presets.clone())
        .with_help_message("Any amount of compression on Maximum Efficiency will corrupt all your hopes and dreams")
        .prompt()
        .unwrap();

    match preset {
        "Maximum efficiency" => {
            let bytes = etcher::rip_bytes(&path)?;

            let data = Data::from_color(bytes);
            // let settings = Settings::new(1, 8, 1, 640, 360);
            let settings = Settings::new(1, 8, 10, 256, 144);

            etcher::etch("output.avi", data, settings)?;

            return Ok(());
        },
        "Optimal compression resistance" => {
            let bytes = etcher::rip_bytes(&path)?;
            let binary = etcher::rip_binary(bytes)?;

            let data = Data::from_binary(binary);
            let settings = Settings::new(2, 8, 10, 1280, 720);

            etcher::etch("output.avi", data, settings)?;

            return Ok(());
        },
        "Paranoid compression resistance" => {
            let bytes = etcher::rip_bytes(&path)?;
            let binary = etcher::rip_binary(bytes)?;

            let data = Data::from_binary(binary);
            let settings = Settings::new(4, 8, 10, 1280, 720);

            etcher::etch("output.avi", data, settings)?;

            return Ok(());
        },
        _ => (),
    }

    let out_mode = Select::new("Pick how data will be embedded", out_modes.clone())
        .with_help_message("Colored mode is useless if the video undergoes compression at any point, B/W survives compression")
        .prompt()
        .unwrap();

    println!("\nI couldn't figure out a weird bug that happens if you set the size to something that isn't a factor of the height");
    println!("If you don't want the files you put in to come out as the audio/visual equivalent of a pipe bomb, account for the above bug\n");

    let size = CustomType::<i32>::new("What size should the blocks be ?")
        .with_error_message("Please type a valid number")
        .with_help_message("Bigger blocks are more resistant to compression, I recommend 2-5.")
        .with_default(2)
        .prompt()?;

    let threads = CustomType::<usize>::new("How many threads to dedicate for processing ?")
        .with_error_message("Please type a valid number")
        .with_help_message("The more threads, the merrier")
        .with_default(8)
        .prompt()?;

    let out_mode = match out_mode {
        "Colored" => OutputMode::Color,
        "B/W (Binary)" => OutputMode::Binary,
        _ => {panic!("AAAAAAAAAAAAAAAA")},
    };

    let fps = CustomType::<i32>::new("What fps should the video be at ?")
        .with_error_message("Please type a valid number")
        .with_help_message("Decreasing fps may decrease chance of compression. ~10fps works")
        .with_default(30)
        .prompt()
        .expect("Invalid fps");

    //Check if higher resolution runs faster
    let resolution = Select::new("Pick a resolution", resolutions)
        .with_help_message("I recommend 720p as the resolution won't affect compression")
        .prompt()
        .unwrap();

    let (width, height) = match resolution {
        "144p" => (256, 144),
        "240p" => (426, 240),
        "360p" => (640, 360),
        "480p" => (854, 480),
        "720p" => (1280, 720),
        _ => (640, 360),
    };

    match out_mode {
        OutputMode::Color => {
            let bytes = etcher::rip_bytes(&path)?;

            let data = Data::from_color(bytes);
            let settings = Settings::new(size, threads, fps, width, height);

            etcher::etch("output.avi", data, settings)?;
        },
        OutputMode::Binary => {
            let bytes = etcher::rip_bytes(&path)?;
            let binary = etcher::rip_binary(bytes)?;

            let data = Data::from_binary(binary);
            let settings = Settings::new(size, threads, fps, width, height);

            etcher::etch("output.avi", data, settings)?;
        },
    }

    return Ok(());
}

async fn download_path()  -> anyhow::Result<()> {
    //
    let url = Text::new("What is the url to the video ?")
        .prompt().unwrap();

    let yt_dlp_path = download_yt_dlp(".").await?;

    println!("Starting the download, there is no progress bar");
    let output = YoutubeDl::new(&url)
        .youtube_dl_path(yt_dlp_path)
        .format("best")
        .download(true)
        .run_async()
        .await?;

    let video = output.into_single_video().unwrap();
    let title = video.title;

    println!("Video downloaded succesfully");

    return Ok(());
}

//TEMPORARY DEFAULTS
fn dislodge_path()  -> anyhow::Result<()> {
    let in_path = Text::new("What is the path to your video ?")
        .with_default("output.avi")
        .prompt().unwrap();

    let out_path = Text::new("Where should the output go ?")
        .with_help_message("Please include name of file and extension")
        .prompt().unwrap();

    // let threads = CustomType::<usize>::new("How many threads to dedicate for processing ?")
    //     .with_error_message("Please type a valid number")
    //     .with_help_message("The more threads, the merrier")
    //     .with_default(8)
    //     .prompt()?;

    let out_data = etcher::read(&in_path, 1)?;
    etcher::write_bytes(&out_path, out_data)?;

    return Ok(());
}
