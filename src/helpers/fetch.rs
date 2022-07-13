use gloo::net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub userId: u32,
}

impl Task {
    pub fn get_trimed_body(&self, max_chars: usize) -> String {
        let body = &self.body.as_str();
        match body.char_indices().nth(max_chars) {
            None => body.to_string(),
            Some((idx, _)) => format!("{}...", &body[..idx]),
        }
    }

    pub fn empty() -> Self {
        Task {
            id: 0,
            title: "".to_string(),
            body: "".to_string(),
            userId: 0,
        }
    }
}

pub type FetchError = gloo::net::Error;
//"https://jsonplaceholder.typicode.com/posts"
pub async fn fetch<T>(url: String) -> Result<T, FetchError>
where
    T: for<'de> Deserialize<'de>,
{
    Request::get(url.as_str())
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
}
