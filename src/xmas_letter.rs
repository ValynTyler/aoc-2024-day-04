use std::fmt::Display;

#[derive(Debug)] pub struct NonXmasLetterError;

#[derive(Debug, PartialEq)] 
pub enum XmasLetter {
    X, M, A, S,
}

impl Display for XmasLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryInto::<XmasLetter> for char {
    type Error = NonXmasLetterError;

    fn try_into(self) -> Result<XmasLetter, Self::Error> {
        match self {
            'X' => Ok(XmasLetter::X),
            'M' => Ok(XmasLetter::M),
            'A' => Ok(XmasLetter::A),
            'S' => Ok(XmasLetter::S),
            _ => Err(NonXmasLetterError),
        }
    }
}

impl XmasLetter {
    pub const XMAS: [XmasLetter; 4] = [
        XmasLetter::X,
        XmasLetter::M,
        XmasLetter::A,
        XmasLetter::S,
    ];
}
