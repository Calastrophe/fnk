use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FilteredTeacher {
    pub id: String,
    pub name: String,
    pub email: String,
}

// For populating the dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct Tests {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Results {}
