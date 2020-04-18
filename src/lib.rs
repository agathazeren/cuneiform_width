extern crate unicode_width;

use unicode_width::UnicodeWidthChar;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

pub fn cuneiform_width(c: char) -> u8 {
    if let Some(n) = get_dat(c) {
        n
    } else {
        UnicodeWidthChar::width(c).unwrap_or(1) as u8
    }
}

fn get_dat(c: char) -> Option<u8> {
    if c as u32 < 0x12000 {
        return None
    }
    let idx = c as u32 - 0x12000;
    if idx > 0x54F {
        None
    } else {
        DATA[idx as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::cuneiform_width;
    #[test]
    fn one_test_case() {
        let c = '𒀿';
        assert_eq!(3, cuneiform_width(c));
    }
}
