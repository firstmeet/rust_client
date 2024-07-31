use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Server Address
    #[arg(short = 's', long)]
    pub server_addr: String,
    /// Server Port
    #[arg(short, long)]
    pub port: u16,
    /// Data Size
    #[arg(long, default_value_t = 100)]
    pub size: usize,
    /// Frequency
    #[arg(short, long, default_value_t = 100)]
    pub frequency: u64,
    /// Duration Minute
    #[arg(short, long, default_value_t = 1)]
    pub duration: u64,
    /// Set NoDelay
    #[arg(short, long, default_value_t = false)]
    pub no_delay: bool,
    /// Protocol Default is tcp
    #[arg(short = 't', long, default_value = "tcp")]
    pub protocol: String,

}