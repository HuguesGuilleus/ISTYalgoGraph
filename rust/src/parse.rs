pub fn csv(s: &str) -> Result<(usize, usize), String> {
    let v: Vec<&str> = s.split(',').filter(|s| s.len() > 0).take(2).collect();

    if v.len() != 2 {
        return Err(format!("This line has no coma to separate the two usize"));
    }

    Ok((
        v[0].parse::<usize>()
            .map_err(|err| format!("First number {:?} parse fail  {}", v[0], err))?,
        v[1].parse::<usize>()
            .map_err(|err| format!("Second number {:?} parse fail  {}", v[1], err))?,
    ))
}
#[test]
fn test_csv() {
    assert_eq!(Ok((1, 2)), csv("1,2"))
}

pub fn tab(s: &str) -> Result<(usize, usize), String> {
    let v: Vec<&str> = s
        .split(char::is_whitespace)
        .filter(|s| s.len() > 0)
        .take(2)
        .collect();

    if v.len() != 2 {
        return Err(format!("This line no contain two usize with is_whitespace"));
    }

    Ok((
        v[0].parse::<usize>()
            .map_err(|err| format!("First number {:?} parse fail  {}", v[0], err))?,
        v[1].parse::<usize>()
            .map_err(|err| format!("Second number {:?} parse fail  {}", v[1], err))?,
    ))
}
#[test]
fn test_tab() {
    assert_eq!(Ok((1, 2)), tab("1	2"))
}
