use std::path::PathBuf;

#[macro_use]
extern crate rocket;

#[get("/box/<name>/files/<path..>")]
fn get_box_file(name: &str, path: PathBuf) {}

#[post("/box/<name>/files/<path..>")]
fn add_box_file(name: &str, path: PathBuf) {}

#[delete("/box/<name>/files/<path..>")]
fn delete_box_file(name: &str, path: PathBuf) {}

#[patch("/box/<name>/files/<path..>")]
fn update_box_file(name: &str, path: PathBuf) -> String {
    format!("Updating file in box {}", name)
}

#[post("/box/<name>")]
fn new_box(name: &str) {}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            get_box_file,
            add_box_file,
            delete_box_file,
            update_box_file,
            new_box
        ],
    )
}
