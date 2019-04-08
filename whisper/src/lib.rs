#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin};

#[derive(Default)]
struct Whisper;

impl Plugin for Whisper {
    fn get_info(&self) -> Info {
        Info {
            name: "Whisper".to_string(),
            unique_id: 1337,
            inputs: 0,
            outputs: 2,
            category: Category::Synth
            ..Default::default()
        }
    }
}

plugin_main!(Whisper);