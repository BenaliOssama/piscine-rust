pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut res : Vec<Box<u32>> = vec![] ; 

    let cleaned = s.split_whitespace().map(|x| {
        if x.contains("k") {
            let cut = &x[..x.len() -1]; //x.to_string().pop().unwrap();
            let res = cut.parse::<f32>().unwrap();
            return res * 1000.0;
        }else{
            return x.parse::<f32>().unwrap();
        }
    });
    for clean in cleaned {
        res.push(Box::new(clean as u32));
    }
    res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut res : Vec<u32> = vec![] ; 
    for b in a {
       res.push(*b);
    }
    res
}
