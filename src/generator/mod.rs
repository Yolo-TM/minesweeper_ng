mod command;
mod batch;
mod single;

use command::{execute_command, CommandResult};

pub fn generate() {
    let res = execute_command();

    if res.count == 1 {
        single::generate_field(res);
    } else {
        batch::generate_fields(res);
    }
}

