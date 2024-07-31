use clap::Parser;
use tokio::time::{sleep, Duration as TokioDuration};
use libs::client::{ClientOption};
use libs::args::Args;
#[macro_use]
mod libs;

/// Test Client


#[tokio::main]
async fn main() {
    let args=Args::parse();
    println!("server_addr is {},port is {},data size is {},frequency is {},duration is {},no_delay is {}",args.server_addr,args.port,args.size,args.frequency,args.duration,args.no_delay);
    sleep(TokioDuration::from_secs(2)).await;
    //new_with_options!
    let mut options=new_with_options!(ClientOption,{
        server_addr:args.server_addr,
        port:args.port,
        duration:args.duration,
        frequency:args.frequency,
        no_delay:args.no_delay,
        size:args.size,
        protocol:args.protocol,
    });
    options.start().await;

}

