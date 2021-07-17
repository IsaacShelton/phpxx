
pub struct Ctx<'a> {
    pub contents: &'a str
}

impl Ctx<'_> {
    pub fn new(contents: &str) -> Ctx {
        Ctx { contents: contents }
    }
}
