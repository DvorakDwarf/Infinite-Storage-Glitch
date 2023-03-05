use youtube_dl::{download_yt_dlp};
use std::process::Command;

use crate::args::DownloadParams;

pub async fn run_download(args: DownloadParams) -> anyhow::Result<()> {
    let yt_dlp_path = download_yt_dlp(".").await?;
    
    let url = args.url.expect("No URL in params when run_download");
    
    // check if the yt_dlp_path exists
    if !yt_dlp_path.exists() {
        println!("yt-dlp not found");
        return Ok(());
    }
    
    // Output path for the video has the format: `downloaded_{timestamp}.mp4`
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let download_path = format!("downloaded_{}.mp4", timestamp);

    // Use the yt-dlp binary to download the video: `yt-dlp -f mp4 -o video.mp4 {url}`
    println!("Starting the download, there is no progress bar");
    let output = Command::new(yt_dlp_path)
        .arg("-f")  // format
        .arg("mp4") // mp4
        .arg("-o")  // output
        .arg(download_path.clone()) // output path
        .arg(url)  // url to download from
        .output()
        .expect("Failed to execute command");

    // check the output of the command
    if output.status.success() {
        println!("Video downloaded successfully");
        println!("Output path: {}", std::fs::canonicalize(download_path).unwrap().display());
    } else {
        println!("Video download failed");
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    return Ok(());
}
