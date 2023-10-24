use std::io::{Error, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

mod capture;

pub fn arc_mutex_server() -> Result<(), std::io::Error> {
    // SET Ip Addr
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 13689);
    let handle = connect_sock(&socket)?;
    for _ in 0..10 {
        println!("{:?}", handle.thread().id());
    }
    Ok(())
}

fn connect_sock(socket: &SocketAddrV4) -> Result<JoinHandle<()>, Error> {
    // Accept client with new Arc(mutex)
    let listener = TcpListener::bind(socket)?;
    let socket_res = Arc::new(Mutex::new(listener.accept().unwrap()));

    let handle = thread::spawn(move || {
        let sock = Arc::clone(&socket_res);
        sock.lock()
            .unwrap()
            .0
            .write_all(&make_video_buffer())
            .unwrap();
        sock.lock().unwrap().0.flush().unwrap();
    });

    Ok(handle)
}

/// *************************************
/// #### GET VIDEO BUFFER (.PNG)
///
/// return Vec<u8> (buffer)
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
