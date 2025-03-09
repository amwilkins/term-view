use clap::Parser;
use image::{imageops, GenericImageView};

mod output;

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
    let (image_width, image_height) = image.dimensions();

    if args.width.as_ref() == None {
        // resize image to terminal width
        let tsize = termsize::get().ok_or("error");
        match tsize {
            Ok(size) => {
                let i = size.cols as u32;
                if image_width > i {
                    image = image.resize(i, image_height, imageops::FilterType::Gaussian);
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
    }

    output::output_image(&image);
}
