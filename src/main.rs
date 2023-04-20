// GNU coreutils wc spaces each segment of output (line count, word count, byte count) equal to the
// width of the number with the most digits.
// For example:
// ` 46 127 866 .viminfo`
// `   36   427 11514 test.txt`

use std::fs::File;
use std::io::BufReader;
use libwc::count::Count;

fn main() -> std::io::Result<()> {
    let mut totals = Count::new();
    let mut file_count = 0;
    for arg in std::env::args().skip(1) {
        file_count += 1;
        let f = File::open(&arg)?;
        let mut reader = BufReader::new(f);

        let count = libwc::count(&mut reader);

        totals += count;
        println!("{count} {arg}");
    }

    if file_count > 1 {
        println!("{totals} total");
    }

    Ok(())
}