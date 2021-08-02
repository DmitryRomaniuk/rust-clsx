use std::collections::HashMap;

fn clsx_h(hash: HashMap<&str, bool>) -> String {
    let mut str = "".to_owned();
    for (key, value) in &hash {
        if *value {
            str.push_str(" ");
            str.push_str(key);
        }
    }
    str.to_string()
}

fn clsx(arr: Box<[(&str, bool)]>) -> String {
    let mut str = "".to_owned();
    for (key, value) in arr.iter() {
        if *value {
            str.push_str(" ");
            str.push_str(key);
        }
    }
    str.to_string()
}

fn main() {
    let mut class_names = HashMap::new();
    class_names.insert("key", true);
    class_names.insert("key1", true);
    class_names.insert("key2", false);
    class_names.insert("key3", true);
    println!("{}", clsx_h(class_names)); // "key1 key3 key"
    println!("{}", clsx(Box::new([("class1", true), ("class2", false), ("class3", true)]))); // "class1 class3"
}
