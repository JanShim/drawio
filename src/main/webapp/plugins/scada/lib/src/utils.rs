use std::collections::HashMap;

use implicit_clone::sync::IString;
use reqwasm::{
    http::Request, 
//    Error
};
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::FetchError;

pub async fn fetch<T>(url: String) -> Result<T, FetchError>
where
    T: DeserializeOwned,
{
    return Request::get(&url)
        .send().await
        .map_err(|err| FetchError::RequestError(err.to_string()))?
        .json::<T>().await
        .map_err(|err| FetchError::SerdeError(err.to_string()));
}

pub async fn fetch_string(url: String) -> Result<String, FetchError>
{
    return Request::get(&url).send().await
        .map_err(|err| FetchError::RequestError(err.to_string()))?
        .text().await
        .map_err(|err| FetchError::RequestError(err.to_string()));
}

pub async fn post<T>(url: String, data: T) -> Result<T, FetchError>
where 
    T: Serialize,
    T: DeserializeOwned,
{
    let json = serde_json::to_string(&data)
        .map_err(|err| FetchError::SerdeError(err.to_string()))?;
    
    return Request::post(&url)
        .header("Content-Type", "application/json")
        .body(json)
        .send().await
        .map_err(|err| FetchError::RequestError(err.to_string()))?
        .json::<T>().await
        .map_err(|err| FetchError::SerdeError(err.to_string()));
}


pub fn string_to_map<'a>(s: &'a str) -> HashMap<&'a str, &'a str> {
    s.split(';')
        .map(|o| o.trim())
        .map(|o| {
            o.split(':')
                .map(|p| p.trim())
                .filter(|s| s.len() > 0)
                .collect::<Vec<_>>()
        })
        .filter(|v| v.len() == 2)
        .map(|kv| (kv[0], kv[1]))
        .fold(HashMap::new(), |mut acc, i| {
            acc.insert(i.0, i.1);
            acc
        })
}

pub fn map_to_string<'a>(m: HashMap<&'a str, &'a str>) -> String {
    m.iter()
        .map(|o| format!("{}:{}", o.0, o.1))
        .collect::<Vec<_>>()
        .join(";")
}

// ==========================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_map_works() {
        let str = "aaa: bbb; ccc:ddd;";

        let map = string_to_map(str);
        println!("{map:#?}");

        assert_eq!(map.get("aaa"), Some(&"bbb"));

        let res = map_to_string(map);
        println!("{res:#?}");

        // assert_eq!(res, "ccc:ddd;aaa:bbb");
    }
}