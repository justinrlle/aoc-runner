use std::borrow::Cow;
use std::fmt::Display;
use std::str::FromStr;
pub use itertools::Itertools;

pub fn b_lines(input: &str) -> impl Iterator<Item = &[u8]> {
    input.lines()
        .map(str::as_bytes)
}

pub fn as_lossy_vec<'a>(iter: impl Iterator<Item = &'a [u8]>) -> Vec<Cow<'a, str>> {
    iter.map(String::from_utf8_lossy)
        .collect::<Vec<_>>()
}

pub fn parse_all<'a, T: FromStr>(input: &'a str) -> impl Iterator<Item=T> + 'a
    where <T as FromStr>::Err: Display {
    input.lines()
        .enumerate()
        .map(|(idx, l)| {
            l.parse::<T>()
                .map_err(|e|
                    format!("failed to parse {} (line {})\n  could not parse into {}: {}",
                            l,
                            idx,
                            std::any::type_name::<T>(),
                            e))
                .unwrap()
        })
}

pub fn day_input(day: u8) -> String {
    let path = format!("./days/{}", day);
    match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => panic!("failed to read {}: {}", path, e),
    }
}

#[macro_export]
macro_rules! day {
    ($day:expr, $part_1:ident) => {
        {
            let input = $crate::day_input($day);
            println!("day {}:", $day);
            println!(" - part 1 ({}): {}", stringify!($part_1), $part_1(&input));
        }
    };
    ($day:expr, $part_1:ident, $part_2:ident) => {
         {
            let input = $crate::day_input($day);
            println!("day {}:", $day);
            println!(" - part 1 ({}): {}", stringify!($part_1), $part_1(&input));
            println!(" - part 2 ({}): {}", stringify!($part_2), $part_2(&input));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn b_lines_test() {
        let iter = b_lines("foo\nbar\nbaz");
        let v = iter.collect::<Vec<&[u8]>>();
        assert_eq!(v, &[b"foo", b"bar", b"baz"]);
        let iter = b_lines("foo\nbar\nbaz\n");
        let v = iter.collect::<Vec<&[u8]>>();
        assert_eq!(v, &[b"foo", b"bar", b"baz"]);
    }

    #[test]
    fn as_lossy_vec_test() {
        let v: Vec<&[u8]> = vec![b"foo", b"bar", b"baz", b"\xF0\x9F\x92\x96"];
        let v = as_lossy_vec(v.iter().copied());
        assert_eq!(v, &["foo", "bar", "baz", "💖"]);
        let v: Vec<&[u8]> = vec![b"foo", b"bar", b"baz\xF0\x90\x80"];
        let v = as_lossy_vec(v.iter().copied());
        assert_eq!(v, &["foo", "bar", "baz�"]);
    }

    #[test]
    fn parse_all_test() {
        let mut iter = parse_all::<u8>("1\n2\n3\n4\n5");
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(3), iter.next());
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(5), iter.next());
        assert_eq!(None, iter.next());
        let parsed = parse_all::<u8>("1\n2\n3\n4\n5").collect::<Vec<_>>();
        assert_eq!(parsed, &[1, 2, 3, 4, 5]);
        let err = catch_unwind(|| {
            parse_all::<u8>("1\n-2\n3").for_each(drop);
        });
        assert!(err.is_err());
        let err = err.unwrap_err();
        let err = err.downcast_ref::<String>().unwrap();
        assert_eq!(err, r#"called `Result::unwrap()` on an `Err` value: "failed to parse -2 (line 1)\n  could not parse into u8: invalid digit found in string""#);
    }
}
