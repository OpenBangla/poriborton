
struct LazyString {
    delayed: [char; 10],
    index: usize,
    string: String,
}

impl LazyString {
    pub(crate) fn new() -> Self {
        LazyString {
            delayed: Default::default(),
            index: 0,
            string: String::with_capacity(20)
        }
    }

    pub(crate) fn push(&mut self, ch: char) {
        if self.index < 11 {
            self.delayed[self.index] = ch;
            self.index += 1;
        } else {
            // Spill the delayed chars into the string.
            self.string.extend(&self.delayed);
            self.string.push(ch);
            self.index = 0;
        }
    }

    pub(crate) fn direct_push(&mut self, ch: char) {
        self.string.push(ch);
    }

    pub(crate) fn give_uncommitted(&self) -> &[char] {
        &self.delayed[0..self.index]
    }

    pub(crate) fn to_string(self) -> String {
        self.string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_string() {
        let mut string = LazyString::new();

        string.push('a');
        string.push('b');
        string.push('c');
        assert_eq!(string.give_uncommitted(), &['a', 'b', 'c']);
    }
}
