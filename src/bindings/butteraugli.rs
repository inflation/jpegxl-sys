use std::ffi::c_void;

use crate::common::*;

// Opaque type
#[repr(C)]
pub struct JxlButteraugliApi {
    _unused: [u8; 0],
}

// Opaque type
#[repr(C)]
pub struct JxlButteraugliResult {
    _unused: [u8; 0],
}

extern "C" {
    pub fn JxlButteraugliResultDestroy(result: *mut JxlButteraugliResult);

    pub fn JxlButteraugliApiCreate(
        memory_manager: *const JxlMemoryManager,
    ) -> *mut JxlButteraugliApi;

    pub fn JxlButteraugliApiSetParallelRunner(
        api: *mut JxlButteraugliApi,
        parallel_runner: JxlParallelRunner,
        parallel_runner_opaque: *mut c_void,
    );

    pub fn JxlButteraugliApiSetHFAsymmetry(api: *mut JxlButteraugliApi, v: f32);

    pub fn JxlButteraugliApiSetIntensityTarget(api: *mut JxlButteraugliApi, v: f32);

    pub fn JxlButteraugliApiDestroy(api: *mut JxlButteraugliApi);

    pub fn JxlButteraugliCompute(
        api: *const JxlButteraugliApi,
        xsize: u32,
        ysize: u32,
        pixel_format_orig: *const JxlPixelFormat,
        buffer_orig: *const c_void,
        size_orig: usize,
        pixel_format_dist: *const JxlPixelFormat,
        buffer_dist: *const c_void,
        size_dist: usize,
    ) -> *mut JxlButteraugliResult;

    pub fn JxlButteraugliResultGetMaxDistance(result: *const JxlButteraugliResult) -> f32;

    pub fn JxlButteraugliResultGetDistance(result: *const JxlButteraugliResult, pnorm: f32) -> f32;

    pub fn JxlButteraugliResultGetDistmap(
        result: *const JxlButteraugliResult,
        buffer: *const *const f32,
        row_stride: *mut u32,
    );
}
