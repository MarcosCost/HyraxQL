enum Value {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Null,
    Json(serde_json::Value),
}

pub enum QueryResult {
    Rows {
        columns: Vec<String>,
        rows: Vec<Vec<Value>>,
    },
    Scalar(Value),
    Mutation {
        affected: u64,
        last_id: Option<i64>,
    },
    Empty,
}
