use super::Spam;

impl Spam {
    pub fn create() -> Spam {
        Spam
    }

    pub fn consume(self) -> Vec<Spam> {
        vec![self]
    }
}
