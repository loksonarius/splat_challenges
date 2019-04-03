extern crate test_case_derive;
extern crate parking_lot;

use crate::challenge::{
    Challenge,
    NewChallenge,
};
use self::parking_lot::Mutex;
use self::test_case_derive::test_case;

use rocket::{
    local::{
        Client,
        LocalResponse,
    },
    http::{
        Status,
        ContentType,
    },
};

// I pulled this straight from Rocket's ToDo Example app tests.rs
// https://github.com/SergioBenitez/Rocket/blob/v0.4/examples/todo/src/tests.rs
static DB_LOCK: Mutex<()> = Mutex::new(());
macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();
        let (rocket, db) = crate::rocket();
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");

        // Clear DB and re-seed with desired initial state
        Challenge::remove_all(&$conn);
        Challenge::add(
            vec![
                NewChallenge {
                    title: "Sample test".to_string(),
                    description: "I'm a toughie!".to_string(),
                },
                NewChallenge {
                    title: "Another sample test".to_string(),
                    description: "I'm not that bad, honestly~".to_string(),
                },
            ],
            &$conn
        );
        $block
    })
}

fn make_response_assertions(mut res: LocalResponse, status: Status, body_substrings: Vec<&str>) {
    assert_eq!(res.status(), status);

    let body = res.body_string().unwrap_or("".to_string());
    for substring in body_substrings {
        assert!(body.contains(substring),
            format!("Expected body to contain:\n{}.\nBody:\n{}", substring, body));
    }
}

#[test_case("",
            Status::BadRequest,
            vec![]
            :: "empty body is a bad request")]
#[test_case(r#"{"lol": "I'm missing a closing double quote!}"#,
            Status::UnprocessableEntity,
            vec![]
            :: "misformatted json is unprocessable")]
#[test_case(r#"[{"title": "I'm missing a description!"}]"#,
            Status::UnprocessableEntity,
            vec![]
            :: "missing fields yields bad request")]
#[test_case("[]",
            Status::UnprocessableEntity,
            vec![]
            :: "empty batch is unprocessable")]
#[test_case(r#"[{
            "title": "Basic baby!",
            "description": "Desc goes here"}]"#,
            Status::Created,
            vec![]
            :: "single well-defined challenge is ok")]
#[test_case(r#"[
            {"title": "Batch 1!", "description": "Foo"},
            {"title": "Batch 2!", "description": "Foo"}]"#,
            Status::Created,
            vec![]
            :: "batch of well-defined challenges is ok")]
// I'm not sure this case is worth testing, so I'm not sure it's worth getting it working either
// #[test_case(format!(
//                 r#"[ {"title": "Big Chungus!", "description": "{}"}]"#,
//                 "Massive Lad".repeat(400)),
//             Status::Ok,
//             vec![]
//             :: "single giant challenge is ok")]
#[test_case(r#"[{
            "title": "Fancy 👶!",
            "description": "FEED ME 🍼🍼🍼"}]"#,
            Status::Created,
            vec![]
            :: "challenge containing emojii is ok")]
fn challenge_insertion(body: &str, expected_status: Status, expected_substrings: Vec<&str>) {
    run_test!(|client, conn| {
        let mut res = client.post("/challenges")
            .header(ContentType::JSON)
            .body(body)
            .dispatch();
        make_response_assertions(res, expected_status, expected_substrings);
    });
}

#[test_case("/",
            Status::Ok,
            vec!["Welcome to Splat Challenges API!", "nav"]
            :: "index page is ok")]
#[test_case("/challenges",
            Status::Ok,
            vec!["Sample test", "Another sample test"]
            :: "challenge index is ok")]
#[test_case("/challenges/-1",
            Status::NotFound,
            vec![]
            :: "challenge at invalid numeric index not found")]
#[test_case("/challenges/lolthisdonotexist",
            Status::NotFound,
            vec![]
            :: "challenge at invalid non-numeric index not found")]
fn challenge_reading(endpoint: &str, expected_status: Status, expected_substrings: Vec<&str>) {
    run_test!(|client, conn| {
        let mut res = client.get(endpoint)
            .header(ContentType::JSON)
            .dispatch();
        make_response_assertions(
            res,
            expected_status,
            expected_substrings);
    })
}

// Not using generated test-case functions as these each require more involved logic
mod challenge_deletion {
    #[allow(unused_imports)]
    use super::*;

   #[test]
    fn invalid_index_is_not_found() {
        run_test!(|client, conn| {
            let res = client.delete("/challenges/lolthisdonotexist")
                .header(ContentType::JSON)
                .dispatch();
            make_response_assertions(
                res,
                Status::NotFound,
                vec![]);
        })
    }

    #[test]
    fn nonexistent_index_is_not_found() {
        run_test!(|client, conn| {
            Challenge::remove_all(&conn);
            let res = client.delete("/challenges/1")
                .header(ContentType::JSON)
                .dispatch();
            make_response_assertions(
                res,
                Status::NotFound,
                vec![]);
        })
    }

    #[test]
    fn valid_index_is_acceptable() {
        run_test!(|client, conn| {
            let preexisting_challenges = Challenge::list(&conn);
            for challenge in preexisting_challenges {
                let res = client.delete(format!("/challenges/{}", challenge.id))
                    .header(ContentType::JSON)
                    .dispatch();
                make_response_assertions(
                    res,
                    Status::Accepted,
                    vec![&format!("{}", challenge.id)]);
            }
        })
    }
}
