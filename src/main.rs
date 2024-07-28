use std::usize;

use clap::Parser;
use tokio::time::{sleep, Duration as TokioDuration};
use client::ClientOption;

mod packet;
mod client;

/// Test Client
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    /// Server Address
    #[arg(short='s',long)]
    server_addr:String,
    /// Server Port
    #[arg(short,long)]
    port:u16,
    /// Data Size
    #[arg(long,default_value_t=100)]
    size:usize,
    /// Frequency
    #[arg(short,long,default_value_t=100)]
    frequency:u64,
    /// Duration Minute
    #[arg(short,long,default_value_t=1)]
    duration:u64,
    /// Set NoDelay
    #[arg(short,long,default_value_t=false)]
    no_delay:bool,
    /// Protocol Default is tcp
    #[arg(short='t',long,default_value="tcp")]
    protocol:String,

}

#[tokio::main]
async fn main() {
    let args=Args::parse();
    println!("server_addr is {},port is {},data size is {},frequency is {},duration is {},no_delay is {}",args.server_addr,args.port,args.size,args.frequency,args.duration,args.no_delay);
    sleep(TokioDuration::from_secs(2)).await;

    let mut options=ClientOption::new(args.server_addr,args.port,args.duration,args.frequency,args.no_delay,args.size,args.protocol);
    options.start().await;

}

