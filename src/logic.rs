
pub struct InGame<T>(T);
pub struct InRust<T>(T);

pub trait Portable {
    fn game(&self) -> InGame<&Self> {
        InGame(self)
    }

    fn rust(&self) -> InRust<&Self> {
        InRust(self)
    }

    fn port(&self) -> crate::parser::Code;
}