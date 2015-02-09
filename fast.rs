pub fn sum(x: &[i32]) -> i32 {
    x.iter().fold(0, |a, b| a + *b)
}

pub fn for_sum(x: &[i32]) -> i32 {
    let mut s = 0;
    for val in x.iter() { s += *val }
    s
}
