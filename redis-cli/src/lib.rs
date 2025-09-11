mod commands;
mod client;
mod error;

use std::{env::{self}, error::Error};
use commands::{*, Cmd::*};
use client::*;

pub fn run() -> Result<(), Box<dyn Error>>{

    let cmd = Cmd::from(env::args())?;
    let client = get_client()?;
    match cmd {
        Get(args) => { get(args, client)?; },
        Set(args) => { set(args, client)?; },
        HGet(args) => { hget(args, client)?; },
        HGetAll(args) => { hgetall(args, client)?; },
        NotFound(cmd) => { println!("Unexpected command: {}", cmd); }
    };
    Ok(())
}

