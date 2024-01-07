
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

pub fn hex_string_to_color(hex: String) -> (u8, u8, u8) {
    if hex.len() != 6 {
        return (0,0,0);
    }

    let r: u8 = u8::from_str_radix(&hex[..2], 16).unwrap();
    let g: u8 = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b: u8 = u8::from_str_radix(&hex[4..6], 16).unwrap();

    (r,g,b)
}

#[cfg(test)]
mod utils_tests {
    use super::{color_to_hex_string,hex_string_to_color};

    #[test]
    fn test_convert_rgb_to_hex_string() {
        assert_eq!(color_to_hex_string(255, 255, 255), "ffffff");
        assert_eq!(color_to_hex_string(255, 254, 253), "fffefd");
        assert_eq!(color_to_hex_string(1, 2, 3), "010203");
        assert_eq!(color_to_hex_string(10,10,10), "0a0a0a");
        assert_eq!(color_to_hex_string(11,12,13), "0b0c0d");
    }

    #[test]
    fn test_convert_hex_string_rgb() {
        assert_eq!(hex_string_to_color("000000".to_string()), (0,0,0));
        assert_eq!(hex_string_to_color("010203".to_string()), (1,2,3));
        assert_eq!(hex_string_to_color("fffefd".to_string()), (255,254,253));

        // Check failure conditions
        assert_eq!(hex_string_to_color("fff".to_string()), (0,0,0));
    }
}