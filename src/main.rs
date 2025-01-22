use core::num;
use std::{fmt::format, net::SocketAddr}; //socket address

use r3bl_rs_utils_core::{friendly_random_id};
use r3bl_tui::*;
use tokio::{
    io::{AsyncBufReadExt,AsyncWriteExt,BufReader,BufWriter},
    net::{tcp::WriteHalf,TcpListener,TcpStream},
    sync::broadcast::{self, error::RecvError, Sender},
};

pub type IOResult<T> = std::io::Result<T>;

#[derive(Debug,Clone)]
pub struct Msgtype {
    pub socket_addr : SocketAddr,
    pub payload : String,
    pub from_id : String,
}

#[tokio::main]
pub async fn main() -> IOResult<()> {
    let addr = "127.0.0.1:3000";

    femme::start();

    let tcp_listener = TcpListener::bind(addr).await?;
    log::info!("Server is ready to accept connections on {}", addr);

    let(tx , _ ) = broadcast::channel::<Msgtype>(10);

    loop {
        let (tcp_stream , socket_addr) = tcp_listener.accept().await?;

        let tx = tx.clone();
        tokio::spawn(async move{
            let result = handle_client_task(tcp_stream, tx, socket_addr).await;
            match result {
                Ok(_) => {
                    log::info!("handle_task_client() terminated gracefully");
                }
                Err(error) => log::error!("handle_client_task() encountered error: {}", error),
            }
        });
    }
}

async fn handle_client_task(
    mut tcp_stream : TcpStream,
    tx: Sender<Msgtype>,
    socket_addr : SocketAddr,
) -> IOResult<()> {
    log::info!("Handle socket connection from client");

    let id = friendly_random_id::generate_friendly_random_id();
    let mut rx = tx.subscribe();

    let (reader, writer) = tcp_stream.split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    let welcome_msg_for_client = &format!("addr: {} , id: {}\n", socket_addr, id);
    writer.write(welcome_msg_for_client.as_bytes()).await?;
    writer.flush().await?;

    let mut incoming = String::new();

    loop {
        let tx = tx.clone();

        tokio::select! {
            result = rx.recv() => { 
                read_from_broadcast_channel(result, socket_addr , &mut writer ,&id).await?;
            }

            network_read_result = reader.read_line(&mut incoming) => {
                let num_bytes_read : usize = network_read_result?;


                if num_bytes_read == 0 {
                    break;
                }

                handle_socket_read(num_bytes_read, &id, &incoming, &mut writer, tx , socket_addr).await?;
                incoming.clear();
            }
        }
    }

    Ok(())
}

async fn read_from_broadcast_channel(
    result : Result<Msgtype, RecvError>,
    socket_addr : SocketAddr,
    writer : &mut BufWriter<WriteHalf<'_>>,
    id: &str,
) -> IOResult<()> {
    match result {
        Ok(it) => {
            let msg: Msgtype = it;
            log::info!("[{}]: channels : {:?} ",id, msg);
            if msg.socket_addr != socket_addr {
                writer.write(msg.payload.as_bytes()).await?;
                writer.flush().await?;
            }   
        }
        Err(error) => {
            log::error!("{:?}", error);
        }
    }
    Ok(())
}

async fn handle_socket_read(
    num_bytes_read: usize,
    id: &str,
    incoming: &str,
    writer: &mut BufWriter<WriteHalf<'_>>,
    tx : Sender<Msgtype>,
    socket_addr : SocketAddr,
) -> IOResult<()> {
    log::info!("[{}]: Incoming : {} , size : {}",
        id,
        incoming.trim(),
        num_bytes_read,
    );

    let outgoing = process(&incoming).await;

    writer.write(outgoing.as_bytes()).await?;
    writer.flush().await?;

    let _ = tx.send(Msgtype {
        socket_addr,
        payload: incoming.to_string(),
        from_id: id.to_string(),
    });

    log::info!(
        "[{}]: outgoing: {} , size : {}",
        id,
        outgoing.trim(),
        num_bytes_read,
    );
    Ok(())
}


async fn process(incoming: &str) -> String {
    let incoming_trimmed = format!("{}", incoming.trim());

    let outgoing = &incoming_trimmed;

    format!("{}\n", outgoing)
} 