use std::collections::HashMap;

type KeywordKey = &'static str;
type KeywordValue = String;
type KeywordInner = HashMap<KeywordKey, KeywordValue>;

#[derive(Clone, Default)]
pub struct KeywordBucket(KeywordInner);

#[macro_export]
macro_rules! keywords {
	($($key: expr => $value: expr),*) => {{
		#[allow(unused_mut)]
  let mut map = std::collections::HashMap::default();
  $(
   map.insert($key, $value);
  )*
  $crate::structures::KeywordBucket::new(map)
 }};
}

impl KeywordBucket {
    pub fn new(map: KeywordInner) -> KeywordBucket {
        KeywordBucket(map)
    }

    pub fn get(&self, key: KeywordKey) -> Option<KeywordValue> {
        self.0.get(key).cloned()
    }

    pub fn insert(&mut self, key: KeywordKey, val: KeywordValue) {
        self.0.insert(key, val);
    }

    pub fn unchecked_get(&self, key: KeywordKey) -> KeywordValue {
        self.0.get(key).unwrap().clone()
    }

    pub fn get_bytes(&self, key: KeywordKey) -> Option<Vec<u8>> {
        let wrapped_value = self.0.get(key).cloned();
        if let Some(v) = wrapped_value {
            return Some(v.as_bytes().to_vec());
        }
        None
    }
}
