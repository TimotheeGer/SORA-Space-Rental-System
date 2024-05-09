mod models;
mod services;
mod schema;
mod db;
mod repository;
mod utils;
mod display;

use crate::models::user::*;
use crate::display::menu;

#[tokio::main]
async fn main() {

    menu::display_menu().await;

}

