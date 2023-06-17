use std::sync::Arc;
use crate::ClassStorage;

pub trait C3Object {
    fn as_class(&self) -> Arc<ClassStorage>;
}

impl<'i> C3Object for &'i str {
    fn as_class(&self) -> Arc<ClassStorage> {
        Arc::new(ClassStorage::new(self))
    }
}

impl C3Object for String {
    fn as_class(&self) -> Arc<ClassStorage> {
        Arc::new(ClassStorage::new(self))
    }
}

