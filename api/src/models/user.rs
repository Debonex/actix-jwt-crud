use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub count: i32,
}

impl From<entity::user::Model> for UserInfo {
    fn from(value: entity::user::Model) -> Self {
        UserInfo {
            name: value.name,
            count: value.count,
        }
    }
}
