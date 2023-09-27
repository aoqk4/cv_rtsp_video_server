// use rayon::{join, prelude::*};
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
// use tokio::task;

mod capture;

#[derive(PartialEq)]
pub enum TASKS {
    WRITE,
    READ,
}

pub fn server() -> Result<(), Error> {
    // SET Ip Addr
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 13689);

    // Bind addr
    let listener = TcpListener::bind(socket)?;

    // Accept client request
    let (mut tcp_stream, addr) = listener.accept()?;
    println!("Connection received! {:?} is sending data.", addr);

    // Send buffer loop
    loop {
        tcp_stream.write_all(&make_video_buffer())?;

        tcp_stream.flush()?;
    }

    // Ok(())
}

pub fn arc_mutex_server() -> Result<(), std::io::Error> {
    // SET Ip Addr
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 13689);

    // Accpet client with new Arc(mutex)
    let listener = TcpListener::bind(socket)?;
    let socket_res = Arc::new(Mutex::new(listener.accept().unwrap()));

    let handle = thread::spawn(move || {
        let sock = Arc::clone(&socket_res);

        let mut buf = String::new();

        let a = sock.lock().unwrap().0.read_to_string(&mut buf).unwrap();

        if a == 0 {
            panic!("NOT READ!!!");
        } else {
            println!("This is Client Buf : {:?}", buf);
        }
    });

    handle.join().unwrap();

    println!("aaabbbccc");

    Ok(())
}

/// *************************************
/// # GET ONE FRAME VIDEO BUFFER (.PNG)
///
/// ### return Vec<u8> (buffer)
/// *************************************
fn make_video_buffer() -> Vec<u8> {
    // Get video buffer
    let video_buffer_vec = capture::video_capture().unwrap();

    // Convert video buffer len usize -> u8
    let mut res_of_video: Vec<u8> = video_buffer_vec
        .len()
        .to_string()
        .split("")
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    // 0 ~ 5 -> Buffer size
    // 6 ~   -> Buffer
    res_of_video.extend(video_buffer_vec);

    res_of_video
}

#[cfg(test)]
mod tests {
    // use super::*;
}
