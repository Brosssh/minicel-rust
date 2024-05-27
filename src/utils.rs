pub trait StringExt {
    fn trim_whitespaces(&self) -> String;
}

impl StringExt for String {
    fn trim_whitespaces(&self) -> String {
        let w: Vec<_> = self.split_whitespace().collect();
        let r = w.join(" ");
        return r;
    }  
}