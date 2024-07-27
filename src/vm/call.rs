use super::op::Op;
use super::val::Func;
use std::rc::Rc;

pub struct Call {
    pub pc: usize,
    func: Rc<Func>,
}

impl From<Rc<Func>> for Call {
    fn from(func: Rc<Func>) -> Self {
        Self { pc: 0, func }
    }
}

impl Call {
    pub fn next(&mut self) -> Option<&Op> {
        let cur_c = self.pc;

        self.pc += 1;
        self.func.get(cur_c)
    }
}
