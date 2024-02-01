fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length(&mut s1);
    println!("The length of '{}' is {}.", s1, len);

    {
        let _r1 = &mut s1;
    }

    let _r2 = &mut s1;
    let r3 = &s1;

    //println!("The value of r2 is {}.", r2);
    println!("The value of r3 is {}.", r3);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("The first word of {} is {}.", my_string, word);
    let word = first_word(&my_string);
    println!("The first word of {} is {}, without slices.", my_string, word);
    let mut new_string = my_string.clone();
    new_string.push_str(" haka oaao");
    println!("new string is {:?}", new_string);
    let sliced_word = first_word(&new_string[8..]);
    println!("The first word of {} is {}. slices.", &new_string[8..], sliced_word);
    // let bad_word = "bad word";
    //let &mut bad_mut = bad_word;


}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
