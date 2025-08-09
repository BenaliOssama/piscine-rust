use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool{drops: Cell::new(0), states: RefCell::new(vec![])}
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        todo!()
    }

    pub fn thread_len(&self) -> usize {
        todo!()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        todo!()
    }

    pub fn drop_thread(&self, id: usize) {
        todo!()
    }
}

#[derive(Debug)]
pub  struct Thread <'a> {
    pid : usize,
    cmd: String,
    parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        todo!()
    }

    pub fn skill(self) {
        todo!()
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.states.borrow_mut().push(true);
    }
}

