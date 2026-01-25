use std::{sync::{Arc, Mutex}, time::Duration};

use common::{ContestStatus, EndContestArgs, NextQuestionArgs, UpdateContestArgs, WebSocketResponse};
use db::Database;
use tokio::{time::interval};
use uuid::Uuid;

use crate::contest_manager::{ContestManager};



pub async fn broadcast_next_question_task(db: &Database, contest_manager: Arc<Mutex<ContestManager>>) {
    let mut interval = interval(Duration::from_secs(30));
    println!("INIT TASK");

    loop {
        println!("START LOOP");
        interval.tick().await;

        let cm;
        {
            let cm_lock = contest_manager.lock().unwrap();
            cm = cm_lock.clone();
        }
        
        let mut contest_ids_to_remove = Vec::<Uuid>::new();

        for (id, contest) in cm.contests.iter() {
            let index_result = cm.determine_next_question_index(id);
            if let Err(e) = index_result {
                println!("{}", e.to_string());
                continue;
            }

            let index = index_result.unwrap();
            if index == -1 {
                contest_ids_to_remove.push(*id);
            } else {
                //update current question id for contest, and release lock
                let mut cm_lock = contest_manager.lock().unwrap();
                let contest_opt = cm_lock.contests.get_mut(id);
                if contest_opt.is_some() {
                    let contest = contest_opt.unwrap();
                    contest.current_question_id = Some(contest.question_ids[index as usize]);
                }
            }


            for user_id in contest.users.iter() {
                let user_opt = cm.clients.get(user_id);
                if user_opt.is_none() {
                    continue;
                }
                let user = user_opt.unwrap();

                //contest has ended
                if index == -1 {
                    let message = WebSocketResponse {
                        data: common::ResponseData::EndContest(EndContestArgs {
                            contest_id: *id
                        })
                    };
                    let message_string = serde_json::to_string(&message);
                    if let Err(e) = message_string {
                        println!("{}", e.to_string());
                        continue;
                    }
                    let _ = user.tx.send(message_string.unwrap()).await; 

                } else {
                    let message = WebSocketResponse {
                        data: common::ResponseData::NextQuestion(NextQuestionArgs {
                            question_id: contest.question_ids[index as usize],
                            contest_id: *id
                        })
                    };
                    let message_string = serde_json::to_string(&message);
                    if let Err(e) = message_string {
                        println!("{}", e.to_string());
                        continue;
                    }
                    let _ = user.tx.send(message_string.unwrap()).await; 
                }
            }
        }
        
        
        if contest_ids_to_remove.len() > 0 {
            //remove contests from in memory variable
            {
                let mut cm_lock = contest_manager.lock().unwrap();
                for id in contest_ids_to_remove.iter() {
                    cm_lock.contests.remove(id);
                }
            }

            //update status to closed in db
            let update_res = db.bulk_update_contests(contest_ids_to_remove, UpdateContestArgs {
                title: None,
                description: None,
                start_date: None,
                end_date: None,
                status: Some(ContestStatus::Closed)
            }).await;

            if let Err(e) = update_res {
                println!("{}", e.to_string());
                continue;
            }
        }
        println!("END LOOP");
    }
}