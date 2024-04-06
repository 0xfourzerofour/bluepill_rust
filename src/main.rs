#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m::interrupt;
use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::afio::Parts;
use stm32f1xx_hal::gpio::{Edge, ExtiPin, Input, Output, PullUp, PushPull, PA0, PA1, PA5, PA6};
use stm32f1xx_hal::pac::EXTI;
use stm32f1xx_hal::prelude::*;

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {

    use super::*;

    #[derive(Debug)]
    pub enum Direction {
        Up,
        Down,
        Stop,
    }

    #[shared]
    struct Shared {
        direction: Direction,
        motor_clockwise: PA5<Output<PushPull>>,
        motor_counter_clockwise: PA6<Output<PushPull>>,
    }

    #[local]
    struct Local {
        up: PA0<Input<PullUp>>,
        down: PA1<Input<PullUp>>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Setup clocks
        let dp = cx.device;
        let mut flash = dp.FLASH.constrain();
        let rcc = dp.RCC.constrain();
        let mut afio = dp.AFIO.constrain();
        let mut exti = dp.EXTI;

        rtt_init_print!();
        rprintln!("init");

        let _clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(36.MHz())
            .pclk1(36.MHz())
            .freeze(&mut flash.acr);

        let mut gpioa = dp.GPIOA.split();
        let mut up = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);
        let mut down = gpioa.pa1.into_pull_up_input(&mut gpioa.crl);

        init_receiver_pins(&mut up, &mut down, &mut afio, &mut exti);

        let mut motor_clockwise = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
        motor_clockwise.set_low();

        let mut motor_counter_clockwise = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
        motor_counter_clockwise.set_low();

        (
            Shared {
                motor_clockwise,
                motor_counter_clockwise,
                direction: Direction::Stop,
            },
            Local { up, down },
        )
    }

    fn init_receiver_pins(
        up: &mut PA0<Input<PullUp>>,
        down: &mut PA1<Input<PullUp>>,
        afio: &mut Parts,
        exti: &mut EXTI,
    ) {
        interrupt::free(|_cs| {
            up.make_interrupt_source(afio);
            up.trigger_on_edge(exti, Edge::RisingFalling);
            up.enable_interrupt(exti);

            down.make_interrupt_source(afio);
            down.trigger_on_edge(exti, Edge::RisingFalling);
            down.enable_interrupt(exti);
        });
    }

    #[task(binds = EXTI0, local = [up], shared= [direction, motor_clockwise, motor_counter_clockwise])]
    fn receive_up(ctx: receive_up::Context) {
        rprintln!("SIGNAL UP");
        ctx.local.up.clear_interrupt_pending_bit();
        // match ctx.local.direction {
        //     Direction::Up => {
        //         *ctx.local.direction = Direction::Down;
        //         ctx.local.motor_counter_clockwise.set_low();
        //         ctx.local.motor_clockwise.set_high();
        //     }
        //     Direction::Down => {
        //         *ctx.local.direction = Direction::Up;
        //         ctx.local.motor_clockwise.set_low();
        //         ctx.local.motor_counter_clockwise.set_high();
        //     }
        //     Direction::Stop => {
        //         *ctx.local.direction = Direction::Up;
        //         ctx.local.motor_clockwise.set_low();
        //         ctx.local.motor_counter_clockwise.set_low();
        //     }
        // }
    }

    #[task(binds = EXTI1, local = [down], shared= [direction, motor_clockwise, motor_counter_clockwise])]
    fn receive_down(ctx: receive_down::Context) {
        rprintln!("SIGNAL DOWN");
        ctx.local.down.clear_interrupt_pending_bit();
        // match ctx.local.direction {
        //     Direction::Up => {
        //         *ctx.local.direction = Direction::Down;
        //         ctx.local.motor_counter_clockwise.set_low();
        //         ctx.local.motor_clockwise.set_high();
        //     }
        //     Direction::Down => {
        //         *ctx.local.direction = Direction::Up;
        //         ctx.local.motor_clockwise.set_low();
        //         ctx.local.motor_counter_clockwise.set_high();
        //     }
        //     Direction::Stop => {
        //         *ctx.local.direction = Direction::Up;
        //         ctx.local.motor_clockwise.set_low();
        //         ctx.local.motor_counter_clockwise.set_low();
        //     }
        // }
    }
}
