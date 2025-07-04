#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f303pac as pac;

/// 启用GPIOE时钟
/// 不使用inline。默认情况下编译器会将此函数优化为inline，不方便查看汇编代码
/// 设置no_mangle,不改变链接符号
#[inline(never)]
#[unsafe(no_mangle)]
fn enable_gpioe_clock(dp: &pac::Peripherals) {
    dp.rcc.ahbenr().modify(|_, w| w.iopeen().set_bit());
}

#[entry]
fn main() -> ! {
    // 初始化RTT打印
    rtt_init_print!();
    rprintln!("Starting simple LED blink program");

    // 获取外设访问
    let dp = pac::Peripherals::take().unwrap();

    // 启用GPIOE时钟
    enable_gpioe_clock(&dp);

    // 配置PE8为推挽输出模式
    let gpioe = &dp.gpioe;
    gpioe
        .moder()
        .modify(|_, w| unsafe { w.moder8().bits(0b01) }); // 设置为输出模式
    gpioe.otyper().modify(|_, w| w.ot8().clear_bit()); // 推挽输出

    rprintln!("LED blink initialized on PE8");

    // 主循环，周期性切换LED状态
    loop {
        // 打开LED (设置高电平)
        gpioe.bsrr().write(|w| w.bs8().set_bit());
        rprintln!("LED ON");
        asm::delay(1_000_000); // 延时

        // 关闭LED (设置低电平)
        gpioe.bsrr().write(|w| w.br8().set_bit());
        rprintln!("LED OFF");
        asm::delay(1_000_000); // 延时
    }
}
