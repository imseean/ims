use serde_json::Value;

pub trait EasyGet {
    fn get_data(&self, key: &str) -> Option<Value>;
    fn get_string_data(&self, key: &str) -> Option<String>;
    fn get_number_data(&self, key: &str) -> Option<i64>;
    fn get_boolean_data(&self, key: &str) -> Option<bool>;
    fn get_Array_data(&self, key: &str) -> Option<Vec<Value>>;
    fn get_object_data(&self, key: &str) -> Option<Value>;
}
impl EasyGet for Value {
    fn get_data(&self, key: &str) -> Option<Value> {
        let s: Vec<&str> = key.split('.').collect();
        let mut result = self.clone();
        for item in s {
            let temp = result;
            match temp.get(item) {
                Some(data) => {
                    result = temp.clone();
                }
                None => {
                    return None;
                }
            }
        }
        return Some(result);
    }
    fn get_string_data(&self, key: &str) -> Option<String> {
        let result = self.get_data(key);
        if let Some(data) = result {
            if data.is_string() {
                return data.as_str().map(|s| s.to_string());
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    fn get_number_data(&self, key: &str) -> Option<i64> {
        let result = self.get_data(key);
        if let Some(data) = result {
            if data.is_number() {
                return data.as_i64();
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    fn get_boolean_data(&self, key: &str) -> Option<bool> {
        let result = self.get_data(key);
        if let Some(data) = result {
            if data.is_boolean() {
                return data.as_bool();
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    fn get_Array_data(&self, key: &str) -> Option<Vec<Value>> {
        let result = self.get_data(key);
        if let Some(data) = result {
            if data.is_array() {
                return data.as_array().map(|s| {
                    let mut list = Vec::new();
                    for item in s {
                        list.push(item.clone());
                    }
                    return list;
                });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    fn get_object_data(&self, key: &str) -> Option<Value> {
        let result = self.get_data(key);
        if let Some(data) = result {
            if data.is_object() {
                return Some(data);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
