use std::{ffi::c_void, os::raw::c_int};

#[repr(C)]
pub enum JxlDataType {
    Float = 0,
    Boolean,
    Uint8,
    Uint16,
    Uint32,
}

#[repr(C)]
pub enum JxlEndianness {
    Native = 0,
    Little,
    Big,
}

#[repr(C)]
pub struct JxlPixelFormat {
    pub num_channels: u32,
    pub data_type: JxlDataType,
    pub endianness: JxlEndianness,
    pub align: usize,
}

#[repr(C)]
pub enum JxlColorSpace {
    Rgb = 0,
    Gray,
    Xyb,
    Unknown,
}

#[repr(C)]
pub enum JxlWhitePoint {
    D65 = 1,
    Custom = 2,
    E = 10,
    Dci = 11,
}

#[repr(C)]
pub enum JxlPrimaries {
    SRgb = 1,
    Custom = 2,
    Rec2100 = 9,
    P3 = 11,
}

#[repr(C)]
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
pub enum JxlRenderingIntent {
    Perceptual = 0,
    Relative,
    Saturation,
    Absolute,
}

#[repr(C)]
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

pub type JpegxlAllocFunc =
    Option<unsafe extern "C" fn(opaque: *mut c_void, size: usize) -> *mut c_void>;
pub type JpegxlFreeFunc = Option<unsafe extern "C" fn(opaque: *mut c_void, address: *mut c_void)>;

#[repr(C)]
pub struct JxlMemoryManager {
    pub opaque: *mut c_void,
    pub alloc: JpegxlAllocFunc,
    pub free: JpegxlFreeFunc,
}

pub type JxlParallelRetCode = c_int;

pub type JxlParallelRunInit = Option<
    unsafe extern "C" fn(jpegxl_opaque: *mut c_void, num_threads: usize) -> JxlParallelRetCode,
>;

pub type JxlParallelRunFunction =
    Option<unsafe extern "C" fn(jpegxl_opaque: *mut c_void, value: u32, thread_id: usize)>;

pub type JxlParallelRunner = ::std::option::Option<
    unsafe extern "C" fn(
        runner_opaque: *mut c_void,
        jpegxl_opaque: *mut c_void,
        init: JxlParallelRunInit,
        func: JxlParallelRunFunction,
        start_range: u32,
        end_range: u32,
    ) -> JxlParallelRetCode,
