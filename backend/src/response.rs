use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredTeacher {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredTeacher,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}
