#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::asm;
use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32f7xx_hal::{
    gpio::{DynamicPin, PinState},
    pac::{TIM2, TIM5},
    prelude::*,
    timer::{CounterUs, Event},
};

const VIH: f32 = 2.3; // V
const VCC: f32 = 3.3; // V
const RREF: f32 = 10_000.0; // Ohm
const DELAY: u32 = 100_000;
const DURATION: u32 = 1_000_000;

#[app(device = stm32f7xx_hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        gp0: DynamicPin<'A', 1>,
        gp1: DynamicPin<'A', 2>,
        gp2: DynamicPin<'A', 5>,
        timer: CounterUs<TIM2>,
        counter: CounterUs<TIM5>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let dp = cx.device;
        let rcc = dp.RCC.constrain();

        rtt_init_print!();

        let clocks = rcc.cfgr.sysclk(216.MHz()).freeze();

        let gpioa = dp.GPIOA.split();
        let gp0 = gpioa.pa1.into_dynamic();
        let gp1 = gpioa.pa2.into_dynamic();
        let gp2 = gpioa.pa5.into_dynamic();

        let mut timer = dp.TIM2.counter(&clocks);
        timer.start(1000.millis()).ok();
        timer.listen(Event::Update);
        let counter = dp.TIM5.counter(&clocks);

        (
            Shared {},
            Local {
                gp0,
                gp1,
                gp2,
                timer,
                counter,
            },
        )
    }

    #[task(binds = TIM2, local = [timer, gp0, gp1, gp2, counter])]
    fn timer_tick(ctx: timer_tick::Context) {
        ctx.local.timer.clear_interrupt(Event::all());

        ctx.local.gp0.make_push_pull_output_in_state(PinState::Low);
        ctx.local.gp1.make_floating_input();
        ctx.local.gp2.make_floating_input();
        asm::delay(DELAY);

        ctx.local.gp0.make_floating_input();
        ctx.local.gp1.make_push_pull_output();
        ctx.local.gp2.make_floating_input();
        ctx.local.counter.start(10.millis()).ok();
        ctx.local.gp1.set_high().ok();

        let t1 = loop {
            if ctx.local.gp0.is_high().unwrap() {
                let t = ctx.local.counter.now();
                ctx.local.counter.cancel().ok();
                break t;
            }
        };
        ctx.local.gp0.make_push_pull_output_in_state(PinState::Low);
        ctx.local.gp1.make_floating_input();
        ctx.local.gp2.make_floating_input();
        asm::delay(DELAY);

        ctx.local.gp0.make_floating_input();
        ctx.local.gp1.make_floating_input();
        ctx.local.gp2.make_push_pull_output();
        ctx.local.counter.start(10.millis()).ok();
        ctx.local.gp2.set_high().ok();

        let t2 = loop {
            if ctx.local.gp0.is_high().unwrap() {
                let t = ctx.local.counter.now();
                ctx.local.counter.cancel().ok();
                break t;
            }
        };
        ctx.local.gp0.make_push_pull_output_in_state(PinState::Low);
        ctx.local.gp1.make_floating_input();
        ctx.local.gp2.make_floating_input();
        asm::delay(DELAY);

        let t1_sec: f32 = t1.ticks() as f32 / DURATION as f32;
        let t2_sec: f32 = t2.ticks() as f32 / DURATION as f32;

        let cap = -t2_sec / (RREF * libm::logf(1.0 - VIH / VCC));
        let r1 = -t1_sec / (cap * libm::logf(1.0 - VIH / VCC));

        rprintln!("R1 {}", r1);
    }
}