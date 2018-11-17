use rodio::source::Source;
use std::fs::File;
use std::io::BufReader;
use rodio::{self, Sink};
use std::time::Duration;
use std::thread;


pub fn start() {
    let test = rodio::output_devices();
    print_all_devices();

    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    let file = File::open("assets/wav/player_attack.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let source = rodio::source::SineWave::new(440);
    sink.append(source)
    thread::spawn(|| {
    });
}

