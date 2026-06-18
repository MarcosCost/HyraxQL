// postgresql://[user[:password]@][netloc][:port][/dbname]

use std::collections::HashMap;

#[derive(Default)]
pub struct ConnectionArgs {
    pub db_type: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub host: String,
    pub port: u32,
    // Extra arguments
    pub extra_params: Option<HashMap<String, String>>
}

