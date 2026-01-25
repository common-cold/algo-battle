use std::{env, time::Duration};

use anyhow::{anyhow};
use chrono::{Utc};
use common::{ContestStatus, FullContest, UpdateContestArgs, WebSocketMessage};
use db::Database;
use futures_util::{SinkExt};
use sqlx::types::Uuid;
use tokio::{net::TcpStream, time::interval};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{client::IntoClientRequest}};

pub async fn init_ws_connection() -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let x_api_key_result = env::var("X_API_KEY");
    if x_api_key_result.is_err() {
        return Err(anyhow!("x-api-key is missing"));
    }

    let mut request = "ws://localhost:8081/ws".into_client_request()?;
    request.headers_mut()
        .insert("x-api-key", x_api_key_result.unwrap().parse()?);
    
    let (ws, _) = connect_async(request).await?;

    Ok(ws)
}


pub async fn get_full_contest(db: &Database, contest_id: Uuid) -> anyhow::Result<FullContest> {
    let contest_result = db.get_contest_by_id(contest_id).await;
    if let Err(e) = contest_result {
        return Err(anyhow!(e.to_string()));
    }
    let contest = contest_result.unwrap();

    let question_ids_result = db.get_all_question_ids_for_contest_id(contest_id).await;
    if let Err(e) = question_ids_result {
        return Err(anyhow!(e.to_string()));
    }
    let question_ids = question_ids_result.unwrap();

    let question_list_result = db.get_questions_by_id(question_ids).await;
    if let Err(e) = question_list_result {
        return Err(anyhow!(e.to_string()));
    }
    let question_list = question_list_result.unwrap();
    Ok(FullContest { 
        contest: contest, 
        questions: question_list 
    })
}


pub async fn cron_task(db: &Database) {
    let mut interval = interval(Duration::from_mins(1));

    println!("INIT CRON");
    loop {
        println!("CRON LOOP START");
        interval.tick().await;

        let connection_result = init_ws_connection().await;
        if let Err(e) = connection_result {
            println!("{}", e.to_string());
            continue;
        }

        let mut ws = connection_result.unwrap();

        let result = db.get_all_contests(common::ContestStatus::Scheduled).await;
        if let Err(e) = result {
            println!("{}", e.to_string());
            continue;
        }
        let now = Utc::now().timestamp();
        // println!("Now: {}", now);
        let scheduled_contests = result.unwrap();
        let mut contest_ids_to_update = Vec::<Uuid>::new();
        for contest in scheduled_contests {
            // println!("Start date: {}", contest.start_date);
            if contest.start_date <= now.into() {
                contest_ids_to_update.push(contest.id);
            }
        }

        if contest_ids_to_update.len() == 0 {
            println!("No active contests found");
            continue;
        }

        let update_args = UpdateContestArgs {
            title: None,
            description: None,
            start_date: None,
            end_date: None,
            status: Some(ContestStatus::Active)
        };
        
        let update_result = db.bulk_update_contests(contest_ids_to_update.clone(), update_args).await;
        if let Err(e) = update_result {
            println!("{}", e.to_string());
            continue;
        }

        for contest_id in contest_ids_to_update {
            let full_contest = get_full_contest(db, contest_id).await;
            match full_contest {
                Ok(contest) => {
                    let message_result = serde_json::to_string(
                        &WebSocketMessage::StartContest(contest)
                    );
                    if let Err(e) = message_result {
                        println!("{}", e.to_string());
                        continue;
                    }
                    let ws_result = ws.send(tokio_tungstenite::tungstenite::Message::Text(message_result.unwrap())).await;
                    if let Err(e) = ws_result {
                        println!("{}", e.to_string());
                        continue;
                    }
                }
                Err(e) => {
                    println!("{}", e.to_string());
                }
            }
        }
        println!("CRON LOOP END");
    }
}