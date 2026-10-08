use std::env;
use std::fmt;
use std::fs;
use std::process::{exit, ExitCode};
mod raylib;

#[allow(nonstandard_style)]
#[derive(Debug)]
struct PPM_Image {
    width: u16,
    height: u16,
    pixels: Vec<u8>,
}

fn parse_ppm_image(ppm_file: &str) -> PPM_Image {
    let mut words = ppm_file.split_ascii_whitespace();
    let ppm_header = words.next().unwrap_or_else(|| {
        print_error_msg_and_exit("expected P3 header in the file");
    });
    if ppm_header != "P3" {
        print_error_msg_and_exit(format!(
            "malformed header. Expected P3 but found {ppm_header}"
        ));
    }
    let width = words
        .next()
        .unwrap_or_else(|| {
            print_error_msg_and_exit("expected width specification");
        })
        .parse()
        .unwrap();
    let height = words
        .next()
        .unwrap_or_else(|| {
            print_error_msg_and_exit("expected height specification");
        })
        .parse()
        .unwrap();
    let _max_color_value = words.next().unwrap_or_else(|| {
        print_error_msg_and_exit("expected maximum value for each color");
    });
    let pixels = words.map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    PPM_Image {
        width,
        height,
        pixels,
    }
}

fn ppm_image_to_rl_image(ppm_image: &mut PPM_Image) -> raylib::Image {
    raylib::Image {
        data: ppm_image.pixels.as_mut_ptr().cast(),
        width: ppm_image.width.into(),
        height: ppm_image.height.into(),
        mipmaps: 1,
        format: 4,
    }
}

fn print_error_msg_and_exit<T: fmt::Display>(msg: T) -> ! {
    eprintln!("ERROR: {}", msg);
    exit(69);
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        print_error_msg_and_exit("please provide a .ppm image to load as an arguement");
    }
    let filepath = &args[1];
    let ppm_file = fs::read_to_string(filepath).unwrap_or_else(|x| {
        print_error_msg_and_exit(format!("could not open `{filepath}`: {x}"));
    });

    let width = 800;
    let height = 800;
    let title = "PPV";
    let mut ppm_image = parse_ppm_image(&ppm_file);
    let rl_image = ppm_image_to_rl_image(&mut ppm_image);
    raylib::init_window(width, height, title);
    raylib::set_target_fps(60);
    let texture = raylib::load_texture_from_image(rl_image);
    let pos_x = (width - texture.width) / 2;
    let pos_y = (height - texture.height) / 2;
    while !raylib::window_should_close() {
        raylib::begin_drawing();
        raylib::clear_background(raylib::get_color(0x09090FFF));
        raylib::draw_texture(texture.clone(), pos_x, pos_y, raylib::get_color(0xFFFFFFFF));
        raylib::end_drawing();
    }
    raylib::close_window();

    ExitCode::SUCCESS
}
