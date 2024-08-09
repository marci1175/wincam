use anyhow::bail;
use opencv::imgproc::{cvt_color_def, COLOR_BGR2RGB};
use opencv::videoio::{VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst, CAP_ANY};
use opencv::core::{Mat, MatTraitConst, MatTraitConstManual, Size_};

/// Webcam struct definition
/// The struct wraps the ```VideoCapture``` type, and has custom function for it.
/// You can create a new instance by the new functions.
pub struct Webcam(VideoCapture);

impl Webcam {
    /// Create new ```Webcam``` instance with api preference and camera index
    /// If you want to use the default api_preference you should use ```new_def(i32)``` instead
    /// API preference consts are available at the [opencv documentation](https://docs.rs/opencv/latest/opencv/index.html). Some exmaples for this const are: ```CAP_MSMF```, ```CAP_V4L```.
    pub fn new(camera_idx: i32, api_preference: i32) -> anyhow::Result<Self> {
        let video_capture_handle = VideoCapture::new(camera_idx, api_preference)?;

        if !video_capture_handle.is_opened()? {
            bail!("Failed to open capture device.")
        }
        
        Ok(
            Self(video_capture_handle)
        )
    }

    /// Create new ```Webcam``` instance with auto camera detection.
    /// Please note that this function tries to auto detect the camera.
    /// If you have more than one camera you should use the ```new_def(i32)``` function to define which camera you are wanting to use.
    pub fn new_def_auto_detect() -> anyhow::Result<Self> {
        let video_capture_handle = VideoCapture::new_def(CAP_ANY)?;

        if !video_capture_handle.is_opened()? {
            bail!("Failed to open capture device.")
        }

        Ok(
            Self(video_capture_handle)
        )
    }

    /// Create new ```Webcam``` instance from the camera index.
    /// The passed in argument defines which camera this function creates a new instance from
    pub fn new_def(camera_idx: i32) -> anyhow::Result<Self> {
        let video_capture_handle = VideoCapture::new_def(camera_idx)?;

        if !video_capture_handle.is_opened()? {
            bail!("Failed to open capture device.")
        }

        Ok(
            Self(video_capture_handle)
        )
    }

    /// Reads an image out of the ```VideoCapture``` buffer, this removes the bytes of the image from the buffer.
    /// Returns a tuple of the raw image bytes and the size of the image.
    /// Please note the image's bytes returned by this function are automaticly converted from [BRG8](https://learn.microsoft.com/en-us/windows/win32/wic/-wic-codec-native-pixel-formats#rgbbgr-color-model) (Which is returned by opencv by default) to RGB8
    pub fn get_frame(&mut self) -> anyhow::Result<(Vec<u8>, Size_<i32>)> {
        //Create frame which will be overwritten
        let mut frame = Mat::default();

        //Read frame
        self.0.read(&mut frame)?;

        //Create corrected_frame
        let mut corrected_frame = Mat::default();

        //Color correction
        cvt_color_def(&mut frame, &mut corrected_frame, COLOR_BGR2RGB)?;

        //Return captured frame
        Ok((frame.data_bytes()?.to_vec(), corrected_frame.size()?))
    }

    /// Get the backend api's name
    pub fn get_backend_name(&self) -> anyhow::Result<String> {
        Ok(self.0.get_backend_name()?)
    }
}