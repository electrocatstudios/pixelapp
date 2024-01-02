
pub fn color_to_hex_string(r: u8, g: u8, b: u8) -> String {
    let r = if r < 16 {
        format!("0{:x}", r)
    } else {
        format!("{:x}", r)
    };
    let g = if g < 16 {
        format!("0{:x}", g)
    } else {
        format!("{:x}", g)
    };
    let b = if b < 16 {
        format!("0{:x}", b)
    } else {
        format!("{:x}", b)
    };
    
    format!("{}{}{}", r, g, b)
}

#[cfg(test)]
mod utils_tests {
    use super::color_to_hex_string;

    #[test]
    fn test_convert_rgb_to_hex_string() {
        assert_eq!(color_to_hex_string(255, 255, 255), "ffffff");
        assert_eq!(color_to_hex_string(255, 254, 253), "fffefd");
        assert_eq!(color_to_hex_string(1, 2, 3), "010203");
        assert_eq!(color_to_hex_string(10,10,10), "0a0a0a");
        assert_eq!(color_to_hex_string(11,12,13), "0b0c0d");
    }
}