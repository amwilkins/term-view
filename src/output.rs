use image::DynamicImage;
use image::GenericImageView;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn output_image(image: &DynamicImage) {
    // upper and full block unicode character
    let chars = ["\u{2580}", "\u{2588}"];
    println!("Image size: {:?}", image.dimensions());

    // output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut c = ColorSpec::new();

    let mut count = 0;
    for p in image.pixels() {
        if (p.1 % 2) == 1 {
            continue;
        }
        count = count + 1;
        //foreground
        c.set_fg(Some(Color::Rgb(p.2[0], p.2[1], p.2[2])));

        // background
        if p.1 < image.dimensions().1 - 1 {
            let p_bg = image.get_pixel(p.0, p.1 + 1);
            c.set_bg(Some(Color::Rgb(p_bg[0], p_bg[1], p_bg[2])));
        }
        stdout.set_color(&c).unwrap();
        write!(&mut stdout, "{}", chars[0]).unwrap();

        // next line
        if count == image.dimensions().0 {
            c.clear();
            stdout.set_color(&c).unwrap();
            writeln!(&mut stdout, "").unwrap();
            count = 0;
        }
    }
}
