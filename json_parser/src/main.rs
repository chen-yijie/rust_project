
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

pub mod implement;

#[derive(Debug, PartialEq)]
pub enum Token {
    Comma, 
    Colon, 
    BracketOn, 
    BracketOff, 
    BraceOn,
    BraceOff,
    String( String ),
    Number( f64 ),
    Boolean( bool ),
    Null,
}

#[derive(Debug)]
pub enum Json {
    Null, 
    String( String ),
    Number( f64 ),
    Boolean( bool ),
    Array( Vec< Json > ),
    Object( HashMap< String, Json > ),
}

impl Json {
    pub fn is_null(&self) -> bool {
        match *self {
            Json::Null => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match *self {
            Json::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Json::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match *self {
            Json::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Json::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match *self {
            Json::Object(_) => true,
            _ => false,
        }
    }
}

pub struct Tokenizer<'a> {
    source: Peekable< Chars<'a> >,
}

impl<'a> Tokenizer <'a> {
    pub fn new( s: &'a str ) -> Self {
        Self {
            source: s.chars().peekable(),
        }
    }

    fn read_symbol( &mut self, first: char ) -> String {
        let mut symbol = first.to_string();

        while let Some( &ch ) = self.source.peek() {
            match ch {
                'a' ..= 'z' => {
                    symbol.push( ch );
                    self.source.next();
                }

                // 遇到非英文小写字母，判定它结束
                _ => break,
            }
        }
        
        symbol
    }

    // "xxx 12.345 xxx"
    //      ^^^^^^
    //        |
    //     这里是数字
    // 可能带有小数点,也可能不带有小数点,
    // 那么,我们可以设立一个游标cursor,去一步一步的前进(cursor++),
    // 如果如果是字符数字,那么保存这个数字,并且向下一步前进.
    // 当遇到小数点的时候,判断这个小数点是否是第一次出现, 如果是第一次出现,
    // 那没什么问题,如果是第二次出现及以上,那么说明这个数字字符串是无效非法的.
    fn read_number( &mut self, first: char ) -> f64 {
        let mut value = first.to_string();
        let mut point = false;

        while let Some( &ch ) = self.source.peek() {
            match ch {
                '0' ..= '9' => {
                    value.push( ch );
                    self.source.next();
                }
                '.' => {
                    if !point {
                        point = true;
                        value.push( ch );
                        self.source.next();
                    }else { 
                        return value.parse::<f64>().unwrap();
                    }
                }
                _ => {
                    return value.parse::<f64>().unwrap();
                }
            }
        }
        value.parse::<f64>().unwrap()
    }

    // r#" "this is a string" "
    //     ^^^^^^^^^^^^^^^^^^
    // 对于字符串来说,以双引号开头,以双引号结尾,同时,字符串中可能会有'\'反斜杠开头的转义字符.
    //
    fn read_string( &mut self, first: char ) -> String {
        let mut value = String::new();
        let mut escape = false;

        while let Some(ch) = self.source.next() {
            if ch == first && !escape {
                return value;
            }
            match ch {
                '\\' => {
                    if escape {
                        escape = false;
                        value.push(ch);
                    } else {
                        escape = true;
                    }
                }
                _ => {
                    value.push(ch);
                    escape = false;
                }
            }
        }
        value
    }

}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next( &mut self ) -> Option< Self::Item> {
        'lex: while let Some( ch ) = self.source.next() {
            return Some( match ch {
                ',' => Token::Comma,
                ':' => Token::Colon,
                '[' => Token::BracketOn,
                ']' => Token::BracketOff,
                '{' => Token::BraceOn,
                '}' => Token::BraceOff,
                '"' => Token::String( self.read_string( ch )),
                '0' ..= '9' => Token::Number( self.read_number( ch )),
                'a' ..= 'z' => {
                    let label = self.read_symbol( ch );
                    match label.as_ref() {
                        "true" => Token::Boolean( true ),
                        "false" => Token::Boolean( false ),
                        "null" => Token::Null,
                        _ => panic!( "Invalid label: {}", label ),
                    }
                }
                _  => {
                    if ch.is_whitespace() {
                        continue 'lex;
                    } else {
                        panic!("Invalid character: {}", ch);
                    }
                }
            });
        }
        None
    }
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new( s: &'a str ) -> Self {
        Self {
            tokenizer: Tokenizer::new( s ),
        }
    }

    pub fn parse( &mut self ) -> Json {
        let token = self.step();
        self.parse_from( token )
    }

    fn step( &mut self ) -> Token {
        self.tokenizer.next().expect( "Unexpected end of JSON!!!" )
    }

    fn parse_array( &mut self ) -> Json {
        let mut array = Vec::new();

        match self.step() {
            Token::BracketOff => return array.into(), 
            token => array.push( self.parse_from( token ) ),
        }

        loop {
            match self.step() {
                Token::Comma => array.push( self.parse() ),
                Token::BracketOff => break,
                token => panic!( "Unexpected token {:?}", token ),
            }
        }

        array.into()
    }

    fn parse_object( &mut self ) -> Json {
        let mut object = HashMap::new();

        match self.step() {
            Token::BraceOff => return object.into(), 
            Token::String( key ) => {
                match self.step() {
                    Token::Colon => do_nothing(), 
                    token => panic!( "Unexpected token {:?}", token ),
                }
                let value = self.parse();
//                dbg!( &key );
//                dbg!( &value );
                object.insert( key, value );
            }
            token => panic!( "Unexpected token {:?}", token ),
        }

        loop {
            match self.step() {
                Token::Comma => {
                    let key = match self.step() {
                        Token::String( key ) => key,
                        token => panic!("Unexpected token {:?}", token),
                    };
                    match self.step() {
                        Token::Colon => {}
                        token => panic!("Unexpected token {:?}", token),
                    }

                    let value = self.parse();
//                    dbg!( &key );
//                    dbg!( &value );
                    object.insert( key, value );
                }

                Token::BraceOff => break,
                token => panic!( "Unexpected token {:?}", token ),
            }
        }
        object.into()
    }

    fn parse_from( &mut self, token: Token ) -> Json {
        match token {
            Token::Null => Json::Null,
            Token::String( s) => Json::String( s ),
            Token::Number( n ) => Json::Number( n ),
            Token::Boolean( b ) => Json::Boolean( b ),
            Token::BracketOn => self.parse_array(),
            Token::BraceOn => self.parse_object(), 
            _ => panic!( "Unexpected token: {:?}", token ),
        }
    }
}

pub fn parse( s: &str ) -> Json {
    let mut parser = Parser::new( s );
    parser.parse()
}

fn do_nothing() {}

fn main() {
    let s = parse( r#"
        {
            "name": "三班",
            "students": [
              {
                "age": 25,
                "gender": "female",
                "grades": "三班",
                "name": "露西",
                "score": {
                  "网络协议": 98,
                  "JavaEE": 92,
                  "计算机基础": 93
                },
                "weight": 51.3
              },
              {
                "age": 26,
                "gender": "male",
                "grades": "三班",
                "name": "杰克",
                "score": {
                  "网络安全": 75,
                  "Linux操作系统": 81,
                  "计算机基础": 92
                },
                "weight": 66.5
              },
              {
                "age": 25,
                "gender": "female",
                "grades": "三班",
                "name": "莉莉",
                "score": {
                  "网络安全": 95,
                  "Linux操作系统": 98,
                  "SQL数据库": 88,
                  "数据结构": 89
                },
                "weight": 55
              }
            ]
          }
          "#
    );
    
    println!( "Josn-> {:#?} ", s );
}
