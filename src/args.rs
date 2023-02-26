use clap::{Args, Parser, Subcommand, ValueEnum};

/// This encodes which, if any, subcommand was picked.
/// If none were picked, default to UI selects.
#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// This encodes the specific subcommand the user requested: embed, download or dislodge.
#[derive(Subcommand)]
pub enum Commands {
    Embed(EmbedParams),
    Download(DownloadParams),
    Dislodge(DislodgeParams),
}

/// This encodes the specific params for embedding.
/// All values are optional, and will be substituted in using UI if missing.
#[derive(Args, Default, Debug)]
pub struct EmbedParams {
    #[arg(short, long)]
    /// Path to the file with the data to encode
    pub in_path: Option<String>,

    /// Preset to use when encoding data.
    /// More specific encoding options override preset options.
    #[arg(short, long)]
    pub preset: Option<EmbedPreset>,

    /// Etching mode
    #[arg(long)]
    pub mode: Option<EmbedOutputMode>,

    /// Block size, in pixels per side
    #[arg(long)]
    pub block_size: Option<i32>,

    /// Threads to use when encoding
    #[arg(long)]
    pub threads: Option<usize>,

    /// Output video FPS
    #[arg(long)]
    pub fps: Option<i32>,

    /// Output video resolution.
    /// Must be one of "144", "240", "360", "480" or "720",
    /// and if the value provided is none of these,
    /// defaults to "360".
    #[arg(long)]
    pub resolution: Option<String>, // TODO: fix this so it's checked at parse time
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedPreset {
    /// Optimal compression resistance
    Optimal,
    /// Paranoid compression resistance
    Paranoid,
    /// Maximum efficiency
    MaxEfficiency,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedOutputMode {
    /// Uses RGB values and breaks under compression
    Colored,
    /// Uses black and white pixels and resists compression
    Binary,
}

impl From<EmbedOutputMode> for crate::settings::OutputMode {
    fn from(value: EmbedOutputMode) -> Self {
        match value {
            EmbedOutputMode::Colored => Self::Color,
            EmbedOutputMode::Binary => Self::Binary,
        }
    }
}

/// This encodes the specific params for downloading.
/// All values are optional, and will be substituted in using UI if missing.
#[derive(Args, Default)]
pub struct DownloadParams {
    /// Video URL
    #[arg(short, long)]
    pub url: Option<String>,
}

/// This encodes the specific params for dislodging.
/// All values are optional, and will be substituted in using UI if missing.
#[derive(Args, Default)]
pub struct DislodgeParams {
    /// Path to input video
    #[arg(short, long)]
    pub in_path: Option<String>,

    /// Path to file output (including extension)
    #[arg(short, long)]
    pub out_path: Option<String>,
}
