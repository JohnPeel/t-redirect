#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use pickledb::{ PickleDb, PickleDbDumpPolicy, SerializationMethod };
use std::collections::HashMap;

fn get_value(to: &str, from: &str) -> u32 {
    let db = PickleDb::load("data.json",
                            PickleDbDumpPolicy::AutoDump,
                            SerializationMethod::Json).unwrap();
    let entry = db.get::<HashMap<String, u32>>(&from)
        .unwrap_or_else(|| HashMap::new());
    *entry.get(to).unwrap_or_else(|| &0u32)
}

fn set_value(to: &str, from: &str, value: u32) {
    let mut db = PickleDb::load("data.json",
                                PickleDbDumpPolicy::AutoDump,
                                SerializationMethod::Json).unwrap();

    let mut map = db.get::<HashMap<String, u32>>(&from)
        .unwrap_or_else(|| HashMap::new());

    map.insert(to.parse().unwrap(), value);
    db.set::<HashMap<String, u32>>(&from, &map).unwrap();
}

#[get("/stat/<to>/<from>")]
fn stat(to: String, from: String) -> String {
    format!("{}", get_value(&to, &from))
}

#[get("/<to>/<from>")]
fn redirect(to: String, from: String) -> Redirect {
    set_value(&to, &from, get_value(&to, &from) + 1);
    Redirect::permanent("https://www.youtube.com/watch?v=d1YBv2mWll0")
}

fn main() {
    if let Err(_) = PickleDb::load("data.json",
                                   PickleDbDumpPolicy::AutoDump,
                                   SerializationMethod::Json) {
        PickleDb::new("data.json",
                      PickleDbDumpPolicy::AutoDump,
                      SerializationMethod::Json);
    }
    rocket::ignite().mount("/", routes![stat, redirect]).launch();
}
