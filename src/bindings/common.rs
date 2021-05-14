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

use std::{ffi::c_void, os::raw::c_int};

#[repr(C)]
#[derive(Clone)]
pub enum JxlDataType {
    Float = 0,
    Boolean,
    Uint8,
    Uint16,
    Uint32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum JxlEndianness {
    Native = 0,
    Little,
    Big,
}

#[repr(C)]
#[derive(Clone)]
pub struct JxlPixelFormat {
    pub num_channels: u32,
    pub data_type: JxlDataType,
    pub endianness: JxlEndianness,
    pub align: usize,
}

#[repr(C)]
#[derive(Clone)]
pub enum JxlColorSpace {
    Rgb = 0,
    Gray,
    Xyb,
    Unknown,
}

#[repr(C)]
#[derive(Clone)]
pub enum JxlWhitePoint {
    D65 = 1,
    Custom = 2,
    E = 10,
    Dci = 11,
}

#[repr(C)]
#[derive(Clone)]
pub enum JxlPrimaries {
    SRgb = 1,
    Custom = 2,
    Rec2100 = 9,
    P3 = 11,
}

#[repr(C)]
#[derive(Clone)]
pub enum JxlTransferFunction {
    Rec709 = 1,
    Unknown = 2,
    Linear = 8,
    SRgb = 13,
    Pq = 16,
    Dci = 17,
    Hlg = 18,
    Gamma = 65535,
}

#[repr(C)]
#[derive(Clone)]
pub enum JxlRenderingIntent {
    Perceptual = 0,
    Relative,
    Saturation,
    Absolute,
}

#[repr(C)]
#[derive(Clone)]
pub struct JxlColorEncoding {
    pub color_space: JxlColorSpace,
    pub white_point: JxlWhitePoint,
    pub white_point_xy: [f64; 2usize],
    pub primaries: JxlPrimaries,
    pub primaries_red_xy: [f64; 2usize],
    pub primaries_green_xy: [f64; 2usize],
    pub primaries_blue_xy: [f64; 2usize],
    pub transfer_function: JxlTransferFunction,
    pub gamma: f64,
    pub rendering_intent: JxlRenderingIntent,
}

#[repr(C)]
pub struct JxlInverseOpsinMatrix {
    pub opsin_inv_matrix: [[f32; 3usize]; 3usize],
    pub opsin_biases: [f32; 3usize],
    pub quant_biases: [f32; 3usize],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum JxlOrientation {
    Identity = 1,
    FlipHorizontal = 2,
    Rotate180 = 3,
    FlipVertical = 4,
    Transpose = 5,
    Rotate90Cw = 6,
    AntiTranspose = 7,
    Rotate90Ccw = 8,
}

#[repr(C)]
pub enum JxlExtraChannelType {
    Alpha,
    Depth,
    SpotColor,
    SelectionMask,
    Black,
    Cfa,
    Thermal,
    Reserved0,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Unknown,
    Optional,
}

#[repr(C)]
pub struct JxlPreviewHeader {
    pub xsize: u32,
    pub ysize: u32,
}

#[repr(C)]
pub struct JxlAnimationHeader {
    pub tps_numerator: u32,
    pub tps_denominator: u32,
    pub num_loops: u32,
    pub have_timecodes: bool,
}

#[repr(C)]
pub struct JxlBasicInfo {
    pub have_container: i32,
    pub xsize: u32,
    pub ysize: u32,
    pub bits_per_sample: u32,
    pub exponent_bits_per_sample: u32,
    pub intensity_target: f32,
    pub min_nits: f32,
    pub relative_to_max_display: i32,
    pub linear_below: f32,
    pub uses_original_profile: i32,
    pub have_preview: i32,
    pub have_animation: i32,
    pub orientation: JxlOrientation,
    pub num_color_channels: u32,
    pub num_extra_channels: u32,
    pub alpha_bits: u32,
    pub alpha_exponent_bits: u32,
    pub alpha_premultiplied: i32,
    pub preview: JxlPreviewHeader,
    pub animation: JxlAnimationHeader,
}

#[repr(C)]
pub struct JxlExtraChannelInfo {
    pub type_: JxlExtraChannelType,
    pub bits_per_sample: u32,
    pub exponent_bits_per_sample: u32,
    pub dim_shift: u32,
    pub name_length: u32,
    pub alpha_associated: bool,
    pub spot_color: [f32; 4usize],
    pub cfa_channel: u32,
}

#[repr(C)]
pub struct JxlHeaderExtensions {
    pub extensions: u64,
}

#[repr(C)]
pub struct JxlFrameHeader {
    pub duration: u32,
    pub timecode: u32,
    pub name_length: u32,
    pub is_last: bool,
}

pub type JpegxlAllocFunc = unsafe extern "C" fn(opaque: *mut c_void, size: usize) -> *mut c_void;
pub type JpegxlFreeFunc = unsafe extern "C" fn(opaque: *mut c_void, address: *mut c_void);

#[repr(C)]
pub struct JxlMemoryManager {
    pub opaque: *mut c_void,
    pub alloc: JpegxlAllocFunc,
    pub free: JpegxlFreeFunc,
}

pub type JxlParallelRetCode = c_int;

pub type JxlParallelRunInit =
    unsafe extern "C" fn(jpegxl_opaque: *mut c_void, num_threads: usize) -> JxlParallelRetCode;

pub type JxlParallelRunFunction =
    unsafe extern "C" fn(jpegxl_opaque: *mut c_void, value: u32, thread_id: usize);

pub type JxlParallelRunner = unsafe extern "C" fn(
    runner_opaque: *mut c_void,
    jpegxl_opaque: *mut c_void,
    init: JxlParallelRunInit,
    func: JxlParallelRunFunction,
    start_range: u32,
    end_range: u32,
) -> JxlParallelRetCode;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum JxlSignature {
    NotEnoughBytes = 0,
    Invalid = 1,
    Codestream = 2,
    Container = 3,
}

#[repr(C)]
pub enum JxlColorProfileTarget {
    Original,
    Data,
}
