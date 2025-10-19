/// Equity and claims classifications used in the U.S. International Investment Position (IIP) dataset.
///
/// These represent the type of equity position or claims relationship, including whether it's
/// an asset or liability position, claims by different entity types, and valuation methods.
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
pub enum IipEquity {
    #[default]
    /// U.S. assets
    #[token("Assets")]
    Assets,
    /// claims
    #[token("Claims")]
    Claims,
    /// by non-SPE affiliates
    #[token("ClaimsByNonSpe")]
    ClaimsByNonSpe,
    /// by SPE affiliates
    #[token("ClaimsBySpe")]
    ClaimsBySpe,
    /// U.S. liabilities
    #[token("Liabs")]
    Liabs,
    /// liabilities in
    #[token("LiabsIn")]
    LiabsIn,
    /// market value
    #[token("MarketValue")]
    MarketValue,
}

impl IipEquity {
    /// Attempts to lex an IipEquity from the beginning of the input string.
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(IipEquity) if a valid token was found, None otherwise
    /// - remaining_str is the unconsumed portion of the input string
    pub fn lex(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut lexer = Self::lexer(input);

        match lexer.next() {
            Some(Ok(iip_equity)) => {
                let span = lexer.span();
                let remainder = &input[span.end..];
                (Some(iip_equity), remainder)
            }
            _ => (None, input),
        }
    }
}
