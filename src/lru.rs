use std::collections::HashMap;
use std::time::SystemTime;

pub struct LRU {
    q: HashMap<String, u128>,
    data: HashMap<String, Vec<u8>>,
    n: u8,
}

impl Default for LRU {
    fn default() -> Self {
        LRU { q: HashMap::new(), data: HashMap::new(), n: 2 }
    }
}

impl LRU {
    pub fn get(&mut self, key: String) -> Option<Vec<u8>> {
        return match self.q.get_mut(key.as_str()) {
            None => None,
            Some(old_time) => {
                *old_time = curr_time_nanos();

                match self.data.get(key.as_str()) {
                    Some(data) => Some(data.to_owned()),
                    None => unreachable!()
                }
            }
        };
    }
    pub fn set(&mut self, key: String, data: Vec<u8>) {
        match self.q.get_mut(key.as_str()) {
            None => {
                self.q.insert(key.to_owned(), curr_time_nanos());
                self.data.insert(key, data);
                if self.q.len() >= (self.n + 1) as usize {
                    let oldest = get_oldest_key(&self.q);
                    self.q.remove(&*oldest);
                    self.data.remove(&*oldest);
                }
            }
            Some(elem) => {
                *elem = curr_time_nanos();
                match self.data.get_mut(key.as_str()) {
                    None => unreachable!(),
                    Some(el) => {
                        *el = data;
                    }
                }
            }
        }
    }
}

fn curr_time_nanos() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos()
}

fn get_oldest_key(map: &HashMap<String, u128>) -> String {
    let key = map
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k);

    return match key {
        None => unreachable!(),
        Some(k) => k.to_owned()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let mut t = LRU::default();
        t.set(String::from("1"), Vec::from("1".as_bytes()));
        assert_eq!(t.get(String::from("1")).is_some(), true);

        t.set(String::from("2"), Vec::from("2".as_bytes()));
        assert_eq!(t.get(String::from("1")).is_some(), true);
        assert_eq!(t.get(String::from("2")).is_some(), true);

        t.set(String::from("3"), Vec::from("3".as_bytes()));
        assert_eq!(t.get(String::from("1")).is_some(), false);
        assert_eq!(t.get(String::from("2")).is_some(), true);
        assert_eq!(t.get(String::from("3")).is_some(), true);

        t.set(String::from("4"), Vec::from("4".as_bytes()));
        assert_eq!(t.get(String::from("1")).is_some(), false);
        assert_eq!(t.get(String::from("2")).is_some(), false);
        assert_eq!(t.get(String::from("3")).is_some(), true);
        assert_eq!(t.get(String::from("4")).is_some(), true);
    }
}