use crate::Describe;

/// Currency and attribute classifications used in the U.S. International Investment Position (IIP) dataset.
///
/// These represent additional modifiers that can appear after the main IipEntity components,
/// specifying currency denominations, cost/valuation methods, SPE classification, and SDR basket membership.
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
pub enum IipCurrency {
    /// Adjustment to revalue equity from historical cost to market value
    #[token("HistCostToMarketValueAdj")]
    HistCostToMarketValueAdj,
    /// Not in special drawing rights basket
    #[token("NotSdrBasket")]
    NotSdrBasket,
    /// In special drawing rights basket
    #[token("SdrBasket")]
    SdrBasket,
    /// Market value
    #[token("MarketValue")]
    MarketValue,
    /// Current cost
    #[token("CurrCost")]
    CurrCost,
    /// Historical cost
    #[token("HistCost")]
    HistCost,
    /// Non-SPEs
    #[token("NonSpe")]
    NonSpe,
    /// Other foreign currency
    #[token("OthFc")]
    OthFc,
    /// Euro
    #[token("Euro")]
    Euro,
    /// SPEs
    #[token("Spe")]
    Spe,
    /// U.S. dollar
    #[token("Usd")]
    Usd,
    /// Yen
    #[token("Yen")]
    Yen,
    #[default]
    /// Foreign currency
    #[token("Fc")]
    Fc,
}

impl IipCurrency {
    /// Attempts to lex an IipCurrency from the beginning of the input string.
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(IipCurrency) if a valid token was found, None otherwise
    /// - remaining_str is the unconsumed portion of the input string
    pub fn lex(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut lexer = Self::lexer(input);

        match lexer.next() {
            Some(Ok(iip_currency)) => {
                let span = lexer.span();
                let remainder = &input[span.end..];
                (Some(iip_currency), remainder)
            }
            _ => (None, input),
        }
    }

    /// Attempts to lex an IipCurrency from the end of the input string (right to left).
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(IipCurrency) if a valid token was found at the end, None otherwise
    /// - remaining_str is the unconsumed portion at the beginning of the input string
    ///
    /// This method finds the longest matching IipCurrency token at the end of the string.
    pub fn lex_from_right(input: &str) -> (Option<Self>, &str) {
        use logos::Logos;

        let mut best_match: Option<(Self, usize)> = None;

        // Try parsing from each position, starting from the beginning
        for start_idx in 0..input.len() {
            let substr = &input[start_idx..];
            let mut lexer = Self::lexer(substr);

            if let Some(Ok(currency)) = lexer.next() {
                let span = lexer.span();
                // Check if the match extends to the end of the substring
                if span.end == substr.len() {
                    // This is a valid match at the end
                    // Keep it if it's longer than our current best match (lower start_idx = longer match)
                    match best_match {
                        None => best_match = Some((currency, start_idx)),
                        Some((_, prev_start)) if start_idx < prev_start => {
                            best_match = Some((currency, start_idx));
                        }
                        _ => {}
                    }
                }
            }
        }

        // Return the longest match found
        match best_match {
            Some((currency, start_idx)) => (Some(currency), &input[..start_idx]),
            None => (None, input),
        }
    }
}

impl Describe for IipCurrency {
    fn description(&self) -> &'static str {
        match self {
            Self::HistCostToMarketValueAdj => {
                "Adjustment to revalue equity from historical cost to market value"
            }
            Self::NotSdrBasket => "Not in special drawing rights basket",
            Self::SdrBasket => "In special drawing rights basket",
            Self::MarketValue => "Market value",
            Self::CurrCost => "Current cost",
            Self::HistCost => "Historical cost",
            Self::NonSpe => "Non-SPEs",
            Self::OthFc => "Other foreign currency",
            Self::Euro => "Euro",
            Self::Spe => "SPEs",
            Self::Usd => "U.S. dollar",
            Self::Yen => "Yen",
            Self::Fc => "Foreign currency",
        }
    }
}
