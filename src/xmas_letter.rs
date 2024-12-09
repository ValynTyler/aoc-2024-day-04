pub struct NonXmasLetterError;

pub enum XmasLetter {
    X, M, A, S,
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
