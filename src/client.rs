use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream, UdpSocket};
use std::time::{Instant,Duration};
use rand::Rng;
use rand::rngs::OsRng;
use tokio::time::{sleep, Duration as TokioDuration};
use crate::packet::Packet;

pub struct ClientOption{
    pub server_addr:String,
    pub port:u16,
    pub duration:u64,
    pub frequency:u64,
    pub no_delay:bool,
    pub size:usize,
    pub protocol:String,
    round_trip_time:Vec<Duration>,
}

impl ClientOption{
    pub fn new(server_addr:String,port:u16,duration:u64,frequency:u64,no_delay:bool,size:usize,protocol:String)->Self{
        Self{
            server_addr,
            port,
            duration,
            frequency,
            no_delay,
            size,
            protocol,
            round_trip_time:Vec::new(),
        }
    }
     pub async fn start(&mut self){
        if self.protocol=="tcp"{
           self.handle_tcp().await;
        }else{
           self.handle_udp().await;
        }
    }

    async fn handle_tcp(&mut self){
        let mut stream: TcpStream = TcpStream::connect((self.server_addr.as_str(),self.port)).expect("connect error");
        stream.set_nodelay(self.no_delay).expect("set nodelay error");
        let app_start=Instant::now();
        let duration=Duration::from_secs(self.duration);
        let sleep_duration=TokioDuration::from_millis(1000/self.frequency);
        loop {
            // 发送文件大小
            println!("running {:?}",app_start.elapsed());
            if app_start.elapsed()>duration{
                let avg=self.calculate_avg();
                println!("Average round trip time: {:?}", avg);
                println!("Time`s up! Exiting client!");
                break;
            }
            let buffer=generate_random_bytes(self.size);
            let packet = Packet {
                data: buffer,
                no_delay: self.no_delay,
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
            self.round_trip_time.push(elapsed_push);
            if self.duration == 0 {
                break;
            }
            sleep(sleep_duration).await;
        }
        stream.shutdown(Shutdown::Both).expect("shutdown error");
        println!("run finished!!");
    }
    async fn handle_udp(&mut self){
        let udp:UdpSocket=UdpSocket::bind("0.0.0.0:0").expect("listen udp error");
        let app_start=Instant::now();
        let duration=Duration::from_secs(self.duration*60);
        let sleep_duration=TokioDuration::from_millis(1000/self.frequency);
        let mut round_trip_time:Vec<Duration>=Vec::new();
        loop{
            // 发送文件大小
            println!("running {:?}",app_start.elapsed());
            if app_start.elapsed()>duration{
                let avg=self.calculate_avg();
                println!("Average round trip time: {:?}", avg);
                println!("Time`s up! Exiting client!");
                break;
            }
            let buffer=generate_random_bytes(self.size);

            let start = Instant::now();
            // 发送文件内容
            match udp.send_to(&(buffer.len() as u32).to_be_bytes(), (self.server_addr.as_str(),self.port)) {
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
            if self.duration == 0 {
                break;
            }
            sleep(sleep_duration).await;
        }
    }
    fn calculate_avg(&mut self)->Duration{
        let mut sum=Duration::from_secs(0);
        let length=self.round_trip_time.len();
        for i in 0..length{
            sum+=self.round_trip_time[i];
        }
        let avg=sum/length as u32;
        avg
    }
}
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut rng = OsRng;  // 获取线程局部的随机数生成器
    let vec = vec![0u8; length];
    let mut random_bytes = vec;  // 创建一个指定长度的零初始化向量
    rng.fill(random_bytes.as_mut_slice());  // 用随机数填充向量
    random_bytes
}