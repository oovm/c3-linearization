use std::sync::Arc;
use crate::C3Class;

pub trait C3Object {
    fn as_class(&self) -> Arc<C3Class>;
}

impl<'i> C3Object for &'i str {
    fn as_class(&self) -> Arc<C3Class> {
        Arc::new(C3Class::new(self))
    }
}

impl C3Object for String {
    fn as_class(&self) -> Arc<C3Class> {
        Arc::new(C3Class::new(self))
    }
}

