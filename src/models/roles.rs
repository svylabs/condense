use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::roles)]
pub struct NewRole {
    pub name: String,
    pub description: String,
}

impl Role {
    pub fn find_by_name(role_name: &str, conn: &mut PgConnection) -> Result<Role, diesel::result::Error> {
        use crate::schema::roles::dsl::*;
        roles.filter(name.eq(role_name))
            .first(conn)
            .map_err(|e| e.into())
    }
}