#![feature(prelude_import)]
#![no_main]
#![no_std]
#[prelude_import]
use core::prelude::rust_2024::*;
#[macro_use]
extern crate core;
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f303pac as pac;
#[doc(hidden)]
#[export_name = "main"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() {
    #[allow(static_mut_refs)] __cortex_m_rt_main()
}
fn __cortex_m_rt_main() -> ! {
    {
        use ::rtt_target::ChannelMode::NoBlockSkip;
        {
            let channels = {
                use core::mem::MaybeUninit;
                use core::ptr;
                use core::cell::Cell;
                use ::rtt_target::UpChannel;
                use ::rtt_target::DownChannel;
                use ::rtt_target::rtt::*;
                #[repr(C)]
                pub struct RttControlBlock {
                    header: RttHeader,
                    up_channels: [RttChannel; (1 + 0)],
                    down_channels: [RttChannel; (0)],
                }
                #[used]
                #[no_mangle]
                #[export_name = "_SEGGER_RTT"]
                pub static mut CONTROL_BLOCK: MaybeUninit<RttControlBlock> = MaybeUninit::uninit();
                #[allow(unused)]
                #[export_name = "rtt_init_must_not_be_called_multiple_times"]
                fn rtt_init_must_not_be_called_multiple_times() {}
                use ::rtt_target::export::critical_section;
                static INITIALIZED: critical_section::Mutex<Cell<bool>> = critical_section::Mutex::new(
                    Cell::new(false),
                );
                critical_section::with(|cs| {
                    if INITIALIZED.borrow(cs).get() {
                        ::core::panicking::panic(
                            "rtt_init! must not be called multiple times",
                        );
                    }
                    INITIALIZED.borrow(cs).set(true);
                });
                unsafe {
                    ptr::write_bytes(CONTROL_BLOCK.as_mut_ptr(), 0, 1);
                    let cb = &mut *CONTROL_BLOCK.as_mut_ptr();
                    let mut name: *const u8 = core::ptr::null();
                    name = "Terminal\u{0}".as_bytes().as_ptr();
                    let mut mode = ::rtt_target::ChannelMode::NoBlockSkip;
                    mode = NoBlockSkip;
                    cb.up_channels[0]
                        .init(
                            name,
                            mode,
                            {
                                static mut _RTT_CHANNEL_BUFFER: MaybeUninit<[u8; 1024]> = MaybeUninit::uninit();
                                _RTT_CHANNEL_BUFFER.as_mut_ptr()
                            },
                        );
                    cb.header.init(cb.up_channels.len(), cb.down_channels.len());
                    pub struct Channels {
                        pub up: (UpChannel,),
                    }
                    Channels {
                        up: (UpChannel::new(&mut cb.up_channels[0] as *mut _),),
                    }
                }
            };
            ::rtt_target::set_print_channel(channels.up.0);
        };
    };
    ::rtt_target::print_impl::write_str(0, "Starting simple LED blink program\n");
    let dp = pac::Peripherals::take().unwrap();
    let rcc = &dp.rcc;
    rcc.ahbenr().modify(|_, w| w.iopeen().set_bit());
    let gpioe = &dp.gpioe;
    gpioe.moder().modify(|_, w| unsafe { w.moder8().bits(0b01) });
    gpioe.otyper().modify(|_, w| w.ot8().clear_bit());
    ::rtt_target::print_impl::write_str(0, "LED blink initialized on PE8\n");
    loop {
        gpioe.bsrr().write(|w| w.bs8().set_bit());
        ::rtt_target::print_impl::write_str(0, "LED ON\n");
        asm::delay(1_000_000);
        gpioe.bsrr().write(|w| w.br8().set_bit());
        ::rtt_target::print_impl::write_str(0, "LED OFF\n");
        asm::delay(1_000_000);
    }
}
