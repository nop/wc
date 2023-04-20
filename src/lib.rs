use std::io::BufRead;

use crate::count::Count;

pub mod count;

pub fn count(reader: &mut dyn BufRead) -> Count {
    let mut count = Count::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                count.lines += 1;
                count.words += l.split_whitespace().count() as u64;
                count.bytes += l.len() as u64 + 1;
            }
            Err(_) => {}
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let mut reader = "Hello, world!".as_bytes();
        let count = count(&mut reader);
        assert_eq!(count.lines, 1);
        assert_eq!(count.words, 2);
        assert_eq!(count.bytes, 14);
        assert_eq!(count.max_digits(), 2);
    }
}