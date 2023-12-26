pub fn hash(string: &str) -> usize {
    string
        .as_bytes()
        .iter()
        .fold(0, |current, &ch| ((current + (ch as usize)) * 17) % 256)
}
