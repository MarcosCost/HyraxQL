use crate::misc::app_structs::ConnectionArgs;

pub fn connect(args: ConnectionArgs){
    sqlx::any::install_default_drivers()


}