

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result : Vec<String> = vec![];

    for name in names {
        let chars : Vec<char> = name.split_ascii_whitespace()
            .map(|x| x.chars().next().unwrap())
            .collect();
        result.push(format!{"{}. {}", chars[0], chars[1]});
    }
    result
}
