pub struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    pub fn from(vec: Vec<T>) -> Self {
        Self { vec: Vec::from(vec) }
    }
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
    pub fn peek_last(&self) -> Option<&T> {
        self.vec.first()
    }
    pub fn peek_last_mut(&mut self) -> Option<&mut T> {
        self.vec.last_mut()
    }
    pub fn for_each<F>(&self, func: F)
    where
        F: Fn(&T),
    {
        for v in &self.vec {
            func(&v);
        }
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}
