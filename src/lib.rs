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