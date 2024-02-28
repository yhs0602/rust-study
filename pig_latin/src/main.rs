// 스트링을 피그 라틴(pig Latin)으로 변경해보세요.
// 각 단어의 첫번째 자음은 단어의 끝으로 이동하고 “ay”를 붙이므로,
// “first”는 “irst-fay”가 됩니다. 모음으로 시작하는 단어는 대신 끝에 “hay”를 붙입니다.
// (“apple”은 “apple-hay”가 됩니다.) UTF-8 인코딩에 대해 기억하세요!
// consonant clusters -> 모두 이동

fn pig_latinize(s: &str) -> String {
    // check the first letter
    // consonant -> find the cluster
    // vowel -> append hay and return
    let chars = s.chars();
    let mut is_first_letter = true;
    let mut consonant_cluster = Vec::new();
    let mut result_vec = Vec::new();
    let mut flushing_mode = false;
    for c in chars {
        if is_first_letter {
            is_first_letter = false;
            if "aeiou".contains(c) {
                let result = format!("{}hay", s);
                return result;
            }
            consonant_cluster.push(c);
        } else {
            if "aeiou".contains(c) {
                // consonant cluster ended
                // get the leftover and convert
                flushing_mode = true;
                result_vec.push(c);
            } else if flushing_mode {
                result_vec.push(c);
            } else {
                consonant_cluster.push(c);
            }
        }
    }
    result_vec.append(&mut consonant_cluster);
    result_vec.push('a');
    result_vec.push('y');
    result_vec.into_iter().collect()
}

fn main() {
    println!("{}", pig_latinize("apple"));
    println!("{}", pig_latinize("first"));
    println!("{}", pig_latinize("bay"));
    println!("{}", pig_latinize("zdrabtzbuite"));
    println!("{}", pig_latinize("안녕하세요"));
    println!("{}", pig_latinize("Здравствуйте"));
    println!("{}", pig_latinize("こんにちは"));
    println!("{}", pig_latinize("السلام عليكم"));
}
