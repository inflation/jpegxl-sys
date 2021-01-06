/*
This file is part of jpegxl-sys.

jpegxl-sys is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

jpegxl-sys is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with jpegxl-sys.  If not, see <https://www.gnu.org/licenses/>.
*/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

macro_rules! trait_impl {
    ($x:ty, [$($struct_:ident ),*]) => {
        $(
            impl $x for $struct_ {}
        )*
    };
}

trait_impl!(NewUninit, [JxlBasicInfo]);

/// Convenient function to just return a block of memory.
/// You need to assign `basic_info.assume_init()` to use as a Rust struct after passing as a pointer.
/// # Examples:
/// ```ignore
/// # use jpegxl_sys::*;
/// # let decoder = JxlDecoderCreate(std::ptr::null());
/// let mut basic_info = JxlBasicInfo::new_uninit();
/// JxlDecoderGetBasicInfo(decoder, basic_info.as_mut_ptr());
/// let basic_info = basic_info.assume_init();
/// ```
pub trait NewUninit {
    #[inline]
    fn new_uninit() -> std::mem::MaybeUninit<Self>
    where
        Self: std::marker::Sized,
    {
        std::mem::MaybeUninit::<Self>::uninit()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{io::Write, ptr};

    use image::io::Reader as ImageReader;
    use image::ImageError;

    macro_rules! jxl_dec_assert {
        ($val:expr, $desc:expr) => {
            if $val != JxlDecoderStatus_JXL_DEC_SUCCESS {
                panic!("Decoder error by: {}, in {}", $val, $desc)
            }
        };
    }

    macro_rules! jxl_enc_assert {
        ($val:expr, $desc:expr) => {
            if $val != JxlEncoderStatus_JXL_ENC_SUCCESS {
                panic!("Encoder error by: {}, in {}", $val, $desc)
            }
        };
    }

    #[test]
    fn test_bindings_version() {
        unsafe {
            assert_eq!(JxlDecoderVersion(), 2000);
        }
    }

    unsafe fn decode(decoder: *mut JxlDecoder, sample: &[u8]) {
        let mut status: u32;

        // Stop after getting the basic info and decoding the image
        status = JxlDecoderSubscribeEvents(
            decoder,
            (JxlDecoderStatus_JXL_DEC_BASIC_INFO | JxlDecoderStatus_JXL_DEC_FULL_IMAGE) as i32,
        );
        jxl_dec_assert!(status, "Subscribe Events");

        // Read everything in memory
        let signature = JxlSignatureCheck(sample.as_ptr(), 2);
        assert_eq!(signature, JxlSignature_JXL_SIG_CODESTREAM, "Signature");

        let next_in = &mut sample.as_ptr();
        let mut avail_in = sample.len() as u64;

        let pixel_format = JxlPixelFormat {
            num_channels: 3,
            data_type: JxlDataType_JXL_TYPE_UINT8,
            endianness: JxlEndianness_JXL_NATIVE_ENDIAN,
            align: 0,
        };

        let mut basic_info = JxlBasicInfo::new_uninit();
        let mut buffer: Vec<f32> = Vec::new();
        let mut xsize = 0;
        let mut ysize = 0;

        loop {
            status = JxlDecoderProcessInput(decoder, next_in, &mut avail_in);

            match status {
                JxlDecoderStatus_JXL_DEC_ERROR => panic!("Decoder error!"),
                JxlDecoderStatus_JXL_DEC_NEED_MORE_INPUT => {
                    panic!("Error, already provided all input")
                }

                // Get the basic info
                JxlDecoderStatus_JXL_DEC_BASIC_INFO => {
                    status = JxlDecoderGetBasicInfo(decoder, basic_info.as_mut_ptr());
                    jxl_dec_assert!(status, "BasicInfo");
                    let basic_info = basic_info.assume_init();
                    xsize = basic_info.xsize;
                    ysize = basic_info.ysize;
                    assert_eq!(basic_info.bits_per_sample, 8, "Bits per sample");
                    assert_eq!(basic_info.xsize, 2122, "Width");
                    assert_eq!(basic_info.ysize, 1433, "Height");
                }

                // Get the output buffer
                JxlDecoderStatus_JXL_DEC_NEED_IMAGE_OUT_BUFFER => {
                    let mut size: u64 = 0;
                    status = JxlDecoderImageOutBufferSize(decoder, &pixel_format, &mut size);
                    jxl_dec_assert!(status, "BufferSize");

                    buffer.resize(size as usize, 0f32);
                    status = JxlDecoderSetImageOutBuffer(
                        decoder,
                        &pixel_format,
                        buffer.as_mut_ptr() as *mut std::ffi::c_void,
                        size,
                    );
                    jxl_dec_assert!(status, "SetBuffer");
                }

                JxlDecoderStatus_JXL_DEC_FULL_IMAGE => continue,
                JxlDecoderStatus_JXL_DEC_SUCCESS => {
                    assert_eq!(buffer.len(), (xsize * ysize * 3) as usize);
                    return;
                }
                _ => panic!("Unknown decoder status: {}", status),
            }
        }
    }

    #[test]
    fn test_bindings_decoding() {
        unsafe {
            let dec = JxlDecoderCreate(ptr::null()); // Default memory manager
            assert!(!dec.is_null());

            // Simple single thread runner
            let sample = std::fs::read("test/sample.jxl").unwrap();
            decode(dec, &sample);

            JxlDecoderDestroy(dec);
        }
    }

    #[test]
    fn test_bindings_thread_pool() {
        unsafe {
            let runner = JxlThreadParallelRunnerCreate(
                std::ptr::null(),
                JxlThreadParallelRunnerDefaultNumWorkerThreads(),
            );

            let dec = JxlDecoderCreate(ptr::null()); // Default memory manager
            assert!(!dec.is_null());

            // Parallel multithread runner
            let status = JxlDecoderSetParallelRunner(dec, Some(JxlThreadParallelRunner), runner);
            jxl_dec_assert!(status, "Set Parallel Runner");

            let sample = std::fs::read("test/sample.jxl").unwrap();
            decode(dec, &sample);

            JxlDecoderDestroy(dec);
            JxlThreadParallelRunnerDestroy(runner);
        }
    }

    fn encode(pixels: &[u8], xsize: usize, ysize: usize) -> Vec<u8> {
        unsafe {
            let enc = JxlEncoderCreate(std::ptr::null());

            let runner = JxlThreadParallelRunnerCreate(
                std::ptr::null(),
                JxlThreadParallelRunnerDefaultNumWorkerThreads(),
            );

            let mut status =
                JxlEncoderSetParallelRunner(enc, Some(JxlThreadParallelRunner), runner);
            jxl_enc_assert!(status, "Set Parallel Runner");

            let pixel_format = JxlPixelFormat {
                num_channels: 3,
                data_type: JxlDataType_JXL_TYPE_UINT8,
                endianness: JxlEndianness_JXL_NATIVE_ENDIAN,
                align: 0,
            };
            status = JxlEncoderSetDimensions(enc, xsize as u64, ysize as u64);
            jxl_enc_assert!(status, "Set Dimension");

            status = JxlEncoderAddImageFrame(
                JxlEncoderOptionsCreate(enc, std::ptr::null()),
                &pixel_format,
                pixels.as_ptr() as *mut std::ffi::c_void,
                pixels.len() as u64,
            );
            jxl_enc_assert!(status, "Add Image Frame");

            const CHUNK_SIZE: usize = 1024 * 512; // 512 KB is a good initial value
            let mut buffer = vec![0u8; CHUNK_SIZE];
            let mut next_out = buffer.as_mut_ptr();
            let mut avail_out = CHUNK_SIZE;

            loop {
                status = JxlEncoderProcessOutput(
                    enc,
                    &mut next_out as *mut *mut u8,
                    &mut (avail_out as u64) as *mut u64,
                );

                if status != JxlEncoderStatus_JXL_ENC_NEED_MORE_OUTPUT {
                    break;
                }

                let offset = next_out as usize - buffer.as_ptr() as usize;
                buffer.resize(buffer.len() * 2, 0);
                next_out = buffer.as_mut_ptr().add(offset);
                avail_out = buffer.len() - offset;
            }
            buffer.truncate(next_out as usize - buffer.as_ptr() as usize);
            jxl_enc_assert!(status, "Encoding");

            JxlEncoderDestroy(enc);
            JxlThreadParallelRunnerDestroy(runner);

            buffer
        }
    }

    #[test]
    fn test_bindings_encoding() {
        || -> Result<(), ImageError> {
            let img = ImageReader::open("test/sample.png")?.decode()?;
            let image_buffer = img.into_rgb8();

            let output = encode(
                image_buffer.as_raw(),
                image_buffer.width() as usize,
                image_buffer.height() as usize,
            );

            unsafe {
                let runner = JxlThreadParallelRunnerCreate(
                    std::ptr::null(),
                    JxlThreadParallelRunnerDefaultNumWorkerThreads(),
                );

                let dec = JxlDecoderCreate(ptr::null()); // Default memory manager
                assert!(!dec.is_null());

                let status =
                    JxlDecoderSetParallelRunner(dec, Some(JxlThreadParallelRunner), runner);
                jxl_dec_assert!(status, "Set Parallel Runner");

                decode(dec, &output);

                JxlDecoderDestroy(dec);
                JxlThreadParallelRunnerDestroy(runner);
            }

            Ok(())
        }()
        .unwrap();
    }
}
