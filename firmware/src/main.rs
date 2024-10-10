#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod config;
mod context;
mod key;
mod macros;

use config::Matrix;
use panic_halt as _;

use core::{cell::UnsafeCell, mem::MaybeUninit};

use arduino_hal::{
    hal::pins,
    pac::{PLL, USB_DEVICE},
    Peripherals,
};
use atmega_usbd::UsbBus;
use avr_device::{asm::sleep, interrupt};
use context::Context;
use usb_device::bus::UsbBusAllocator;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    init_pll(&dp.PLL);

    // Initialize global context
    Context::init(Context::new(init_usb_bus(dp.USB_DEVICE, dp.PLL)));

    let _matrix = Matrix::new(pins!(dp));

    unsafe {
        interrupt::enable();
    }
    loop {
        sleep();
    }
}

fn init_pll(pll: &PLL) {
    // Configure PLL interface
    pll.pllcsr.write(|w| w.pindiv().set_bit());
    // 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
    pll.pllfrq
        .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());
    // Enable PLL
    pll.pllcsr.modify(|_, w| w.plle().set_bit());

    // Check PLL lock
    while pll.pllcsr.read().plock().bit_is_clear() {}
}

type PllUsbBus = UsbBus<PLL>;

fn init_usb_bus(usb: USB_DEVICE, pll: PLL) -> &'static UsbBusAllocator<PllUsbBus> {
    struct Store(UnsafeCell<MaybeUninit<UsbBusAllocator<PllUsbBus>>>);

    unsafe impl Send for Store {}
    unsafe impl Sync for Store {}

    static GLOBAL: Store = Store(UnsafeCell::new(MaybeUninit::uninit()));

    unsafe { &mut *GLOBAL.0.get() }.write(UsbBus::with_suspend_notifier(usb, pll))
}

#[interrupt(atmega32u4)]
fn USB_GEN() {
    Context::interrupt_poll()
}

#[interrupt(atmega32u4)]
fn USB_COM() {
    Context::interrupt_poll()
}
