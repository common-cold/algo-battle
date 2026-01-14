use std::env;

use anyhow::Ok;
use common::{Contest, ContestStatus, CreateContestArgs, CreateQuestionArgs, CreateUserArgs, GetContestArgs, Question, QuestionType, Role, User};
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

    pub async fn get_questions_by_id(&self, question_ids: Vec<Uuid>) -> anyhow::Result<Vec<Question>> {
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
}