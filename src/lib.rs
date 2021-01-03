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

trait_impl!(
    NewUninit,
    [
        JxlBasicInfo,
        JxlExtraChannelInfo,
        JxlFrameHeader,
        JxlPreviewHeader,
        JxlColorEncoding,
        JxlPixelFormat
    ]
);

/// Convenient function to just return a block of memory.
/// You need to assign `basic_info.assume_init()` to use as a Rust struct after passing as a pointer.
/// # Examples:
/// ```ignore
/// # use jpegxl_sys::*;
/// # let decoder = JxlDecoderCreate(std::ptr::null());
/// let mut basic_info = JxlBasicInfo::new_uninit();
/// JxlDecoderGetBasicInfo(decoder, basic_info.as_mut_ptr());
/// let basic_info = basic_info.assumu_init();
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
        let mut status: i32;

        // Stop after getting the basic info and decoding the image
        status = JxlDecoderSubscribeEvents(
            decoder,
            (JxlDecoderStatus_JXL_DEC_BASIC_INFO | JxlDecoderStatus_JXL_DEC_FULL_IMAGE)
                as i32,
        );
        assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);

        // Read everything in memory
        let sample = std::fs::read("test/sample.jxl").unwrap();
        let signature = JxlSignatureCheck(sample.as_ptr(), 2);
        assert_eq!(signature, JxlSignature_JXL_SIG_CODESTREAM);

        let next_in = &mut sample.as_ptr();
        let mut avail_in = sample.len() as u64;
        status = JxlDecoderProcessInput(decoder, next_in, &mut avail_in);
        assert_eq!(
            status, JxlDecoderStatus_JXL_DEC_BASIC_INFO,
            "Read Basic Info"
        );

        // Get the basic info
        let mut basic_info = JxlBasicInfo::new_uninit();
        status = JxlDecoderGetBasicInfo(decoder, basic_info.as_mut_ptr());
        assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);
        let basic_info = basic_info.assume_init();
        assert_eq!(basic_info.bits_per_sample, 8, "Bits per sample");
        assert_eq!(basic_info.xsize, 1404, "Width");
        assert_eq!(basic_info.ysize, 936, "Height");

        status = JxlDecoderProcessInput(decoder, next_in, &mut avail_in);
        assert_eq!(
            status, JxlDecoderStatus_JXL_DEC_NEED_IMAGE_OUT_BUFFER,
            "Give Image Out Buffer"
        );

        // Get the buffer size
        let mut size: u64 = 0;
        let pixel_format = JxlPixelFormat {
            data_type: JxlDataType_JXL_TYPE_UINT8,
            num_channels: 3,
            endianness: JxlEndianness_JXL_NATIVE_ENDIAN,
            align: 0,
        };
        status = JxlDecoderImageOutBufferSize(decoder, &pixel_format, &mut size);
        assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);

        // Create a buffer to hold decoded image
        let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
        buffer.set_len(size as usize);
        status = JxlDecoderSetImageOutBuffer(
            decoder,
            &pixel_format,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            size,
        );
        assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);

        // Read what left of the image
        status = JxlDecoderProcessInput(decoder, next_in, &mut avail_in);
        assert_eq!(
            status, JxlDecoderStatus_JXL_DEC_FULL_IMAGE,
            "Read Whole Image"
        );
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

            JxlDecoderDestroy(decoder);
        }
    }

    #[test]
    #[cfg(feature = "build-jpegxl")]
    fn test_bindings_thread_pool() {
        unsafe {
            let runner = JxlThreadParallelRunnerCreate(std::ptr::null(), JxlThreadParallelRunnerDefaultNumWorkerThreads());

            let decoder = JxlDecoderCreate(ptr::null()); // Default memory manager
            assert!(!decoder.is_null());

            // Parallel multithread runner
            let status = JxlDecoderSetParallelRunner(decoder, Some(JxlThreadParallelRunner), runner);
            assert_eq!(status, JxlDecoderStatus_JXL_DEC_SUCCESS);

            // panic!("TODO: Figure out if there is a dead lock");
            decode(decoder);

            JxlThreadParallelRunnerDestroy(runner);
            JxlDecoderDestroy(decoder);
        }
    }
}
