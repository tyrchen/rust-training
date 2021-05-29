// strtok(s = "hello world", " ")
// return "hello", s = "world"

pub fn strtok<'a>(s: &'a mut &str, pat: char) -> &'a str {
    match s.find(pat) {
        Some(i) => {
            let prefix = &s[..i]; // hello
            let suffix = &s[i + pat.len_utf8()..];
            *s = suffix;
            prefix
        }
        None => {
            let prefix = *s;
            *s = "";
            prefix
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut s = "hello world";
        assert_eq!(s.find(' '), Some(5));

        let s1 = &mut s;
        let t = strtok(s1, ' ');
        assert_eq!(t, "hello");
        assert_eq!(*s1, "world");
    }
}
