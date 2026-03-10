use std::str::FromStr;

use anyhow::anyhow;
use db::Database;
use redis::{AsyncTypedCommands, aio::ConnectionManager};
use uuid::Uuid;

#[derive(Clone)]
pub struct LeaderboardService {
    pub db: Database,
    pub redis: ConnectionManager,
}

impl LeaderboardService {
    pub fn new(db: Database, redis: ConnectionManager) -> Self {
        Self {
            db: db,
            redis: redis
        }
    }

    pub async fn add_user_to_leaderboard_if_not_exists(&self, contest_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        let mut redis_conn = self.redis.clone();

        let score = redis_conn.zscore(contest_id.to_string(), user_id.to_string()).await?;
        if score.is_none() {
            redis_conn.zadd(contest_id.to_string(), user_id.to_string(), 0).await?;
        }

        Ok(())
    }   

    pub async fn update_score(&self, contest_id: Uuid, user_id: Uuid, delta_score: i32) -> anyhow::Result<()> {
        let mut redis_conn = self.redis.clone();

        redis_conn.zincr(contest_id.to_string(), user_id.to_string(), delta_score).await?;
        
        Ok(())
    }

    pub async fn get_user_rank(&self, contest_id: Uuid, user_id: Uuid) -> anyhow::Result<i32> {
        let mut redis_conn = self.redis.clone();

        let rank_score_result: Result<(i32, String), redis::RedisError> = redis::cmd("ZREVRANK")
            .arg(contest_id.to_string())
            .arg(user_id.to_string())
            .arg("WITHSCORE")
            .query_async(&mut redis_conn)
        .await?;    

        if let Err(e) = rank_score_result {
            return Err(anyhow!(e));
        }

        let (rank, score) = rank_score_result.unwrap();
        
        if rank == 0 {
            if score == "0" {
                return Ok(-1);
            }
        }

        return Ok(rank + 1);
    }

    pub async fn publish_leaderboard_to_db(&self, contest_id: Uuid) -> anyhow::Result<()> {
        let mut redis_conn = self.redis.clone();

        let leaderboard: Vec<(String, i32)> = redis::cmd("ZREVRANGE")
            .arg(contest_id.to_string())
            .arg(0)
            .arg(-1)
            .arg("WITHSCORES")
            .query_async(&mut redis_conn)
        .await?;
        
        let user_id_scores: Vec<(Uuid, i32)> = leaderboard.iter().map(
            |l| 
                (
                    Uuid::from_str(&l.0).unwrap(),
                    l.1
                )   
        ).collect();

        self.db.add_leaderboard(contest_id, user_id_scores).await?;

        Ok(())
    }

    pub async fn remove_leaderboard(&self, contest_id: Uuid) -> anyhow::Result<()> {
        let mut redis_conn = self.redis.clone();
        redis_conn.del(contest_id.to_string()).await?;

        Ok(())
    }

}