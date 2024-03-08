use uniform_logic::*;
use serde_json::{from_str, to_string};
use std::collections::HashMap;

#[test]
fn test_deserialize_init() {
    let init_str = "{ \"initdata\": \"hello world\" }";
    let init: InitInput<String> = from_str(init_str).unwrap();
    assert!(init.initdata == "hello world");
}

#[test]
fn test_serialize_request() {
    let mut req_content = HashMap::new();
    req_content.insert("player0".to_owned(), "go".to_owned());
    req_content.insert("player1".to_owned(), "stop".to_owned());
    let request = RoundOutput {
        content: req_content.into(),
        display: "hello world".to_owned(),
    };
    let req_str = to_string(&request).unwrap();
    println!("{}", req_str);
}

#[test]
fn test_serialize_finish() {
    let mut fin_content = HashMap::new();
    fin_content.insert(
        "player0".to_owned(),
        FinishMessage {
            score: 100.,
            state: "OK".to_owned(),
        },
    );
    fin_content.insert(
        "player1".to_owned(),
        FinishMessage {
            score: 99.,
            state: "OK".to_owned(),
        },
    );
    let finish = FinishOutput {
        content: fin_content.into(),
        display: "hello world".to_owned(),
    };
    let fin_str = to_string(&finish).unwrap();
    println!("{}", fin_str);
}

#[test]
fn test_deserialize_response() {
    let res_str = r#"{
        "log": {
            "player0": {
                "verdict": "OK",
                "response": "hello"
            },
            "player1": {
                "verdict": "TLE",
                "response": "world"
            }
        } 
    }"#;
    let response: RoundInput<String> = from_str(res_str).unwrap();
    assert_eq!(
        response.log.0.get("player0").unwrap().verdict,
        AgentVerdict::OK
    );
}
