use std::collections::{HashMap, HashSet};

use anyhow::{Ok, anyhow};
use chrono::Utc;
use common::{FullContest, NextQuestionArgs, WebSocketResponse};
use db::Database;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct LocalUser {
    pub tx: Sender<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalContest {
    pub users: HashSet<Uuid>,
    pub start_date: i64,
    pub end_date: i64,
    pub question_ids: Vec<Uuid>,
    pub question_times: Vec<i64>,
    pub current_question_id: Option<Uuid>
}

#[derive(Debug, Clone)]
pub struct ContestManager {
    pub clients: HashMap<Uuid, LocalUser>,
    pub contests: HashMap<Uuid, LocalContest>
}

impl ContestManager {
    pub async fn sync_active_contests_from_db(db: &Database) -> anyhow::Result<ContestManager> {
        let clients = HashMap::<Uuid, LocalUser>::new();
        let mut contests = HashMap::<Uuid, LocalContest>::new();
        
        let db_contests = db.get_all_contests(common::ContestStatus::Active).await?;
        for contest in db_contests {
            let question_ids = db.get_all_question_ids_for_contest_id(contest.id).await?;
            let question_list = db.get_questions_by_id(question_ids.clone()).await?;
            let question_times = question_list
            .iter()
            .map(|q| q.time_limit )
            .collect::<Vec<i64>>();
            
            let user_ids_joined = db.get_contest_user_ids(contest.id).await?;
            let users: HashSet<Uuid> = user_ids_joined.into_iter().collect();
            
            let local_contest = LocalContest {
                users: users,
                start_date: contest.start_date,
                end_date: contest.end_date,
                question_ids: question_ids,
                question_times: question_times,
                current_question_id: None          //after syncing the broadcast_next_question_task runs immediately which updates this
            };

            contests.insert(contest.id, local_contest);
        }

        let contest_manager = ContestManager {
            clients: clients,
            contests: contests
        };
        
        Ok(contest_manager)
    }


    pub fn start_contest(&mut self, full_contest: FullContest) -> anyhow::Result<()> {
        let question_ids = full_contest.questions
            .iter()
            .map(|q| q.id )
            .collect::<Vec<Uuid>>();

        let question_times = full_contest.questions
            .iter()
            .map(|q| q.time_limit )
            .collect::<Vec<i64>>();

        let first_question_id = question_ids[0];
        
        let local_contest= LocalContest {
            users: HashSet::new(),
            start_date: full_contest.contest.start_date,
            end_date: full_contest.contest.end_date,
            question_ids: question_ids,
            question_times: question_times,
            current_question_id: Some(first_question_id)
        };

        self.contests.insert(full_contest.contest.id, local_contest);
        Ok(())
    }

    pub async fn join_contest(&mut self, user_id: Uuid, contest_id: Uuid) -> anyhow::Result<()> {
        if let Some(contest) = self.contests.get_mut(&contest_id) {
            contest.users.insert(user_id);

            let local_user = self.clients.get(&user_id).unwrap();

            if contest.current_question_id.is_some() {
                let message = WebSocketResponse {
                    data: common::ResponseData::NextQuestion(NextQuestionArgs {
                        question_id: contest.current_question_id.unwrap(),
                        contest_id: contest_id,
                        new_rank: -1
                    })
                };

                let msg = serde_json::to_string(&message);
                if msg.is_ok() {
                    let _ = local_user.tx.send(msg.unwrap()).await;
                }
            }
        
        } else {
            return Err(anyhow::anyhow!("Contest does not exist"))
        }

        Ok(())
    }

    pub fn determine_next_question_index(&self, contest_id: &Uuid) -> anyhow::Result<i16> {
        let local_contest = self.contests.get(contest_id).unwrap();

        let now = Utc::now().timestamp();
        let mut time_passed = now - local_contest.start_date;

        if time_passed < 0 {
           return  Err(anyhow!("Contest has not started"));
        };

        for (index, &question_time) in local_contest.question_times.iter().enumerate() {
            if time_passed < question_time {
                return Ok(index as i16);
            } 
            time_passed = time_passed - question_time;
        }

        Ok(-1)
    }
}