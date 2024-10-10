use core::{cell::UnsafeCell, mem::MaybeUninit};

use impl_opaque::opaque;
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder},
};
use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::HIDClass,
};

use crate::{
    config::{MANUFACTURER, POLL_MS, PRODUCT, SERIAL_NUMBER, SETTINGS, VERSION, VID_PID},
    PllUsbBus,
};

#[opaque(
    as pub,
    bus: &'static UsbBusAllocator<PllUsbBus>,
)]
impl Context {
    field!(
        device: UsbDevice<'static, PllUsbBus> = UsbDeviceBuilder::new(bus, VID_PID)
            .manufacturer(MANUFACTURER)
            .product(PRODUCT)
            .serial_number(SERIAL_NUMBER)
            .device_release(VERSION)
            .build()
    );

    field!(
        main: HIDClass<'static, PllUsbBus> = HIDClass::new_with_settings(
            bus,
            KeyboardReport::desc(),
            POLL_MS,
            SETTINGS
        )
    );

    fn poll(&mut self) {
        if self.device.poll(&mut [&mut self.main]) {
            let mut report_buf = [0u8; 1];
            if self.main.pull_raw_output(&mut report_buf).is_ok() {
                report_buf[0] = 2;
            }
        }
    }
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
            unsafe { (&mut *GLOBAL.0.get()).assume_init_mut() }
        }

        #[inline(always)]
        pub fn interrupt_poll() {
            Self::get_mut().poll();
        }
    }
};
