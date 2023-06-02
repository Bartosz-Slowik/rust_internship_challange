use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub api_key: String,
}

impl User {
    pub fn find(conn: &PgConnection, id: Uuid) -> Result<Self, diesel::result::Error> {
        users::table.filter(users::id.eq(id)).first(conn)
    }

    pub fn find_by_email(conn: &PgConnection, email: &str) -> Result<Self, diesel::result::Error> {
        users::table.filter(users::email.eq(email)).first(conn)
    }
}

#[table_name = "users"]
#[derive(Debug, Insertable)]
pub struct NewUser {
    pub email: String,
    pub api_key: String,
}

pub fn create_user(
    conn: &PgConnection,
    email: &str,
    api_key: &str,
) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        email: email.to_string(),
        api_key: api_key.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod integration {
        use super::*;

        use crate::db_connection::testing::create_testing_pool;

        #[test]
        fn create_and_get_user() {
            let testing_pool = create_testing_pool();
            let conn = testing_pool.get().unwrap();

            let user_email = "email@domain.com";

            let new_user = create_user(&conn, "email@domain.com", "SECRETAPIKEY")
                .expect("Failed to create user");

            let existing_user =
                User::find(&conn, new_user.id).expect("Failed to find existing user");

            assert_eq!(existing_user.email, user_email);
        }
    }
}
