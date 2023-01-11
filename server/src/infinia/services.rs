use crate::infinia::models::People;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/infinia/people")]
async fn to_inches(param_obj: web::Json<People>) -> impl Responder {
    let mut people = param_obj.people.clone();

    people
        .iter_mut()
        .for_each(|person| person.height = conversion(person.height));
    HttpResponse::Ok().json(People { people })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(to_inches);
}

/// This function converts cm to mm using only bitwise operations
fn conversion(height: u32) -> u32 {
    let conversion_factor: u32 = 10;
    multiply(conversion_factor, height)
}

// Todo: add overflow exception handling
fn multiply(mut a: u32, mut b: u32) -> u32 {
    let mut result: u32 = 0;
    while b > 0 {
        if b & 1 == 1 {
            result += a;
        }
        a = a << 1;
        b = b >> 1;
    }
    result
}
