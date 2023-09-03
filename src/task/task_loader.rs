use alloc::string::String;

use crate::{error::MyError, println};

use super::spawner::SPAWNER;

pub fn load_task(input_str: String) -> Result<(), MyError> {
    match input_str.as_str() {
        "example_task" => SPAWNER.lock().add(example_task()),
        _ => return Err(MyError::InvalidFuture),
    };

    Ok(())
}


async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
