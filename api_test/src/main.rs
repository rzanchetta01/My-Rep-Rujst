use rand::Rng;

#[macro_use] extern crate rocket;

#[get("/")]
fn rnd_roll() -> String {
    rand::thread_rng().gen_range(1..=6).to_string()
}

#[launch]
fn api() -> _ {
    rocket::build().mount("/", routes![rnd_roll])
}
