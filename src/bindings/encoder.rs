use std::{ffi::c_void, os::raw::c_int};

use crate::common::*;

// Opaque type
#[repr(C)]
pub struct JxlEncoder {
    _unused: [u8; 0],
}

// Opaque type
#[repr(C)]
pub struct JxlEncoderOptions {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum JxlEncoderStatus {
    Success = 0,
    Error = 1,
    NeedMoreOutput = 2,
    NotSupported = 3,
}

extern "C" {
    pub fn JxlEncoderVersion() -> u32;

    pub fn JxlEncoderCreate(memory_manager: *const JxlMemoryManager) -> *mut JxlEncoder;

    pub fn JxlEncoderReset(enc: *mut JxlEncoder);

    pub fn JxlEncoderDestroy(enc: *mut JxlEncoder);

    pub fn JxlEncoderSetParallelRunner(
        enc: *mut JxlEncoder,
        parallel_runner: JxlParallelRunner,
        parallel_runner_opaque: *mut c_void,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderProcessOutput(
        enc: *mut JxlEncoder,
        next_out: *mut *mut u8,
        avail_out: *mut usize,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderAddJPEGFrame(
        options: *const JxlEncoderOptions,
        buffer: *const u8,
        size: usize,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderAddImageFrame(
        options: *const JxlEncoderOptions,
        pixel_format: *const JxlPixelFormat,
        buffer: *const c_void,
        size: usize,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderCloseInput(enc: *mut JxlEncoder);

    pub fn JxlEncoderSetColorEncoding(
        enc: *mut JxlEncoder,
        color: *const JxlColorEncoding,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderSetBasicInfo(
        enc: *mut JxlEncoder,
        info: *const JxlBasicInfo,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderStoreJPEGMetadata(
        enc: *mut JxlEncoder,
        store_jpeg_metadata: bool,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderUseContainer(enc: *mut JxlEncoder, use_container: bool) -> JxlEncoderStatus;

    pub fn JxlEncoderOptionsSetLossless(
        options: *mut JxlEncoderOptions,
        lossless: bool,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderOptionsSetEffort(
        options: *mut JxlEncoderOptions,
        effort: c_int,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderOptionsSetDistance(
        options: *mut JxlEncoderOptions,
        distance: f32,
    ) -> JxlEncoderStatus;

    pub fn JxlEncoderOptionsCreate(
        enc: *mut JxlEncoder,
        source: *const JxlEncoderOptions,
    ) -> *mut JxlEncoderOptions;

    pub fn JxlColorEncodingSetToSRGB(color_encoding: *mut JxlColorEncoding, is_gray: bool);

    pub fn JxlColorEncodingSetToLinearSRGB(color_encoding: *mut JxlColorEncoding, is_gray: bool);
}