>;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_jxl_pixel_format() {
        assert_eq!(
            std::mem::size_of::<JxlPixelFormat>(),
            24usize,
            concat!("Size of: ", stringify!(JxlPixelFormat))
        );
        assert_eq!(
            std::mem::align_of::<JxlPixelFormat>(),
            8usize,
            concat!("Alignment of ", stringify!(JxlPixelFormat))
        );
        assert_eq!(
            unsafe { &(*(std::ptr::null::<JxlPixelFormat>())).num_channels as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlPixelFormat),
                "::",
                stringify!(num_channels)
            )
        );
        assert_eq!(
            unsafe { &(*(std::ptr::null::<JxlPixelFormat>())).data_type as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlPixelFormat),
                "::",
                stringify!(data_type)
            )
        );
        assert_eq!(
            unsafe { &(*(std::ptr::null::<JxlPixelFormat>())).endianness as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlPixelFormat),
                "::",
                stringify!(endianness)
            )
        );
        assert_eq!(
            unsafe { &(*(std::ptr::null::<JxlPixelFormat>())).align as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlPixelFormat),
                "::",
                stringify!(align)
            )
        );
    }

    #[test]
    fn test_layout_jxl_color_encoding() {
        assert_eq!(
            ::std::mem::size_of::<JxlColorEncoding>(),
            104usize,
            concat!("Size of: ", stringify!(JxlColorEncoding))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlColorEncoding>(),
            8usize,
            concat!("Alignment of ", stringify!(JxlColorEncoding))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlColorEncoding>())).color_space as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(color_space)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlColorEncoding>())).white_point as *const _ as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(white_point)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlColorEncoding>())).white_point_xy as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(white_point_xy)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlColorEncoding>())).primaries as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(primaries)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlColorEncoding>())).primaries_red_xy as *const _ as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(primaries_red_xy)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlColorEncoding>())).primaries_green_xy as *const _ as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(primaries_green_xy)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlColorEncoding>())).primaries_blue_xy as *const _ as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(primaries_blue_xy)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlColorEncoding>())).transfer_function as *const _ as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(transfer_function)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlColorEncoding>())).gamma as *const _ as usize },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(gamma)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlColorEncoding>())).rendering_intent as *const _ as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlColorEncoding),
                "::",
                stringify!(rendering_intent)
            )
        );
    }

    #[test]
    fn test_layout_jxl_inverse_opsin_matrix() {
        assert_eq!(
            ::std::mem::size_of::<JxlInverseOpsinMatrix>(),
            60usize,
            concat!("Size of: ", stringify!(JxlInverseOpsinMatrix))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlInverseOpsinMatrix>(),
            4usize,
            concat!("Alignment of ", stringify!(JxlInverseOpsinMatrix))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlInverseOpsinMatrix>())).opsin_inv_matrix as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlInverseOpsinMatrix),
                "::",
                stringify!(opsin_inv_matrix)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlInverseOpsinMatrix>())).opsin_biases as *const _ as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlInverseOpsinMatrix),
                "::",
                stringify!(opsin_biases)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlInverseOpsinMatrix>())).quant_biases as *const _ as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlInverseOpsinMatrix),
                "::",
                stringify!(quant_biases)
            )
        );
    }

    #[test]
    fn test_layout_jxl_preview_header() {
        assert_eq!(
            ::std::mem::size_of::<JxlPreviewHeader>(),
            8usize,
            concat!("Size of: ", stringify!(JxlPreviewHeader))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlPreviewHeader>(),
            4usize,
            concat!("Alignment of ", stringify!(JxlPreviewHeader))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlPreviewHeader>())).xsize as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlPreviewHeader),
                "::",
                stringify!(xsize)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlPreviewHeader>())).ysize as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlPreviewHeader),
                "::",
                stringify!(ysize)
            )
        );
    }

    #[test]
    fn test_layout_jxl_animation_header() {
        assert_eq!(
            ::std::mem::size_of::<JxlAnimationHeader>(),
            16usize,
            concat!("Size of: ", stringify!(JxlAnimationHeader))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlAnimationHeader>(),
            4usize,
            concat!("Alignment of ", stringify!(JxlAnimationHeader))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlAnimationHeader>())).tps_numerator as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlAnimationHeader),
                "::",
                stringify!(tps_numerator)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlAnimationHeader>())).tps_denominator as *const _ as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlAnimationHeader),
                "::",
                stringify!(tps_denominator)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlAnimationHeader>())).num_loops as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlAnimationHeader),
                "::",
                stringify!(num_loops)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlAnimationHeader>())).have_timecodes as *const _ as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlAnimationHeader),
                "::",
                stringify!(have_timecodes)
            )
        );
    }

    #[test]
    fn test_layout_jxl_basic_info() {
        assert_eq!(
            ::std::mem::size_of::<JxlBasicInfo>(),
            96usize,
            concat!("Size of: ", stringify!(JxlBasicInfo))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlBasicInfo>(),
            4usize,
            concat!("Alignment of ", stringify!(JxlBasicInfo))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).have_container as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(have_container)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).xsize as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(xsize)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).ysize as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(ysize)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).bits_per_sample as *const _ as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(bits_per_sample)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).exponent_bits_per_sample as *const _
                    as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(exponent_bits_per_sample)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).intensity_target as *const _ as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(intensity_target)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).min_nits as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(min_nits)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).relative_to_max_display as *const _
                    as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(relative_to_max_display)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).linear_below as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(linear_below)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).uses_original_profile as *const _ as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(uses_original_profile)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).have_preview as *const _ as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(have_preview)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).have_animation as *const _ as usize },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(have_animation)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).orientation as *const _ as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(orientation)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).num_color_channels as *const _ as usize
            },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(num_color_channels)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).num_extra_channels as *const _ as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(num_extra_channels)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).alpha_bits as *const _ as usize },
            60usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(alpha_bits)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).alpha_exponent_bits as *const _ as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(alpha_exponent_bits)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlBasicInfo>())).alpha_premultiplied as *const _ as usize
            },
            68usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(alpha_premultiplied)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).preview as *const _ as usize },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(preview)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlBasicInfo>())).animation as *const _ as usize },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlBasicInfo),
                "::",
                stringify!(animation)
            )
        );
    }

    #[test]
    fn test_layout_jxl_extra_channel_info() {
        assert_eq!(
            ::std::mem::size_of::<JxlExtraChannelInfo>(),
            44usize,
            concat!("Size of: ", stringify!(JxlExtraChannelInfo))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlExtraChannelInfo>(),
            4usize,
            concat!("Alignment of ", stringify!(JxlExtraChannelInfo))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlExtraChannelInfo>())).type_ as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlExtraChannelInfo),
                "::",
                stringify!(type_)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlExtraChannelInfo>())).bits_per_sample as *const _ as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlExtraChannelInfo),
                "::",
                stringify!(bits_per_sample)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlExtraChannelInfo>())).exponent_bits_per_sample as *const _
                    as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlExtraChannelInfo),
                "::",
                stringify!(exponent_bits_per_sample)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlExtraChannelInfo>())).dim_shift as *const _ as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlExtraChannelInfo),
                "::",
                stringify!(dim_shift)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlExtraChannelInfo>())).name_length as *const _ as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlExtraChannelInfo),
                "::",
                stringify!(name_length)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlExtraChannelInfo>())).alpha_associated as *const _
                    as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlExtraChannelInfo),
                "::",
                stringify!(alpha_associated)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlExtraChannelInfo>())).spot_color as *const _ as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlExtraChannelInfo),
                "::",
                stringify!(spot_color)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlExtraChannelInfo>())).cfa_channel as *const _ as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlExtraChannelInfo),
                "::",
                stringify!(cfa_channel)
            )
        );
    }

    #[test]
    fn test_layout_jxl_header_extensions() {
        assert_eq!(
            ::std::mem::size_of::<JxlHeaderExtensions>(),
            8usize,
            concat!("Size of: ", stringify!(JxlHeaderExtensions))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlHeaderExtensions>(),
            8usize,
            concat!("Alignment of ", stringify!(JxlHeaderExtensions))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<JxlHeaderExtensions>())).extensions as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlHeaderExtensions),
                "::",
                stringify!(extensions)
            )
        );
    }

    #[test]
    fn test_layout_jxl_frame_header() {
        assert_eq!(
            ::std::mem::size_of::<JxlFrameHeader>(),
            16usize,
            concat!("Size of: ", stringify!(JxlFrameHeader))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlFrameHeader>(),
            4usize,
            concat!("Alignment of ", stringify!(JxlFrameHeader))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlFrameHeader>())).duration as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlFrameHeader),
                "::",
                stringify!(duration)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlFrameHeader>())).timecode as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlFrameHeader),
                "::",
                stringify!(timecode)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlFrameHeader>())).name_length as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlFrameHeader),
                "::",
                stringify!(name_length)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlFrameHeader>())).is_last as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlFrameHeader),
                "::",
                stringify!(is_last)
            )
        );
    }

    #[test]
    fn test_layout_jxl_memory_manager_struct() {
        assert_eq!(
            ::std::mem::size_of::<JxlMemoryManager>(),
            24usize,
            concat!("Size of: ", stringify!(JxlMemoryManagerStruct))
        );
        assert_eq!(
            ::std::mem::align_of::<JxlMemoryManager>(),
            8usize,
            concat!("Alignment of ", stringify!(JxlMemoryManagerStruct))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlMemoryManager>())).opaque as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlMemoryManagerStruct),
                "::",
                stringify!(opaque)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlMemoryManager>())).alloc as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlMemoryManagerStruct),
                "::",
                stringify!(alloc)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<JxlMemoryManager>())).free as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(JxlMemoryManagerStruct),
                "::",
                stringify!(free)
            )
        );
    }
}
