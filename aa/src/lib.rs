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

pub struct AA {
    editor: Option<Gui>,
}

impl Default for AA {
    fn default() -> Self {
        Self::new(HostCallback::default())     
    }
}

impl AA {
 /// MIDI keyboard support

    pub fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.key_off(data[1]),
            144 => self.key_on(data[1], data[2]),
            _   => ()
        }
    }

    fn key_on(&mut self, pitch: u8, velocity: u8) {
       
    }

    fn key_off(&mut self, pitch: u8) {
       
    }
}

impl Plugin for AA {
    fn new(host: HostCallback) -> Self {
        info!("Plugin::new()");
        
        let editor = Gui::new();
        
        Self {
            editor: Some(editor),    
        }
    }
    
    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        // Lots todo
        info!("Plugin::process()");
    }

    fn get_info(&self) -> Info {
        Info {
            name: PLUGIN_NAME.to_string(),
            vendor: PLUGIN_VENDOR.to_string(),
            version: crate_version_to_vst_format(crate_version!()),
            unique_id: PLUGIN_UNIQUE_ID,
            category: Category::Synth,
            inputs: 2,  // 
            outputs: 2, // FIXUP
            presets: 0 as i32,
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
            log_folder.join(format!("{}.log", PLUGIN_NAME))
        ).unwrap();

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