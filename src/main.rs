use image::{ExtendedColorType, ImageFormat};
use opencv::{core::{Mat, MatTraitConst, MatTraitConstManual}, imgproc::{cvt_color_def, COLOR_BGR2RGB}, videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst}};
use anyhow::{bail};
fn main() -> anyhow::Result<()> {
    //Create video capture instance
    let mut video_capture = videoio::VideoCapture::new_def(0)?;

    //Check if the camera is open already
    if !video_capture.is_opened()? {
        bail!("Failed to open camera!")
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
        cvt_color_def(&mut frame, &mut corrected_frame, COLOR_BGR2RGB)?;

        //Write image
        image::save_buffer_with_format("img.bmp", corrected_frame.data_bytes()?, frame_size.width as u32, frame_size.height as u32, ExtendedColorType::Rgb8, ImageFormat::Bmp)?;
    }

    Ok(())
}
