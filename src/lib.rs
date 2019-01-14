/*
Copyright [2019] [Adrian Lloyd Flanagan]

DUAL LICENSE

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

OR

Licensed under the MIT License, see file LICENSE-MIT.txt
*/

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

    /**
     * Known directory entry types.
     */
    #[derive(Debug, PartialEq)]
    pub enum EntryType {
        File,
        Directory,
        Link,
        BlockSpecial,
        CharSpecial,
    }

    /**
     * Lexical tokens in our DSL.
     */
    #[derive(Debug, PartialEq)]
    pub enum Tok {
        /// A fixed word used to construct the query
        Keyword(String),
        /// A file or directory name
        Pathspec(String),
        /// A pattern to be matched to file or directory names
        /// Problem? unescaped globspec that matches nothing looks exactly like escaped
        GlobSpec(String),
        /// A word selecting a specific type of directory entry (file, directory, link, etc.)
        EntryType(String),
        /// A fixed word defining what to do with the selected entries
        Actionword(String),
        /// A file type
        Etype(EntryType),
    }

    /**
     * List of keywords in our DSL
     */
    pub fn keywords() -> Vec<&'static str> {
        vec!["named", "type"]
    }

    /**
     * List of words we recognize for directory entry types.
     */
    pub fn etypes() -> Vec<&'static str> {
        vec!["file", "dir", "directory", "link", "block", "char"]
    }

    /**
     * List of words that specify what to do with found entries.
     */
    pub fn actions() -> Vec<&'static str> {
        vec!["print", "delete"]
    }

    /**
     * A custom lexer for LARLPOP. It takes as input a list of arguments passed on the command line,* matches the arguments to classify them for the parser.
     */
    pub struct Lexer<'input> {
        args: Vec<&'input str>,
        index: usize,
    }

    impl<'input> Lexer<'input> {
        pub fn new(input: Vec<&'input str>) -> Self {
            Lexer {
                args: input,
                index: 0,
            }
        }
    }

    impl<'input> Iterator for Lexer<'input> {
        type Item = Spanned<Tok, usize, LexicalError>;

        fn next(&mut self) -> Option<Self::Item> {
            let arg = self.args[self.index];
            let word_index = self.index;
            self.index += 1;
            let word = &arg.to_lowercase()[..];
            if keywords().contains(&word) {
                Some(Ok((word_index, Tok::Keyword(word.to_string()), word_index)))
            } else if actions().contains(&word) {
                Some(Ok((
                    word_index,
                    Tok::Actionword(word.to_string()),
                    word_index,
                )))
            } else {
                Some(Ok((
                    word_index,
                    Tok::Pathspec(word.to_string()),
                    word_index,
                )))
            }
        }
    }

    #[derive(Debug)]
    pub enum LexicalError {
        // anything not a keyword, action, etc. is a path or a pattern
    }
}

#[cfg(test)]
mod tests {
    use crate::arg_lexer::{Lexer, Tok};

    #[test]
    fn parses_keyword() {
        let args = vec!["named"];
        let mut lex = Lexer::new(args);
        let item = lex.next().unwrap();
        match item {
            Ok((loc1, token, loc2)) => {
                assert_eq!(loc1, 0);
                assert_eq!(token, Tok::Keyword("named".to_string()));
                assert_eq!(loc2, 0);
            }
            Err(error) => {
                panic!("Test failed: {:?}", error);
            }
        }
    }

    #[test]
    fn parses_path() {
        let args = vec!["/home/fred/devel/src", "named", "*.rs"];
        let mut lex = Lexer::new(args);
        let (index, token, _) = lex.next().unwrap().unwrap();
        assert_eq!(index, 0);
        assert_eq!(token, Tok::Pathspec("/home/fred/devel/src".to_string()));
        let (index, token, _) = lex.next().unwrap().unwrap();
        assert_eq!(index, 1);
        assert_eq!(token, Tok::Keyword("named".to_string()));
        let (index, token, _) = lex.next().unwrap().unwrap();
        assert_eq!(index, 2);
        assert_eq!(token, Tok::Pathspec("*.rs".to_string()));
    }
}
