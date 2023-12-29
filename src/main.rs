extern crate redis;

use todo_cli::utils::*;

use clap::Parser;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    set: bool,

    #[arg(short, long)]
    name: String,

    #[arg(long, use_value_delimiter = true, value_delimiter = ',')]
    todo: Vec<String>,

    #[arg(long, default_value_t = TodoStatus::NotDone.to_string())]
    status: String,

    #[arg(short, long, default_value_t = false)]
    get: bool,

    #[arg(short, long)]
    uid: Option<String>,

    #[arg(short, long)]
    topic: Vec<String>,
}

fn add_todo(
    todo_text: String,
    project_name: &String,
    status: &String,
    topic: &Vec<String>,
) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    let uid = Uuid::new_v4();

    // let key = uid.to_string() + "|" +
    let key = project_name.to_owned().to_lowercase().replace(" ", "-") + "|" + &uid.to_string();

    let todo = Todo {
        uid: uid.to_string(),
        text: todo_text.clone(),
        status: status.clone(),
        topic: topic.clone(),
    };

    println!("TODO STRUCT PASSED");

    let j = serde_json::to_string(&todo).expect("Failed to serialize to JSON");

    // println!("{:?}", j);

    // for t in todo {
    let _: () = redis::cmd("SET").arg(&key).arg(&j).query(&mut con)?;

    println!("key: {} -> todo: {:?}", &key, &j);
    // }

    let result: Option<String> = redis::cmd("GET").arg(&key).query(&mut con).unwrap();

    if let Some(value) = result {
        println!("Value retrieved from Redis: {:?}", value);
        let h = serde_json::from_str::<Todo>(&value);
        println!("------------------> {:}", h.unwrap().text)
    } else {
        println!("Key not found in Redis");
    }
    Ok(())
}

fn get_todo(
    project_name: &String,
    uid: &String,
    status: &String,
    topic: &Vec<String>,
) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    let mut search_keys: String = Default::default();

    if project_name != "" && uid != "" {
        search_keys.push_str(&project_name.to_lowercase().replace(" ", "-"));
        search_keys.push_str("|");
        search_keys.push_str(uid);
    } else if project_name == "" && uid != "" {
        search_keys.push_str("*");
        search_keys.push_str("|");
        search_keys.push_str(uid);
    } else if project_name != "" && uid == "" {
        search_keys.push_str(&project_name.to_lowercase().replace(" ", "-"));
        search_keys.push_str("|");
        search_keys.push_str("*");
    } else {
        search_keys.push_str("*");
    }

    let keys: Vec<String> = redis::cmd("keys").arg(search_keys).query(&mut con).unwrap();
    println!("TODOS: for the Project \"{}\"", &project_name);

    for key in keys {
        let result: Option<String> = redis::cmd("GET").arg(&key).query(&mut con).unwrap();

        if let Some(value) = result {
            println!("    \u{02192} {:}", value);
        } else {
            println!("Key not found in Redis");
        }
    }
    Ok(())
}

fn update_todo(
    project_name: &String,
    uid: &String,
    status: &String,
    topic: &Vec<String>,
) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let mut con = client.get_connection()?;

    let mut search_keys: String = Default::default();

    let key = project_name.to_owned().to_lowercase().replace(" ", "-") + "|" + &uid.to_string();

    let result: Option<String> = redis::cmd("GET").arg(&key).query(&mut con).unwrap();

    if let Some(value) = result {
        let todo = serde_json::from_str::<Todo>(&value).unwrap();
        let todo_updated = Todo {
            uid: todo.uid,
            text: todo.text,
            status: status.to_string(),
            topic: todo.topic,
        };

        println!("TODO STRUCT PASSED");

        let j = serde_json::to_string(&todo_updated).expect("Failed to serialize to JSON");

        let _: () = redis::cmd("SET").arg(&key).arg(&j).query(&mut con)?;

        println!("Updated the TODO {:?}", &j);
    } else {
        println!("Key not found in Redis");
    }

    Ok(())
}
fn main() {
    let args = Args::parse();

    if args.set {
        if args.todo.len() > 0 {
            for t in args.todo {
                let _ = add_todo(t, &args.name, &args.status, &args.topic);
            }
        } else {
            let _ = update_todo(
                &args.name,
                &args.uid.clone().unwrap(),
                &args.status,
                &args.topic,
            );
        }
    }
    if args.get {
        if let Some(uid_val) = &args.uid.clone() {
            let _ = get_todo(&args.name, uid_val, &args.status, &args.topic);
        } else {
            let _ = get_todo(&args.name, &"".to_string(), &args.status, &args.topic);
        }
    }
}
