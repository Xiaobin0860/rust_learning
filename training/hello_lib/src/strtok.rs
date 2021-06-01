// strtok(s = "hello world", ' ')
// return "hello", s = "world"

pub fn strtok<'a>(s: &'a mut &str, pat: char) -> &'a str {
    if let Some(idx) = s.find(pat) {
        let prefix = &s[..idx];
        *s = &s[idx + pat.len_utf8()..];
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
        let mut s = "hello world";
        assert_eq!(strtok(&mut s, ' '), "hello");
        assert_eq!(s, "world");
    }
}
