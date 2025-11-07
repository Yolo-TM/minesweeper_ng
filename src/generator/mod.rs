mod batch;
mod command;
mod single;

use command::{CommandResult, execute_command};

pub fn generate() {
    let res = execute_command();

    if res.count == 1 {
        single::generate_field(res);
    } else {
        batch::generate_fields(res);
    }
}
