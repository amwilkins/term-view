use clap::Parser;
use image::{imageops, GenericImageView};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    #[arg(short, long)]
    image: std::path::PathBuf,

    #[arg(long)]
    height: Option<u32>,
    #[arg(short, long)]
    width: Option<u32>,
}

fn main() {
    let args = Args::parse();

    //image
    let mut image = image::open(args.image).unwrap();
    let (mut image_width, image_height) = image.dimensions();

    if args.width.as_ref() == None {
        // resize image to terminal width
        let tsize = termsize::get().ok_or("error");
        match tsize {
            Ok(size) => {
                let i = size.cols as u32;
                if image_width > i {
                    image = image.resize(i, image_height, imageops::FilterType::Nearest);
                    image_width = i;
                }
            }
            Err(_) => {}
        };
    } else {
        image = image.resize(
            args.width.unwrap(),
            image_height,
            imageops::FilterType::Nearest,
        );
        image_width = args.width.unwrap();
    }

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
        c.set_fg(Some(Color::Rgb(p.2[0], p.2[1], p.2[2])));

        if p.1 < image.dimensions().1 - 1 {
            let p_bg = image.get_pixel(p.0, p.1 + 1);
            c.set_bg(Some(Color::Rgb(p_bg[0], p_bg[1], p_bg[2])));
        }
        stdout.set_color(&c).unwrap();
        write!(&mut stdout, "{}", chars[0]).unwrap();

        // next line
        if count == image_width {
            c.clear();
            stdout.set_color(&c).unwrap();
            writeln!(&mut stdout, "").unwrap();
            count = 0;
        }
    }
}
