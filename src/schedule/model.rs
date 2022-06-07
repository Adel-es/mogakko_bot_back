use diesel;
use diesel::prelude::*;
use chrono::prelude::*;
use crate::schema::schedules::{self};
// use crate::schema::schedules::dsl::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user_id: String,
    pub title: String,
    pub content: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "schedules"]
pub struct NewSchedule<'a> {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user_id: &'a str,
    pub title: &'a str,
    pub content: &'a str
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "schedules"]
pub struct UpdateSchedule<'a> {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub user_id: Option<&'a str>,
    pub title: Option<&'a str>,
    pub content: Option<&'a str>
}

impl Schedule {
    pub fn read_by_cond(start: Option<NaiveDateTime>, end: Option<NaiveDateTime>, user_id: Option<&str>, connection: &PgConnection) -> QueryResult<Vec<Schedule>>  {
        let mut query = schedules::table.into_boxed();

        if let Some(date) = start {
            query = query.filter(schedules::start.ge(date));
        }
        if let Some(date) = end {
            query = query.filter(schedules::end.le(date));
        }
        if let Some(id) = user_id {
            query = query.filter(schedules::user_id.eq(id));
        }
        query.load::<Schedule>(&*connection)
    }

    pub fn read_by_id(id: i32, connection: &PgConnection) -> QueryResult<Schedule> {
        schedules::table.find(id).get_result::<Schedule>(connection)
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