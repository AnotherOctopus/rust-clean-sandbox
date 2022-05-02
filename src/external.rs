use crate::controllers;
use actix_web::{get, web, App, HttpServer, Responder};
use std::fs;

#[get("/createuser/{name}/{pass}/{admin}")]
async fn greet(params: web::Path<(String, String, String)>) -> impl Responder {
    let admin_truth_value: bool = "true" == params.2.to_string();
    let ret = controllers::create(
        params.0.to_string(),
        params.1.to_string(),
        admin_truth_value,
    );
    saveuser(ret);
    format!("{}", params.0)
}

fn saveuser(to_save: controllers::Saveable) {
    fs::write("a_database.txt", to_save.data).expect("Unable to write file");
}
