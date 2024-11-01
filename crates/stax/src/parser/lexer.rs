//! Reading XML in the cursor approach.

use core::fmt;

use rexml_encoding::Encoding;

/// `Token` represents a single lexeme of an XML docoment.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Token {
    /// `<?`
    ProcessingInstructionStart,
    /// `?>`
    ProcessingInstructionEnd,
    /// `<!DOCTYPE…`
    DoctypeStart,
    /// `<`
    OpeningTagStart,
    /// `</`
    ClosingTagStart,
    /// `>`
    TagEnd,
    /// `/>`
    EmptyTagEnd,
    /// `<!--`
    CommentStart,
    /// `-->`
    CommentEnd,
    /// Any non-special character except whitespace.
    Character(char),
    /// `=`
    EqualsSign,
    /// `'`
    SingleQuote,
    /// `"`
    DoubleQuote,
    /// `<![CDATA[`
    CDataStart,
    /// `]]>`
    CDataEnd,
    /// `&`
    ReferenceStart,
    /// `;`
    ReferenceEnd,
    /// `<!` of `ENTITY`
    MarkupDeclarationStart,
    /// End of file
    Eof,
}

impl fmt::Display for Token {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Token::Character(c) => c.fmt(f),
            other => match other {
                Token::OpeningTagStart => "<",
                Token::ProcessingInstructionStart => "<?",
                Token::DoctypeStart => "<!DOCTYPE",
                Token::ClosingTagStart => "</",
                Token::CommentStart => "<!--",
                Token::CDataStart => "<![CDATA[",
                Token::TagEnd => ">",
                Token::EmptyTagEnd => "/>",
                Token::ProcessingInstructionEnd => "?>",
                Token::CommentEnd => "-->",
                Token::CDataEnd => "]]>",
                Token::ReferenceStart => "&",
                Token::ReferenceEnd => ";",
                Token::EqualsSign => "=",
                Token::SingleQuote => "'",
                Token::DoubleQuote => "\"",
                Token::MarkupDeclarationStart => "<!",
                Token::Eof | Token::Character(_) => {
                    debug_assert!(false);
                    ""
                }
            }
            .fmt(f),
        }
    }
}

/// `Lexer` is a lexer for XML documents, which implements pull API.
#[allow(unused)]
pub struct Lexer<R> {
    /// Underlying xml stream.
    stream: R,
    ///
    encoding: Option<Encoding>,
}
