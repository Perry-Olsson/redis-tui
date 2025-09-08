use std::{env, error::Error};

use crate::client::RedisClient;
use crate::error::*;

pub fn get<C: RedisClient>(args: Vec<String>, mut client: C) -> Result<(), Box<dyn Error>> {
    let mut args = args.iter();
    let key = match args.next() {
        Some(key) => key,
        None => {
            return Err(Er::new("No key argument provided for GET command"));
        },
    };
    let maybe_val = client.get(key)?;
    match maybe_val {
        Some(val) => {
            println!("GET({}): {}", key, val);
        },
        None => {
            println!("GET({}): nil", key);
        },
    }
    Ok(())
}

pub fn set<C: RedisClient>(args: Vec<String>, mut client: C) -> Result<(), Box<dyn Error>> {
    let mut args = args.iter();
    let key = args.next().expect("No key argument provided for SET command");
    let value = args.next().expect("No value argument provided for SET command");
    client.set(key, value)?;
    Ok(())
}

pub enum Cmd {
    Get(Vec<String>),
    Set(Vec<String>),
    NotFound(String)
}

impl Cmd {
    pub fn from(mut args: env::Args) -> Result<Cmd, &'static str> {
        args.next();
        let arg = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("No command line args were passed")
            }
        };
        let cmd = match arg.as_str() {
            "GET" => Cmd::Get(Cmd::rest(args)),
            "SET" => Cmd::Set(Cmd::rest(args)),
            _ => Cmd::NotFound(arg)
        };
        Ok(cmd)
    }

    fn rest(args: env::Args) -> Vec<String> {
        let mut rest = Vec::new();
        for arg in args {
            rest.push(arg)
        }
        rest
    }
}
