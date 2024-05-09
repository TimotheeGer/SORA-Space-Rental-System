use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::User;

pub static USER: Lazy<Mutex<Option<User>>> = Lazy::new(|| Mutex::new(None));

pub fn set_user(user: User) {
    let mut user_guard = USER.lock().unwrap();
    *user_guard = Some(user);
}

pub fn get_user() -> Option<User> {
    let user_guard = USER.lock().unwrap();
    user_guard.clone()
}

pub fn clear_user() {
    let mut user = USER.lock().unwrap();
    *user = None;
}