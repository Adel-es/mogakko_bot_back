use diesel;
use diesel::prelude::*;
use chrono::prelude::*;
use crate::schema::schedules;
// use crate::schema::schedules::dsl::*;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "schedules"]
pub struct Schedule {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user_id: String,
    pub title: Option<String>,
    pub content: Option<String>
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="schedules"]
pub struct NewSchedule {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user_id: String,
    pub title: Option<String>,
    pub content: Option<String>
}

impl Schedule {
    pub fn read_all(connection: &PgConnection) -> QueryResult<Vec<Schedule>>  {
        // schedules::table.filter(.eq(true))
        schedules::table.load::<Schedule>(&*connection)
    }

    pub fn read(id: i32, connection: &PgConnection) -> QueryResult<Schedule> {
        schedules::table.find(id).get_result::<Schedule>(connection)
    }

    pub fn update(id: i32, schedule: Schedule, connection: &PgConnection) -> QueryResult<Schedule> {
        diesel::update(schedules::table.find(id))
                .set(&schedule)
                .get_result(connection)
    }

    pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(schedules::table.find(id))
                .execute(connection)
    }
}

impl NewSchedule {
    pub fn create(new_schedule: NewSchedule, connection: &PgConnection) -> QueryResult<Schedule> {
        diesel::insert_into(schedules::table)
            .values(&new_schedule)
            .get_result(connection)
    }
}