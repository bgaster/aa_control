#[cfg(feature = "logging")]
#[macro_use]
extern crate log;

pub mod constants;
pub mod gui;

use vst::api::{Supported, Events};
use vst::editor::Editor;
use vst::event::Event;
use vst::plugin::{Category, Plugin, Info, CanDo, HostCallback, PluginParameters};
use vst::host::Host;

use constants::*;
use gui::Gui;

use std::f64::consts::PI;
pub const TAU: f64 = PI * 2.0;

fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}


pub struct AA {
    sample_rate: f64,
    time: f64,
    note_duration: f64,
    note: Option<u8>,

    editor: Option<Gui>,
}

impl Default for AA {
    fn default() -> Self {
        Self::new(HostCallback::default())     
    }
}

impl AA {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    /// MIDI keyboard support

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1], data[2]),
            _   => ()
        }
    }

    fn note_on(&mut self, note: u8, velocity: u8) {
        self.note_duration = 0.0;
        self.note = Some(note)
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None
        }
    }

    #[cfg(feature = "standalone")]
    pub fn get_gui(&mut self) -> Option<Box<Gui>> {
        if let Some(editor) = self.editor.take(){
            Some(Box::new(editor) as Box<Gui>)
        } else {
            None
        }
    }
}

impl Plugin for AA {
    
    fn new(host: HostCallback) -> Self {
        info!("Plugin::new()");
        
        let editor = Gui::new();
        
        Self {
            sample_rate: 44100.0,
            note_duration: 0.0,
            time: 0.0,
            note: None,

            editor: Some(editor),    
        }
    }        

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            if let Event::Midi(ev) = event {
                self.process_midi_event(ev.data);
            } 
        }
    }

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        info!("Plugin::process()");

        let samples = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();
        let per_sample = self.time_per_sample();
        let mut output_sample;
        for sample_idx in 0..samples {
            let time = self.time;
            let note_duration = self.note_duration;
            if let Some(current_note) = self.note {
                let signal = (time * midi_pitch_to_freq(current_note) * TAU).sin();

                // Apply a quick envelope to the attack of the signal to avoid popping.
                let attack = 0.5;
                let alpha = if note_duration < attack {
                    note_duration / attack
                } else {
                    1.0
                };

                output_sample = (signal * alpha) as f32;

                self.time += per_sample;
                self.note_duration += per_sample;
            } else {
                output_sample = 0.0;
            }
            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                buff[sample_idx] = output_sample;
            }
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: PLUGIN_NAME.to_string(),
            vendor: PLUGIN_VENDOR.to_string(),
            version: crate_version_to_vst_format(crate_version!()),
            unique_id: PLUGIN_UNIQUE_ID,
            category: Category::Synth,
            inputs: PLUGIN_NUMBER_INPUTS,  
            outputs: PLUGIN_NUMBER_OUTPUTS, 
            presets: 0 as i32, // TODO: add support
            parameters: 0 as i32,
            initial_delay: 0,
            preset_chunks: false,
            f64_precision: false,
            ..Info::default()
        }
    }

    #[cfg(feature = "logging")]
	fn init(&mut self) {
        let log_folder = dirs::home_dir().unwrap().join("tmp");

        let _ = ::std::fs::create_dir(log_folder.clone());

		let log_file = ::std::fs::File::create(
            log_folder.join(format!("{}.log", PLUGIN_NAME))).unwrap();

        let log_config = simplelog::ConfigBuilder::new()
            .set_time_to_local(true)
            .build();

		let _ = simplelog::WriteLogger::init(
            simplelog::LevelFilter::Info,
            log_config,
            log_file
        );

        log_panics::init();

		info!("init");
    }

    fn set_sample_rate(&mut self, rate: f32) {
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent | CanDo::ReceiveTimeInfo
            | CanDo::SendEvents | CanDo::ReceiveEvents => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
    
    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        if let Some(editor) = self.editor.take(){
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION").to_string()
    };
}

fn crate_version_to_vst_format(crate_version: String) -> i32 {
    format!("{:0<4}", crate_version.replace(".", ""))
        .parse()
        .expect("convert crate version to i32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::zero_prefixed_literal)]
    #[test]
    fn test_crate_version_to_vst_format(){
        assert_eq!(crate_version_to_vst_format("1".to_string()), 1000);
        assert_eq!(crate_version_to_vst_format("0.1".to_string()), 0100);
        assert_eq!(crate_version_to_vst_format("0.0.2".to_string()), 0020);
        assert_eq!(crate_version_to_vst_format("0.5.2".to_string()), 0520);
        assert_eq!(crate_version_to_vst_format("1.0.1".to_string()), 1010);
    }
}