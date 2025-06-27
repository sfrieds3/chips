mod chips;

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    thread,
    time::Duration,
};

use minifb::Key;
use structopt::StructOpt;

use crate::chips::Chips;

#[derive(Debug, StructOpt)]
struct Params {
    input: PathBuf,

    #[structopt(default_value = "64")]
    width: usize,

    #[structopt(default_value = "32")]
    height: usize,
}

fn main() {
    let mut chips = Chips::default();
    let params = Params::from_args();
    let mut buffer = vec![255; params.width * params.height];
    println!("{params:?}");
    let mut window = minifb::Window::new(
        "CHIPS",
        params.width,
        params.height,
        minifb::WindowOptions {
            scale: minifb::Scale::X16,
            ..Default::default()
        },
    )
    .unwrap();
    window.set_target_fps(60);

    let bin: Vec<_> = BufReader::new(File::open(params.input).unwrap())
        .bytes()
        .collect::<Result<_, _>>()
        .unwrap();

    chips.load_bin(&bin);

    buffer = chips.memory_as_pixels(params.width, params.height);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.set_title("CHIPS");
        window
            .update_with_buffer(&buffer, params.width, params.height)
            .unwrap();
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
