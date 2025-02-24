#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

const PIXELS_SQUARE: [(usize, usize); 16] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
];
const PIXELS_SPIRAL: [(usize, usize); 25] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
    // The 1st square ends
    (1, 1),
    (1, 2),
    (1, 3),
    (2, 3),
    (3, 3),
    (3, 2),
    (3, 1),
    (2, 1),
    // The 2nd square ends
    (2, 2),
];

const PIXELS_SPIRAL_MIRROR: [(usize, usize); 24] = [
    (2, 3),
    (3, 3),
    (3, 2),
    (3, 1),
    (2, 1),
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut last_led = (0, 0);

    loop {
        for cur_led in PIXELS_SPIRAL.iter() {
            leds[last_led.0][last_led.1] = 0;
            leds[cur_led.0][cur_led.1] = 1;
            display.show(&mut timer, leds, 100);
            last_led = *cur_led;
        }
        for cur_led in PIXELS_SPIRAL_MIRROR.iter() {
            leds[last_led.0][last_led.1] = 0;
            leds[cur_led.0][cur_led.1] = 1;
            display.show(&mut timer, leds, 80);
            last_led = *cur_led;
        }

        for cur_led in PIXELS_SPIRAL_MIRROR.iter().rev() {
            leds[last_led.0][last_led.1] = 0;
            leds[cur_led.0][cur_led.1] = 1;
            display.show(&mut timer, leds, 100);
            last_led = *cur_led;
        }
        for cur_led in PIXELS_SPIRAL.iter().rev() {
            leds[last_led.0][last_led.1] = 0;
            leds[cur_led.0][cur_led.1] = 1;
            display.show(&mut timer, leds, 80);
            last_led = *cur_led;
        }
    }
}
