pub trait StringExt {
    fn trim_whitespaces(&self) -> String;
}

impl StringExt for String {
    fn trim_whitespaces(&self) -> String {
        let w: Vec<_> = self.split_whitespace().collect();
        w.join(" ")
    }
}

#[macro_export]
macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            // #1
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}