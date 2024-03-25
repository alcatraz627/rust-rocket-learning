use std::{io, path::PathBuf};

use rocket::{
    fs::FileServer,
    tokio::{
        task::spawn_blocking,
        time::{sleep, Duration},
    },
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "We are sooooo in the game!"
}

#[get("/sleep/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waiter for {seconds} seconds")
}

#[get("/path/<path_val..>")]
fn path_val(path_val: PathBuf) -> String {
    let joined_path = path_val.to_str().unwrap().to_string();

    format!("You have navigated to: {joined_path}")
}

// Example of a blocking task
#[get("/read/file/<filename..>")]
async fn read_file(filename: PathBuf) -> io::Result<Vec<u8>> {
    let filename_ref = filename.to_str().unwrap().to_string();

    println!("{}", filename_ref);

    let vec = spawn_blocking(move || std::fs::read(filename_ref))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, delay, read_file, path_val])
        .mount("/static", FileServer::from("static")) // Serving static assets
}
