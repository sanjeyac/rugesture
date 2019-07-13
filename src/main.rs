mod event_input;
mod logging;
mod utils;

use event_input::*;
use logging::*;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use std::time::{SystemTime, UNIX_EPOCH};

extern crate clap;
use clap::{Arg, App};


fn main() -> io::Result<()> {

// CLI MENU GENERATOR
    let matches = App::new("Multi touch logger")
        .version("0.1")
        .author("Sanjeya Cooray")
        .about("Logging multitouch to files as csv and svg")
        .arg(Arg::with_name("input")
                 .short("i")
                 .long("input")
                 .takes_value(true)
                 .help("Input device in /dev/input"))        
        .arg(Arg::with_name("csv")
                 .short("c")
                 .long("csv")
                 .takes_value(true)
                 .help("CSV output file"))
        .arg(Arg::with_name("svg")
                 .short("s")
                 .long("svg")
                 .takes_value(true)
                 .help("SVG output file"))
        .arg(Arg::with_name("time")
                 .short("t")
                 .long("time")
                 .takes_value(true)
                 .help("recording time in secs"))         
        .get_matches();

    let input = matches.value_of("input").unwrap_or("/dev/input/event7");
    let csv = matches.value_of("csv").unwrap_or("output.csv");
    let svg = matches.value_of("svg").unwrap_or("output.svg");

    let seconds = matches.value_of("time").unwrap_or("3").parse::<u128>().unwrap_or(3);

// TIMER
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let mut device_input_file = File::open(input)?;
    let mut buffer = [0; 24];
    let mut event_queue: Vec<EventInput> = Vec::new();
    let mut status_history: Vec<TouchStatus> = Vec::new();
    let mut status = init_status();

    loop {
        device_input_file.read(&mut buffer[..])?; //wait to an event in the device file e put data inside the buffer
        let current_input_event = create_input_from(&buffer);     

        let current_time = SystemTime::now();
        let current_since_the_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");

        let diff = current_since_the_epoch.as_millis() - since_the_epoch.as_millis();
        
        if diff > seconds*1000 { // record  3 second then stop everything at the next click
            break;
        }

        if current_input_event.code == 0 && current_input_event.evtype == 0 {
            update_status_by(&event_queue, &mut status);
            status_history.push(status.clone());
            event_queue.clear();
        } else {
            event_queue.push(current_input_event);
        }
    }

// log to svg and csv
    save_as_csv(csv, &status_history);
    save_as_svg(svg, &status_history);
   
    Ok(())
}