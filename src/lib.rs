use std::str::Chars;

mod kyu_8;
fn code_war() {
    let x: u64 = 9119; // 81 1 1 81
    let y = &9119; // 81 1 1 81

    let mut res: String = String::new();
    for c in x.to_string().chars() {
        println!("{:?}", c.to_digit(10));

    }
    println!("{res}")
}
#[cfg(test)]
mod tests {
    use crate::code_war;

    #[test]
    fn it_works() {
        code_war();
    }
}
