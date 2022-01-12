mod parser;

fn usage () {
	println!("abcdict - Password Dictionary Generator");
	println!("https://github.com/b23r0/abcdict");
	println!("Usage: abcdict \"pass[n1-100]word[sabc-456]\"");
}

fn main() {
    
    if std::env::args().count() != 2{
        usage()
    }

    let d = std::env::args().nth(1).unwrap().as_str().to_string();

    parser::exec(d).unwrap();
}
