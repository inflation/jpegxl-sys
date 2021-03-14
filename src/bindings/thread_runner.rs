use std::ffi::c_void;

use crate::common::*;

extern "C" {
    pub fn JxlThreadParallelRunner(
        runner_opaque: *mut c_void,
        jpegxl_opaque: *mut c_void,
        init: JxlParallelRunInit,
        func: JxlParallelRunFunction,
        start_range: u32,
        end_range: u32,
    ) -> JxlParallelRetCode;

    pub fn JxlThreadParallelRunnerCreate(
        memory_manager: *const JxlMemoryManager,
        num_worker_threads: usize,
    ) -> *mut c_void;

    pub fn JxlThreadParallelRunnerDestroy(runner_opaque: *mut c_void);

    pub fn JxlThreadParallelRunnerDefaultNumWorkerThreads() -> usize;
}
