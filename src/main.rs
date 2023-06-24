use std::sync::Mutex;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use TSA_VR_2023_Web_API::{AnimalOptions, get_and_cache, look_up_url, State};

#[get("/api")]
async fn get_animal(data: web::Data<State>, options: web::Query<AnimalOptions>) -> impl Responder {
    let mut counter = data.requests.lock().unwrap();
    *counter += 1;
    
    let data = get_and_cache(options.into_inner(), &data).await;
    
    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/animal/{pet}")]
async fn redirect_to_animal(path: web::Path<String>) -> impl Responder {
    let pet = path.into_inner();
    let url = look_up_url(pet).await;
    match url {
        Ok(url) => {
            if let Some(i) = url {
                HttpResponse::TemporaryRedirect().append_header(("Location", i)).finish()
            } else {
                HttpResponse::NotFound().body("Animal not found")
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let state = web::Data::new(State {
        requests: Mutex::new(0),
        access_token: Mutex::new("".to_string()),
        access_token_expiration: Mutex::new(0),
        client_id: Mutex::new(args[1].clone()),
        client_secret: Mutex::new(args[2].clone()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_animal)
            .service(redirect_to_animal)
        })
        .bind(("127.0.0.1", 1522))?
        .run()
        .await
}