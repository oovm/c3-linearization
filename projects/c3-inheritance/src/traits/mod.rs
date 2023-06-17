use crate::c3::C3Class;

pub trait C3Object {
    fn as_class(&self) -> C3Class;
}

impl C3Object for C3Class {
    fn as_class(&self) -> C3Class {
        self.clone()
    }
}

impl<'i> C3Object for &'i str {
    fn as_class(&self) -> C3Class {
        C3Class::new(self.to_string())
    }
}

impl C3Object for String {
    fn as_class(&self) -> C3Class {
        C3Class::new(self)
    }
}
