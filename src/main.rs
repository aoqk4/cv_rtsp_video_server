// mod capture;
use cv_rtsp_video_server::{arc_mutex_server, server};

fn main() {
    // server().unwrap();

    arc_mutex_server().unwrap();
}
