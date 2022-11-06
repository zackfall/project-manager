#![warn(missing_docs)]

//! A CLI task manager writen with tui-rs, for personal use.

fn main() {
    let h = String::from("Hola mundo");
    println!("{}!", h);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
