use std::fmt::Display;

#[derive(Debug)]
pub struct NonXmasLetterError;

#[derive(Debug, Clone, Copy, PartialEq)] 
pub enum XmasLetter {
    X, M, A, S,
}

impl Display for XmasLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom::<char> for XmasLetter {
    type Error = NonXmasLetterError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(XmasLetter::X),
            'M' => Ok(XmasLetter::M),
            'A' => Ok(XmasLetter::A),
            'S' => Ok(XmasLetter::S),
            _ => Err(NonXmasLetterError)
        }
    }
}
