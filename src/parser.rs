use std::fmt::Debug;
use std::fmt;
use std::error;

pub enum SyntaxError {
	NotFindStatRightUntilEnd = 1,
	UnknowError = 2
}

impl Debug for SyntaxError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SyntaxError::NotFindStatRightUntilEnd => write!(f, "NotFindStatRightUntilEnd"),
    		SyntaxError::UnknowError=> write!(f, "UnknowError")
		}
	}
}

impl PartialEq for SyntaxError {
	fn eq(&self, other: &Self) -> bool {
		core::mem::discriminant(self) == core::mem::discriminant(other)
	}
}

impl fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			SyntaxError::NotFindStatRightUntilEnd => write!(f, "NotFindStatRightUntilEnd"),
			SyntaxError::UnknowError=> write!(f, "UnknowError")
		}
	}
}

impl error::Error for SyntaxError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		None
	}
}

#[derive(PartialEq)]
enum Token{
	Char = 1,
	StatLeft = 2,
	StatRight = 3
}

impl Clone for Token {
	fn clone(&self) -> Self {
		match self {
			Self::Char => Self::Char,
			Self::StatLeft => Self::StatLeft,
			Self::StatRight => Self::StatRight,
		}
	}
}

pub struct TokenSt{
	t : Token,
	v : u8
}

impl Clone for TokenSt {
	fn clone(&self) -> Self {
		Self { t: self.t.clone(), v: self.v }
	}
}

#[derive(PartialEq)]
enum Statement{
	Chars = 1,
	Numbers = 2,
	Strings = 3
}

impl Clone for Statement {
	fn clone(&self) -> Self {
		match self {
			Self::Chars => Self::Chars,
			Self::Numbers => Self::Numbers,
			Self::Strings => Self::Strings
		}
	}
}

pub struct StatementSt{
	t : Statement,
	v : String
}

impl Clone for StatementSt {
	fn clone(&self) -> Self {
		Self { t: self.t.clone(), v: self.v.clone() }
	}
}

fn is_escape( s : &Vec<u8> , pos : usize) -> bool{
	if pos == 0 {
		return false;
	}
	
	s[pos-1] as char == '\\'
}

fn backspace( s : &mut Vec<TokenSt>){
	s.pop();
}

fn tokenizer( s : Vec<u8> ) -> Result<Vec<TokenSt> ,SyntaxError> {
	let mut ret : Vec<TokenSt> = vec![];

	let mut i = 0;
	while i < s.len() {
		match s[i] as char {
			'[' => {
				if is_escape(&s , i) {
					backspace(&mut ret);
					ret.push(TokenSt{t : Token::Char ,v: s[i]});
				} else {
					ret.push(TokenSt{t : Token::StatLeft ,v: s[i]});
				}
			},
			']' => {
				if is_escape(&s , i) {
					backspace(&mut ret);
					ret.push(TokenSt{t : Token::Char ,v: s[i]});
				} else {
					ret.push(TokenSt{t : Token::StatRight ,v: s[i]});
				}
			},
			_ => {
				ret.push(TokenSt{t : Token::Char ,v: s[i]});
			}
		}
		i += 1;
	}

	Ok(ret)
}

fn eat(s : Vec<TokenSt> , end : Vec<Token>) -> (String , u8) {
	let mut ret: Vec<u8> = vec![];
	let mut i = 0;

	while i < s.len() {
		if !end.contains(&s[i].t) {
			ret.push(s[i].v);
		} else {
			break;
		}
		i += 1;
	}

	if i == s.len() {
		return ( String::from_utf8(ret).unwrap() , 1 ) ;
	}

	( String::from_utf8(ret).unwrap() , 0 )
}

fn parser(s : Vec<TokenSt>) -> Result<Vec<StatementSt> , SyntaxError> {
	let mut ret : Vec<StatementSt> = vec![];
	
	let mut i = 0;
	while i < s.len() {
		match s[i].t {
			Token::Char => {
				let (a, _) = eat(s[i..].to_vec(), [Token::StatLeft , Token::StatRight].to_vec());
				i += a.len();
				ret.push(StatementSt{t : Statement::Chars , v : a});
			}
			Token::StatLeft => {
				if i != s.len() - 1 {
					let (a, e) = eat(s[i + 1..].to_vec(), [Token::StatRight].to_vec());

					if e != 0 {
						return Err(SyntaxError::NotFindStatRightUntilEnd);
					}

					let mut statype = Statement::Chars;

					i += a.len() + 2;
					match a.as_bytes()[0] as char{
						'n' => {
							statype = Statement::Numbers;
						},
						's' => {
							statype = Statement::Strings;
						},
						_ => {
							return Err(SyntaxError::UnknowError); 
						}
					}
					ret.push(StatementSt{t : statype, v : String::from_utf8(a.as_bytes().to_vec()[1..].to_vec()).unwrap()});
				} else {
					return Err(SyntaxError::NotFindStatRightUntilEnd);
				}

			},
			_ => {
				return Err(SyntaxError::UnknowError);
			},
		}
	} 

	Ok(ret)
}

pub fn exec_vm(v : &mut Vec<StatementSt>) {

	match v[0].t {
		Statement::Chars => {
			print!("{}" , v[0].v);
			if v.len() == 1 {
				print!("\n");
			} else {
				exec_vm(&mut v[1..].to_vec());
			}
		}
		Statement::Numbers => {
			let ops :Vec<&str> = v[0].v.split("-").collect();
			let mut op1 = ops[0].parse::<i64>().unwrap();
			let op2 = ops[1].parse::<i64>().unwrap();

			while op1 < op2 {
				let mut tmp = [StatementSt{t : Statement::Chars , v : op1.to_string()}].to_vec();
				if v.len() != 1 {
					tmp.append(&mut v[1..].to_vec());
				}
				exec_vm(&mut tmp);
				op1 += 1;
			}
		},
		Statement::Strings => {
			let ops :Vec<&str> = v[0].v.split("-").collect();

			let mut i = 0 ;

			while i < ops.len() {
				let mut tmp = [StatementSt{t : Statement::Chars , v : ops[i].to_string()}].to_vec();
				if v.len() != 1 {
					tmp.append(&mut v[1..].to_vec());
				}
				exec_vm(&mut tmp);
				i += 0;
			}
		}
	}
}

pub fn exec(input : String) -> Result<Vec<StatementSt> , SyntaxError>{

	let s = input.as_bytes();

	let tokens = match tokenizer(s.to_vec()) {
		Ok(p) => p,
		Err(e) => {
			return Err(e);
		}
	};

	let mut ret = match parser(tokens) {
		Ok(p) => p,
		Err(e) => {
			return Err(e)
		},
	};

	exec_vm(&mut ret);

	Ok(ret)
}