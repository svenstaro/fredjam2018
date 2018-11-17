use rodio::source::Source;
use std::fs::File;
use std::io::{BufReader, Cursor};
use rodio::{self, Sink};
use std::time::Duration;
use std::thread;
use std::sync::mpsc::Receiver;
use rodio::dynamic_mixer::mixer;
use rodio::source::Zero;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AudioEvent {
    Effect(Effect),
    Music,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Effect {
    BeepLong,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Music {
    Intro,
}

impl AudioEvent {
    fn data_cursor(&self) -> Cursor<&'static [u8]> {
        Cursor::new(self.data())
    }

    fn data(&self) -> &'static[u8] {
        match self {
            AudioEvent::Effect(effect) => match effect {
                Effect::BeepLong => &include_bytes!("../assets/wav/beep_long.wav")[..],
            },
            _ => panic!("No audio file for this event")
        }
    }
}

pub fn start(recv: Receiver<AudioEvent>) {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let (effect_mixer_controller, effect_mixer):
        (std::sync::Arc<rodio::dynamic_mixer::DynamicMixerController<i16>>,
         rodio::dynamic_mixer::DynamicMixer<i16>) = mixer(2, 44800);

    sink.append(effect_mixer);
    effect_mixer_controller.add(Zero::new(2, 44800));
    loop {
        let message = recv.recv().unwrap();
        let source = rodio::Decoder::new(message.data_cursor()).unwrap();
        effect_mixer_controller.add(source);
    }

}
