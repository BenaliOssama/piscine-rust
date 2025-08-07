#[derive(Debug, PartialEq)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s : &str)-> Self{
        match s {
            "CEO" => Role::CEO, 
            "Manager" => Role::Manager, 
            _ => Role::Worker,
        }
    }    
}
 
 
#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;
#[derive(Debug, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment{grade: None}
    }

    pub fn add_worker(&mut self, name: &str, role: &str) { 
        // create a worker
        let mut role = role ; 
        if role == "Normal Worker" {
             role = "Worker"; 
        }
        let mut worker = Worker{
            role: role.to_string(),
            name: name.to_string(),
            next: None,
        };

        // grade is link , link is option, 
        match &self.grade {
            // if some add to head 
            Some(w) => {
                worker.next =  Some((*w).clone());
                self.grade = Some(Box::new(worker));
            },
            // if none worker is the head
            None => {
                self.grade = Some(Box::new(worker));
            },
         }

    }  
    pub fn remove_worker(&mut self) -> Option<String> {
        let worker = self.grade.clone().unwrap().name;
        self.grade = self.grade.clone().unwrap().next ; 
        return Some(worker);
    }


    // pub fn remove_worker(&mut self) -> Option<String> {
    //
    //     let mut current: Box<Worker> = self.grade.clone().unwrap();
    //     let mut prev : Box<Worker> = current.clone();
    //     if current.role == "Worker" {
    //         self.grade = current.next ;
    //         return Some(current.name.clone());
    //     }
    //     loop {
    //         if current.role == "Worker" {
    //             break
    //         }
    //         if current.next.is_none() {
    //             return None;
    //         }
    //
    //         current = current.next.unwrap();
    //         prev = current.clone();
    //     }
    //
    //
    //     if current.next.is_none() {
    //         prev.next = None ;
    //         return Some(current.name.clone());
    //     } else {
    //         prev.next = current.next;
    //         return Some(current.name.clone());
    //     }
    // }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        let mut last_worker = None;
        let mut current: Box<Worker> = self.grade.clone().unwrap();
        loop {
            if current.role == "Worker" {
                return Some((current.name ,Role::Worker));
            }
            if current.next.is_none() {
                //return Some((current.name, Role::from(&current.role as &str)));
                return last_worker;
            }
            current = current.next.unwrap();
        }
    }
}
