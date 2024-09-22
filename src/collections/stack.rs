pub struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { vec }
    }
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
    pub fn peek_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index)
    }
    pub fn peek_last_mut(&mut self) -> Option<&mut T> {
        self.vec.last_mut()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
