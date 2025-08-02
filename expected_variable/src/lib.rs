use convert_case::{Case, Casing};
use edit_distance::edit_distance;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {


    let expected = expected.to_lowercase();
    // not logical because it would never be cammel case if lowered
    let compare = compare.to_lowercase();

    if ! (/*compare.is_case(Case::Camel) || */compare.is_case(Case::Snake)) {
        return None;
    }


    let dis = edit_distance(&compare, &expected);

    let p =  dis * 100 / expected.len() ;
    if p < 50 {
        Some(format!("{}%",100-  p))
    } else {
        None
    }
}

