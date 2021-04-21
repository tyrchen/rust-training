pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x1 = "hello world".to_owned();
        let mut x = x1.as_str();
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        // assert_eq!(x, "world");
    }
}
