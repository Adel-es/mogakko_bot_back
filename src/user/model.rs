use diesel;
use diesel::prelude::*;
use chrono::prelude::*;
use crate::schema::users;
use crate::schema::schedules;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pw: String,
    pub nickname: String,
    pub discord_id: String,
    pub created: NaiveDateTime,
    pub memo: String
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub nickname: String,
    pub discord_id: String,
    pub created: NaiveDateTime,
    pub memo: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pw: String,
    pub nickname: &'a str,
    pub discord_id: &'a str,
    pub created: Option<NaiveDateTime>
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub username: Option<&'a str>,
    pub pw: Option<String>,
    pub memo: Option<&'a str>
}

impl User {
    pub fn read(id: i32, connection: &PgConnection) -> QueryResult<User> {
        users::table.find(id)
            .get_result::<User>(connection)
    }

    pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
        diesel::update(schedules::table.filter(schedules::user_id.eq(id)))
                .set(schedules::user_id.eq(1))
                .execute(connection)?;
        diesel::delete(users::table.find(id))
                .execute(connection)
    }

    pub fn to_response(self) -> UserResponse {
        UserResponse { 
            id: self.id, 
            username: self.username, 
            nickname: self.nickname, 
            discord_id: self.discord_id, 
            created: self.created, 
            memo: self.memo 
        }
    }
}

impl<'a> NewUser<'a> {
    pub fn create(new_user: NewUser, connection: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(connection)
    }
}

impl<'a> UpdateUser<'a> {
    pub fn update(id: i32, user: UpdateUser, connection: &PgConnection) -> QueryResult<User> {
        diesel::update(users::table.find(id))
                .set(&user)
                .get_result(connection)
    }
}