use rocket;

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
