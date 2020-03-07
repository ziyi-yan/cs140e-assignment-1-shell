// FIXME: Make me pass! Diff budget: 25 lines.
#[derive(Debug, Copy, Clone)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

use Duration::{MilliSeconds, Seconds, Minutes};

impl std::cmp::PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        match (*self, *other) {
            (MilliSeconds(lhs), MilliSeconds(rhs)) => lhs == rhs,
            (Seconds(lhs), Seconds(rhs)) => lhs == rhs,
            (Minutes(lhs), Minutes(rhs)) => lhs == rhs,
            (MilliSeconds(ms), Seconds(s)) => ms == 1000 * (s as u64),
            (MilliSeconds(ms), Minutes(mins)) => ms == 1000 * 60 * (mins as u64),
            (Seconds(s), Minutes(mins)) => s == 60 * (mins as u32),
            (_, _) => other.eq(self),
        }
    }
}

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
