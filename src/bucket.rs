

pub struct Bucket<'a> {
    name: &'a str
}

impl<'a> Bucket<'a> {
    pub fn new(name: &'a str) -> Self { Self { name } }
}

