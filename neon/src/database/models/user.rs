#[derive(Debug, Queryable)]
pub struct UserModel {
    pub id:                i32,
    pub username:          String,
    pub password_md5:      String,
    pub email:             String,
    pub register_datetime: i32,
    pub latest_activity:   i32,
    pub privileges:        i32,
    pub donor_expire:      i32,
}
