// GNU coreutils wc spaces each segment of output (line count, word count, byte count) equal to the
// width of the number with the most digits.
// For example:
// ` 46 127 866 .viminfo`
// `   36   427 11514 test.txt`

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut total_line_count = 0;
    let mut total_word_count = 0;
    let mut total_byte_count = 0;
    let mut total_file_count = 0;
    for arg in std::env::args().skip(1) {
        total_file_count += 1;
        let f = File::open(&arg)?;
        let reader = BufReader::new(f);
        let mut line_count = 0;
        let mut word_count = 0;
        let mut byte_count = 0;
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    line_count += 1;
                    word_count += l.split_whitespace().count();
                    byte_count += l.len() + 1;
                }
                Err(_) => {}
            }
        }

        total_line_count += line_count;
        total_word_count += word_count;
        total_byte_count += byte_count;
        println!("{line_count:>7} {word_count:>7} {byte_count:>7} {arg:>7}");
    }

    if total_file_count > 1 {
        println!("{total_line_count:>7} {total_word_count:>7} {total_byte_count:>7} total");
    }

    Ok(())
}