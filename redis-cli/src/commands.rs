use std::env::Args;
use std::{env, error::Error};

use crate::client::RedisClient;
use crate::error::*;

pub fn get<C: RedisClient, A: CmdArgs>(mut args: A, mut client: C) -> Result<(), Box<dyn Error>> {
    let key = match args.get_arg() {
        Some(key) => key,
        None => {
            return Err(Er::new("No key argument provided for GET command"));
        },
    };
    let maybe_val = client.get(&key)?;
    match maybe_val {
        Some(val) => {
            println!("GET({}): \"{}\"", key, val);
        },
        None => {
            println!("GET({}): nil", key);
        },
    }
    Ok(())
}

pub fn hget<C: RedisClient, A: CmdArgs>(mut args: A, mut client: C) -> Result<(), Box<dyn Error>> {
    let key = match args.get_arg() {
        Some(key) => key,
        None => {
            return Err(Er::new("No key argument provided for HGET command"));
        },
    };

    let field = match args.get_arg() {
        Some(key) => key,
        None => {
            return Err(Er::new("No field argument provided for HGET command"));
        },
    };

    let maybe_val = client.hget(&key, &field)?;
    match maybe_val {
        Some(val) => {
            println!("HGET({}): \"{}\"", key, val);
        },
        None => {
            println!("HGET({}): nil", key);
        },
    }
    Ok(())
}

pub fn hgetall<C: RedisClient, A: CmdArgs>(mut args: A, mut client: C) -> Result<(), Box<dyn Error>> {
    let key = match args.get_arg() {
        Some(key) => key,
        None => {
            return Err(Er::new("No key argument provided for HGET command"));
        },
    };

    let maybe_val = client.hgetall(&key)?;
    match maybe_val {
        Some(res) => {
            print_hgetall_result(res, key);
        },
        None => {
            println!("HGETALL({}): nil", key);
        },
    }
    Ok(())
}

fn print_hgetall_result(res: Vec<String>, key: String) {
    let mut i = 0;
    let mut longest_field = 0;
    while i < res.len() {
        if res[i].len() > longest_field {
            longest_field = res[i].len();
        }
        i += 2;
    }
    println!("HGETALL({})", key);
    let mut i = 0;
    while i < res.len() {
        println!("\"{}\"{} -> \"{}\"", res[i], " ".repeat(longest_field - res[i].len()), res[i + 1]);
        i += 2;
    }
}

pub fn set<C: RedisClient, A: CmdArgs>(mut args: A, mut client: C) -> Result<(), Box<dyn Error>> {
    let key = args.get_arg().expect("No key argument provided for SET command");
    let value = args.get_arg().expect("No value argument provided for SET command");
    client.set(&key, &value)?;
    Ok(())
}

pub enum Cmd<T: CmdArgs> {
    Get(T),
    Set(T),
    HGet(T),
    HGetAll(T),
    NotFound(String)
}

pub trait CmdArgs {
    fn get_arg(&mut self) -> Option<String>;
}

impl Cmd<Args> {
    pub fn from(mut args: env::Args) -> Result<Cmd<Args>, &'static str> {
        args.next();
        let arg = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("No command line args were passed")
            }
        };
        let cmd = match arg.as_str() {
            "GET" => Cmd::Get(args),
            "SET" => Cmd::Set(args),
            "HGET" => Cmd::HGet(args),
            "HGETALL" => Cmd::HGetAll(args),
            _ => Cmd::NotFound(arg)
        };
        Ok(cmd)
    }
}

impl CmdArgs for Args {
    fn get_arg(&mut self) -> Option<String> {
        self.next()
    }
}

