pub fn compare_strings(s1: &str, s2: &str) -> Vec<bool> {
    let mut results = Vec::new();

    if s1.len() != s2.len() {
        // Handle the case where lengths are different
        // For now, we just return the empty vector
        return results;
    }

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        results.push(c1 == c2);
    }

    results
}

pub fn mask_word(matches: Vec<bool>, input: &str) -> String {
    let mut masked_word = String::new();
    
    if matches.len() != input.len() {
        // Handle the case where lengths are different
        // For now, we return an error message or handle it as needed
        return "Length mismatch".to_string();
    }

    for (&match_char, char_from_input) in matches.iter().zip(input.chars()) {
        if match_char {
            masked_word.push(char_from_input);
        } else {
            masked_word.push('*');
        }
    }
    return masked_word
}