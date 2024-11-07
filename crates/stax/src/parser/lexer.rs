//! Reading XML in the cursor approach.

use core::fmt;

use super::{InputStream, IntoInputStream};

/// `Token` represents a single lexeme of an XML docoment.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Token<'a> {
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
    /// Name token.
    Name(&'a str),
    /// NmToken.
    Nmtoken(&'a str),
    /// White space
    S(&'a str),
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
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
            Token::Name(v) => v,
            Token::Nmtoken(v) => v,
            Token::S(v) => v,
            Token::Eof => {
                debug_assert!(false);
                ""
            }
        }
        .fmt(f)
    }
}

/// `Lexer` is a lexer for XML documents, which implements pull API.
#[allow(unused)]
pub struct Lexer<I> {
    /// Input stream for lexer.
    input: I,
}

impl<I> Lexer<I>
where
    I: InputStream,
{
    /// Create a new `Lexer` instance from an `IntoInputStream`.
    pub fn new<II>(input: II) -> Self
    where
        II: IntoInputStream<InputStream = I>,
    {
        Self {
            input: input.into_input_stream(),
        }
    }

    pub fn next(&self) -> Option<Token> {
        todo!()
    }
}
