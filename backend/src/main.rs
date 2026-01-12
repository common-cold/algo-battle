use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use db::Database;

use crate::routes::create_user;

mod routes;

#[derive(Clone)]
pub struct AppData {
    db: Database
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::init_db().await.unwrap();

    let app_data = AppData {
        db: database
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .wrap(cors)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    
    Ok(())
}
