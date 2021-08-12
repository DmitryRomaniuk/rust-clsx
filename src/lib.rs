use std::collections::HashMap;

pub fn clsx_h(hash: HashMap<&str, bool>) -> String {
    let mut str = "".to_owned();
    for (key, value) in &hash {
        if *value {
            str.push_str(&' '.to_string());
            str.push_str(key);
        }
    }
    str.trim().to_string()
}

pub fn clsx(arr: Vec<(&str, bool)>) -> String {
    let mut str = "".to_owned();
    for (key, value) in arr.iter() {
        if *value {
            str.push_str(&' '.to_string());
            str.push_str(key);
        }
    }
    str.trim().to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::{clsx, clsx_h};
    #[test]
    fn clsx_array() {
        assert_eq!(clsx(vec![("class1", true), ("class2", false), ("class3", true)]), "class1 class3");
    }
    #[test]
    fn clsx_hash() {
        let mut class_names = HashMap::new();
        class_names.insert("key", true);
        class_names.insert("key1", true);
        class_names.insert("key2", false);
        class_names.insert("key3", true);
        assert_eq!(clsx_h(class_names), "key key1 key3");
    }
}
