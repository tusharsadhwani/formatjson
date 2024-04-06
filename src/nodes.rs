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
