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
    use std::ptr;

    #[test]
    fn test_bindings_version() {
        unsafe {
            assert_eq!(JxlDecoderVersion(), 2000);
        }
    }

    unsafe fn decode(decoder: *mut JxlDecoder) {
        let mut status: u32;

        // Stop after getting the basic info and decoding the image
        status = JxlDecoderSubscribeEvents(
            decoder,
            (JxlDecoderStatus_JXL_DEC_BASIC_INFO | JxlDecoderStatus_JXL_DEC_FULL_IMAGE) as i32,
        );
        assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);

        // Read everything in memory
        let sample = std::fs::read("test/sample.jxl").unwrap();
        let signature = JxlSignatureCheck(sample.as_ptr(), 2);
        assert_eq!(signature, JxlSignature_JXL_SIG_CODESTREAM);

        let next_in = &mut sample.as_ptr();
        let mut avail_in = sample.len() as u64;

        let pixel_format = JxlPixelFormat {
            num_channels: 3,
            data_type: JxlDataType_JXL_TYPE_UINT8,
            endianness: JxlEndianness_JXL_NATIVE_ENDIAN,
            align: 0,
        };

        let mut basic_info = JxlBasicInfo::new_uninit();
        let mut buffer: Vec<u8> = Vec::new();
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
                    assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);
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
                    assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);

                    buffer = Vec::with_capacity(size as usize);
                    buffer.set_len(size as usize);
                    status = JxlDecoderSetImageOutBuffer(
                        decoder,
                        &pixel_format,
                        buffer.as_mut_ptr() as *mut std::ffi::c_void,
                        size,
                    );
                    assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);
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
            let decoder = JxlDecoderCreate(ptr::null()); // Default memory manager
            assert!(!decoder.is_null());

            // Simple single thread runner
            let status = JxlDecoderSetParallelRunner(decoder, Option::None, ptr::null_mut());
            assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);

            decode(decoder);
        }
    }

    #[test]
    fn test_bindings_thread_pool() {
        unsafe {
            let runner = JxlThreadParallelRunnerCreate(
                std::ptr::null(),
                JxlThreadParallelRunnerDefaultNumWorkerThreads(),
            );

            let decoder = JxlDecoderCreate(ptr::null()); // Default memory manager
            assert!(!decoder.is_null());

            // Parallel multithread runner
            let status =
                JxlDecoderSetParallelRunner(decoder, Some(JxlThreadParallelRunner), runner);
            assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);

            decode(decoder);
        }
    }
}
