use diesel;
use diesel::prelude::*;
use chrono::prelude::*;
use crate::schema::schedules::{self};
use crate::schema::users;
// use crate::schema::schedules::dsl::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user_id: i32,
    pub title: String,
    pub content: String
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct DisplaySchedule {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub nickname: String,
    pub title: String,
    pub content: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "schedules"]
pub struct NewSchedule<'a> {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user_id: i32,
    pub title: &'a str,
    pub content: &'a str
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "schedules"]
pub struct UpdateSchedule<'a> {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub title: Option<&'a str>,
    pub content: Option<&'a str>
}

impl Schedule {
    pub fn read_by_cond(start: Option<NaiveDateTime>, end: Option<NaiveDateTime>, user_id: Option<i32>, connection: &PgConnection) -> QueryResult<Vec<DisplaySchedule>>  {
        let mut query = schedules::table.inner_join(users::table).into_boxed();

        if let Some(date) = start {
            query = query.filter(schedules::start.ge(date));
        }
        if let Some(date) = end {
            query = query.filter(schedules::end.le(date));
        }
        if let Some(id) = user_id {
            query = query.filter(schedules::user_id.eq(id));
        }
        query.select((schedules::id, schedules::start, schedules::end, users::nickname, schedules::title, schedules::content))
            .load::<DisplaySchedule>(&*connection)
    }

    pub fn read_by_id(id: i32, connection: &PgConnection) -> QueryResult<DisplaySchedule> {
        schedules::table.inner_join(users::table)
            .filter(schedules::id.eq(id))
            .select((schedules::id, schedules::start, schedules::end, users::nickname, schedules::title, schedules::content))
            .get_result::<DisplaySchedule>(connection)
    }

    pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(schedules::table.find(id))
                .execute(connection)
    }
}

impl<'a> NewSchedule<'a> {
    pub fn create(new_schedule: NewSchedule, connection: &PgConnection) -> QueryResult<Schedule> {
        diesel::insert_into(schedules::table)
            .values(&new_schedule)
            .get_result(connection)
    }
}

impl<'a> UpdateSchedule<'a> {
    pub fn update(id: i32, schedule: UpdateSchedule, connection: &PgConnection) -> QueryResult<Schedule> {
        diesel::update(schedules::table.find(id))
                .set(&schedule)
                .get_result(connection)
    }
}