#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Count {
    pub lines: u64,
    pub words: u64,
    pub bytes: u64,
}

impl Count {
    pub fn new() -> Count {
        Count {
            lines: 0,
            words: 0,
            bytes: 0,
        }
    }

    fn num_digits(n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            n.ilog10() as u64 + 1
        }
    }

    pub fn max_digits(&self) -> usize {
        Count::num_digits(
            [self.lines, self.words, self.bytes]
                .iter().max().expect("empty iterator").clone()) as usize
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:>width$} {:>width$} {:>width$}",
               self.lines, self.words, self.bytes, width = self.max_digits())
    }
}

impl std::ops::Add for Count {
    type Output = Count;

    fn add(self, other: Count) -> Count {
        Count {
            lines: self.lines + other.lines,
            words: self.words + other.words,
            bytes: self.bytes + other.bytes,
        }
    }
}

impl std::ops::AddAssign for Count {
    fn add_assign(&mut self, other: Count) {
        *self = *self + other;
    }
}