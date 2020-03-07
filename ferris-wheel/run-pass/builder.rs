// FIXME: Make me pass! Diff budget: 30 lines.

#[derive(Default)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl std::fmt::Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (self.string.as_ref(), self.number) {
            (Some(s), Some(n)) => write!(f, "{} {}", s, n),
            (Some(s), _) => write!(f, "{}", s),
            (_, Some(n)) => write!(f, "{}", n),
            (_, _) => write!(f, ""),
        }
    }
}

impl Builder {
    fn string<S: Into<String>>(self, s: S) -> Builder {
        Builder {
            string: Some(s.into()),
            number: self.number,
        }
    }
    fn number(self, n: usize) -> Builder {
        Builder {
            string: self.string,
            number: Some(n),
        }
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
