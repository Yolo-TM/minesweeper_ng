use super::command::execute_command;
use super::{batch, single};

pub fn generate() {
    let res = execute_command();

    if res.count == 1 {
        single::generate_field(res);
    } else {
        batch::generate_fields(res);
    }
}
