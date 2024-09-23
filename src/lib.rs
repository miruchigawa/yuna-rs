pub mod commands;

use std::{collections::HashMap, sync::Mutex};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub user_backlist: Mutex<HashMap<String, bool>>,
}

