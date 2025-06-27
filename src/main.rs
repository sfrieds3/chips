use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    thread,
    time::Duration,
};

use minifb::Key;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Params {
    input: PathBuf,

    #[structopt(default_value = "64")]
    height: usize,

    #[structopt(default_value = "32")]
    width: usize,
}

fn main() {
    let params = Params::from_args();
    println!("{params:?}");
    let mut buffer = vec![0; params.width * params.height];
    let mut window = minifb::Window::new(
        "CHIPS",
        params.width,
        params.height,
        minifb::WindowOptions {
            scale: minifb::Scale::X32,
            ..Default::default()
        },
    )
    .unwrap();
    window.set_target_fps(60);

    let _bin: Vec<_> = BufReader::new(File::open(params.input).unwrap())
        .bytes()
        .collect::<Result<_, _>>()
        .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0;
        }

        window.set_title("CHIPS");
        window
            .update_with_buffer(&buffer, params.width, params.height)
            .unwrap();
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
