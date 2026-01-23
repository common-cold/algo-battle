use std::{collections::HashMap};

use anyhow::Ok;
use common::{FullContest};
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
    pub users: Vec<Uuid>,
    pub start_date: i64,
    pub end_date: i64,
    pub question_ids: Vec<Uuid>,
    pub question_time: Vec<i64>
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
            
            let local_contest = LocalContest {
                users: user_ids_joined,
                start_date: contest.start_date,
                end_date: contest.end_date,
                question_ids: question_ids,
                question_time: question_times
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
        
        let local_contest= LocalContest {
            users: Vec::new(),
            start_date: full_contest.contest.start_date,
            end_date: full_contest.contest.end_date,
            question_ids: question_ids,
            question_time: question_times
        };

        self.contests.insert(full_contest.contest.id, local_contest);
        Ok(())
    }

    pub fn join_contest(&mut self, user_id: Uuid, contest_id: Uuid) -> anyhow::Result<()> {
        if let Some(contest) = self.contests.get_mut(&contest_id) {
            contest.users.push(user_id);
        } else {
           return Err(anyhow::anyhow!("Contest does not exist"))
        }
        
        Ok(())
    }
}