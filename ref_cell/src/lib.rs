use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Tracker {
    pub messages : RefCell<Vec<String>>, // -> !
    pub value: RefCell<usize>, //times value was refrenced,less max
    pub max : usize,
}

impl Tracker {
    pub fn new(max : usize) -> Self{
        Tracker{messages: RefCell::new(vec![]), value : RefCell::new(0) , max: max }
    }

    pub fn set_value(&self, value : &Rc<usize>) {
        let new_value = Rc::strong_count(value);

        if new_value * 100 / self.max >=  75 && new_value < self.max {

            self.messages.borrow_mut().push(
                format!("Warning: You have used up over {}% of your quota!",
                new_value * 100 / (self.max ) )
            );

        }else if new_value > self.max {
            self.messages.borrow_mut().push("Error: You can't go over your quota!".to_string());
        }
        *self.value.borrow_mut() = new_value; 
    }

    pub fn peek(&self, arg: &Rc<usize>){
        let count : usize = Rc::strong_count(arg);
        let percent : usize = count * 100 / self.max;

        let mut val_mut = self.messages.borrow_mut();

       val_mut.push( 
           format!("Info: This value would use {}% of your quota", percent)
        ); 
    }    
}
