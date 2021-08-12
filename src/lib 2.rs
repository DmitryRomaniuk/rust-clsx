use std::collections::HashMap;

pub enum Args {
    hash(HashMap<String, bool>),
    arr(Box<[(String, bool)]>)
}

pub fn clsx(arr: Args) -> String {
    let mut str = "".to_owned();
    match arr {
        Args::arr(arr) => {
            for (key, value) in arr.iter() {
                if *value {
                    str.push_str(" ");
                    str.push_str(key);
                }
            }
        }
        Args::hash(arr) => {
            for (key, value) in arr {
                if value {
                    str.push_str(" ");
                    str.push_str(&*key);
                }
    }
        }
    }
    str.trim().to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{clsx, Args};
    #[test]
    fn clsx_array() {
        assert_eq!(clsx(Args::arr(Box::new([("class1".to_string(), true), ("class2".to_string(), false), ("class3".to_string(), true)]))), "class1 class3");
    }
    #[test]
    fn clsx_hash() {
        let mut class_names = HashMap::new();
        class_names.insert("key".to_string(), true);
        class_names.insert("key1".to_string(), true);
        class_names.insert("key2".to_string(), false);
        class_names.insert("key3".to_string(), true);
        assert_eq!(clsx(Args::hash(class_names)), "key key1 key3");
    }
}
