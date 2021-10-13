#![feature(int_log)]

mod tfidf;

pub use serde::{Deserialize, Serialize};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn map_test() {
        // 可变 借用与数据取出
        let mut p = HashMap::new();
        p.insert(String::from("h1"), Some(String::from("V1")));

        let b = p.get_mut(String::from("h1").as_str()).unwrap();
        let pp = b.take();
        println!("pp: {:?}", pp);
        println!("p: {:?}", p);
        p.remove(String::from("h1").as_str());
        println!("p: {:?}", p);
    }
}
