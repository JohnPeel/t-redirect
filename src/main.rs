#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::response::Redirect;
use pickledb::{ PickleDb, PickleDbDumpPolicy, SerializationMethod };

lazy_static! {
    pub static ref DB: Mutex<PickleDb> = Mutex::new(PickleDb::load("data.json",
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json)
        .unwrap_or_else(|_| PickleDb::new("data.json",
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Json)));
}

fn get_value(to: &str, from: &str) -> u32 {
    let map = DB.lock().unwrap().get::<HashMap<String, u32>>(&from)
        .unwrap_or_else(|| HashMap::new());

    *map.get(to).unwrap_or_else(|| &0u32)
}

fn inc_value(to: &str, from: &str) {
    let mut map = DB.lock().unwrap().get::<HashMap<String, u32>>(&from)
        .unwrap_or_else(|| HashMap::new());

    map.insert(to.to_string(), *map.get(to).unwrap_or_else(|| &0u32) + 1);
    DB.lock().unwrap().set::<HashMap<String, u32>>(&from, &map).unwrap();
}

#[get("/stat/<to>/<from>")]
fn stat(to: String, from: String) -> String {
    format!("{}", get_value(&to, &from))
}

#[get("/<to>/<from>")]
fn redirect(to: String, from: String) -> Redirect {
    inc_value(&to, &from);
    Redirect::permanent("https://www.youtube.com/watch?v=d1YBv2mWll0")
}

fn main() {
    rocket::ignite().mount("/", routes![stat, redirect]).launch();
}
