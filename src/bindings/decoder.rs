use std::{
    ffi::c_void,
    os::raw::{c_char, c_int},
};

use crate::common::*;

// Opaque type
#[repr(C)]
pub struct JxlDecoder {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum JxlDecoderStatus {
    Success = 0,
    Error = 1,
    NeedMoreInput = 2,
    NeedPreviewOutBuffer = 3,
    NeedDcOutBuffer = 4,
    NeedImageOutBuffer = 5,
    JpegNeedMoreOutput = 6,
    BasicInfo = 0x40,
    Extensions = 0x80,
    ColorEncoding = 0x100,
    PreviewImage = 0x200,
    Frame = 0x400,
    DcImage = 0x800,
    FullImage = 0x1000,
    JpegReconstruction = 0x2000,
}

impl std::ops::BitOr for JxlDecoderStatus {
    type Output = i32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as i32 | rhs as i32
    }
}

extern "C" {
    pub fn JxlSignatureCheck(buf: *const u8, len: usize) -> JxlSignature;
    pub fn JxlDecoderCreate(memory_manager: *const JxlMemoryManager) -> *mut JxlDecoder;
    pub fn JxlDecoderReset(dec: *mut JxlDecoder);
    pub fn JxlDecoderDestroy(dec: *mut JxlDecoder);
    pub fn JxlDecoderVersion() -> u32;
    pub fn JxlDecoderDefaultPixelFormat(
        dec: *const JxlDecoder,
        format: *mut JxlPixelFormat,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderSetParallelRunner(
        dec: *mut JxlDecoder,
        parallel_runner: JxlParallelRunner,
        parallel_runner_opaque: *mut c_void,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderSizeHintBasicInfo(dec: *const JxlDecoder) -> usize;

    pub fn JxlDecoderSubscribeEvents(
        dec: *mut JxlDecoder,
        events_wanted: c_int,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderSetKeepOrientation(
        dec: *mut JxlDecoder,
        keep_orientation: bool,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderProcessInput(dec: *mut JxlDecoder) -> JxlDecoderStatus;

    pub fn JxlDecoderSetInput(
        dec: *mut JxlDecoder,
        data: *const u8,
        size: usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderReleaseInput(dec: *mut JxlDecoder) -> usize;

    pub fn JxlDecoderGetBasicInfo(
        dec: *const JxlDecoder,
        info: *mut JxlBasicInfo,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderGetExtraChannelInfo(
        dec: *const JxlDecoder,
        index: usize,
        info: *mut JxlExtraChannelInfo,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderGetExtraChannelName(
        dec: *const JxlDecoder,
        index: usize,
        name: *mut c_char,
        size: usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderGetColorAsEncodedProfile(
        dec: *const JxlDecoder,
        format: *const JxlPixelFormat,
        target: JxlColorProfileTarget,
        color_encoding: *mut JxlColorEncoding,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderGetICCProfileSize(
        dec: *const JxlDecoder,
        format: *const JxlPixelFormat,
        target: JxlColorProfileTarget,
        size: *mut usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderGetColorAsICCProfile(
        dec: *const JxlDecoder,
        format: *const JxlPixelFormat,
        target: JxlColorProfileTarget,
        icc_profile: *mut u8,
        size: usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderPreviewOutBufferSize(
        dec: *const JxlDecoder,
        format: *const JxlPixelFormat,
        size: *mut usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderSetPreviewOutBuffer(
        dec: *mut JxlDecoder,
        format: *const JxlPixelFormat,
        buffer: *mut c_void,
        size: usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderGetFrameHeader(
        dec: *const JxlDecoder,
        header: *mut JxlFrameHeader,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderGetFrameName(
        dec: *const JxlDecoder,
        name: *mut c_char,
        size: usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderDCOutBufferSize(
        dec: *const JxlDecoder,
        format: *const JxlPixelFormat,
        size: *mut usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderSetDCOutBuffer(
        dec: *mut JxlDecoder,
        format: *const JxlPixelFormat,
        buffer: *mut c_void,
        size: usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderImageOutBufferSize(
        dec: *const JxlDecoder,
        format: *const JxlPixelFormat,
        size: *mut usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderSetJPEGBuffer(
        dec: *mut JxlDecoder,
        data: *mut u8,
        size: usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderReleaseJPEGBuffer(dec: *mut JxlDecoder) -> usize;

    pub fn JxlDecoderSetImageOutBuffer(
        dec: *mut JxlDecoder,
        format: *const JxlPixelFormat,
        buffer: *mut c_void,
        size: usize,
    ) -> JxlDecoderStatus;

    pub fn JxlDecoderFlushImage(dec: *mut JxlDecoder) -> JxlDecoderStatus;
}
