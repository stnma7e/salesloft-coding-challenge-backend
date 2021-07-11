use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::client::Client;
use actix_web::client::SendRequestError;

use backend::models::{PeopleResponse};


#[get("/people")]
async fn get_people(req: HttpRequest) -> impl Responder {
    let bearer_token = match dotenv::var("SALESLOFT_APPLICATION_SECRET") {
        Ok(token) => token,
        Err(err) => {
            eprintln!("an error occurred: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let client = Client::default();
    let people_result = client.get("https://api.salesloft.com/v2/people.json")
        .header("Authorization", format!("Bearer {}", bearer_token))
        .send()
        .await;
    
    let people_data: PeopleResponse = match people_result {
        Err(err) => {
            eprintln!("an error occurred: {}", err);
            return HttpResponse::ServiceUnavailable().finish()
        }
        Ok(mut people) => {
            match people.json().await {
                Err(err) => {
                    eprintln!("an error occurred: {}", err);
                    return HttpResponse::ServiceUnavailable().finish();
                }
                Ok(people) => people
            }
        }
    };
    
    HttpResponse::Ok().json(people_data.data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_people)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
