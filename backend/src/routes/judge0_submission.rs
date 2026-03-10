use actix_web::{HttpResponse, get, post, put, web::{self, Data, Json}};
use common::{CompactTestcase, FetchAttemptIdArgs, Judge0SubmissionCallBackArgs, JwtClaims, SubmissionStatusResponse};
use serde_json::json;

use crate::{AppData};



#[put("/judge0-submission/callback")]
pub async fn judge0_submission_callback(data: Data<AppData>, body: web::Json<Judge0SubmissionCallBackArgs>) -> HttpResponse {
    let db = &data.db;

    let submission_result = db.get_dsa_submission_by_submission_id(body.token.clone()).await;
    if let Err(e) = submission_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    let submission = submission_result.unwrap();

    let update_submission_result = db.update_dsa_submission_by_submission_id(submission.submission_id, body.status.description.clone(), body.compile_output.clone()).await;
    if let Err(e) = update_submission_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    return HttpResponse::Ok().finish();
}

#[get("/judge0-submission/status/{attempt_id}")]
pub async fn get_judge0_submission_status(data: Data<AppData>, path: web::Path<String>) -> HttpResponse {
    let attempt_id = path.into_inner();
    let db = &data.db;

    let submissions_result = db.get_dsa_submissions_by_attempt_id(attempt_id).await;
    if let Err(e) = submissions_result {
        return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }));
    }

    let submissions = submissions_result.unwrap();
    
    let mut total_testcases = 0;
    let mut pending = 0;
    let mut failed = 0;
    let mut passed = 0;
    let mut failed_testcase: Option<CompactTestcase> = None;
    let mut compile_result: Option<String> = None;

    for submission in submissions {
        total_testcases += 1;
        if submission.status == "Pending" {
            pending += 1;
        } else if submission.status == "Accepted" {
            passed += 1;
        } else {
            failed += 1;
            if failed == 1 {
                let testcase_result = db.get_testcase_by_id(submission.testcase_id).await;
                if let Err(e) = testcase_result {
                    return HttpResponse::InternalServerError().json(json!({
                        "error": e.to_string()
                    }));
                }
                let testcase = testcase_result.unwrap();
                failed_testcase = Some(CompactTestcase {
                    input: testcase.input,
                    output: testcase.output
                });
                if submission.compile_result.is_some() {
                    compile_result = submission.compile_result;
                }
            }
        }
    }

    let response = SubmissionStatusResponse {
        total_testcases: total_testcases,
        pending: pending,
        passed: passed,
        failed: failed,
        failed_testcase: failed_testcase,
        compile_result: compile_result
    };     

    return HttpResponse::Ok().json(response);
}

#[post("/judge0-submission/fetch/active/attemptId")]
pub async fn get_active_attempt_id(data: Data<AppData>, body: Json<FetchAttemptIdArgs>, jwt_claims: JwtClaims) -> HttpResponse {
    let db = &data.db;

    let attempt_id_result = db.get_active_attempt_id(jwt_claims.id, body.contest_id, body.problem_id).await;
    if let Err(e) = attempt_id_result {
       return HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        })); 
    }
    
    let attempt_id_option = attempt_id_result.unwrap();
    let found = attempt_id_option.is_some();

    return HttpResponse::Ok().json(json!({
        "attempt_id": attempt_id_option,
        "found": found
    }));
}