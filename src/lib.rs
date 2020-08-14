#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/cppbindings.rs"));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bindings() -> Result<(), std::io::Error> {
        unsafe {
            let one = JpegxlDecoderVersion();
            assert_eq!(one, 1);
        }

        Ok(())
    }
}
