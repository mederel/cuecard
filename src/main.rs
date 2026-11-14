fn main() {
    println!("{}", greeting());
}

fn greeting() -> &'static str {
    "Cuecard: learn your lines."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert_eq!(greeting(), "Cuecard: learn your lines.");
    }
}
