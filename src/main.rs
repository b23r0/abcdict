mod parser;

fn usage () {
	println!("abcdict - A better customization dictionary generator");
	println!("https://github.com/b23r0/abcdict");
	println!("Usage: ./abcdict pass[n1-100]word[sabc-456][ca-Z]");
}

fn main() {
    
    if std::env::args().count() != 2{
        return usage();
    }

    let d = std::env::args().nth(1).unwrap().as_str().to_string();

    parser::exec(d).unwrap();
}
