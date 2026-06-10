
fn shift(c: char, alph: &str, shift_by: usize) ->char {
   let alph_len = alph.len(); 
   let char_loc = alph.chars().position(|x| x == c).unwrap();
   if char_loc + shift_by < alph_len {
       alph.chars().nth(char_loc + shift_by).unwrap()
   } else {
       alph.chars().nth(shift_by - 1).unwrap()
   }
}

fn cipher(input: String, alph: &str, shift_by: usize) -> String {
    let mut ciphered = String::new();

    for i in input.chars() {
        ciphered.push(shift(i, alph, shift_by));
    }
    ciphered
}


fn main() {
    const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    println!("{}", cipher(String::from("Hello"), ALPHABETS, 3));
}
