use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map : HashMap<&'a str, usize> = HashMap::new();

    for word in words {
        match map.get(word) {
            Some(&number) => {map.insert(word ,  number + 1);} ,
            _ => {map.insert(word, 1);},
        }
    }

    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut count : usize = 0 ; 
    for (word , freq) in frequency_count {
        //if *freq == 1  {
            count += 1 ;
        //}
    }
    count
}
