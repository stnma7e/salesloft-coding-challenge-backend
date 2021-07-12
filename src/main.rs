extern crate pretty_env_logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::client::Client;
use log::{error};

use backend::models::{PeopleResponse};


async fn get_people(_req: HttpRequest) -> HttpResponse {
    let bearer_token = match dotenv::var("SALESLOFT_APPLICATION_SECRET") {
        Ok(token) => token,
        Err(err) => {
            error!("Failed to get API token: {}", err);
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
            eprintln!("Failed to request people from API: {}", err);
            return HttpResponse::ServiceUnavailable().finish()
        }
        Ok(mut people) => {
            match people.json().await {
                Err(err) => {
                    error!("Failed to deserialize API response: {}", err);
                    return HttpResponse::ServiceUnavailable().finish();
                },
                Ok(people) => people
            }
        }
    };
    
    HttpResponse::Ok().json(people_data.data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/people", web::get().to(get_people))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_get_people_ok() {
        let req = test::TestRequest::get().to_http_request();
        let resp = get_people(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_people_not_empty() {
        let req = test::TestRequest::get().to_http_request();
        let resp = get_people(req).await;
        
        let people_body = resp.body();
        assert!(people_body.as_ref().is_some());
    }
}