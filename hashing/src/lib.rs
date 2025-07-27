use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut temp: Vec<i32> =  list.to_vec();
    temp.sort();
    let middle = temp.len() / 2 ; 
    if list.len() & 1 == 0 /*0000_0001*/  {
        return (temp[middle -1] + temp[middle]) / 2 ;
    }else{
        return temp[middle] ; 
    }
}

pub fn mode(list: &[i32]) -> i32 {
   
    let mut map : HashMap<i32, usize> = HashMap::new();

   for num in list.iter() {
       *map.entry(*num).or_insert(0) += 1 ; 
   } 

    let mut most : usize = 0 as usize;
    let mut res : i32  = 0 ; 
    for (num, freq) in map {
        if  freq > most {
            res = num ; 
            most = freq;
        }
    }
    res
}
