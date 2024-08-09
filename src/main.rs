use anyhow::bail;
use image::{ExtendedColorType, ImageFormat};
use opencv::{
    core::{Mat, MatTraitConst, MatTraitConstManual},
    imgproc::{cvt_color_def, COLOR_BGR2RGB},
    videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst, CAP_ANY},
};
fn main() -> anyhow::Result<()> {
    //Create video capture instance
    let mut video_capture = videoio::VideoCapture::new_def(CAP_ANY)?;

    //Check if the camera is open already
    if !video_capture.is_opened()? {
        bail!("Failed to open camera.")
    }

    //Start reading
    loop {
        //Create frame
        let mut frame = Mat::default();

        //Read frame
        video_capture.read(&mut frame)?;

        //Write frame
        let frame_size = frame.size()?;

        //Create corrected_frame
        let mut corrected_frame = Mat::default();

        //Color correction
        cvt_color_def(&frame, &mut corrected_frame, COLOR_BGR2RGB)?;

        //Write image
        image::save_buffer_with_format(
            "image.bmp",
            corrected_frame.data_bytes()?,
            frame_size.width as u32,
            frame_size.height as u32,
            ExtendedColorType::Rgb8,
            ImageFormat::Bmp,
        )?;
    }
}

#[cfg(test)]
mod test {
    use wincam::Webcam;

    #[test]
    #[serial_test::serial]
    fn create_instance() {
        let webcam = Webcam::new_def_auto_detect();

        assert!(webcam.is_ok())
    }

    #[test]
    #[serial_test::serial]
    fn read_image_bytes() {
        let mut webcam = Webcam::new_def_auto_detect().unwrap();

        let (bytes, size) = webcam.get_frame().unwrap();

        assert_eq!(bytes.len() as i32, size.area() * 3)
    }
}
