use std::env;

use anyhow::Ok;
use common::{Contest, ContestStatus, CreateContestArgs, CreateQuestionArgs, CreateUserArgs, GetContestArgs, Question, QuestionType, QuestionWithoutAnswer, Role, User};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, types::Uuid};
use dotenv::dotenv;


#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>
}

impl Database {
    pub async fn init_db() -> anyhow::Result<Self> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;
        
        Ok(Self {
            pool: pool
        })
    }

    //user
    pub async fn create_user(&self, args: CreateUserArgs) -> anyhow::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO USERS (name, email, password, role)
                VALUES ($1, $2, $3, $4)
                RETURNING 
                    id, 
                    email, 
                    name, 
                    password, 
                    role AS "role: Role", 
                    created_at
            "#,
            args.name,
            args.email,
            args.password,
            args.role as Role
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> anyhow::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT
                    id, 
                    email, 
                    name, 
                    password, 
                    role AS "role: Role", 
                    created_at
                FROM USERS
                WHERE id = $1    
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    
    //question
    pub async fn create_question(&self, args: CreateQuestionArgs) -> anyhow::Result<Question> {
        let question = sqlx::query_as!(
            Question,
            r#"
                INSERT INTO QUESTIONS (
                    question_type, 
                    title, 
                    description, 
                    options,
                    correct_option,
                    time_limit,
                    points,
                    owner_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING 
                    id, 
                    question_type AS "question_type: QuestionType", 
                    title, 
                    description, 
                    options,
                    correct_option,
                    time_limit,
                    points,
                    owner_id,
                    created_at
            "#,
            args.question_type as QuestionType,
            args.title,
            args.description,
            args.options.as_slice(),
            args.correct_option,
            args.time_limit,
            args.points,
            args.owner_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(question)
    }

    pub async fn get_questions_by_id(&self, question_ids: Vec<Uuid>) -> anyhow::Result<Vec<QuestionWithoutAnswer>> {
        let questions = sqlx::query_as!(
            QuestionWithoutAnswer,
            r#"
                SELECT 
                    id,
                    question_type AS "question_type: QuestionType",
                    title,
                    description,
                    options,
                    time_limit,
                    points,
                    owner_id,
                    created_at
                FROM QUESTIONS WHERE
                   id = ANY($1)     
            "#,
            &question_ids
        ).fetch_all(&self.pool)
        .await?;
        
        Ok(questions)
    }

    pub async fn get_all_examiner_questions(&self, examiner_id: Uuid) -> anyhow::Result<Vec<Question>> {
        let questions = sqlx::query_as!(
            Question,
            r#"
                SELECT 
                    id,
                    question_type AS "question_type: QuestionType",
                    title,
                    description,
                    options,
                    correct_option,
                    time_limit,
                    points,
                    owner_id,
                    created_at
                FROM QUESTIONS WHERE
                   owner_id = $1     
            "#,
            examiner_id
        ).fetch_all(&self.pool)
        .await?;
        
        Ok(questions)
    }

    //contest
    pub async fn create_contest(&self, args: CreateContestArgs) -> anyhow::Result<Contest> {
        let contest = sqlx::query_as!(
            Contest,
            r#"
                INSERT INTO CONTESTS (
                    title,
                    description,
                    start_date,
                    end_date,
                    status,
                    owner_id
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING 
                    id, 
                    title,
                    description,
                    start_date,
                    end_date,
                    status AS "status: ContestStatus",
                    owner_id,
                    created_at
            "#,
            args.title,
            args.description,
            args.start_date,
            args.end_date,
            ContestStatus::Scheduled as ContestStatus,
            args.owner_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(contest)
    }

    pub async fn get_all_examiner_contests(&self, args: GetContestArgs) -> anyhow::Result<Vec<Contest>> {
        let contests = sqlx::query_as!(
            Contest,
            r#"
                SELECT 
                    id,
                    title,
                    description,
                    start_date,
                    end_date,
                    status AS "status: ContestStatus",
                    owner_id,
                    created_at
                FROM CONTESTS WHERE
                   owner_id = $1
                AND status = $2       
            "#,
            args.id,
            args.status as ContestStatus
        ).fetch_all(&self.pool)
        .await?;
        
        Ok(contests)
    }

    pub async fn get_contest_by_id(&self, contest_id: Uuid) -> anyhow::Result<Contest> {
        let contest = sqlx::query_as!(
            Contest,
            r#"
                SELECT 
                    id,
                    title,
                    description,
                    start_date,
                    end_date,
                    status AS "status: ContestStatus",
                    owner_id,
                    created_at
                FROM CONTESTS WHERE
                   id = $1
            "#,
            contest_id,
        ).fetch_one(&self.pool)
        .await?;

        Ok(contest)
    }

    
    //contest_question join table
    pub async fn create_contest_question_join_entry(&self, contest_id: Uuid, question_ids: &Vec<Uuid>) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"
                INSERT INTO CONTEST_QUESTION (contest_id, question_id, position)
                SELECT $1, qid, row_number() OVER() * 10
                FROM UNNEST($2::uuid[]) AS qid
            "#,
            contest_id,
            &question_ids
        ).execute(&mut *tx)
        .await?;
        
        tx.commit().await?;

        Ok(())
    }

    pub async fn get_all_question_ids_for_contest_id(&self, contest_id: Uuid) -> anyhow::Result<Vec<Uuid>> {
        let questions = sqlx::query!(
            r#"
                SELECT 
                    question_id
                FROM CONTEST_QUESTION   
                WHERE 
                    contest_id = $1  
                ORDER BY position    
            "#,
            contest_id,
        ).fetch_all(&self.pool)
        .await?
        .iter()
        .map(|r| r.question_id)
        .collect::<Vec<Uuid>>();
        

        Ok(questions)
    }
    

    //contest_attempts table
    pub async fn create_contest_attempts_entry(&self, user_id: Uuid, contest_id: Uuid,) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"
                INSERT INTO CONTEST_ATTEMPTS (user_id, contest_id)
                VALUES($1, $2)
            "#,
            user_id,
            contest_id
        ).execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_contest_joined_at(&self, user_id: Uuid, contest_id: Uuid) -> anyhow::Result<i64> {
        let record = sqlx::query!(
            r#"
                SELECT 
                    joined_at
                FROM CONTEST_ATTEMPTS 
                WHERE 
                    user_id = $1
                    AND contest_id = $2 
            "#,
            user_id,
            contest_id
        ).fetch_one(&self.pool)
        .await?;

        Ok(record.joined_at)
    }

}