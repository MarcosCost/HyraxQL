// postgresql://[user[:password]@][netloc][:port][/dbname]

#[derive(Default)]
pub struct ConnectionArgs {
    db_type: String,
    db_name: String,
    db_user: String,
    db_pass: String,
    host: String,
    port: u32
}

