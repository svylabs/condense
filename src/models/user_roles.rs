use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::{roles::Role};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32
}

impl UserRole {
    pub fn find_by_user_id(usr_id: i32, conn: &mut PgConnection) -> QueryResult<Vec<UserRole>> {
        use crate::schema::user_roles::dsl::*;
        user_roles.filter(user_id.eq(usr_id)).load(conn)
    }

    pub fn find_users_by_role_id(id_role: i32, conn: &mut PgConnection) -> Result<Vec<UserRole>, diesel::result::Error> {
        use crate::schema::user_roles::dsl::*;
        user_roles.filter(role_id.eq(id_role))
            .load(conn)
            .map_err(|e| e.into())
    }
}


#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32
}

impl NewUserRole {
    pub fn new(user_id: i32, roles: &Vec<Role>) -> Vec<NewUserRole> {
        roles.into_iter().map(|role| {
            NewUserRole {
                user_id,
                role_id: role.id
            }
        }).collect::<Vec<NewUserRole>>()
    }
}