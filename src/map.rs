//asd
use super::kv;

pub struct Map<'a>(pub &'a str);

impl<'a> Map<'a> {
    pub fn new(name: &str) -> Map {
        Map(name)
    }
    pub fn insert(&self, k: &str, v: &str) {
        kv::set_str((self.0.to_owned() + k).as_str(), v)
    }
    pub fn delete(&self, k: &str) {
        kv::del_str((self.0.to_owned() + k).as_str())
    }
    pub fn get(&self, k: &str) -> Option<&str> {
        kv::get_str((self.0.to_owned() + k).as_str())
    }
}
