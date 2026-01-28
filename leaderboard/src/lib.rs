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

    pub async fn add_user_to_leaderboard(&self, contest_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
        let mut redis_conn = self.redis.clone();

        redis_conn.zadd(contest_id.to_string(), user_id.to_string(), 0).await?;
        
        Ok(())
    }   

    pub async fn update_score(&self, contest_id: Uuid, user_id: Uuid, delta_score: i16) -> anyhow::Result<()> {
        let mut redis_conn = self.redis.clone();

        redis_conn.zincr(contest_id.to_string(), user_id.to_string(), delta_score).await?;
        
        Ok(())
    }

    pub async fn get_user_rank(&self, contest_id: Uuid, user_id: Uuid) -> anyhow::Result<i32> {
        let mut redis_conn = self.redis.clone();

        let rank_option = redis_conn.zrevrank(contest_id.to_string(), user_id.to_string()).await?;
        if rank_option.is_some() {
            return Ok(rank_option.unwrap() as i32);
        }

        return Err(anyhow!("User does not exit in leaderboard"));
    }

    pub async fn publish_leaderboard_to_db(&self, contest_id: Uuid) -> anyhow::Result<()> {
        let mut redis_conn = self.redis.clone();

        let leaderboard: Vec<(String, f64)> = redis::cmd("ZREVRANGE")
            .arg(contest_id.to_string())
            .arg(0)
            .arg(-1)
            .arg("WITHSCORES")
            .query_async(&mut redis_conn)
        .await?;
        
        let user_id_scores: Vec<(Uuid, i16)> = leaderboard.iter().map(
            |l| 
                (
                    Uuid::from_str(&l.0).unwrap(),
                    l.1 as i16
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