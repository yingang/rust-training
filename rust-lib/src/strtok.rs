pub fn strtok<'a>(s: &'a mut &str, pattern: char) -> &'a str {
    match s.find(pattern) {
        Some(i) => {
            let prefix = &s[..i];
            let suffix = &s[i + pattern.len_utf8()..];
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
        let token = strtok(s1, ' ');
        assert_eq!(token, "hello");
        assert_eq!(*s1, "world");
    }
}