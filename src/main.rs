#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod challenge;

#[cfg(test)] mod tests;

use rocket::{
    Rocket,
    response::status,
    http::Status,
};
use diesel::SqliteConnection;

use rocket_contrib::json::{Json, JsonValue};

use challenge::{Challenge, NewChallenge};

#[database("splat_challenges")]
pub struct DbConn(SqliteConnection);

#[get("/", format = "json")]
fn index() -> JsonValue {
    json!({
        "error": false,
        "details": "Welcome to Splat Challenges API!",
        "nav": [
            "http://localhost:8000/challenges"
        ]
    })
}

#[get("/", format = "json")]
fn list(conn: DbConn) -> Json<Vec<Challenge>> {
    Json(Challenge::list(&conn))
}

#[get("/<id>", format = "json")]
fn get(id: i32, conn: DbConn) -> Result<Json<Challenge>, status::NotFound<String>>  {
    match Challenge::get(id, &conn) {
        Some(challenge) => Ok(Json(challenge)),
        _ => Err(status::NotFound(format!("Unable to find challenge with ID ({})", id))),
    }
}

#[post("/", format = "json", data = "<input>")]
fn new(input: Json<Vec<NewChallenge>>, conn: DbConn) -> Result<status::Created<JsonValue>, status::Custom<String>> {
    let added_ids: Vec<i32> = Challenge::add(input.into_inner(), &conn)
        .into_iter()
        .map( |challenge| challenge.id )
        .collect();
    match added_ids.len() {
        0 => Err(status::Custom(Status::UnprocessableEntity,
                                "Unable to store given challenges".to_string())),
        _ => Ok(status::Created(
                added_ids
                    .clone()
                    .into_iter()
                    .map( |id| format!("http://localhost:8000/challenges/{}", id) )
                    .collect(),
                Some(json!({
                    "error": false,
                    "details": "Stored given challenges successfully",
                    "modified_ids": added_ids}))),
            ),
    }
}

#[delete("/<id>", format = "json")]
fn delete(id: i32, conn: DbConn) -> Result<status::Accepted<JsonValue>, status::Custom<String>> {
    let removed_challenge = Challenge::remove(id, &conn);
    match removed_challenge {
        Ok(challenge) => Ok (status::Accepted(Some(json!({
            "error": false,
            "details": "Challenge with give ID has been deleted",
            "modified_ids": [
                challenge.id
            ]})))),
        Err(challenge::ChallengeError::NotFoundError) =>
            Err(status::Custom(
                    Status::NotFound,
                    format!("Unable to delete challenge with id ({})", id))),
        Err(challenge::ChallengeError::DatabaseError) =>
            Err(status::Custom(
                    Status::InternalServerError,
                    format!("Encountered internal error trying to delete id ({})", id))),
    }
}

#[catch(400)]
fn bad_request() -> JsonValue {
    json!({
        "error": true,
        "details": "Unable to parse given body."
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "error": true,
        "details": "Couldn't find that resource."
    })
}

#[catch(422)]
fn unprocessable() -> JsonValue {
    json!({
        "error": true,
        "details": "Couldn't process body data in given request."
    })
}

#[catch(500)]
fn internal_error() -> JsonValue {
    json!({
        "error": true,
        "details": "Server made some unhandled poop trying to process request."
    })
}

fn rocket() -> (Rocket, Option<DbConn>) {
    let rocket = rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
        .mount("/challenges", routes![
               list,
               get,
               new,
               delete
        ])
        .register(catchers![
                  bad_request,
                  not_found,
                  unprocessable,
                  internal_error,
        ]);

    let conn = match cfg!(test) {
        true => DbConn::get_one(&rocket),
        false => None,
    };

    (rocket, conn)
}

fn main() {
    rocket().0.launch();
}
