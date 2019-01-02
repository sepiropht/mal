use std::io;

fn main() {
    loop {
    println!("user>");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("something went wrong");

    repl(input);
    }
}

fn read(input: String) -> String {
        input
}

fn eval(token: &str) -> &str {
    token
}

fn print(token: &str) -> &str {
    token
}

fn repl(input: String) {
        let read = read(input);
        let eval = eval(&read);
        print(eval);
}
