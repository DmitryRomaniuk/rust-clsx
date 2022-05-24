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
    use super::{clsx};
    #[test]
    fn clsx_array() {
        assert_eq!(clsx(vec![("class1", true), ("class2", false), ("class3", true)]), "class1 class3");
    }

    #[test]
    fn clsx_concatinated_classes() {
        assert_eq!(clsx(vec![("class1 class4 class5", true), ("class2", false), ("class3", true)]), "class1 class4 class5 class3"); 
    }
}
