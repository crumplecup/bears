/// Position and change measurement types in the U.S. International Investment Position (IIP) dataset.
///
/// These represent the final component of time series identifiers, indicating whether the data
/// represents a position measurement or a change in position, along with optional maturity
/// (long-term or short-term) and attribution of changes to specific factors.
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
pub enum Position {
    /// Long term change in position attributable to changes in volume and valuation n.i.e.
    #[token("LtChgPosNie")]
    LtChgPosNie,
    /// Long term change in position not attributable to financial-account transactions
    #[token("LtChgPosOth")]
    LtChgPosOth,
    /// Long term change in position attributable to price changes
    #[token("LtChgPosPrice")]
    LtChgPosPrice,
    /// Long term change in position attributable to financial-account transactions
    #[token("LtChgPosTrans")]
    LtChgPosTrans,
    /// Long term change in position attributable to exchange-rate changes
    #[token("LtChgPosXRate")]
    LtChgPosXRate,
    /// Short term change in position attributable to changes in volume and valuation n.i.e.
    #[token("StChgPosNie")]
    StChgPosNie,
    /// Short term change in position not attributable to financial-account transactions
    #[token("StChgPosOth")]
    StChgPosOth,
    /// Short term change in position attributable to price changes
    #[token("StChgPosPrice")]
    StChgPosPrice,
    /// Short term change in position attributable to financial-account transactions
    #[token("StChgPosTrans")]
    StChgPosTrans,
    /// Short term change in position attributable to exchange-rate changes
    #[token("StChgPosXRate")]
    StChgPosXRate,
    /// Change in position attributable to changes in volume and valuation n.i.e.
    #[token("ChgPosNie")]
    ChgPosNie,
    /// Change in position not attributable to financial-account transactions
    #[token("ChgPosOth")]
    ChgPosOth,
    /// Change in position attributable to price changes
    #[token("ChgPosPrice")]
    ChgPosPrice,
    /// Change in position attributable to financial-account transactions
    #[token("ChgPosTrans")]
    ChgPosTrans,
    /// Change in position attributable to exchange-rate changes
    #[token("ChgPosXRate")]
    ChgPosXRate,
    /// Long term change in position
    #[token("LtChgPos")]
    LtChgPos,
    /// Short term change in position
    #[token("StChgPos")]
    StChgPos,
    /// Change in position
    #[token("ChgPos")]
    ChgPos,
    /// Long term position
    #[token("LtPos")]
    LtPos,
    /// Short term position
    #[token("StPos")]
    StPos,
    #[default]
    /// Position
    #[token("Pos")]
    Pos,
}

impl Position {
    /// Attempts to lex a Position from the beginning of the input string.
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(Position) if a valid token was found, None otherwise
    /// - remaining_str is the unconsumed portion of the input string
    pub fn lex(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut lexer = Self::lexer(input);

        match lexer.next() {
            Some(Ok(position)) => {
                let span = lexer.span();
                let remainder = &input[span.end..];
                (Some(position), remainder)
            }
            _ => (None, input),
        }
    }

    /// Attempts to lex a Position from the end of the input string (right to left).
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(Position) if a valid token was found at the end, None otherwise
    /// - remaining_str is the unconsumed portion at the beginning of the input string
    ///
    /// This method finds the longest matching Position token at the end of the string.
    pub fn lex_from_right(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut best_match: Option<(Self, usize)> = None;

        // Try parsing from each position, starting from the beginning
        for start_idx in 0..input.len() {
            let substr = &input[start_idx..];
            let mut lexer = Self::lexer(substr);

            if let Some(Ok(position)) = lexer.next() {
                let span = lexer.span();
                // Check if the match extends to the end of the substring
                if span.end == substr.len() {
                    // This is a valid match at the end
                    // Keep it if it's longer than our current best match (lower start_idx = longer match)
                    match best_match {
                        None => best_match = Some((position, start_idx)),
                        Some((_, prev_start)) if start_idx < prev_start => {
                            best_match = Some((position, start_idx));
                        }
                        _ => {}
                    }
                }
            }
        }

        // Return the longest match found
        match best_match {
            Some((position, start_idx)) => (Some(position), &input[..start_idx]),
            None => (None, input),
        }
    }
}
