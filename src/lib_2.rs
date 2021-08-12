use std::collections::HashMap;

pub enum Args {
    Hash(HashMap<String, bool>),
    Arr(Vec<(String, bool)>),
}

pub fn clsx(arr: Args) -> String {
    let mut str = "".to_owned();
    match arr {
        Args::Arr(arr) => {
            for (key, value) in arr.iter() {
                if *value {
                    str.push_str(&' '.to_string());
                    str.push_str(key);
                }
            }
        }
        Args::Hash(arr) => {
            for (key, value) in arr {
                if value {
                    str.push_str(&' '.to_string());
                    str.push_str(&*key);
                }
            }
        }
    }
    str.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::{clsx, Args};
    use std::collections::HashMap;
    #[test]
    fn clsx_array() {
        assert_eq!(
            clsx(Args::Arr(vec![
                ("class1".to_string(), true),
                ("class2".to_string(), false),
                ("class3".to_string(), true)
            ])),
            "class1 class3"
        );
    }
    #[test]
    fn clsx_hash() {
        let mut class_names = HashMap::new();
        class_names.insert("key".to_string(), true);
        class_names.insert("key1".to_string(), true);
        class_names.insert("key2".to_string(), false);
        class_names.insert("key3".to_string(), true);
        assert_eq!(clsx(Args::Hash(class_names)), "key key1 key3");
    }
}
