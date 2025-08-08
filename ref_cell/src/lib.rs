use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Tracker {
    pub messages : RefCell<Vec<String>>, // -> !
    pub value: usize, //times value was refrenced,less max
    pub max : usize,
}

impl Tracker {
    pub fn new(max : usize) -> Self{
        Tracker{messages: RefCell::new(vec![]), value : 0 , max: max }
    }

    pub fn set_value(&mut self, value : &Rc<usize>) {
        if self.value * 100 / self.max  >=  75 && self.value < self.max {
            let mut val_mut = self.messages.borrow_mut();

            val_mut.push(
                format!("Warning: You have used up over {}% of your quota!",
                self.value * 100 / (self.max ) )
            );

        }else if self.value > self.max {
            let mut val_mut = self.messages.borrow_mut();
            val_mut.push("Error: You can't go over your quota!".to_string());
        }else{
            self.value = Rc::strong_count(value);
        }
    }

    pub fn peek(&mut self, arg: &Rc<usize>){
        let count : usize = Rc::strong_count(arg);
        let percent : usize = count * 100 / self.max;

        let mut val_mut = self.messages.borrow_mut();

       val_mut.push( 
           format!("Info: This value would use {}% of your quota", percent)
        ); 
    }    
}
