use std::fmt::format;
use std::fs::File;
use std::{fs, io};
use std::io::{Read, Write};
use env_logger::init_from_env;
use log::{debug, error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::assistance::error::MyError;
use crate::assistance::metadata::METADATA;

pub async fn listen(ip: String, port: String) {
    let ip_port = format!("{}:{}",ip,port);
    let listener = TcpListener::bind(format!("{}:{}",ip,port)).await.unwrap();
    info!("Listening: {ip_port}");
    loop {
        match listener.accept().await{
            Ok((socket, addr)) => {
                info!("[{ip_port}] accept remote socket: {addr}");
                tokio::spawn(process_connection(socket, ip_port.clone()));
            },
            Err(e) => {
                info!("[{ip_port}] {e:?}");
            }
        }
    }
}
pub async fn process_connection(mut socket: TcpStream, ip_port: String) {
    // 读入file_name
    // FIXME: 不能使用Vec作为buffer，使用slice
    let mut buf = [0u8; 4096];
    // FIXME: read_to_end无法与vec一起使用
    let n = socket.read(&mut buf).await.unwrap();
    info!("[{ip_port}] read file_path {n} Bytes");
    // FIXME: 这边必须用read返回的n指定读取实际值的Bytes
    let file_path = std::str::from_utf8(&buf[0..n]).unwrap();
    info!("[{ip_port}] {file_path}");
    if file_path != "" {
        // 上传相应file的内容
        upload_file(socket, ip_port, file_path).await;
    }
}
pub async fn upload_file(mut socket: TcpStream, ip_port: String, file: &str){
    info!("[{ip_port}] to upload {file}");
    let mut file_to_upload = File::open(file).unwrap();
    let mut buffer = [0u8; 4096];
    // FIXME: 同上，凡是读取IO内容到buffer，都需要使用read返回的size来指明内容实际大小
    while let Ok(n) = file_to_upload.read(&mut buffer){
        if (n == 0) {
            break;
        }
        socket.write_all(&buffer[..n]).await.unwrap();
        info!("[{ip_port}] upload {file} {n} Bytes");
    }
}
pub async fn download_file(user_name: String, metadata: METADATA) -> Result<String, MyError> {
    match tokio::net::TcpStream::connect(format!("{}:{}", metadata.ip, metadata.port)).await{
        Ok(mut stream) => {
            // write file_path to peer
            let file_path = format!("{}/{}", metadata.path, metadata.name);
            stream.write_all(file_path.as_bytes()).await.unwrap();
            info!("[this host] write file_path {file_path} to remote peer");
            // create file to receive peer's file
            // ZIHAO: 避免无权限，就在本目录下创建吧
            fs::create_dir_all(format!("./Downloads/{user_name}/{}/{}", metadata.ip, metadata.port)).unwrap();
            let file_path = format!("./Downloads/{user_name}/{}/{}/{}", metadata.ip, metadata.port, metadata.name);
            let mut local_file = File::create(&file_path).unwrap();
            info!("[this host] create file {file_path}");
            // FIXME: 同上，将IO内容读入到buffer，都需要read返回的size作为实际大小
            let mut buffer = [0u8; 4096];
            stream.readable().await.unwrap();
            while let Ok(n) = stream.read(&mut buffer).await{
                info!("[this host] read {n}");
                if n== 0{
                    break;
                }
                local_file.write_all(&buffer[..n]).unwrap();
                info!("[this host] write {n} Bytes in local file {file_path}");
            }
            Ok("success".to_string())
        },
        Err(e) => {
            Err(MyError::ClientError {
                code: 1,
                message: e.to_string()
            })
        }
    }
}