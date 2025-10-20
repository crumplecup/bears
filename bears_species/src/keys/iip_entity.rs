/// Entity classifications used in the U.S. International Investment Position (IIP) dataset.
///
/// These represent various dimensions of classification including asset/liability type,
/// institutional sectors, currencies, maturities, valuation methods, and other attributes
/// that combine to form the complete time series identifiers.
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
pub enum IipEntity {
    /// Nonfinancial institutions except general government
    #[token("NonFinExclGenGovt")]
    NonFinExclGenGovt,
    /// Deposit-taking institutions except central bank
    #[token("DepExclCenBank")]
    DepExclCenBank,
    /// Central bank
    #[token("CenBank")]
    CenBank,
    /// General government
    #[token("GenGovt")]
    GenGovt,
    /// Other financial institutions
    #[token("OthFin")]
    OthFin,
    #[default]
    /// Foreign official agencies
    #[token("Foa")]
    Foa,
}

impl IipEntity {
    /// Attempts to lex an IipEntity from the beginning of the input string.
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(IipEntity) if a valid token was found, None otherwise
    /// - remaining_str is the unconsumed portion of the input string
    pub fn lex(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut lexer = Self::lexer(input);

        match lexer.next() {
            Some(Ok(iip_entity)) => {
                let span = lexer.span();
                let remainder = &input[span.end..];
                (Some(iip_entity), remainder)
            }
            _ => (None, input),
        }
    }

    /// Attempts to lex an IipEntity from the end of the input string (right to left).
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(IipEntity) if a valid token was found at the end, None otherwise
    /// - remaining_str is the unconsumed portion at the beginning of the input string
    ///
    /// This method finds the longest matching IipEntity token at the end of the string.
    pub fn lex_from_right(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut best_match: Option<(Self, usize)> = None;

        // Try parsing from each position, starting from the beginning
        for start_idx in 0..input.len() {
            let substr = &input[start_idx..];
            let mut lexer = Self::lexer(substr);

            if let Some(Ok(entity)) = lexer.next() {
                let span = lexer.span();
                // Check if the match extends to the end of the substring
                if span.end == substr.len() {
                    // This is a valid match at the end
                    // Keep it if it's longer than our current best match (lower start_idx = longer match)
                    match best_match {
                        None => best_match = Some((entity, start_idx)),
                        Some((_, prev_start)) if start_idx < prev_start => {
                            best_match = Some((entity, start_idx));
                        }
                        _ => {}
                    }
                }
            }
        }

        // Return the longest match found
        match best_match {
            Some((entity, start_idx)) => (Some(entity), &input[..start_idx]),
            None => (None, input),
        }
    }

    /// Returns the description for this IIP entity.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::NonFinExclGenGovt => "Nonfinancial institutions except general government",
            Self::DepExclCenBank => "Deposit-taking institutions except central bank",
            Self::CenBank => "Central bank",
            Self::GenGovt => "General government",
            Self::OthFin => "Other financial institutions",
            Self::Foa => "Foreign official agencies",
        }
    }
}
