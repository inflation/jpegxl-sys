use jpegxl_sys::*;

fn main() {
    unsafe {
        let one = JpegxlDecoderVersion();
        println!("{}", one);
    }
}
