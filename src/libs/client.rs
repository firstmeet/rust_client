use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream, UdpSocket};
use std::time::Duration;

use rand::Rng;
use rand::rngs::OsRng;
use tokio::time::{Duration as TokioDuration, Instant, sleep};

use crate::libs::packet::Packet;

#[macro_export]
macro_rules! new_with_options {
    ($struct_name:ident,) => {
        $struct_name{
            ..Default::default()
        }
    };
    ($struct_name:ident, {$($field_name:ident: $field_value:expr),* $(,)?}) => {
        {
            let mut instance = $struct_name {
                // 使用默认值初始化所有字段
                ..Default::default()
            };
            // 使用传入的参数更新字段值
            $(
                instance.$field_name = $field_value;
            )*
            instance
        }
    };
}

#[derive(Default,Debug)]
pub struct ClientOption {
    pub server_addr: String,
    pub port: u16,
    pub duration: u64,
    pub frequency: u64,
    pub no_delay: bool,
    pub size: usize,
    pub protocol: String,
    pub round_trip_time: Vec<Duration>,
    pub data: Vec<u8>,
}

impl ClientOption {
    pub fn new(server_addr: String, port: u16, duration: u64, frequency: u64, no_delay: bool, size: usize, protocol: String) -> Self {
        Self {
            server_addr,
            port,
            duration,
            frequency,
            no_delay,
            size,
            protocol,
            round_trip_time: Vec::new(),
            data: Vec::new(),
        }
    }
    pub async fn start(&mut self) {
        self.generate_data();
        if self.protocol == "tcp" {
            self.handle_tcp().await;
        } else {
            self.handle_udp().await;
        }
        println!("run finished!!");
    }
    fn generate_data(&mut self) {
        let buffer = self.generate_random_bytes();
        let packet = Packet {
            data: buffer,
            no_delay: self.no_delay,
        };
        let serialized = serde_json::to_vec(&packet).expect("serialize error");
        self.data = serialized;
    }

    async fn handle_tcp(&mut self) {
        let mut stream: TcpStream = TcpStream::connect((self.server_addr.as_str(), self.port)).expect("connect error");
        stream.set_nodelay(self.no_delay).expect("set nodelay error");
        let app_start = Instant::now();
        let duration = Duration::from_secs(self.duration);
        let sleep_duration = TokioDuration::from_millis(1000 / self.frequency);
        loop {
            // 发送文件大小
            println!("running {:?}", app_start.elapsed());
            if app_start.elapsed() > duration {
                let avg = self.calculate_avg();
                println!("Average round trip time: {:?}", avg);
                println!("Time`s up! Exiting client!");
                break;
            }

            let size = self.data.len() as u32;
            let mut len_buf = [0u8; 4];
            len_buf.copy_from_slice(&size.to_be_bytes());
            let start = Instant::now();
            stream.write_all(&len_buf).expect("write size error");
            stream.write_all(&self.data).expect("write msg error");
            //
            // 等待服务端响应
            let mut buffer_read = [0; 4]; // 假设响应大小为4字节
            stream.read_exact(&mut buffer_read).expect("read error");
            let elapsed = start.elapsed();
            println!("Round trip time: {:?}", elapsed);
            self.round_trip_time.push(elapsed);
            if self.duration == 0 {
                break;
            }
            sleep(sleep_duration).await;
        }
        stream.shutdown(Shutdown::Both).expect("shutdown error");
    }
    async fn handle_udp(&mut self) {
        let udp: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("listen udp error");
        let app_start = Instant::now();
        let duration = Duration::from_secs(self.duration);
        let sleep_duration = TokioDuration::from_millis(1000 / self.frequency);
        loop {
            // 发送文件大小
            println!("running {:?}", app_start.elapsed());
            if app_start.elapsed() > duration {
                let avg = self.calculate_avg();
                println!("Average round trip time: {:?}", avg);
                println!("Time`s up! Exiting client!");
                break;
            }
            let start = Instant::now();
            // 发送文件内容
            match udp.send_to(&self.data, (self.server_addr.as_str(), self.port)) {
                Ok(size) => {
                    println!("send data size:{:?}", &size);
                }
                Err(e) => {
                    println!("send error:{:?}", e);
                }
            }
            //read
            let mut buffer_read = [0u8; 4]; // 假设响应大小为4字节
            udp.recv_from(&mut buffer_read).expect("read udp err");
            let elapsed = start.elapsed();
            println!("Round trip time: {:?}", elapsed);

            self.round_trip_time.push(elapsed);
            if self.duration == 0 {
                break;
            }
            sleep(sleep_duration).await;
        }
    }
    fn calculate_avg(&mut self) -> Duration {
        //去掉最大值和最小值，求平均值
        if self.round_trip_time.len() <= 2 {
            return Duration::from_secs(0);
        }
        let mut times = self.round_trip_time.clone();
        times.sort();
        times.pop(); // Remove max
        times.remove(0); // Remove min
        let total: Duration = times.iter().sum();
        let len = times.len();
        let avg = total / len as u32;
        avg
    }
    fn generate_random_bytes(&mut self) -> Vec<u8> {
        let mut rng = OsRng;  // 获取线程局部的随机数生成器
        let vec = vec![0u8; self.size];
        let mut random_bytes = vec;  // 创建一个指定长度的零初始化向量
        rng.fill(random_bytes.as_mut_slice());  // 用随机数填充向量
        random_bytes
    }

    pub fn set_round_trip_time(&mut self, round_trip_time: Vec<Duration>) {
        self.round_trip_time = round_trip_time;
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn set_server_addr(&mut self, server_addr: String) {
        self.server_addr = server_addr;
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_duration(&mut self, duration: u64) {
        self.duration = duration;
    }

    pub fn set_frequency(&mut self, frequency: u64) {
        self.frequency = frequency;
    }

    pub fn set_no_delay(&mut self, no_delay: bool) {
        self.no_delay = no_delay;
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_protocol(&mut self, protocol: String) {
        self.protocol = protocol;
    }
}
