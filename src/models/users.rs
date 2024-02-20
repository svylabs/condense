use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
}

impl User {
    pub fn find_by_username(name: &str, conn: &mut PgConnection) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        users.filter(crate::schema::users::username.eq(name))
            .first(conn)
            .map_err(|e| e.into())
    }

    pub fn find_users_by_ids(user_ids: Vec<i32>, conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        users.filter(id.eq_any(user_ids))
            .load(conn)
            .map_err(|e| e.into())
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
}
