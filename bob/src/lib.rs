trait Style {
    fn asking(&self) -> bool;
    fn yelling(&self) -> bool;
    fn nothing(&self) -> bool;
}


impl Style for str {
    fn asking(&self) -> bool {
        self.trim().ends_with('?')
    }
    fn yelling(&self) -> bool {
        self.to_uppercase() == self && self.to_lowercase() != self
    }
    fn nothing(&self) -> bool {
        self.trim().is_empty()
    }
}


pub fn reply(message: &str) -> &str {
    if message.nothing() {
        "Fine. Be that way!"
    } else if message.yelling() && message.asking() {
        "Calm down, I know what I'm doing!"
    } else if message.asking() {
        "Sure."
    } else if message.yelling() {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
