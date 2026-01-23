use std::sync::{Arc, Mutex};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, get, rt, web::{Data, Payload}};
use actix_ws::{Message, handle};
use common::{JwtClaims, WebSocketMessage};
use db::Database;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{contest_manager::{ContestManager, LocalUser}, utils::prepare_log};

mod contest_manager;
mod utils;

#[derive(Clone)]
pub struct WsData {
    pub database: Database,
    pub contest_manager: Arc<Mutex<ContestManager>>
}

#[get("/ws")]
pub async fn ws_handler(request: HttpRequest, body: Payload, state: Data<WsData>, jwt_claims: JwtClaims) -> Result<HttpResponse> {
    let (response, mut session, mut stream) = handle(&request, body)?;
    let (tx, mut rx) = mpsc::channel::<String>(32);

    let contest_manager = state.contest_manager.clone();
    let database = state.database.clone();

    let user_id = jwt_claims.id;
    let username = jwt_claims.username;
    let role = jwt_claims.role;
    

    contest_manager.lock()
        .unwrap()
        .clients
        .insert(user_id, LocalUser {
            tx: tx
        });

    let mut session_clone = session.clone();

    rt::spawn(async move {
        while let Some(msg) = stream.recv().await {
            match msg.unwrap() {
                Message::Text(data) => {
                    let message = serde_json::from_slice::<WebSocketMessage>(&data.as_bytes());
                    if let Ok(command) = message {
                        match command {
                            WebSocketMessage::StartContest(args) => {
                                let mut cm = contest_manager.lock().unwrap();
                                if let Err(e) = cm.start_contest(args) {
                                    let log = prepare_log(e.to_string(), true);
                                    let _ = session.text(log).await;
                                }
                            },
                            WebSocketMessage::JoinContest(args) => {
                                let mut cm = contest_manager.lock().unwrap();
                                if let Err(e) = cm.join_contest(user_id, args.contest_id) {
                                    let log = prepare_log(e.to_string(), true);
                                    let _ = session.text(log).await;
                                }
                            },
                            WebSocketMessage::Debug => {
                                let cm = contest_manager.lock().unwrap();
                                let client_list = cm.clients.iter().map(|c| c.0).collect::<Vec<&Uuid>>(); 
                                
                                println!("Clients: {:?}", client_list);
                                println!("Contests: {:?}", cm.contests);
                                println!("----------------------------------------------------");
                            } 
                        }  

                    } else {
                        let _ = session.text("Message not supported").await;
                    }
                }

                _ => {}
            }
        }
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
    let contest_manager = ContestManager::sync_active_contests_from_db(&db).await.unwrap();
    let mut_contest_manager = Arc::new(Mutex::new(contest_manager));

    let state = WsData {
        database: db,
        contest_manager: mut_contest_manager
    };


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