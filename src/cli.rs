use clap::Parser;

/// ターミナルで拍手する黒猫。タイポを煽る。
#[derive(Parser, Debug)]
#[command(name = "clpcat", version, about, long_about = None)]
pub struct Cli {
    /// 再生回数 (0 で無限ループ)
    #[arg(short = 'n', long, default_value_t = 1, value_name = "N")]
    pub repeat: u32,

    /// 再生速度 (fps)
    #[arg(long, default_value_t = 12, value_name = "FPS")]
    pub fps: u32,
}
