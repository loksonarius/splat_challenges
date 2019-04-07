#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use diesel::SqliteConnection;
use rocket::{
    Rocket,
    http::Status,
    response::status,
};
use rocket_contrib::json::{
    Json,
    JsonValue,
};

use challenge::{
    Challenge,
    NewChallenge,
};
use errors::ChallengeError::{
    // DatabaseError,
    NotFoundError,
    // SerializationError,
};

mod challenge;
mod errors;
mod schema;

#[cfg(test)] mod tests;

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
fn list(conn: DbConn) -> Result<Json<Vec<Challenge>>, status::Custom<String>> {
    match Challenge::list(&conn) {
        Ok(list) => Ok(Json(list)),
        Err(_) =>
            Err(status::Custom(
                    Status::InternalServerError,
                    format!("Encountered internal error trying to list challenges"))),

    }
}

#[get("/<id>", format = "json")]
fn get(id: i32, conn: DbConn) -> Result<Json<Challenge>, status::NotFound<String>>  {
    match Challenge::get(id, &conn) {
        Ok(challenge) => Ok(Json(challenge)),
        _ => Err(status::NotFound(format!("Unable to find challenge with ID ({})", id))),
    }
}

#[post("/", format = "json", data = "<input>")]
fn new(input: Json<Vec<NewChallenge>>, conn: DbConn) -> Result<status::Created<JsonValue>, status::Custom<String>> {
    let result = Challenge::add(input.into_inner(), &conn);
    let id_url_list = |challenges: &Vec<Challenge>| -> Vec<String> {
        challenges
            .into_iter()
            .map( |challenge| format!("http://localhost:8000/challenges/{}", challenge.id) )
            .collect()
    };

    match result {
        Ok(challenges) => {
            let ids = id_url_list(&challenges);
            Ok(status::Created(
                    ids.join(","),
                    Some(json!({
                            "error": false,
                            "details": "Stored given challenges successfully",
                            "modified_ids": ids}))))
        },
        Err(_) => Err(status::Custom(Status::InternalServerError,
                           "Encountered internal error trying to store challenges".to_string())),
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
        Err(NotFoundError) =>
            Err(status::Custom(
                    Status::NotFound,
                    format!("No such challenge with id ({})", id))),
        Err(_) =>
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
