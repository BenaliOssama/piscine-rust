mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut max_square_meters = 0;
    let mut name_of_biggest = String::new();
    let mut biggest = None;

    for (floor_id, floor) in mall.floors.iter() {  
        for (store_name, store) in floor.stores.iter() {
            if store.square_meters >= max_square_meters {
                max_square_meters = store.square_meters;
                name_of_biggest = store_name.to_string();
                biggest = Some(store.clone());
            }
        }
    }

    if biggest.is_none() {
        panic!("the mall contain one store");
    }

    (name_of_biggest, biggest.unwrap())
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let mut highest_salary = 0.0;
    let mut top_employees = vec![];

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (name, employee) in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    top_employees.clear();
                    top_employees.push((&name, *employee));
                } else if employee.salary == highest_salary {
                    top_employees.push((name, *employee));
                }
            }
        }
    }

    top_employees
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut total = 0;

    total = mall.guards.len(); 

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            total = store.employees.len();
        }
    }

    return total;
}

pub fn check_for_securities(mall: &mut Mall, available_guards: HashMap<String, Guard>) {
    let mut total_meters = 0;

    for floor in mall.floors.values() {
        total_meters += floor.size_limit;
    }

    let needed = total_meters / 200;
    let current = mall.guards.len();

    if needed <= current as u64 {
        return;
    }

    let mut guards_to_add = needed - current as u64;

    for (name, guard) in &available_guards {
        if guards_to_add == 0 {
            break;
        }

        mall.hire_guard(name, guard.clone());
        guards_to_add -= 1;
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (_, employee) in &store.employees {
                let hours = employee.working_hours.1 - employee.working_hours.0;

                if hours >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.cut(employee.salary * 0.1);
                }
            }
        }
    }
}

