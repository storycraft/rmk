use usb_device::device::UsbVidPid;
use usbd_hid::hid_class::{
    HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig,
};

use crate::macros::define_matrix;

pub const VID_PID: UsbVidPid = UsbVidPid(0xFEED, 0x0A0C);

pub const MANUFACTURER: &str = "pancake.sh";
pub const PRODUCT: &str = "Storyboard Keyboard";
pub const VERSION: u16 = 0x68;
pub const SERIAL_NUMBER: &str = "0";

pub const POLL_MS: u8 = 1;

pub const SETTINGS: HidClassSettings = HidClassSettings {
    subclass: HidSubClass::Boot,
    protocol: HidProtocol::Keyboard,
    config: ProtocolModeConfig::ForceReport,
    locale: HidCountryCode::NotSupported,
};

define_matrix!(
    pub struct Matrix {
        pub inputs: (
            PB0,
            PB1,
            PB2,
            PB3,
            PB4,
            PB5,
            PB6,
            PB7,
            PC6,
            PC7,
            PD0,
            PD1,
            PD2,
            PD3,
            PD4,
            PD5,
        ),
        pub outputs: (PF0, PF1, PF4, PF5, PF6),
    }
);
