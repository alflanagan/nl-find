/**
 * Define a custom lexer for LALRPOP, which takes as input the shell-parsed list
 * of arguments.
 *
 * This has the advantage that we don't have to figure out quoting and file/directory
 * names with unusual characters.
 */

pub mod arg_lexer {
    /**
     * The type expected by the parser.
     *
     * For our purposes, both Loc values are the same: the index of the token in the
     * args array.
     */
    pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

    #[derive(Debug, PartialEq)]
    pub enum Tok {
        /// A fixed word used to construct the query
        Keyword,
        /// A file or directory name
        Pathspec,
        /// A word selecting a specific type of directory entry (file, directory, link, etc.)
        EntryType,
        /// A fixed word defining what to do with the selected entries
        Actionword,
    }

    pub fn keywords() -> Vec<&'static str> {
        vec!["name", "type"]
    }

    pub fn etypes() -> Vec<&'static str> {
        vec!["file", "dir", "directory", "link", "block", "char"]
    }

    pub fn actions() -> Vec<&'static str> {
        vec!["print", "delete"]
    }

    pub struct Lexer<'input> {
        args: Vec<&'input String>,
        index: usize,
    }

    impl<'input> Lexer<'input> {
        pub fn new(input: Vec<&'input String>) -> Self {
            Lexer {
                args: input,
                index: 0,
            }
        }
    }

    #[derive(Debug)]
    pub enum LexicalError {
        // anything not a keyword, action, etc. is a path or a pattern
    }

    impl<'input> Iterator for Lexer<'input> {
        type Item = Spanned<Tok, usize, LexicalError>;

        fn next(&mut self) -> Option<Self::Item> {
            let arg = self.args[self.index];
            self.index += 1;
            let word = &arg.to_lowercase()[..];
            if keywords().contains(&word) {
                Some(Ok((self.index - 1, Tok::Keyword, self.index - 1)))
            } else {
                Some(Ok((self.index - 1, Tok::Pathspec, self.index - 1)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::arg_lexer::{Lexer, Tok};

    #[test]
    fn parses_keyword() {
        let name = String::from("name");
        let args = vec![&name];
        let mut lex = Lexer::new(args);
        let item = lex.next().unwrap();
        match item {
            Ok((loc1, token, loc2)) => {
                assert_eq!(loc1, 0);
                assert_eq!(token, Tok::Keyword);
                assert_eq!(loc2, 0);
            }
            Err(error) => {
                panic!("Test failed: {:?}", error);
            }
        }
    }
}
