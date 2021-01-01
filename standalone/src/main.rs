use baseview::{Parent, WindowScalePolicy};
use iced_baseview::{settings, Runner, Settings};
use simplelog::{ConfigBuilder, SimpleLogger, LevelFilter};

use vst::host::Host;
use vst::plugin::{Category, Plugin, Info, CanDo, HostCallback, PluginParameters};

use aa::constants::*;

use aa::AA;

fn main() {
    SimpleLogger::init(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_to_local(true)
            .build()
    ).unwrap();

    let aa = Box::new(AA::new(HostCallback::default())) as Box<dyn Plugin>;

    println!("{:?}", aa.get_info());
}
