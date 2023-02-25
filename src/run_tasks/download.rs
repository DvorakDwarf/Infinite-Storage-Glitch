use youtube_dl::{download_yt_dlp, YoutubeDl};

use crate::args::DownloadParams;

pub async fn run_download(args: DownloadParams) -> anyhow::Result<()> {
    let yt_dlp_path = download_yt_dlp(".").await?;

    println!("Starting the download, there is no progress bar");
    let output = YoutubeDl::new(&args.url.expect("No URL in params when run_download"))
        .youtube_dl_path(yt_dlp_path)
        .format("best")
        .download(true)
        .run_async()
        .await?;

    let _video = output.into_single_video().unwrap();
    // let title = video.title;

    println!("Video downloaded successfully");

    return Ok(());
}
