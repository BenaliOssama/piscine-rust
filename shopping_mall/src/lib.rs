mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut max_square_meters = 0;
    let mut biggest_name = String::new();
    let mut biggest_store: Option<Store> = None;

    for (_, floor) in &mall.floors {
        for (store_name, store) in &floor.stores {
            if store.square_meters > max_square_meters {
                max_square_meters = store.square_meters;
                biggest_name = store_name.clone();
                biggest_store = Some(store.clone());
            }
        }
    }

    if let Some(store) = biggest_store {
        (biggest_name, store)
    } else {
        panic!("Mall has no stores");
    }
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let mut highest_salary = 0.0;
    let mut result = Vec::new();

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (name, employee) in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    result.clear();
                    result.push((name.as_str(), *employee));
                } else if employee.salary == highest_salary {
                    result.push((name.as_str(), *employee));
                }
            }
        }
    }

    result
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut total = 0;

    total += mall.guards.len();

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            total += store.employees.len();
        }
    }

    total
}

pub fn check_for_securities(mall: &mut Mall, available_guards: HashMap<String, Guard>) {
    let mut total_size = 0;

    for (_, floor) in &mall.floors {
        total_size += floor.size_limit;
    }

    let required = total_size / 200;
    let already_hired = mall.guards.len() as u64;

    if required <= already_hired {
        return;
    }

    let mut to_hire = required - already_hired;

    for (name, guard) in available_guards {
        if to_hire == 0 {
            break;
        }

        mall.hire_guard(&name, guard);
        to_hire -= 1;
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_, floor) in mall.floors.iter_mut() {
        for (_, store) in floor.stores.iter_mut() {
            for (_, employee) in store.employees.iter_mut() {
                let start = employee.working_hours.0;
                let end = employee.working_hours.1;

                let shift_hours = if end >= start {
                    end - start
                } else {
                    0
                };

                if shift_hours >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.cut(employee.salary * 0.1);
                }
            }
        }
    }
}

