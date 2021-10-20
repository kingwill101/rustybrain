use regex::Regex;

pub fn get_variables(val: &str) -> Vec<String> {
    let mut vars = vec![];

    for cap in Regex::new(r"\[([A-Za-z_-])+]").unwrap().find_iter(val) {
        // println!("{:#?}", cap.as_str());
        vars.push(String::from(cap.as_str()));
    }
    vars
}

pub fn pluralize(singular: &str, plural: &str, count: i32) -> String {
    if plural.is_empty() && singular.is_empty() {
        return String::new();
    } else if plural.is_empty() {
        return singular.to_string();
    } else if singular.is_empty() {
        return String::new();
    }

    if count == 1 {
        return singular.to_string();
    }
    plural.to_string()
}

pub fn clean_var_name(var: &str) -> String {
    String::from(var).replace("[", "").replace("]", "")
}

#[test]
fn get_variable_count() {
    let src = "[a] [qwerty_] [sadfsd]";
    let vars = get_variables(src);

    assert_eq!(vars.len(), 3)
}

#[test]
fn clean_var_test() {
    assert_eq!("a", clean_var_name("[a]"))
}
