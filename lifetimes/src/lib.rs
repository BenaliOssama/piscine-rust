#[derive(Debug, Clone)]
pub struct Person<'a>{
	pub name: &'a str,
	pub age: u8,
}

impl<'a> Person<'a> {
	pub fn new<'b>(name:  &str) -> Person<'_> {
        return Person{name, age: 0};
	}
}