//! Pure-Rust core for the cross-platform spike: no platform code, no dependencies.

/// Result of comparing what was said to the expected line.
#[derive(Debug, PartialEq, Eq)]
pub struct Match {
    /// Expected words found, in order, in what was said.
    pub matched: usize,
    /// Words in the expected line.
    pub total: usize,
}

/// Lowercases, removes French accents and turns punctuation into spaces.
pub fn normalise(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.to_lowercase().chars() {
        match c {
            'à' | 'â' | 'ä' => out.push('a'),
            'ç' => out.push('c'),
            'é' | 'è' | 'ê' | 'ë' => out.push('e'),
            'î' | 'ï' => out.push('i'),
            'ô' | 'ö' => out.push('o'),
            'ù' | 'û' | 'ü' => out.push('u'),
            'ÿ' => out.push('y'),
            'œ' => out.push_str("oe"),
            'æ' => out.push_str("ae"),
            c if c.is_alphanumeric() => out.push(c),
            _ => out.push(' '),
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Compares `said` to `expected` word by word after normalising both.
///
/// A word counts as matched when it appears in the same order in both, so a
/// replaced or missing word is missed, and extra words are not penalised.
pub fn matching_words(expected: &str, said: &str) -> Match {
    let expected = normalise(expected);
    let said = normalise(said);
    let expected: Vec<&str> = expected.split(' ').filter(|w| !w.is_empty()).collect();
    let said: Vec<&str> = said.split(' ').filter(|w| !w.is_empty()).collect();

    // Longest common subsequence of words, one row at a time.
    let mut previous = vec![0; said.len() + 1];
    for e in &expected {
        let mut current = vec![0; said.len() + 1];
        for (j, s) in said.iter().enumerate() {
            current[j + 1] = if e == s {
                previous[j] + 1
            } else {
                previous[j + 1].max(current[j])
            };
        }
        previous = current;
    }
    Match {
        matched: previous[said.len()],
        total: expected.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE: &str = "Vous êtes donc bien tendre à la tentation ?";

    #[test]
    fn normalise_lowercases_and_strips_accents() {
        assert_eq!(
            normalise("Vous êtes ÉLÈVE à Noël"),
            "vous etes eleve a noel"
        );
    }

    #[test]
    fn normalise_handles_ligatures_and_cedilla() {
        assert_eq!(normalise("Cœur, façade, æther"), "coeur facade aether");
    }

    #[test]
    fn normalise_treats_apostrophes_alike() {
        assert_eq!(normalise("l’amour"), normalise("l'amour"));
        assert_eq!(normalise("l’amour"), "l amour");
    }

    #[test]
    fn normalise_drops_punctuation_and_guillemets() {
        assert_eq!(normalise("« Eh bien ! », dit-il ?"), "eh bien dit il");
    }

    #[test]
    fn normalise_collapses_whitespace() {
        assert_eq!(normalise("  a \t b\n\nc  "), "a b c");
    }

    #[test]
    fn identical_line_matches_fully() {
        assert_eq!(
            matching_words(LINE, LINE),
            Match {
                matched: 8,
                total: 8
            }
        );
    }

    #[test]
    fn accents_case_and_punctuation_do_not_matter() {
        let said = "vous etes DONC bien tendre a la tentation";
        assert_eq!(
            matching_words(LINE, said),
            Match {
                matched: 8,
                total: 8
            }
        );
    }

    #[test]
    fn a_replaced_word_counts_as_missed() {
        let said = "Vous êtes donc très tendre à la tentation";
        assert_eq!(
            matching_words(LINE, said),
            Match {
                matched: 7,
                total: 8
            }
        );
    }

    #[test]
    fn a_missing_word_counts_as_missed() {
        let said = "Vous êtes bien tendre à la tentation";
        assert_eq!(
            matching_words(LINE, said),
            Match {
                matched: 7,
                total: 8
            }
        );
    }

    #[test]
    fn extra_words_are_not_penalised() {
        let said = "Alors vous êtes donc bien tendre à la tentation";
        assert_eq!(
            matching_words(LINE, said),
            Match {
                matched: 8,
                total: 8
            }
        );
    }

    #[test]
    fn order_matters() {
        let said = "tentation la à tendre bien donc êtes vous";
        let result = matching_words(LINE, said);
        assert_eq!(result.total, 8);
        assert_eq!(result.matched, 1);
    }

    #[test]
    fn saying_nothing_matches_nothing() {
        assert_eq!(
            matching_words(LINE, ""),
            Match {
                matched: 0,
                total: 8
            }
        );
    }

    #[test]
    fn an_empty_expected_line_has_no_words() {
        assert_eq!(
            matching_words("", "bonjour"),
            Match {
                matched: 0,
                total: 0
            }
        );
    }
}
