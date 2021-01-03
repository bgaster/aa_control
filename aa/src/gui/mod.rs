use baseview::{Parent, Size, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::{settings, Runner, Settings };

use vst::editor::Editor;
use raw_window_handle::RawWindowHandle;

use crate::constants::{PLUGIN_NAME, GUI_WIDTH, GUI_HEIGHT};

pub mod interface;

use interface::AAIcedApplication;

pub struct Gui {
    opened: bool,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            opened: false,
        }
    }

    #[cfg(feature = "standalone")]
    pub fn app_run(&mut self) {
        let parent = Parent::None;

        let settings = Settings {
            window: WindowOpenOptions {
                parent,
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
                title: PLUGIN_NAME.to_string(),
            },
            flags: (),
        };

        let (_, opt_runner) = Runner::<AAIcedApplication>::open(settings);

        opt_runner.unwrap().app_run_blocking();
    }
}

impl Editor for Gui {
    fn size(&self) -> (i32, i32) {
        (GUI_WIDTH as i32, GUI_HEIGHT as i32)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut ::core::ffi::c_void) -> bool {
        if self.opened {
            return false;
        }

        let parent = Parent::WithParent(
            raw_window_handle_from_parent(parent)
        );

        let settings = Settings {
            window: WindowOpenOptions {
                parent,
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
                title: PLUGIN_NAME.to_string(),
            },
            flags: (),
        };

        Runner::<AAIcedApplication>::open(settings);

        false
    }

    fn close(&mut self) {
        self.opened = false;
    }

    fn is_open(&mut self) -> bool {
        self.opened
    }
}

#[cfg(target_os = "macos")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::macos::MacOSHandle;

    RawWindowHandle::MacOS(MacOSHandle {
        ns_view: parent,
        ..MacOSHandle::empty()
    })
}


#[cfg(target_os = "windows")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::windows::WindowsHandle;

    RawWindowHandle::Windows(WindowsHandle {
        hwnd: parent,
        ..WindowsHandle::empty()
    })
}


#[cfg(target_os = "linux")]
fn raw_window_handle_from_parent(
    parent: *mut ::std::ffi::c_void
) -> RawWindowHandle {
    use raw_window_handle::unix::XcbHandle;

    RawWindowHandle::Xcb(XcbHandle {
        window: parent as u32,
        ..XcbHandle::empty()
    })
}