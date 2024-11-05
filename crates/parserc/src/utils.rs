pub trait FindChar {
    fn find(&self, c: char) -> bool;
}

impl FindChar for &str {
    fn find(&self, c: char) -> bool {
        self.chars().find(|cc| *cc == c).is_some()
    }
}

impl FindChar for String {
    fn find(&self, c: char) -> bool {
        self.chars().find(|cc| *cc == c).is_some()
    }
}
