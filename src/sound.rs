use rodio::dynamic_mixer::mixer;
use rodio::source::{Repeat, Zero};
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

static FADE_SAMPLES: u32 = 44_100;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AudioEvent {
    Effect(Effect),
    Track(Track),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Effect {
    BeepLong,
    EnemyAttack,
    PlayerAttack,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, Hash)]
pub enum Track {
    Intro,
    Compliactions,
}

struct MusicPlayback {
    track: Arc<Mutex<Track>>,
    data_cursors: HashMap<Track, Repeat<Decoder<Cursor<&'static [u8]>>>>,
    current_track: Track,
    last_track: Option<Track>,
    samples_since_check: u32,
    samples_since_switch: u32,
}

struct MusicPlaybackController {
    track: Arc<Mutex<Track>>,
}

impl MusicPlaybackController {
    fn set_track(&mut self, track: Track) {
        *self.track.lock().unwrap() = track;
    }
}

impl MusicPlayback {
    fn create() -> (Self, MusicPlaybackController) {
        let track = Arc::new(Mutex::new(Track::Intro));
        let mut cursors = HashMap::new();
        for track in Track::iter() {
            cursors.insert(
                track,
                rodio::Decoder::new(AudioEvent::Track(track).data_cursor())
                    .unwrap()
                    .repeat_infinite(),
            );
        }
        (
            MusicPlayback {
                track: track.clone(),
                data_cursors: cursors,
                current_track: Track::Intro,
                last_track: None,
                samples_since_check: 0,
                samples_since_switch: 0,
            },
            MusicPlaybackController { track: track },
        )
    }
}

impl Source for MusicPlayback {
    fn current_frame_len(&self) -> Option<usize> {
        let decoder = self.data_cursors.get(&self.current_track).unwrap();
        decoder.current_frame_len()
    }

    fn channels(&self) -> u16 {
        let decoder = self.data_cursors.get(&self.current_track).unwrap();
        if decoder.channels() != 2 {
            panic!("Channels!")
        }
        decoder.channels()
    }

    fn sample_rate(&self) -> u32 {
        let decoder = self.data_cursors.get(&self.current_track).unwrap();
        decoder.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for MusicPlayback {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        self.samples_since_check += 1;
        self.samples_since_switch += 1;
        if self.samples_since_switch >= FADE_SAMPLES {
            self.last_track = None
        }
        if self.samples_since_check > 10_000 {
            let new_track = *self.track.lock().unwrap();
            if new_track != self.current_track {
                self.last_track = Some(self.current_track);
                self.current_track = new_track;
                self.samples_since_switch = 0;
            }
            self.samples_since_check = 0;
        };
        let last_track_sample = if let Some(last_track) = self.last_track {
            let decoder_option = self.data_cursors.get_mut(&last_track);
            decoder_option.and_then(|dec| dec.next())
        } else {
            None
        };

        let decoder_option = self.data_cursors.get_mut(&self.current_track);
        // println!("{}, {}", self.samples_since_switch , FADE_SAMPLES);
        decoder_option.and_then(|dec| dec.next()).map(|val| {
            if self.last_track == Some(self.current_track) {
                return val;
            };
            match last_track_sample {
                None => val,
                Some(last_track_sample) => i16::lerp(
                    val,
                    last_track_sample,
                    self.samples_since_switch,
                    FADE_SAMPLES,
                ),
            }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.data_cursors
                .get(&self.current_track)
                .unwrap()
                .size_hint()
                .0,
            None,
        )
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
                Effect::EnemyAttack => &include_bytes!("../assets/wav/enemy_attack.wav")[..],
                Effect::PlayerAttack => &include_bytes!("../assets/wav/player_attack.wav")[..],
            },
            AudioEvent::Track(track) => match track {
                Track::Intro => &include_bytes!("../assets/music/intro.mp3")[..],
                Track::Compliactions => &include_bytes!("../assets/music/complications.mp3")[..],
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
    ) = mixer(2, 44_100);
    let (music, mut music_controller) = MusicPlayback::create();

    sink.append(effect_mixer);
    effect_mixer_controller.add(Zero::new(2, 44_100));
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
