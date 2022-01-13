use std::fmt::Debug;
use std::fmt;
use std::error;
use std::fmt::format;

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
	Strings = 3,
	Char = 4,
	PaddedNumber = 5
}

impl Clone for Statement {
	fn clone(&self) -> Self {
		match self {
			Self::Chars => Self::Chars,
			Self::Numbers => Self::Numbers,
			Self::Strings => Self::Strings,
			Self::Char => Self::Char,
			Self::PaddedNumber => Self::PaddedNumber,
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
			' ' => {
				if is_escape(&s , i) {
					backspace(&mut ret);
					ret.push(TokenSt{t : Token::Char ,v: s[i]});
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
						'c' => {
							statype = Statement::Char;
						},
						'p' => {
							statype = Statement::PaddedNumber;
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

pub fn exec_stat(v : &mut Vec<StatementSt> , curstate : &mut String) {

	match v[0].t {
		Statement::Chars => {
			
			if v.len() == 1 {
				println!("{}{}" , curstate ,v[0].v);
			} else {
				let tmp = curstate.clone();
				*curstate += &v[0].v;
				exec_stat(&mut v[1..].to_vec(), curstate);
				*curstate = tmp;
			}
		}
		Statement::Numbers => {
			let ops :Vec<&str> = v[0].v.split("-").collect();
			let mut op1 = ops[0].parse::<i64>().unwrap();
			let op2 = ops[1].parse::<i64>().unwrap();

			while op1 <= op2 {
				let mut tmp = [StatementSt{t : Statement::Chars , v : op1.to_string()}].to_vec();
				if v.len() != 1 {
					tmp.append(&mut v[1..].to_vec());
				}
				exec_stat(&mut tmp , curstate);
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
				exec_stat(&mut tmp , curstate);
				i += 1;
			}
		},
		Statement::Char => {
			let ops :Vec<&str> = v[0].v.split("-").collect();
			let mut op1 = ops[0].parse::<char>().unwrap();
			let op2 = ops[1].parse::<char>().unwrap();

			while op1 <= op2 {
				let mut tmp = [StatementSt{t : Statement::Chars , v : op1.to_string()}].to_vec();
				if v.len() != 1 {
					tmp.append(&mut v[1..].to_vec());
				}
				exec_stat(&mut tmp , curstate);
				op1 = (op1 as u8 + 1) as char;
			}
		},
		Statement::PaddedNumber => {
			let ops :Vec<&str> = v[0].v.split("-").collect();
			let op1 = ops[0].parse::<char>().unwrap();
			let op2 = ops[1].parse::<usize>().unwrap();
			let mut op3 = ops[2].parse::<i64>().unwrap();
			let op4 = ops[3].parse::<i64>().unwrap();

			while op3 <= op4 {

				let mut full = String::new();

				if op3.to_string().len() < op2{
					let mut i = 0 ;
					while i < op2 - op1.to_string().len(){
						full += &op1.to_string();
						i += 1;
					}
				}
				full += &op3.to_string();

				let mut tmp = [StatementSt{t : Statement::Chars , v : full}].to_vec();
				if v.len() != 1 {
					tmp.append(&mut v[1..].to_vec());
				}
				exec_stat(&mut tmp , curstate);
				op3 += 1;
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
	let mut curstate = String::new();
	exec_stat(&mut ret , &mut curstate);

	Ok(ret)
}