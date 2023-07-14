use mfmt::{flatten, indent, r#break, sequence, Document};
use std::alloc::Allocator;

#[derive(Clone, Debug)]
pub struct Builder<A: Allocator> {
    allocator: A,
}

impl<'a, A: Allocator + Clone + 'a> Builder<A> {
    pub fn new(allocator: A) -> Self {
        Self { allocator }
    }

    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    pub fn sequence(&self, values: impl IntoIterator<Item = Document<'a>>) -> Document<'a> {
        sequence(self.allocate_vec(values))
    }

    pub fn flatten(&self, value: Document<'a>) -> Document<'a> {
        flatten(self.allocate(value))
    }

    pub fn indent(&self, value: Document<'a>) -> Document<'a> {
        indent(self.allocate(value))
    }

    pub fn r#break(&self, value: Document<'a>) -> Document<'a> {
        r#break(self.allocate(value))
    }

    pub fn allocate<T>(&self, value: T) -> &'a T {
        Box::leak(Box::new_in(value, self.allocator.clone()))
    }

    pub fn allocate_vec<T>(&self, values: impl IntoIterator<Item = T>) -> &'a [T] {
        let mut vec = Vec::new_in(self.allocator.clone());

        vec.extend(values);

        Vec::leak(vec)
    }
}
