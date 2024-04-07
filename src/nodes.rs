// This file is not used yet. But by adding a parser module that implements a
// function to turn the tokens into these structs, and another function to go
// from the nodes to tokens, we can implement serialization and deserialization
// of JSON files.
// It's also possible to implement a Visitor API to write lints for JSON files.
pub struct JSONFile {
    pub value: JSONNode,
}

pub struct JSONBool {
    pub value: bool,
}

pub struct JSONNull {}

pub struct JSONNumber {
    pub value: f64,
}

pub struct JSONString {
    pub value: String,
}

pub struct JSONArray {
    pub items: Vec<JSONNode>,
}

pub struct JSONObject {
    pub keys: Vec<JSONNode>,
    pub values: Vec<JSONNode>,
}

pub enum JSONNode {
    Bool(JSONBool),
    Null(JSONNull),
    Number(JSONNumber),
    String(JSONString),
    Array(JSONArray),
    Object(JSONObject),
}
