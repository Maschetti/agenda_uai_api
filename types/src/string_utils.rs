pub trait StringUtils {
    fn has_whitespace(&self) -> bool;
    fn has_non_letters_or_spaces(&self) -> bool;
    fn normalize_spaces(&self) -> String;
    fn remove_spaces(&self) -> String;
    fn only_digits(&self) -> String; // << nova função
}

impl StringUtils for String {
    fn has_whitespace(&self) -> bool {
        self.contains(' ')
    }

    fn has_non_letters_or_spaces(&self) -> bool {
        self.chars().any(|c| !c.is_alphabetic() && !c.is_whitespace())
    }

    fn normalize_spaces(&self) -> String {
        self.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn remove_spaces(&self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }

    fn only_digits(&self) -> String {
        self.chars().filter(|c| c.is_ascii_digit()).collect()
    }
}
