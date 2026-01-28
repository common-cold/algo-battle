use std::{str::FromStr, sync::{Arc, Mutex}};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, body::MessageBody, get, rt, web::{Data, Payload}};
use actix_ws::{Message, handle};
use common::{Role, WebSocketMessage, WebsocketAuth};
use db::Database;
use leaderboard::LeaderboardService;
use redis_service::RedisConnection;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{contest_manager::{ContestManager, LocalUser}, service::broadcast_next_question_task, utils::prepare_log};

mod contest_manager;
mod utils;
mod service;

#[derive(Clone)]
pub struct WsData {
    pub database: Database,
    pub contest_manager: Arc<Mutex<ContestManager>>,
    pub leaderboard_service: LeaderboardService
}

#[get("/ws")]
pub async fn ws_handler(request: HttpRequest, body: Payload, state: Data<WsData>, ws_auth: WebsocketAuth) -> Result<HttpResponse> {
    let (response, mut session, mut stream) = handle(&request, body)?;
    let (tx, mut rx) = mpsc::channel::<String>(32);

    let contest_manager = state.contest_manager.clone();
    let _database = state.database.clone();
    let leaderboard_service = state.leaderboard_service.clone();

    let mut user_id: Option<Uuid> = None;
    let mut username: Option<String> = None;
    let mut role: Option<Role> = None;

    match ws_auth {
        WebsocketAuth::User(ref jwt_claims) => {
            user_id = Some(jwt_claims.id);
            username = Some(jwt_claims.username.clone());
            role = Some(jwt_claims.role);

            contest_manager.lock()
            .unwrap()
            .clients
            .insert(user_id.unwrap(), LocalUser {
                tx: tx
            });
        }
        WebsocketAuth::Service => {

        }
    }

    
    let mut session_clone = session.clone();

    rt::spawn(async move {
        while let Some(msg) = stream.recv().await {
            match msg.unwrap() {
                Message::Text(data) => {
                    let message = serde_json::from_slice::<WebSocketMessage>(&data.as_bytes());
                    if let Ok(command) = message {
                        match (&ws_auth, command) {
                            (WebsocketAuth::Service, WebSocketMessage::StartContest(args)) => {
                                let mut cm = contest_manager.lock().unwrap();
                                if let Err(e) = cm.start_contest(args) {
                                    let log = prepare_log(e.to_string(), true);
                                    let _ = session.text(log).await;
                                }
                            },

                            (WebsocketAuth::User(_), WebSocketMessage::JoinContest(args)) => {
                                let mut cm = contest_manager.lock().unwrap();
                                if let Err(e) = cm.join_contest(user_id.unwrap(), args.contest_id).await {
                                    let log = prepare_log(e.to_string(), true);
                                    let _ = session.text(log).await;
                                    continue;
                                }    
                                if let Err(e) = leaderboard_service.add_user_to_leaderboard(args.contest_id, user_id.unwrap()).await {
                                    let log = prepare_log(e.to_string(), true);
                                    let _ = session.text(log).await;
                                }
                            },

                            (WebsocketAuth::User(_), WebSocketMessage::Debug) => {
                                let cm = contest_manager.lock().unwrap();
                                let client_list = cm.clients.iter().map(|c| c.0).collect::<Vec<&Uuid>>(); 
                                
                                println!("Clients: {:?}", client_list);
                                println!("Contests: {:?}", cm.contests);
                                println!("----------------------------------------------------");
                            },

                            (WebsocketAuth::User(_), _) => {
                                let _ = session.text("User is not allowed to send this message").await;
                            },

                            (WebsocketAuth::Service, _) => {
                                let _ = session.text("Service is not allowed to send this message").await;
                            }
                        }  

                    } else {
                        let log = prepare_log(String::from("came in message not supported"), true);
                        let _ = session.text(log).await;
                    }
                }

                _ => {}
            }
        }

        let mut cm = contest_manager.lock().unwrap();
        if user_id.is_some() {
            cm.clients.remove(&user_id.unwrap());
        };
        
    });


    rt::spawn(async move {
        while let Some(message) = rx.recv().await {
            let _ = session_clone.text(message).await;
        } 
    });

    Ok(response)
}

#[actix_web::main]
pub async fn main() -> Result<()> {
    let db = Database::init_db().await.unwrap();
    let db_clone = db.clone();
    
    let redis = RedisConnection::init_redis().await.unwrap();

    let leaderboard_service = LeaderboardService::new(db.clone(), redis.connection_manager);

    let contest_manager = ContestManager::sync_active_contests_from_db(&db).await.unwrap();
    let mut_contest_manager = Arc::new(Mutex::new(contest_manager));
    let contest_manager_clone = mut_contest_manager.clone();

    let state = WsData {
        database: db,
        contest_manager: mut_contest_manager,
        leaderboard_service: leaderboard_service.clone()
    };

    tokio::spawn(async move {
        broadcast_next_question_task(&db_clone, contest_manager_clone, leaderboard_service).await;
    });

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(state.clone()))
            .service(ws_handler)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await?;

    Ok(())
}