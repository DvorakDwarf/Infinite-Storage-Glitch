use anyhow;

use inquire::{
    CustomType, min_length, Confirm, MultiSelect, Password, Select, Text,
};

// use crate::settings::{Settings, OutputMode};
// use crate::etcher::Embedder;

pub fn summon_gooey() {

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
        "Download" => return download_path(),
        "Dislodge" => return dislodge_path(),
        _ => {panic!("Something weird happened when selecting modes");}
    }
}


fn embed_path() {
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

    let out_mode = Select::new("Pick how data will be embedded", out_modes.clone())
        .with_help_message("Colored mode is useless if the video undergoes compression at any point, B/W survives compression")
        .prompt()
        .unwrap();

    let size = 1usize;
    if out_mode == out_modes[1] {
        let size = CustomType::<usize>::new("What size should the blocks be ?")
            .with_error_message("Please type a valid number")
            .with_help_message("Bigger blocks are more resistant to compression, I recommend 5-15 if you use this feature.")
            .with_default(1)
            .prompt();
    }

    // let out_mode = match out_mode {
    //     "Colored" => OutputMode::Color,
    //     "B/W (Binary)" => OutputMode::Binary,
    //     _ => {panic!("AAAAAAAAAAAAAAAA")},
    // };

    let fps = CustomType::<f64>::new("What fps should the video be at ?")
        .with_error_message("Please type a valid number")
        .with_help_message("Decreasing fps may decrease chance of compression")
        .with_default(30.0)
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

    let (width, height) = match resolution {
        "144p" => (256, 144),
        "240p" => (426, 240),
        "360p" => (640, 360),
        "480p" => (854, 480),
        "720p" => (1280, 720),
        _ => (640, 360),
    };

    // let settings = Settings::new(out_mode, size, fps, width, height).expect("Could not finish making settings");
    
    // let mut embdr = Embedder::encode(&path, settings).expect("Could not encode");
    // embdr.embed("output.avi").expect("Could not embed");
}

fn download_path() {

}

fn dislodge_path() {

}