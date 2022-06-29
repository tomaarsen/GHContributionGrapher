use anyhow::Result;
use clap::Parser;
use log::trace;

mod print;
mod requests;
mod structs;

use crate::print::print_calendar;
use crate::requests::post;
use crate::structs::Cli;

fn main() -> Result<(), anyhow::Error> {
    // Initialising environment logger
    env_logger::init();

    trace!("Parsing Command Line arguments...");
    // Parse the program arguments
    let args = Cli::parse();
    trace!("Parsed Command Line arguments.");

    // Get the Calendar instance for the desired user
    let calendar = post(&args)?;

    // Print out the Calendar instance
    print_calendar(&calendar, &args)?;

    Ok(())
}
