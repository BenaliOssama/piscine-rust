use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Node{ref_list: ref_list}
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let mut pos : i32 = 0 ; 
        for (i, e) in self.ref_list.clone().iter().enumerate() {
            if **e == *element {
                self.ref_list.remove(i - pos as usize);
                pos +=  1;
            }            
        }
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    return Rc::strong_count(&ref_list);
}
