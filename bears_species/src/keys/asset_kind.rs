use logos::Logos;

/// Asset kind categories derived from the U.S. International Investment Position (IIP) dataset.
///
/// These categories represent the high-level classification of different types of international
/// investment positions, extracted from the first portion of the time series identifiers.
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
pub enum AssetKind {
    /// Direct investment; debt instruments; U.S. affiliates
    #[token("DiInvDebtInstUsAffiliates")]
    DiInvDebtInstUsAffiliates,
    /// Direct investment; debt instruments; U.S. parents
    #[token("DiInvDebtInstUsParents")]
    DiInvDebtInstUsParents,
    /// Direct investment; directional basis adjustment
    #[token("DiInvDirectionalBasisAdj")]
    DiInvDirectionalBasisAdj,
    /// Financial derivatives; exchange-traded
    #[token("FinDerivExchTraded")]
    FinDerivExchTraded,
    /// Financial derivatives; single currency
    #[token("FinDerivSingleCurr")]
    FinDerivSingleCurr,
    /// Financial liabilities excluding financial derivatives
    #[token("FinLiabsExclFinDeriv")]
    FinLiabsExclFinDeriv,
    /// Financial assets excluding financial derivatives
    #[token("FinAssetsExclFinDeriv")]
    FinAssetsExclFinDeriv,
    /// Financial derivatives; reserve assets
    #[token("FinDerivReserve")]
    FinDerivReserve,
    /// Currency and deposits; reserve assets
    #[token("CurrAndDepReserve")]
    CurrAndDepReserve,
    /// U.S. debt assets except reserve assets
    #[token("DebtAssetsExclReserve")]
    DebtAssetsExclReserve,
    /// Treasury bills and certificates
    #[token("TreasBillsAndCerts")]
    TreasBillsAndCerts,
    /// Treasury bonds and notes
    #[token("TreasBondsAndNotes")]
    TreasBondsAndNotes,
    /// Equity and investment fund shares
    #[token("EquityAndInvFundShares")]
    EquityAndInvFundShares,
    /// Net excluding financial derivatives
    #[token("NetExclFinDeriv")]
    NetExclFinDeriv,
    /// Direct investment; debt instruments
    #[token("DiInvDebtInst")]
    DiInvDebtInst,
    /// Long-term debt securities; Treasury
    #[token("LtDebtSecTreas")]
    LtDebtSecTreas,
    /// Short-term debt securities; Treasury
    #[token("StDebtSecTreas")]
    StDebtSecTreas,
    /// Insurance technical reserves
    #[token("InsTechReserves")]
    InsTechReserves,
    /// Trade credit and advances
    #[token("TrdCredAndAdv")]
    TrdCredAndAdv,
    /// Financial derivatives; foreign exchange
    #[token("FinDerivForEx")]
    FinDerivForEx,
    /// Other long-term debt securities
    #[token("OthLtDebtSec")]
    OthLtDebtSec,
    /// Other short-term debt securities
    #[token("OthStDebtSec")]
    OthStDebtSec,
    /// Other claims; reserve assets
    #[token("OthClmReserve")]
    OthClmReserve,
    /// Financial derivatives; over-the-counter
    #[token("FinDerivOtc")]
    FinDerivOtc,
    /// Financial derivatives; other
    #[token("FinDerivOth")]
    FinDerivOth,
    /// Financial derivatives; net
    #[token("FinDerivNet")]
    FinDerivNet,
    /// Direct investment; equity
    #[token("DiInvEquity")]
    DiInvEquity,
    /// Long-term debt securities
    #[token("LtDebtSec")]
    LtDebtSec,
    /// Short-term debt securities
    #[token("StDebtSec")]
    StDebtSec,
    /// Currency and deposits
    #[token("CurrAndDep")]
    CurrAndDep,
    /// Gold; reserve assets
    #[token("GoldReserve")]
    GoldReserve,
    /// IMF; reserve assets
    #[token("ImfReserve")]
    ImfReserve,
    /// Other reserve assets
    #[token("OthReserve")]
    OthReserve,
    /// SDR; reserve assets
    #[token("SdrReserve")]
    SdrReserve,
    /// Securities; reserve assets
    #[token("SecReserve")]
    SecReserve,
    /// Financial derivatives
    #[token("FinDeriv")]
    FinDeriv,
    /// Other equity
    #[token("OthEquity")]
    OthEquity,
    /// SDR allocations
    #[token("SdrAlloc")]
    SdrAlloc,
    /// Debt securities
    #[token("DebtSec")]
    DebtSec,
    /// Direct investment
    #[token("DiInv")]
    DiInv,
    /// Other investment
    #[token("OthInv")]
    OthInv,
    /// Portfolio investment
    #[token("PfInv")]
    PfInv,
    /// Reserve assets
    #[token("Reserve")]
    Reserve,
    /// Loans
    #[token("Loans")]
    Loans,
    /// Debt
    #[token("Debt")]
    Debt,
    /// Financial
    #[token("Fin")]
    Fin,
    #[default]
    /// Net
    #[token("Net")]
    Net,
}

impl AssetKind {
    /// Attempts to lex an AssetKind from the beginning of the input string.
    ///
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(AssetKind) if a valid token was found, None otherwise
    /// - remaining_str is the unconsumed portion of the input string
    pub fn lex(input: &str) -> (Option<Self>, &str) {
        let mut lexer = Self::lexer(input);

        match lexer.next() {
            Some(Ok(asset_kind)) => {
                let span = lexer.span();
                let remainder = &input[span.end..];
                (Some(asset_kind), remainder)
            }
            _ => (None, input),
        }
    }
}
