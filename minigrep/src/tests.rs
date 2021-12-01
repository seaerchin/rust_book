use super::*;

#[cfg(test)]
#[test]
fn case_sensitive() {
    let query = String::from("duct");
    let contents = String::from(
        "\
        Rust:
        safe, fast, productive. 
        Pick three.",
    );
    let config = Config::from_data(&query, contents);
    let expected = vec![LineData::new(1, "safe, fast, productive.")];
    let actual = search(&config);
    assert_eq!(expected, actual)
}

#[test]
fn case_insensitive() {
    let query = String::from("rUST");
    let contents = String::from(
        "Rust:
        safe, fast, productive. 
        Pick three.
        Trust pls.",
    );
    let config = Config::from_data(&query, contents);
    let expected = vec![LineData::new(0, "Rust:"), LineData::new(3, "Trust pls.")];
    let actual = search(&config);
    assert_eq!(expected, actual)
}
