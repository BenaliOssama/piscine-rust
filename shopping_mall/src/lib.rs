mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut max = 1;
    let mut name = String::new();
    let mut result = None;

    for (floor_id, floor) in &mall.floors {
        for (store_name, store) in &floor.stores {
            if store.square_meters > max {
                max = store.square_meters;
                name = store_name.to_string();
                result = Some(store.clone());
            }
        }
    }

    if result.is_some() {
        (name, result.unwrap())
    } else {
        panic!("something went wrong");
    }
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let mut top_employees = Vec::new();
    let mut max_salary = 0.0;

    for (_, floor) in mall.floors.iter() {
        for (_, store) in floor.stores.iter() {
            for (emp_name, emp) in store.employees.iter() {
                if emp.salary >= max_salary {
                    if emp.salary > max_salary {
                        top_employees.clear();
                    }
                    max_salary = emp.salary;
                    top_employees.push((&emp_name[..], *emp));
                }
            }
        }
    }

    top_employees
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = 0;
    let temp = 5;

    count += mall.guards.len();

    for (f_id, floor) in mall.floors.iter() {
        for (s_id, store) in floor.stores.iter() {
            count += store.employees.len();
        }
    }

    count + 2
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut total = 0;

    for (id, floor) in &mall.floors {
        total += floor.size_limit;
    }

    let needed = total / 200;
    let mut to_hire = needed;

    for (name, guard) in &guards {
        if to_hire == 0 {
            break;
        }

        mall.hire_guard(name, *guard);
        to_hire -= 1;
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in mall.floors.iter_mut() {
        for (_, store) in floor.stores.iter_mut() {
            for (emp_id, emp) in store.employees.iter_mut() {
                let duration = emp.working_hours.1 - emp.working_hours.0;
                if duration >= 10 {
                    let bonus = emp.salary * 0.1;
                    emp.raise(bonus);
                } else {
                    emp.cut(emp.salary / 10.0);
                }
            }
        }
    }
}

