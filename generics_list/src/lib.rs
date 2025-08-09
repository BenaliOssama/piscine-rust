#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T: Clone> List<T> {
    pub fn new() -> List<T> {
        return List {
            head: None,
        };
    }

    pub  fn push(&mut self, value: T) {
        let mut new_node = Node{value: value, next: None };
            
        match &self.head {
            Some(node) => new_node.next = Some(Box::new(node.clone())), 
            _ =>() ,
        }

        self.head = Some(new_node); 
    }

    pub fn pop(&mut self) {
        match &self.head {
            Some(head) => {
                match &head.next {
                    Some(next) => self.head = Some(*next.clone()),
                    _ => self.head = None, 
                }
            } ,
            _ => (), 
        }
    }


    pub fn len(&self) -> usize {
        let mut count : usize = 0 ;

        if self.head.is_none() {
            return count; 
        }
        count += 1;


        let mut current = self.head.as_ref().unwrap();//.clone();
        
        while current.next.is_some() {
            count += 1;
            current = current.next.as_ref().unwrap();//.clone();
        }

        return count ; 
    }
}
