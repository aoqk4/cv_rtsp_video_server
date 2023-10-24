use opencv::{prelude::*, videoio};

pub fn video_capture() -> opencv::Result<Vec<u8>, opencv::Error> {
    // SET RTSP URL
    let video_url = "myIPIP";

    // INIT CAPTURE CAMERA
    let mut cam = videoio::VideoCapture::default()?;

    // SET RTSP URL AT CAPTURE
    let rtsp_test = videoio::VideoCapture::open_file(&mut cam, video_url, videoio::CAP_FFMPEG)?;

    // IF OPEN ERROR PANIC! (FILENAME OR URL ADDR ERROR)
    if !rtsp_test {
        panic!("RTSP OPEN ERROR!!");
    }

    // CAPTURE VIDEO LOOP
    loop {
        // INIT FRAMES
        let mut frame = Mat::default();

        // READ CAPTURE FRAMES
        cam.read(&mut frame)?;

        // IF INPUT FRAME -> ENCODE IMG LOGIC | ELSE -> PANIC!
        if frame.size()?.width > 0 {
            // INIT IMG ENCODE
            let mut encode_image = opencv::core::Vector::<u8>::new();

            // ENCODE IMG
            let flag = opencv::imgcodecs::imencode(
                ".PNG",
                &frame,
                &mut encode_image,
                &opencv::core::Vector::<i32>::new(),
            )?;

            // IF SUCCESS | FAIL -> PANIC!
            if flag {
                return Ok(encode_image.to_vec());
            } else {
                panic!("NO FRAME IN HERE!!")
            }
        } else {
            panic!("FRAME READ ERROR!!")
        }
    }
}
