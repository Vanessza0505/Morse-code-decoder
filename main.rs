#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::gpio::{Output, OutputConfig};
use embassy_time::{Duration, Timer};

use embassy_executor::Spawner;

use esp_println as _;
use defmt::info;
use defmt::error;


#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}


esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {


    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    // The variable name those for whose watched troll hunters :)
    let mut blinky: Output<'static> = Output::new(peripherals.GPIO12,esp_hal::gpio::Level::Low, OutputConfig::default()); // I am using GPI012. The level is Low 'cause it is off by default and then starts.

    let user_input_text = "Rust"; // The user write a text in this variable
    let wpm: u64 = 10; // User input speed 

    let user_speed: u64 = 1200 / wpm; // Calculate the speed

    play_morse(&mut blinky, user_speed, user_input_text).await;
    
    let morse = text_to_morse(user_input_text);
    info!("The morsecode is: {}", morse.as_str());


    // debug:
    // let result = char_to_morse('V');
    // info!("{}", result);

    // let morse = text_to_morse("I love cats");
    // info!("{}", morse.as_str());

    // play_small(&mut blinky, dot_ms).await;
    // play_long(&mut blinky, dot_ms).await;

    // play_morse(&mut blinky, dot_ms, "Rust").await;

    loop {}
}


// A function that converts a character into a string and this string match with Morse code.
fn char_to_morse(c: char) -> &'static str {
    match c {
        'A' => ".-",
        'B' => "-...",
        'C' => "-.-.",
        'D' => "-..",
        'E' => ".",
        'F' => "..-.",
        'G' => "--.",
        'H' => "....",
        'I' => "..",
        'J' => ".---",
        'K' => "-.-",
        'L' => ".-..",
        'M' => "--",
        'N' => "-.",
        'O' => "---",
        'P' => ".--.",
        'Q' => "--.-",
        'R' => ".-.",
        'S' => "...",
        'T' => "-",
        'U' => "..-",
        'V' => "...-",
        'W' => ".--",
        'X' => "-..-",
        'Y' => "-.--",
        'Z' => "--..",
        '0' => "-----",
        '1' => ".----",
        '2' => "..---",
        '3' => "...--",
        '4' => "....-",
        '5' => ".....",
        '6' => "-....",
        '7' => "--...",
        '8' => "---..",
        '9' => "----.",
        _ => ""
    }
}


// This function goes through the given text of max 250 characters and splits it into words with the / sign and transcribes each letter into Morse code.
fn text_to_morse(text: &str) -> heapless::String<250>{
    let mut result: heapless::String<250> = heapless::String::new();

    for i in text.chars(){
        if i == ' ' { 
            result.push_str("/ ").ok();
        }
        else {
            let capital_letter = i.to_ascii_uppercase();
            let morse = char_to_morse(capital_letter);
            result.push_str(morse).ok();
            result.push_str(" ").ok();

        }
    }
    result
}

// The small beep function:
async fn play_small(blinky: &mut Output<'_>, speed: u64) {
    // The beeper is on
    blinky.set_high();
    Timer::after(Duration::from_millis(speed)).await;
    // The beeper is on break
    blinky.set_low();
    Timer::after(Duration::from_millis(speed)).await;
}

// The long beep function
async fn play_long(blinky: &mut Output<'_>, speed: u64){
    // The beeper is on
    blinky.set_high();
    Timer::after(Duration::from_millis(speed *3)).await;
    // The beeper is on break
    blinky.set_low();
    Timer::after(Duration::from_millis(speed)).await;
}

// The final function that sum all in one and play the morse code.
async fn play_morse(blinky: &mut Output<'_>, speed: u64, text: &str){
    let morse = text_to_morse(text);

    for i in morse.chars(){
        match i {
            '.' => play_small(blinky, speed).await,
            '-' => play_long(blinky, speed).await,
            ' ' => Timer::after(Duration::from_millis(speed *3)).await,
            '/' => Timer::after(Duration::from_millis(speed *7)).await,
            _ => {}
        }
    }
}
