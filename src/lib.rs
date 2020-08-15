#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/cppbindings.rs"));

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
        JpegxlBasicInfo,
        JpegxlExtraChannelInfo,
        JpegxlPreviewHeader,
        JpegxlColorProfileSource,
        JpegxlColorEncoding,
        JpegxlPixelFormat
    ]
);

/// Convinient function to just return a block of memory.
/// You need to assign `basic_info.assume_init()` to use as a Rust struct after passing as a pointer.
/// # Examples:
/// ```ignore
/// # use jpegxl_sys::*;
/// # let decoder = JpegxlDecoderCreate(std::ptr::null());
/// let mut basic_info = JpegxlBasicInfo::new_uninit();
/// JpegxlDecoderGetBasicInfo(decoder, basic_info.as_mut_ptr());
/// let basic_info = basic_info.assumu_init();
/// ```
trait NewUninit {
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
    fn test_bindings_version() -> Result<(), std::io::Error> {
        unsafe {
            assert_eq!(JpegxlDecoderVersion(), 1);
        }
        Ok(())
    }

    #[test]
    fn test_bindings_decoding() -> Result<(), std::io::Error> {
        unsafe {
            let decoder = JpegxlDecoderCreate(ptr::null()); // Default memory manager
            assert!(!decoder.is_null());

            // Simple single thread runner
            let mut status = JpegxlDecoderSetParallelRunner(decoder, Option::None, ptr::null_mut());
            assert_eq!(status, JpegxlDecoderStatus_JPEGXL_DEC_SUCCESS);

            // Stop after getting the basic info and decoding the image
            status = JpegxlDecoderSubscribeEvents(
                decoder,
                (JpegxlDecoderStatus_JPEGXL_DEC_BASIC_INFO
                    | JpegxlDecoderStatus_JPEGXL_DEC_FULL_IMAGE) as i32,
            );
            assert_eq!(status, JpegxlDecoderStatus_JPEGXL_DEC_SUCCESS);

            // Read everything in memory
            let sample = std::fs::read("test/sample.jxl").unwrap();
            let signature = JpegxlSignatureCheck(sample.as_ptr(), 2);
            assert_eq!(signature, JpegxlSignature_JPEGXL_SIG_VALID);

            let next_in = &mut sample.as_ptr();
            let mut avail_in = sample.len() as u64;
            status = JpegxlDecoderProcessInput(decoder, next_in, &mut avail_in);
            assert_eq!(
                status, JpegxlDecoderStatus_JPEGXL_DEC_BASIC_INFO,
                "Read Basic Info"
            );

            // Get the basic info
            let mut basic_info = JpegxlBasicInfo::new_uninit();
            status = JpegxlDecoderGetBasicInfo(decoder, basic_info.as_mut_ptr());
            assert_eq!(status, JpegxlDecoderStatus_JPEGXL_DEC_SUCCESS);
            let basic_info = basic_info.assume_init();
            assert_eq!(basic_info.bits_per_sample, 8, "Bits per sample");
            assert_eq!(basic_info.xsize, 2122, "Width");
            assert_eq!(basic_info.ysize, 1433, "Height");

            // Get the buffer size
            let mut size: u64 = 0;
            let pixel_format = JpegxlPixelFormat {
                data_type: JpegxlDataType_JPEGXL_TYPE_UINT8,
                num_channels: 3,
            };
            status = JpegxlDecoderImageOutBufferSize(decoder, &pixel_format, &mut size);
            assert_eq!(status, JpegxlDecoderStatus_JPEGXL_DEC_SUCCESS);

            // Create a buffer to hold decoded image
            let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
            buffer.set_len(size as usize);
            status = JpegxlDecoderSetImageOutBuffer(
                decoder,
                &pixel_format,
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                size,
            );
            assert_eq!(status, JpegxlDecoderStatus_JPEGXL_DEC_SUCCESS);

            // Read what left of the image
            status = JpegxlDecoderProcessInput(decoder, next_in, &mut avail_in);
            assert_eq!(
                status, JpegxlDecoderStatus_JPEGXL_DEC_FULL_IMAGE,
                "Read Whole Image"
            );

            // Cleanups
            JpegxlDecoderDestroy(decoder);
        }
        Ok(())
    }
}
