use super::{APIError, ErrorResponse, LoginTeacher, RegisterTeacher};

pub async fn register_teacher(email: &str, username: &str, password: &str) -> Result<(), APIError> {
    let ret = reqwest::Client::new()
        .post("http://localhost:8080/v1/teacher/register")
        .json(&RegisterTeacher {
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?;

    match ret.status().is_success() {
        false => {
            let resp = ret.json::<ErrorResponse>().await?;
            let err = match resp.errors {
                Some(validation_errs) => APIError::Validation(validation_errs),
                None => APIError::ServerResponse(resp.message),
            };
            Err(err)
        }
        _ => Ok(()),
    }
}

pub async fn login_teacher(email: &str, password: &str) -> Result<(), APIError> {
    let ret = reqwest::Client::new()
        .post("http://localhost:8080/v1/teacher/login")
        .json(&LoginTeacher {
            email: email.to_string(),
            password: password.to_string(),
        })
        .send()
        .await?;

    match ret.status().is_success() {
        false => {
            let resp = ret.json::<ErrorResponse>().await?;
            let err = match resp.errors {
                Some(validation_errs) => APIError::Validation(validation_errs),
                None => APIError::ServerResponse(resp.message),
            };
            Err(err)
        }
        _ => Ok(()),
    }
}

pub async fn register_student(name: &str) -> Result<(), String> {
    Ok(())
}
