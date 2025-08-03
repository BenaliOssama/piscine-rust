pub fn pig_latin(text: &str) -> String {
    let mut result = String::new();

    for word in text.split_whitespace() {
        let mut nb = 0;
        let word_chars: Vec<char> = word.chars().collect();

        // count how many starting consonants
        while nb < word_chars.len() && !is_vowel(word_chars[nb]) {
            nb += 1;
        }

        // check for "qu" special case
        if nb >= 2 && nb < word_chars.len() &&
            word_chars[nb - 1] == 'q' && word_chars[nb] == 'u' {
            nb += 1;
        }

        // build new word
        let mut new_word = String::new();

        for i in nb..word_chars.len() {
            new_word.push(word_chars[i]);
        }
        for i in 0..nb {
            new_word.push(word_chars[i]);
        }

        new_word.push_str("ay");

        // add space if needed
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&new_word);
    }

    result
}

fn is_vowel(c: char) -> bool {
    let c = c.to_ascii_lowercase();
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

