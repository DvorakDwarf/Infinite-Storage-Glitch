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
    let out_modes = vec![
        "Colored",
        "B/W (Binary)",
    ];

    let resolutions = vec![
        "144p",
        "360p",
        "720p",
    ];

    let out_mode = Select::new("Pick how data will be embedded", out_modes.clone())
        .with_help_message("Colored mode is useless if the video undergoes compression at any point, B/W survives compression")
        .prompt()
        .unwrap();

    let size = CustomType::<i32>::new("What size should the blocks be ?")
        .with_error_message("Please type a valid number")
        .with_help_message("Bigger blocks are more resistant to compression, I recommend 5-15 if you use this feature.")
        .with_default(1)
        .prompt()?;

    let out_mode = match out_mode {
        "Colored" => OutputMode::Color,
        "B/W (Binary)" => OutputMode::Binary,
        _ => {panic!("AAAAAAAAAAAAAAAA")},
    };

    let fps = CustomType::<i32>::new("What fps should the video be at ?")
        .with_error_message("Please type a valid number")
        .with_help_message("Decreasing fps may decrease chance of compression")
        .with_default(30)
        .prompt()
        .expect("Invalid fps");

    //Check if higher resolution runs faster
    let resolution = Select::new("Pick a resolution", resolutions)
        .with_help_message("I recommend 360p")
        .prompt()
        .unwrap();

    let path = Text::new("What is the path to your file ?")
    .with_default("src/tests/Baby.wav")
    .prompt().unwrap();

    //"144p" => (192, 144),
    //For some reason only 360p and 720p work
    let (width, height) = match resolution {
        "144p" => (100, 100),
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
            let settings = Settings::new(size, fps, width, height);

            etcher::etch("output.avi", data, settings)?;
        },
        OutputMode::Binary => {
            let bytes = etcher::rip_bytes(&path)?;
            let binary = etcher::rip_binary(bytes)?;

            let data = Data::from_binary(binary);
            let settings = Settings::new(size, fps, width, height);

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
        .with_default("setting_tests/Baby.wav")
        .with_help_message("Please include name of file and extension")
        .prompt().unwrap();

    let out_data = etcher::read(&in_path)?;
    etcher::write_bytes(&out_path, out_data)?;

    return Ok(());
}