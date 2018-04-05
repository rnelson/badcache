#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
#[macro_use]

extern crate lazy_static;
extern crate rocket;
mod cache;
use cache::memory::MemoryCache;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<MemoryCache<String>> = {
        let cache = MemoryCache::new();
        Mutex::new(cache)
    };
}

fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, add, remove, get])
}

#[get("/")]
fn index() -> String {
    format!("Welcome to badcache. Don't use this.")
}

#[get("/add/<key>/<value>")]
fn add(key: String, value: String) -> String {
    let mut cache = CACHE.lock().unwrap();
    cache.add(key.clone(), value);
    format!("{} added", key)
}

#[get("/remove/<key>")]
fn remove(key: String) -> String {
    let mut cache = CACHE.lock().unwrap();
    cache.remove(key.clone());
    format!("{} removed", key)
}

#[get("/get/<key>")]
fn get(key: String) -> String {
    let cache = CACHE.lock().unwrap();
    let value = cache.get(key);
    match value {
        Some(v) => format!("{}", v),
        None => format!("null"),
    }
}