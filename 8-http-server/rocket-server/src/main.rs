#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/mccmmancc/hello")]
fn mccmmancc() -> &'static str {
    "MCCMMANCCに参加してくれてありがとうございます！"
}

#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, mccmmancc, hello])
}
