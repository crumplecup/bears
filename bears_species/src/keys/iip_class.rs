/// Classification types used in the U.S. International Investment Position (IIP) dataset.
///
/// These represent the high-level category or type of investment instrument, transaction,
/// or adjustment, such as debt instruments, equity, derivative contracts, and directional flows.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
    logos::Logos,
)]
pub enum IipClass {
    /// Claims by
    #[token("ClaimsBy")]
    ClaimsBy,
    /// Outward (U.S. direct investment abroad)
    #[token("Outward")]
    Outward,
    /// Liabilities in
    #[token("LiabsIn")]
    LiabsIn,
    /// Inward (foreign direct investment in the United States)
    #[token("Inward")]
    Inward,
    /// U.S. assets
    #[token("Assets")]
    Assets,
    /// Claims
    #[token("Claims")]
    Claims,
    #[default]
    /// U.S. liabilities
    #[token("Liabs")]
    Liabs,
}

impl IipClass {
    /// Attempts to lex an IipClass from the beginning of the input string.
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(IipClass) if a valid token was found, None otherwise
    /// - remaining_str is the unconsumed portion of the input string
    pub fn lex(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut lexer = Self::lexer(input);

        match lexer.next() {
            Some(Ok(iip_class)) => {
                let span = lexer.span();
                let remainder = &input[span.end..];
                (Some(iip_class), remainder)
            }
            _ => (None, input),
        }
    }

    /// Attempts to lex an IipClass from the end of the input string (right to left).
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(IipClass) if a valid token was found at the end, None otherwise
    /// - remaining_str is the unconsumed portion at the beginning of the input string
    ///
    /// This method finds the longest matching IipClass token at the end of the string.
    pub fn lex_from_right(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut best_match: Option<(Self, usize)> = None;

        // Try parsing from each position, starting from the beginning
        for start_idx in 0..input.len() {
            let substr = &input[start_idx..];
            let mut lexer = Self::lexer(substr);

            if let Some(Ok(class)) = lexer.next() {
                let span = lexer.span();
                // Check if the match extends to the end of the substring
                if span.end == substr.len() {
                    // This is a valid match at the end
                    // Keep it if it's longer than our current best match (lower start_idx = longer match)
                    match best_match {
                        None => best_match = Some((class, start_idx)),
                        Some((_, prev_start)) if start_idx < prev_start => {
                            best_match = Some((class, start_idx));
                        }
                        _ => {}
                    }
                }
            }
        }

        // Return the longest match found
        match best_match {
            Some((class, start_idx)) => (Some(class), &input[..start_idx]),
            None => (None, input),
        }
    }

    /// Returns the description for this IIP class.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::ClaimsBy => "Claims by",
            Self::Outward => "Outward (U.S. direct investment abroad)",
            Self::LiabsIn => "Liabilities in",
            Self::Inward => "Inward (foreign direct investment in the United States)",
            Self::Assets => "U.S. assets",
            Self::Claims => "Claims",
            Self::Liabs => "U.S. liabilities",
        }
    }
}
