use core::{cell::UnsafeCell, mem::MaybeUninit};

use impl_opaque::opaque;
use usb_device::{
    bus::{UsbBus, UsbBusAllocator},
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

use crate::{PllUsbBus, CONFIG};

#[opaque(
    as pub,
    bus: &'static UsbBusAllocator<PllUsbBus>,
)]
impl Context {
    field!(
        hid: HIDClass<'static, PllUsbBus> = HIDClass::new(
            bus,
            KeyboardReport::desc(),
            CONFIG.hid.poll_ms)
    );

    field!(device: UsbDevice<'static, PllUsbBus> = build_from_config(bus).build());

    fn poll(&mut self) {
        if self.device.poll(&mut [&mut self.hid]) {
            let mut report_buf = [0u8; 1];
            if self.hid.pull_raw_output(&mut report_buf).is_ok() {
                report_buf[0] = 2;
            }
        }
    }
}

fn build_from_config<B: UsbBus>(bus: &UsbBusAllocator<B>) -> UsbDeviceBuilder<B> {
    let mut builder =
        UsbDeviceBuilder::new(bus, UsbVidPid(CONFIG.descriptor.vid, CONFIG.descriptor.pid));

    if let Some(manufacturer) = CONFIG.descriptor.manufacturer {
        builder = builder.manufacturer(manufacturer);
    }

    if let Some(product) = CONFIG.descriptor.product {
        builder = builder.product(product);
    }

    if let Some(serial_number) = CONFIG.descriptor.serial_number {
        builder = builder.serial_number(serial_number);
    }

    if let Some(version) = CONFIG.descriptor.version {
        builder = builder.device_release(version);
    }

    builder
}

const _: () = {
    struct Store(UnsafeCell<MaybeUninit<Context>>);

    unsafe impl Send for Store {}
    unsafe impl Sync for Store {}

    static GLOBAL: Store = Store(UnsafeCell::new(MaybeUninit::uninit()));

    impl Context {
        pub fn init(ctx: Context) {
            unsafe { &mut *GLOBAL.0.get() }.write(ctx);
        }

        pub fn get_mut() -> &'static mut Self {
            unsafe { (*GLOBAL.0.get()).assume_init_mut() }
        }

        #[inline(always)]
        pub fn interrupt_poll() {
            Self::get_mut().poll();
        }
    }
};
