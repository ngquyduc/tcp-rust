use std::error::Error;
use std::sync::{mpsc, Arc};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;

use crate::task::{Task, TaskType};

pub trait ServerTrait {
    async fn start_server(
        &self,
        address: String,
        tx: mpsc::Sender<Result<(), Box<dyn Error + Send>>>,
    );
}

pub struct Server;

impl ServerTrait for Server {
    async fn start_server(
        &self,
        address: String,
        tx: mpsc::Sender<Result<(), Box<dyn Error + Send>>>,
    ) {
        println!("Starting the server");
        let listener = TcpListener::bind(address).await;
        let cpu_limit = Arc::new(Semaphore::new(40));
        match listener {
            Ok(listener) => {
                tx.send(Ok(())).unwrap();
                println!("Server started");
                loop {
                    let (stream, _) = listener.accept().await.unwrap();
                    let cpu_limit_clone = cpu_limit.clone();
                    println!("Connection established");
                    tokio::spawn(async move {
                        Self::handle_connection(stream, cpu_limit_clone).await;
                    });
                }
            }
            Err(e) => {
                println!("here {}", e);
                let _ = tx.send(Err(Box::new(e)));
            }
        }
    }
}

impl Server {
    async fn handle_connection(mut stream: TcpStream, cpu_limit: Arc<Semaphore>) {
        loop {
            let mut buf_reader = BufReader::new(&mut stream);
            let mut line = String::new();
            match buf_reader.read_line(&mut line).await {
                Ok(0) => {
                    return;
                }
                Ok(_) => {
                    let response = Self::get_task_value(line, cpu_limit.clone()).await;
                    if let Some(r) = response {
                        stream.write(&[r]).await.unwrap();
                    }
                }
                Err(e) => {
                    eprintln!("Unable to get command due to: {}", e);
                    return;
                }
            }
        }
    }

    async fn get_task_value(buf: String, cpu_limit: Arc<Semaphore>) -> Option<u8> {
        let numbers: Vec<&str> = buf.trim().split(':').collect();
        let task_type = numbers.first().unwrap().parse::<u8>().ok()?;
        let seed = numbers.last().unwrap().parse::<u64>().ok()?;

        if TaskType::from_u8(task_type).unwrap() == TaskType::CpuIntensiveTask {
            let _ = cpu_limit.acquire().await.unwrap();
            let result = Task::execute_async(task_type, seed).await;
            return Some(result);
        } else {
            let result = Task::execute_async(task_type, seed).await;
            return Some(result);
        }

    }
}
