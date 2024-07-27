mod packet;

use std::io::{ Read, Write};
use std::net::{TcpStream, Shutdown, UdpSocket};
use std::time::{Duration, Instant};
use std::{u8, usize, u32};
use clap::{Parser};
use packet::Packet;
use rand::Rng;
use rand::rngs::OsRng;
use tokio::time::{sleep, Duration as TokioDuration};
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
struct ClientOption{
     server_addr:String,
     port:u16,
     duration:u64,
    frequency:u64,
    no_delay:bool,
    size:usize,
}
#[tokio::main]
async fn main() {
    let args=Args::parse();
    println!("server_addr is {},port is {},data size is {},frequency is {},duration is {},no_delay is {}",args.server_addr,args.port,args.size,args.frequency,args.duration,args.no_delay);
    sleep(TokioDuration::from_secs(2)).await;
   
    let options=ClientOption{
        server_addr:args.server_addr,
        port:args.port,
        duration:args.duration,
        frequency:args.frequency,
        no_delay:args.no_delay,
        size:args.size,
    };
    if args.protocol=="tcp"{
        handle_tcp(options).await;
    }else{
        handle_udp(options).await;
    }
}
fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut rng = OsRng;  // 获取线程局部的随机数生成器
    let vec = vec![0u8; length];
    let mut random_bytes = vec;  // 创建一个指定长度的零初始化向量
    rng.fill(random_bytes.as_mut_slice());  // 用随机数填充向量
    random_bytes
}
async fn handle_tcp(option: ClientOption){
    let mut stream: TcpStream = TcpStream::connect((option.server_addr.as_str(),option.port)).expect("connect error");
    stream.set_nodelay(option.no_delay).expect("set nodelay error");
    let app_start=Instant::now();
    let duration=Duration::from_secs(option.duration);
    let sleep_duration=TokioDuration::from_millis(1000/option.frequency);
    let mut round_trip_time:Vec<Duration>=Vec::new();
    loop {
        // 发送文件大小
        println!("running {:?}",app_start.elapsed());
        if app_start.elapsed()>duration{
            calculate_avg(&round_trip_time,round_trip_time.len());
            println!("Time`s up! Exiting client!");
            break;
        }
        let buffer=generate_random_bytes(option.size);
        let packet = Packet {
            data: buffer,
            no_delay: option.no_delay,
        };
        let serialized = serde_json::to_vec(&packet).expect("serialize error");
        let size = serialized.len() as u32;
        let start = Instant::now();
        let mut len_buf = [0u8; 4];
        len_buf.copy_from_slice(&size.to_be_bytes());
        stream.write_all(&len_buf).expect("write size error");
        // 发送文件内容
        stream.write_all(&serialized).expect("write msg error");

        // 等待服务端响应
        let mut buffer_read = [0; 4]; // 假设响应大小为4字节
        stream.read_exact(&mut buffer_read).expect("read error");
        let elapsed = start.elapsed();
        println!("Round trip time: {:?}", elapsed);
        let elapsed_push=elapsed.clone();
        round_trip_time.push(elapsed_push);
        if option.duration == 0 {
            break;
        }
        sleep(sleep_duration).await;
    }
    stream.shutdown(Shutdown::Both).expect("shutdown error");
    println!("run finished!!");
}
async fn handle_udp(option: ClientOption){
    let udp:UdpSocket=UdpSocket::bind("0.0.0.0:0").expect("listen udp error");
    let app_start=Instant::now();
    let duration=Duration::from_secs(option.duration*60);
    let sleep_duration=TokioDuration::from_millis(1000/option.frequency);
    let mut round_trip_time:Vec<Duration>=Vec::new();
    loop{
        // 发送文件大小
        println!("running {:?}",app_start.elapsed());
        if app_start.elapsed()>duration{
            calculate_avg(&round_trip_time,round_trip_time.len());
            println!("Time`s up! Exiting client!");
            break;
        }
        let buffer=generate_random_bytes(option.size);

        let start = Instant::now();
        // 发送文件内容
        match udp.send_to(&(buffer.len() as u32).to_be_bytes(), (option.server_addr.as_str(),option.port)) {
            Ok(size)=>{
                println!("send data size:{:?}",&size);
            }
            Err(e)=>{
                println!("send error:{:?}",e);
            }
        }
        let mut buffer_read = [0; 4]; // 假设响应大小为4字节
        udp.recv_from(&mut buffer_read).expect("read udp error");
        let elapsed = start.elapsed();
        println!("Round trip time: {:?}", elapsed);
        let elapsed_push=elapsed.clone();
        round_trip_time.push(elapsed_push);
        if option.duration == 0 {
            break;
        }
        sleep(sleep_duration).await;
    }
}
fn calculate_avg(arr:&Vec<Duration>,length:usize){
    let mut sum=Duration::from_secs(0);
    for i in 0..length{
        sum+=arr[i];
    }
    let avg=sum/length as u32;
    println!("Average round trip time: {:?}", avg);
}