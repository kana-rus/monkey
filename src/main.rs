mod lexer; use lexer::{Lexer};


fn main() {
    println!("{:?}", Lexer::toknize(String::from("
        let x = fn(a, b);
        let y = fna + 5;
    ")));
    println!("{:?}", Lexer::toknize(String::from("
        letx=fn(adcadv,let);lety=fn5 fna;
    ")));
}
