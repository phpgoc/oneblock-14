pub enum Signal {
    Red,
    Yellow,
    Green,
}

impl Signal {
    pub fn time(&self) -> usize {
        match self {
            Signal::Red => 60,
            Signal::Yellow => 5,
            Signal::Green => 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        assert_eq!(Signal::Red.time(), 60);
        assert_eq!(Signal::Yellow.time(), 5);
        assert_eq!(Signal::Green.time(), 30);
    }
}
