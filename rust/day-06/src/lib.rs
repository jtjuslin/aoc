use std::ops::Range;

pub mod parser;

pub struct Race {
    time: u64,
    record: u64,
}

impl Race {
    pub fn new(time: u64, record: u64) -> Race {
        Race { time, record }
    }

    fn distance(&self, hold: u64) -> u64 {
        hold * (self.time - hold)
    }

    // Do binary search to find where the current record lies.
    // Then form a range starting from the next value
    // and ending with the corresponding value over the parabola's
    // symmetry axis.
    pub fn winning_starts(&self) -> Range<u64> {
        let mut limits: Range<u64> = 0..(self.time / 2 + 1);
        loop {
            let middle: u64 =
                limits.start + TryInto::<u64>::try_into((limits.end - limits.start) / 2).unwrap();
            let distance = self.distance(middle);

            if self.record < distance {
                if limits.end == middle {
                    limits = limits.end..(self.time - limits.end + 1);
                    break;
                }
                limits = limits.start..middle;
            } else if self.record > distance {
                if limits.start == middle {
                    limits = limits.end..(self.time - limits.end + 1);
                    break;
                }
                limits = middle..limits.end;
            } else {
                limits = (middle + 1)..(self.time - middle);
                break;
            }
        }
        limits
    }
}
