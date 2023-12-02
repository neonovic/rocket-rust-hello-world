#[macro_use]
extern crate rocket;

#[cfg(test)] mod tests;

use std::collections::HashMap;
use rocket::form::{Form, FromForm};
use rocket::serde::Serialize;
use rocket_dyn_templates::{Template, context};
use rocket::response::{Flash, Redirect, Responder};
use rocket::http::Status;

#[derive(Debug, FromForm)]
struct ContactForm {
    name: String,
    phone: String,
}

#[derive(Serialize)]
struct TemplateContext {
    message: String,
}

#[allow(dead_code)]
#[derive(Responder)]
enum TemplateRedirect {
    Template(Template),
    Redirect(Redirect),
    Flash(Flash<Redirect>),
    NotFound(Status),
}

#[get("/")]
fn form() -> TemplateRedirect {
    let mut context = HashMap::new();
    context.insert("message", "");
    TemplateRedirect::Template(Template::render("result", &context))
}

#[post("/", data = "<form>")]
fn submit(form: Form<ContactForm>) -> Template {
    let name = &form.name;
    let phone = &form.phone;

    // Process the submitted form data here (e.g., save to a database)

    let message = format!("Thank you, {}! We have received your submission with phone: {}", name, phone);

    let context = context! { message };

    Template::render("result", context)
}

#[rocket::main]
async fn main() {
    let result = rocket::build()
        .mount("/", routes![submit, form])
        .attach(Template::fairing())
        .launch();

    if let Err(e) = result.await {
        println!("This rocket did not launch:");
        drop(e);
    };
}
