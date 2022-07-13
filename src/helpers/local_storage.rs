use crate::console_log;

#[allow(dead_code)]
pub struct LocalStorage;

impl LocalStorage {
    #[allow(dead_code)]
    fn get_local_storage() -> web_sys::Storage {
        web_sys::window().unwrap().local_storage().unwrap().unwrap()
    }

    #[allow(dead_code)]
    pub fn set(key: &str, value: &str) {
        let ls = Self::get_local_storage();
        let result = ls.set(key, value);
        match result {
            Ok(_res) => console_log!("LS :: set key: {}, value: {}", key, value),
            Err(_err) => console_log!("ERROR :: LS :: set key: {}, value: {}", key, value),
        }
    }

    #[allow(dead_code)]
    pub fn get(key: &str) -> Option<String> {
        let ls = Self::get_local_storage();
        let result = ls.get(key);
        match result {
            Ok(res) => res,
            Err(_err) => {
                console_log!("ERROR :: LS :: get key: {}", key);
                None
            }
        }
    }
}
