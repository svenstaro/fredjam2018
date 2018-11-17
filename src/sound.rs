use rodio::dynamic_mixer::mixer;
use rodio::source::Zero;
use rodio::Decoder;
use rodio::Sample;
use rodio::Source;
use rodio::{self, Sink};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use strum::IntoEnumIterator;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AudioEvent {
    Effect(Effect),
    Track(Track),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Effect {
    BeepLong,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, Hash)]
pub enum Track {
    Intro,
}

struct MusicPlayback {
    track: Arc<Mutex<Track>>,
    inner_source: Box<Source<Item = i16> + Send>,
    data_cursors: HashMap<Track, Decoder<Cursor<&'static [u8]>>>,
    current_cursor: Track,
}

struct MusicPlaybackController {
    track: Arc<Mutex<Track>>,
}

impl MusicPlaybackController {
    fn set_track(&mut self, track: Track) {
        *self.track.lock().unwrap() = track;
        // let audio = AudioEvent::Track(track);
        // self.inner_source = Box::new(Decoder::new(audio.data_cursor()).unwrap().repeat_infinite())
    }
}

impl MusicPlayback {
    fn create() -> (Self, MusicPlaybackController) {
        let track = Arc::new(Mutex::new(Track::Intro));
        let mut cursors = HashMap::new();
        for track in Track::iter() {
            cursors.insert(
                track,
                rodio::Decoder::new(AudioEvent::Track(track).data_cursor()).unwrap(),
            );
        }
        (
            MusicPlayback {
                track: track.clone(),
                inner_source: Box::new(Zero::new(2, 44800)),
                data_cursors: cursors,
                current_cursor: Track::Intro,
            },
            MusicPlaybackController { track: track },
        )
    }
}

impl Source for MusicPlayback {
    fn current_frame_len(&self) -> Option<usize> {
        let decoder = self.data_cursors.get(&*self.track.lock().unwrap()).unwrap();
        decoder.current_frame_len()
    }

    fn channels(&self) -> u16 {
        let decoder = self.data_cursors.get(&*self.track.lock().unwrap()).unwrap();
        decoder.channels()
    }

    fn sample_rate(&self) -> u32 {
        let decoder = self.data_cursors.get(&*self.track.lock().unwrap()).unwrap();
        decoder.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for MusicPlayback {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        let decoder_option = self.data_cursors.get_mut(&*self.track.lock().unwrap());
        decoder_option.and_then(|dec| dec.next())
    }
}

impl AudioEvent {
    fn data_cursor(&self) -> Cursor<&'static [u8]> {
        Cursor::new(self.data())
    }

    fn data(&self) -> &'static [u8] {
        match self {
            AudioEvent::Effect(effect) => match effect {
                Effect::BeepLong => &include_bytes!("../assets/wav/beep_long.wav")[..],
            },
            AudioEvent::Track(track) => match track {
                Track::Intro => &include_bytes!("../assets/music/intro.mp3")[..],
            },
        }
    }
}

pub fn start(recv: Receiver<AudioEvent>) {
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let (effect_mixer_controller, effect_mixer): (
        std::sync::Arc<rodio::dynamic_mixer::DynamicMixerController<i16>>,
        rodio::dynamic_mixer::DynamicMixer<i16>,
    ) = mixer(2, 44800);
    let (music, mut music_controller) = MusicPlayback::create();

    sink.append(effect_mixer);
    effect_mixer_controller.add(Zero::new(2, 44800));
    effect_mixer_controller.add(music);
    loop {
        let message = recv.recv().unwrap();
        match message {
            AudioEvent::Effect(_) => {
                let source = rodio::Decoder::new(message.data_cursor()).unwrap();
                effect_mixer_controller.add(source);
            }
            AudioEvent::Track(ref track) => music_controller.set_track(*track),
        }
    }
}
