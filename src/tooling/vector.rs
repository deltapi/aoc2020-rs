pub fn split_each_at(split: Vec<&str>, identifier: &str) -> Vec<Vec<String>> {
    split.iter()
        .map(|x| x.to_string()
            .split(identifier)
            .map(|y| y.to_string())
            .collect())
        .collect()
}
