use std::{io::{BufWriter, Write}, ptr};

use anyhow::Error;
use windows::{
    core::{w, PCWSTR},
    Win32::{
        self,
        Foundation::E_POINTER,
        Graphics::Gdi::{BITMAPFILEHEADER, BITMAPINFOHEADER, BI_RGB},
        Media::MediaFoundation::{
            IMFActivate, IMFAttributes, IMFMediaSource, IMFSample, IMFSourceReader, MFCreateAttributes, MFCreateMediaType, MFCreateSourceReaderFromMediaSource, MFCreateSourceReaderFromURL, MFEnumDeviceSources, MFMediaType_Video, MFShutdown, MFStartup, MFVideoFormat_RGB32, MFSTARTUP_FULL, MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID, MF_MT_MAJOR_TYPE, MF_MT_SUBTYPE, MF_SOURCE_READER_FIRST_VIDEO_STREAM, MF_VERSION
        }, System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
    },
};

pub unsafe fn mf_startup() -> anyhow::Result<()> {
    Ok(MFStartup(MF_VERSION, MFSTARTUP_FULL)?)
}

pub unsafe fn save_bitmap_file(width: i32, height: i32, bytes: &[u8]) -> anyhow::Result<()> {
    let mut file = std::fs::File::create("image.bmp")?;

    let mut writer = BufWriter::new(file);

    let header = BITMAPFILEHEADER {
        bfType: 0x4D42, // 'BM'
        bfSize: (std::mem::size_of::<BITMAPFILEHEADER>()
            + std::mem::size_of::<BITMAPINFOHEADER>()
            + (width * height * 4) as usize) as u32,
        bfReserved1: 0,
        bfReserved2: 0,
        bfOffBits: (std::mem::size_of::<BITMAPFILEHEADER>()
            + std::mem::size_of::<BITMAPINFOHEADER>()) as u32,
    };

    let info_header = BITMAPINFOHEADER {
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width as i32,
        biHeight: -(height as i32), // Top-down DIB
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB.0,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };

    writer.write_all(unsafe {
        std::slice::from_raw_parts(
            (&header as *const BITMAPFILEHEADER) as *const u8,
            std::mem::size_of::<BITMAPFILEHEADER>(),
        )
    })?;
    writer.write_all(unsafe {
        std::slice::from_raw_parts(
            (&info_header as *const BITMAPINFOHEADER) as *const u8,
            std::mem::size_of::<BITMAPINFOHEADER>(),
        )
    })?;
    writer.write_all(bytes)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(Some(ptr::null_mut()), COINIT_MULTITHREADED).unwrap();
        
        mf_startup()?;

        let mut attributes: Option<IMFAttributes> = None;

        MFCreateAttributes(&mut attributes, 1)?;

        let attributes = attributes.unwrap();

        attributes.SetGUID(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID)?;

        let mut devices: *mut Option<IMFActivate> = ptr::null_mut();
        let mut count = 0;

        MFEnumDeviceSources(&attributes, &mut devices, &mut count)?;

        let device = &*devices.offset(0);

        let reader = MFCreateSourceReaderFromMediaSource(&device.clone().unwrap().ActivateObject::<IMFMediaSource>()?, &attributes)?;

        let media_type = MFCreateMediaType()?;

        media_type.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;

        media_type.SetGUID(&MF_MT_SUBTYPE, &MFVideoFormat_RGB32)?;

        reader.SetCurrentMediaType(
                    MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32,
                    None,
                    &media_type,
                )?;

        let mut stream_index = 0;
        let mut flags: u32 = 0;
        let mut timestamp: i64 = 0;
        let mut sample: Option<IMFSample> = None;

        //Read sample
        reader.ReadSample(
            MF_SOURCE_READER_FIRST_VIDEO_STREAM.0 as u32,
            0,
            Some(&mut stream_index),
            Some(&mut flags),
            Some(&mut timestamp),
            Some(&mut sample),
        )?;

        let buffer = sample.unwrap().ConvertToContiguousBuffer()?;

        let mut data_ptr: *mut u8 = std::ptr::null_mut();
        let mut max_length = 0;
        let mut current_length = 0;

        buffer.Lock(
            &mut data_ptr,
            Some(&mut max_length),
            Some(&mut current_length),
        )?;

        let data = std::slice::from_raw_parts(data_ptr, current_length as usize);

        save_bitmap_file(640, 480, data)?;

        buffer.Unlock()?;

        MFShutdown()?;
    }

    Ok(())
}
