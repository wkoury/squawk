#[macro_use] extern crate rocket;
extern crate squawk;

#[get("/")]
fn index() -> String {
	squawk::format_squawk_as_string()
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
}
