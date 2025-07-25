pub fn nbr_function(c: i32) -> (i32, f64, f64) {

    (c, (c as f64).exp(), (c as f64).abs().ln() )
}


pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    (b.clone(), b.iter().map(|x| (*x as f64).abs().ln()).collect())
}

pub fn str_function(a: String) -> (String, String) {
    let str = a.clone();

    (a , str.split_whitespace().map( |num| 
        num.parse::<f64>().unwrap().abs().exp().to_string()
    ).collect::<Vec<String>>().join(" ") /*vec<String>*/)
}
