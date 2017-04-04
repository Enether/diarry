#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{DiaryEntry, NewDiaryEntry, ErrorDetails};
use rocket::response::status;
use rocket::http::Status;
use rocket::response::content;
use rocket_contrib::JSON;
use rocket::response::status::{Created};

pub fn create_diary_entry<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> DiaryEntry {
    /* Creates a new DiaryEntry in the database */
    use schema::diary_entries;

    let new_entry = NewDiaryEntry {
        title: String::from(title),
        body: String::from(body),
    };

    diesel::insert(&new_entry).into(diary_entries::table)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn fetch_diary_entry(conn: &PgConnection, id: i32) -> Option<DiaryEntry> {
    /* Given the ID, queries the database for a DiaryEntry row and returns a DiaryEntry struct */
    use self::schema::diary_entries::dsl::diary_entries;
    let result = diary_entries.find(id).first(conn);
    match result {
        Ok(r) => Some(r),
        Err(r) => None
    }
}

pub fn fetch_all_diary_entries(conn: &PgConnection) -> Vec<DiaryEntry> {
    /* Returns a Vector of all the Diary Entries*/
    use self::schema::diary_entries::dsl::diary_entries;
    diary_entries.load::<DiaryEntry>(conn).unwrap()
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[post("/api/entries/new", format = "application/json", data = "<new_entry>")]
fn new_diary_controller(new_entry: JSON<NewDiaryEntry>) -> Result<Created<JSON<DiaryEntry>>, status::Custom<JSON<ErrorDetails>>>{
    if new_entry.body.len() <= 3 || new_entry.title.len() <= 3 {
        let json_err = JSON(ErrorDetails{ error_message: String::from("The length of the body and title must be greater than 3 characters!") });
        return Err(status::Custom(Status::BadRequest, json_err));
    }
    let new_entry: DiaryEntry = create_diary_entry(&establish_connection(), new_entry.title.as_str(), new_entry.body.as_str());

    Ok(Created(new_entry.get_absolute_url(), None))
}

#[get("/api/entries/<id>")]
fn diary_details_controller(id: i32) -> Option<JSON<DiaryEntry>> {
    if let Some(diary) = fetch_diary_entry(&establish_connection(), id) {
        return Some(JSON(diary))
    } else {
        return None
    }
}

#[get("/api/entries/all")]
fn all_diary_entries_controller() -> JSON<Vec<DiaryEntry>> {
    let connection: PgConnection = establish_connection();
    JSON(fetch_all_diary_entries(&connection))
}

fn main() {
    fetch_all_diary_entries(&establish_connection());
    rocket::ignite()
        .mount("/", routes![new_diary_controller, diary_details_controller, all_diary_entries_controller])
        .launch();
}

#[cfg(test)]
mod tests {
    use establish_connection;
    #[test]
    fn test_connects_to_db() {
        // should not panic
        establish_connection();
    }

    use {fetch_diary_entry, diary_details_controller};
    #[test]
    fn test_details_should_return_correct_entry() {
        let expected_entry = fetch_diary_entry(&establish_connection(), 1).unwrap();
        assert_eq!(diary_details_controller(1).unwrap().into_inner(), expected_entry);
    }
    #[test]
    fn test_details_should_return_none_with_incorrect_id() {
        assert!(diary_details_controller(i32::max_value()).is_none())
    }
    // mod schema;
    use schema::diary_entries::dsl::diary_entries;
    use diesel::prelude::*;
    
    use diesel::pg::PgConnection;
    use models::DiaryEntry;
    #[test]
    fn test_fetch_diary_entry_should_return_corresponding_entry() {
        let connection: PgConnection = establish_connection();
        let expected_entry: QueryResult<DiaryEntry> = diary_entries.find(1).first(&connection);
        assert_eq!(fetch_diary_entry(&connection, 1).unwrap(), expected_entry.unwrap());
    }
    #[test]
    fn test_fetch_diary_entry_non_existing_id_should_return_none() {
        let connection: PgConnection = establish_connection();
        assert!(fetch_diary_entry(&connection, i32::max_value()).is_none());
    }

    use fetch_all_diary_entries;
    #[test]
    fn test_fetch_all_diary_entries_should_return_them_all() {
        let connection: PgConnection = establish_connection();
        let expected_entries: Vec<DiaryEntry> = diary_entries.load::<DiaryEntry>(&connection).unwrap();

        assert_eq!(fetch_all_diary_entries(&connection), expected_entries);
    }

    use rocket_contrib::JSON;
    use all_diary_entries_controller;
    #[test]
    fn test_all_diary_entries_controller_should_return_all_entries_in_json() {
        let connection: PgConnection = establish_connection();
        let expected_entries: JSON<Vec<DiaryEntry>> = JSON(diary_entries.load::<DiaryEntry>(&connection).unwrap());

        assert_eq!(all_diary_entries_controller().into_inner(), expected_entries.into_inner());
    }
}