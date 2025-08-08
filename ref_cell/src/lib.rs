use std::rc::Rc;

#[derive(Debug)]
pub struct Tracker {
    pub messages : Vec<String>, // -> !
    pub value: usize, //times value was refrenced,less max
    pub max : usize,
}

impl Tracker {
    pub fn new(max : usize) -> Self{
        Tracker{messages: vec![], value : 0 , max: max }
    }

    pub fn set_value(&mut self, value : &Rc<usize>) {
        if self.value * 100 / self.max  >=  75 && self.value < self.max {

            self.messages.push(
                format!("Warning: You have used up over {}% of your quota!",
                self.value * 100 / (self.max ) )
            );

        }else if self.value > self.max {
            self.messages.push("Error: You can't go over your quota!".to_string());
        }else{
            self.value = Rc::strong_count(value);
        }
    }

    pub fn peek(&mut self, arg: &Rc<usize>){
        let count : usize = Rc::strong_count(arg);
        let percent : usize = count * 100 / self.max;


       self.messages.push( 
           format!("Info: This value would use {}% of your quota", percent)
        ); 
    }    
}
