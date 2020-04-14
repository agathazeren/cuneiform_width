use rusttype::{Font, Scale};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static FONT: &[u8] = include_bytes!("font.ttf");

fn main() {
    let mut output_file =
        File::create(Path::new(&env::var_os("OUT_DIR").unwrap()).join("data.rs")).unwrap();
    output_file.set_len(0).unwrap(); //Is there a better way to clear the file?

    write!(output_file, "{}", FILE_PREFIX).unwrap();

    let font = Font::from_bytes(FONT).unwrap();
    for c in (0x12000..=0x123FF)
        .chain(0x12400..=0x1247F)
        .chain(0x12480..=0x1254F)
        .map(std::char::from_u32)
    {
        if let Some(c) = c {
            dbg!(c);
            let scaled = font.glyph(c).scaled(Scale { x: 2.0, y: 2.0 });
            let h_metrics = scaled.h_metrics();
            eprintln!(
                "advance_width: {}\tleft_side_bearing: {}",
                h_metrics.advance_width, h_metrics.left_side_bearing
            );
            let size = (h_metrics.advance_width * 2.0).ceil() as u8;
            dbg!(size);
            writeln!(output_file, "    Some({}),", size).unwrap();
        } else {
            writeln!(output_file, "    None,").unwrap();
        }
    }

    println!("units_per_em {}", font.units_per_em());
    write!(output_file, "{}", FILE_SUFFIX).unwrap();
}

static FILE_PREFIX: &str = r#"
pub static DATA: [Option<u8>;0x54F + 1] = [
"#;

static FILE_SUFFIX: &str = r#"
];
"#;
