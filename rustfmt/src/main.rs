/*
set CFG_RELEASE=1.46.0-nightly
set CFG_RELEASE_CHANNEL=nightly
cargo build --all-features
*/

//
// 修改rustfmt\src\formatting\visitor.rs文件
//

pub( crate ) fn format_separate_mod( &mut self, m: &Module<'_>, end_pos: BytePos ) {
    self.block_indent = Indent::empty();

    if self.visit_attrs( m.attrs(), ast::AttrStyle::Inner ) {
        self.push_skipped_with_span( m.attrs(), m.as_ref().inner, m.as_ref().inner );
    } else {
        self.walk_mod_items( m.as_ref() );

        self.format_missing_with_indent( end_pos );

        //
        // 需要正则 lookahead lookbehind特性所以使用onig::Regex
        // https://blog.csdn.net/liuxiao723846/article/details/83278067
        //
        use onig::Captures;
        use onig::Regex;

        // 1. 左括号+1的字符如果是右括号，不做处理
        // 2. 左括号+1的字符如果是空格，不做处理
        // 3. 左括号+1的字符如果是其他字符，将左括号替换成空格
        // 4. 如果是注释行// 就不处理。如果括号在""里面表示是字符串也不处理

        // 匹配左括号
        let left = Regex::new( r#"(?<!(?:\/\/|"|').*)\((?!=\s|\)|[ ]|\s)"# ).unwrap();

        self.buffer = left.replace_all( &self.buffer, |caps: &Captures| {
            format!( "{} ", caps.at(0).unwrap_or("" ) )
        } );

        // 匹配右括号
        let right = Regex::new( r#"(?<!(?:\/\/).*)(?<!\(|[ ])\)(?!.*(?:\"|\'))"# ).unwrap();

        self.buffer = right.replace_all( &self.buffer, |caps: &Captures| {
            format!( " {}", caps.at(0).unwrap_or("" ) )
        } );

        //println!( "{}", self.buffer );
        /*
        extern{ fn my_regex( input: *mut u8, output: *mut u8, length: usize ) -> i32; }
        let mut buffer: Vec<u8> = vec![ 0; end_pos.to_u32() as usize * 2 ].into_boxed_slice().to_vec();

        unsafe {
            my_regex( self.buffer.as_mut_ptr(), buffer.as_mut_ptr(), buffer.len() );
        }

        println!( "{}", self.buffer );
        self.buffer.clear();*/
    }
}

fn main() {
    println!( "Hello, world!" );
}
