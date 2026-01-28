use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use db::Database;
use leaderboard::LeaderboardService;
use redis_service::RedisConnection;

use crate::{routes::{create_contest, create_question, create_user, get_all_contests, get_all_examiner_contests, get_all_examiner_questions, get_contest_joined_at, get_full_contest_by_id, get_questions_by_id, join_contest, signin, submit_mcq_question}, service::cron_task};

mod routes;
mod service;

#[derive(Clone)]
pub struct AppData {
    db: Database,
    leaderboard_service: LeaderboardService
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::init_db().await.unwrap();
    let database_clone = database.clone();

    let redis = RedisConnection::init_redis().await.unwrap();

    let leaderboard_service = LeaderboardService::new(database.clone(), redis.connection_manager);

    let app_data = AppData {
        db: database,
        leaderboard_service: leaderboard_service
    };

    
    tokio::spawn(async move {
        cron_task(&database_clone).await;
    });

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
            .service(signin)
            .service(create_question)
            .service(create_contest)
            .service(get_all_examiner_questions)
            .service(get_all_contests)
            .service(get_all_examiner_contests)
            .service(get_questions_by_id)
            .service(get_full_contest_by_id)
            .service(join_contest)
            .service(get_contest_joined_at)
            .service(submit_mcq_question)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    
    Ok(())
}
