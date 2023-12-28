extern crate redis;

use clap::Parser;
use std::fmt::{Debug, Display, Formatter, Result};
use uuid::Uuid;

#[derive(Debug)]
enum TodoStatus {
    Done,
    NotDone,
    InProgress,
}

impl Display for TodoStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    set: bool,

    #[arg(short, long)]
    name: String,

    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    todo: Vec<String>,

    #[arg(long, default_value_t = TodoStatus::NotDone.to_string())]
    status: String,

    #[arg(short, long, default_value_t = true)]
    get: bool,

    #[arg(short, long)]
    uid: Option<String>,
}

fn add_todo(todo: Vec<String>, name: &String, status: &String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    let uid = Uuid::new_v4();

    let key = uid.to_string() + "|" + name + "|" + status;

    for t in todo {
        let _: () = redis::cmd("LPUSH").arg(&key).arg(&t).query(&mut con)?;

        println!("key: {} -> todo: {}", &key, &t);
    }

    let result: Option<Vec<String>> = redis::cmd("LRANGE")
        .arg(&key)
        .arg(0)
        .arg(-1)
        .query(&mut con)
        .unwrap();

    if let Some(value) = result {
        println!("Value retrieved from Redis: {:?}", value);
    } else {
        println!("Key not found in Redis");
    }
    Ok(())
}

fn get_todo(uid: String, name: &String, status: &String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    let mut search_keys: String = Default::default();

    if uid.to_string() != "" {
        search_keys.push_str(&uid);
        search_keys.push_str("|");
        search_keys.push_str(name);
        search_keys.push_str("|*");
    } else if status != "" {
        search_keys.push_str("*|");
        search_keys.push_str(name);
        search_keys.push_str("|");
        search_keys.push_str(status);
    } else {
        search_keys.push_str("*");
    }

    let keys: Vec<String> = redis::cmd("keys").arg(search_keys).query(&mut con).unwrap();
    for key in keys {
        println!("{:?}", key);
        let result: Option<Vec<String>> = redis::cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .query(&mut con)
            .unwrap();

        if let Some(value) = result {
            println!("TODOS: for the Project \"{}\"", &name);
            for td in value {
                println!("    \u{02192} {:}", td);
            }
        } else {
            println!("Key not found in Redis");
        }
    }
    Ok(())
}

fn update_status(uid: String, name: &String, status: &String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    let mut search_keys: String = Default::default();

    if uid.to_string() != "" && status != "" {
        // eprintln!("ERROR: Please provide proper uid to update status");
        search_keys.push_str(&uid);
        search_keys.push_str("|");
        search_keys.push_str(&name);
        search_keys.push_str("|");
        search_keys.push_str(&status);
    } else {
        search_keys.push_str("*");
    }

    let keys: Vec<String> = redis::cmd("keys").arg(search_keys).query(&mut con).unwrap();
    for key in keys {
        println!("{:?}", key);
        let result: Option<Vec<String>> = redis::cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .query(&mut con)
            .unwrap();

        if let Some(value) = result {
            println!("GET U+02192: Value retrieved from Redis: {:?}", value);
        } else {
            println!("Key not found in Redis");
        }
    }
    Ok(())
}

fn main() {
    // let mut args = env::args();
    // let _program = args.next().expect("expected program");

    // let mut todos_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    //

    let args = Args::parse();

    if args.todo.len() > 0 {
        let _ = add_todo(args.todo, &args.name, &args.status);
    }
    if args.get {
        let c = match args.uid {
            Some(value) => value,
            None => "".to_string(), // _ => "",
        };
        let _ = get_todo(c, &args.name, &args.status);
    }

    // println!("Hello {:?}!", args.todo)
    // if let Some(command_name) = args.next() {
    // if command_name == "--todo" {
    // let mut todos: Vec<String> = Vec::new();
    // for arg in args {
    // todos.push(arg);
    // }

    // println!("Todos: {todos:?}");
    // }
    // }
}
