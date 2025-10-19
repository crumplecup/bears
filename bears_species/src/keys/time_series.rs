use crate::{AssetKind, IipClass, IipCurrency, IipEntity, Position};
use std::str::FromStr;
use strum::IntoEnumIterator;

/// Time series codes for U.S. International Investment Position (IIP) data.
///
/// This enumeration represents various economic indicators related to U.S. international
/// financial assets, liabilities, and investment positions. Each variant corresponds to
/// a specific time series that tracks changes in position, assets, liabilities, and
/// reserve positions across different investment categories and currencies.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum TimeSeriesRaw {
    #[default]
    /// U.S. assets; other investment; currency and deposits (Change in position attributable to changes in volume and valuation n.i.e.)
    CurrAndDepAssetsChgPosNie,
    /// U.S. assets; other investment; currency and deposits (Change in position not attributable to financial-account transactions)
    CurrAndDepAssetsChgPosOth,
    /// U.S. assets; other investment; currency and deposits (Change in position attributable to price changes)
    CurrAndDepAssetsChgPosPrice,
    /// U.S. assets; other investment; currency and deposits (Change in position attributable to financial-account transactions)
    CurrAndDepAssetsChgPosTrans,
    /// U.S. assets; other investment; currency and deposits (Change in position attributable to exchange-rate changes)
    CurrAndDepAssetsChgPosXRate,
    /// U.S. assets; other investment; currency and deposits (Change in position)
    CurrAndDepAssetsChgPos,
    /// U.S. assets; other investment; currency and deposits
    CurrAndDepAssetsPos,
    /// U.S. liabilities; other investment; currency and deposits (Change in position attributable to changes in volume and valuation n.i.e.)
    CurrAndDepLiabsChgPosNie,
    /// U.S. liabilities; other investment; currency and deposits (Change in position not attributable to financial-account transactions)
    CurrAndDepLiabsChgPosOth,
    /// U.S. liabilities; other investment; currency and deposits (Change in position attributable to price changes)
    CurrAndDepLiabsChgPosPrice,
    /// U.S. liabilities; other investment; currency and deposits (Change in position attributable to financial-account transactions)
    CurrAndDepLiabsChgPosTrans,
    /// U.S. liabilities; other investment; currency and deposits (Change in position attributable to exchange-rate changes)
    CurrAndDepLiabsChgPosXRate,
    /// U.S. liabilities; other investment; currency and deposits (Change in position)
    CurrAndDepLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; other investment; currency and deposits
    CurrAndDepLiabsFoaPos,
    /// U.S. liabilities; other investment; currency and deposits
    CurrAndDepLiabsPos,
    /// U.S. assets; other reserve assets; currency and deposits (Change in position attributable to changes in volume and valuation n.i.e.)
    CurrAndDepReserveAssetsChgPosNie,
    /// U.S. assets; other reserve assets; currency and deposits (Change in position not attributable to financial-account transactions)
    CurrAndDepReserveAssetsChgPosOth,
    /// U.S. assets; other reserve assets; currency and deposits (Change in position attributable to price changes)
    CurrAndDepReserveAssetsChgPosPrice,
    /// U.S. assets; other reserve assets; currency and deposits (Change in position attributable to financial-account transactions)
    CurrAndDepReserveAssetsChgPosTrans,
    /// U.S. assets; other reserve assets; currency and deposits (Change in position attributable to exchange-rate changes)
    CurrAndDepReserveAssetsChgPosXRate,
    /// U.S. assets; other reserve assets; currency and deposits (Change in position)
    CurrAndDepReserveAssetsChgPos,
    /// U.S. assets; other reserve assets; currency and deposits
    CurrAndDepReserveAssetsPos,
    /// U.S. debt assets except reserve assets; central bank; euro; long term
    DebtAssetsExclReserveCenBankEuroLtPos,
    /// U.S. debt assets except reserve assets; central bank; euro
    DebtAssetsExclReserveCenBankEuroPos,
    /// U.S. debt assets except reserve assets; central bank; euro; short term
    DebtAssetsExclReserveCenBankEuroStPos,
    /// U.S. debt assets except reserve assets; central bank; foreign currency; long term
    DebtAssetsExclReserveCenBankFcLtPos,
    /// U.S. debt assets except reserve assets; central bank; foreign currency
    DebtAssetsExclReserveCenBankFcPos,
    /// U.S. debt assets except reserve assets; central bank; foreign currency; short term
    DebtAssetsExclReserveCenBankFcStPos,
    /// U.S. debt assets except reserve assets; central bank; long term
    DebtAssetsExclReserveCenBankLtPos,
    /// U.S. debt assets except reserve assets; central bank; other foreign currency; long term
    DebtAssetsExclReserveCenBankOthFcLtPos,
    /// U.S. debt assets except reserve assets; central bank; other foreign currency
    DebtAssetsExclReserveCenBankOthFcPos,
    /// U.S. debt assets except reserve assets; central bank; other foreign currency; short term
    DebtAssetsExclReserveCenBankOthFcStPos,
    /// U.S. debt assets except reserve assets; central bank
    DebtAssetsExclReserveCenBankPos,
    /// U.S. debt assets except reserve assets; central bank; short term
    DebtAssetsExclReserveCenBankStPos,
    /// U.S. debt assets except reserve assets; central bank; U.S. dollar; long term
    DebtAssetsExclReserveCenBankUsdLtPos,
    /// U.S. debt assets except reserve assets; central bank; U.S. dollar
    DebtAssetsExclReserveCenBankUsdPos,
    /// U.S. debt assets except reserve assets; central bank; U.S. dollar; short term
    DebtAssetsExclReserveCenBankUsdStPos,
    /// U.S. debt assets except reserve assets; central bank; yen; long term
    DebtAssetsExclReserveCenBankYenLtPos,
    /// U.S. debt assets except reserve assets; central bank; yen
    DebtAssetsExclReserveCenBankYenPos,
    /// U.S. debt assets except reserve assets; central bank; yen; short term
    DebtAssetsExclReserveCenBankYenStPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; long term
    DebtAssetsExclReserveDepExclCenBankEuroLtPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro
    DebtAssetsExclReserveDepExclCenBankEuroPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; short term
    DebtAssetsExclReserveDepExclCenBankEuroStPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; long term
    DebtAssetsExclReserveDepExclCenBankFcLtPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency
    DebtAssetsExclReserveDepExclCenBankFcPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; short term
    DebtAssetsExclReserveDepExclCenBankFcStPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; long term
    DebtAssetsExclReserveDepExclCenBankLtPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; long term
    DebtAssetsExclReserveDepExclCenBankOthFcLtPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency
    DebtAssetsExclReserveDepExclCenBankOthFcPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; short term
    DebtAssetsExclReserveDepExclCenBankOthFcStPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank
    DebtAssetsExclReserveDepExclCenBankPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; short term
    DebtAssetsExclReserveDepExclCenBankStPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; long term
    DebtAssetsExclReserveDepExclCenBankUsdLtPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar
    DebtAssetsExclReserveDepExclCenBankUsdPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; short term
    DebtAssetsExclReserveDepExclCenBankUsdStPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; long term
    DebtAssetsExclReserveDepExclCenBankYenLtPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen
    DebtAssetsExclReserveDepExclCenBankYenPos,
    /// U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; short term
    DebtAssetsExclReserveDepExclCenBankYenStPos,
    /// U.S. debt assets except reserve assets; euro; long term
    DebtAssetsExclReserveEuroLtPos,
    /// U.S. debt assets except reserve assets; euro
    DebtAssetsExclReserveEuroPos,
    /// U.S. debt assets except reserve assets; euro; short term
    DebtAssetsExclReserveEuroStPos,
    /// U.S. debt assets except reserve assets; foreign currency; long term
    DebtAssetsExclReserveFcLtPos,
    /// U.S. debt assets except reserve assets; foreign currency
    DebtAssetsExclReserveFcPos,
    /// U.S. debt assets except reserve assets; foreign currency; short term
    DebtAssetsExclReserveFcStPos,
    /// U.S. debt assets except reserve assets; general government; euro; long term
    DebtAssetsExclReserveGenGovtEuroLtPos,
    /// U.S. debt assets except reserve assets; general government; euro
    DebtAssetsExclReserveGenGovtEuroPos,
    /// U.S. debt assets except reserve assets; general government; euro; short term
    DebtAssetsExclReserveGenGovtEuroStPos,
    /// U.S. debt assets except reserve assets; general government; foreign currency; long term
    DebtAssetsExclReserveGenGovtFcLtPos,
    /// U.S. debt assets except reserve assets; general government; foreign currency
    DebtAssetsExclReserveGenGovtFcPos,
    /// U.S. debt assets except reserve assets; general government; foreign currency; short term
    DebtAssetsExclReserveGenGovtFcStPos,
    /// U.S. debt assets except reserve assets; general government; long term
    DebtAssetsExclReserveGenGovtLtPos,
    /// U.S. debt assets except reserve assets; general government; other foreign currency; long term
    DebtAssetsExclReserveGenGovtOthFcLtPos,
    /// U.S. debt assets except reserve assets; general government; other foreign currency
    DebtAssetsExclReserveGenGovtOthFcPos,
    /// U.S. debt assets except reserve assets; general government; other foreign currency; short term
    DebtAssetsExclReserveGenGovtOthFcStPos,
    /// U.S. debt assets except reserve assets; general government
    DebtAssetsExclReserveGenGovtPos,
    /// U.S. debt assets except reserve assets; general government; short term
    DebtAssetsExclReserveGenGovtStPos,
    /// U.S. debt assets except reserve assets; general government; U.S. dollar; long term
    DebtAssetsExclReserveGenGovtUsdLtPos,
    /// U.S. debt assets except reserve assets; general government; U.S. dollar
    DebtAssetsExclReserveGenGovtUsdPos,
    /// U.S. debt assets except reserve assets; general government; U.S. dollar; short term
    DebtAssetsExclReserveGenGovtUsdStPos,
    /// U.S. debt assets except reserve assets; general government; yen; long term
    DebtAssetsExclReserveGenGovtYenLtPos,
    /// U.S. debt assets except reserve assets; general government; yen
    DebtAssetsExclReserveGenGovtYenPos,
    /// U.S. debt assets except reserve assets; general government; yen; short term
    DebtAssetsExclReserveGenGovtYenStPos,
    /// U.S. debt assets except reserve assets; long term
    DebtAssetsExclReserveLtPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; long term
    DebtAssetsExclReserveNonFinExclGenGovtEuroLtPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro
    DebtAssetsExclReserveNonFinExclGenGovtEuroPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; short term
    DebtAssetsExclReserveNonFinExclGenGovtEuroStPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; long term
    DebtAssetsExclReserveNonFinExclGenGovtFcLtPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency
    DebtAssetsExclReserveNonFinExclGenGovtFcPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; short term
    DebtAssetsExclReserveNonFinExclGenGovtFcStPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; long term
    DebtAssetsExclReserveNonFinExclGenGovtLtPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; long term
    DebtAssetsExclReserveNonFinExclGenGovtOthFcLtPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency
    DebtAssetsExclReserveNonFinExclGenGovtOthFcPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; short term
    DebtAssetsExclReserveNonFinExclGenGovtOthFcStPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government
    DebtAssetsExclReserveNonFinExclGenGovtPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; short term
    DebtAssetsExclReserveNonFinExclGenGovtStPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; long term
    DebtAssetsExclReserveNonFinExclGenGovtUsdLtPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar
    DebtAssetsExclReserveNonFinExclGenGovtUsdPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; short term
    DebtAssetsExclReserveNonFinExclGenGovtUsdStPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; long term
    DebtAssetsExclReserveNonFinExclGenGovtYenLtPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen
    DebtAssetsExclReserveNonFinExclGenGovtYenPos,
    /// U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; short term
    DebtAssetsExclReserveNonFinExclGenGovtYenStPos,
    /// U.S. debt assets except reserve assets; other foreign currency; long term
    DebtAssetsExclReserveOthFcLtPos,
    /// U.S. debt assets except reserve assets; other foreign currency
    DebtAssetsExclReserveOthFcPos,
    /// U.S. debt assets except reserve assets; other foreign currency; short term
    DebtAssetsExclReserveOthFcStPos,
    /// U.S. debt assets except reserve assets; other financial institutions; euro; long term
    DebtAssetsExclReserveOthFinEuroLtPos,
    /// U.S. debt assets except reserve assets; other financial institutions; euro
    DebtAssetsExclReserveOthFinEuroPos,
    /// U.S. debt assets except reserve assets; other financial institutions; euro; short term
    DebtAssetsExclReserveOthFinEuroStPos,
    /// U.S. debt assets except reserve assets; other financial institutions; foreign currency; long term
    DebtAssetsExclReserveOthFinFcLtPos,
    /// U.S. debt assets except reserve assets; other financial institutions; foreign currency
    DebtAssetsExclReserveOthFinFcPos,
    /// U.S. debt assets except reserve assets; other financial institutions; foreign currency; short term
    DebtAssetsExclReserveOthFinFcStPos,
    /// U.S. debt assets except reserve assets; other financial institutions; long term
    DebtAssetsExclReserveOthFinLtPos,
    /// U.S. debt assets except reserve assets; other financial institutions; other foreign currency; long term
    DebtAssetsExclReserveOthFinOthFcLtPos,
    /// U.S. debt assets except reserve assets; other financial institutions; other foreign currency
    DebtAssetsExclReserveOthFinOthFcPos,
    /// U.S. debt assets except reserve assets; other financial institutions; other foreign currency; short term
    DebtAssetsExclReserveOthFinOthFcStPos,
    /// U.S. debt assets except reserve assets; other financial institutions
    DebtAssetsExclReserveOthFinPos,
    /// U.S. debt assets except reserve assets; other financial institutions; short term
    DebtAssetsExclReserveOthFinStPos,
    /// U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; long term
    DebtAssetsExclReserveOthFinUsdLtPos,
    /// U.S. debt assets except reserve assets; other financial institutions; U.S. dollar
    DebtAssetsExclReserveOthFinUsdPos,
    /// U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; short term
    DebtAssetsExclReserveOthFinUsdStPos,
    /// U.S. debt assets except reserve assets; other financial institutions; yen; long term
    DebtAssetsExclReserveOthFinYenLtPos,
    /// U.S. debt assets except reserve assets; other financial institutions; yen
    DebtAssetsExclReserveOthFinYenPos,
    /// U.S. debt assets except reserve assets; other financial institutions; yen; short term
    DebtAssetsExclReserveOthFinYenStPos,
    /// U.S. debt assets except reserve assets
    DebtAssetsExclReservePos,
    /// U.S. debt assets except reserve assets; short term
    DebtAssetsExclReserveStPos,
    /// U.S. debt assets except reserve assets; U.S. dollar; long term
    DebtAssetsExclReserveUsdLtPos,
    /// U.S. debt assets except reserve assets; U.S. dollar
    DebtAssetsExclReserveUsdPos,
    /// U.S. debt assets except reserve assets; U.S. dollar; short term
    DebtAssetsExclReserveUsdStPos,
    /// U.S. debt assets except reserve assets; yen; long term
    DebtAssetsExclReserveYenLtPos,
    /// U.S. debt assets except reserve assets; yen
    DebtAssetsExclReserveYenPos,
    /// U.S. debt assets except reserve assets; yen; short term
    DebtAssetsExclReserveYenStPos,
    /// U.S. debt liabilities; central bank; euro; long term
    DebtLiabsCenBankEuroLtPos,
    /// U.S. debt liabilities; central bank; euro
    DebtLiabsCenBankEuroPos,
    /// U.S. debt liabilities; central bank; euro; short term
    DebtLiabsCenBankEuroStPos,
    /// U.S. debt liabilities; central bank; foreign currency; long term
    DebtLiabsCenBankFcLtPos,
    /// U.S. debt liabilities; central bank; foreign currency
    DebtLiabsCenBankFcPos,
    /// U.S. debt liabilities; central bank; foreign currency; short term
    DebtLiabsCenBankFcStPos,
    /// U.S. debt liabilities; central bank; long term
    DebtLiabsCenBankLtPos,
    /// U.S. debt liabilities; central bank; other foreign currency; long term
    DebtLiabsCenBankOthFcLtPos,
    /// U.S. debt liabilities; central bank; other foreign currency
    DebtLiabsCenBankOthFcPos,
    /// U.S. debt liabilities; central bank; other foreign currency; short term
    DebtLiabsCenBankOthFcStPos,
    /// U.S. debt liabilities; central bank
    DebtLiabsCenBankPos,
    /// U.S. debt liabilities; central bank; short term
    DebtLiabsCenBankStPos,
    /// U.S. debt liabilities; central bank; U.S. dollar; long term
    DebtLiabsCenBankUsdLtPos,
    /// U.S. debt liabilities; central bank; U.S. dollar
    DebtLiabsCenBankUsdPos,
    /// U.S. debt liabilities; central bank; U.S. dollar; short term
    DebtLiabsCenBankUsdStPos,
    /// U.S. debt liabilities; central bank; yen; long term
    DebtLiabsCenBankYenLtPos,
    /// U.S. debt liabilities; central bank; yen
    DebtLiabsCenBankYenPos,
    /// U.S. debt liabilities; central bank; yen; short term
    DebtLiabsCenBankYenStPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; euro; long term
    DebtLiabsDepExclCenBankEuroLtPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; euro
    DebtLiabsDepExclCenBankEuroPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; euro; short term
    DebtLiabsDepExclCenBankEuroStPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; long term
    DebtLiabsDepExclCenBankFcLtPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency
    DebtLiabsDepExclCenBankFcPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; short term
    DebtLiabsDepExclCenBankFcStPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; long term
    DebtLiabsDepExclCenBankLtPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; long term
    DebtLiabsDepExclCenBankOthFcLtPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency
    DebtLiabsDepExclCenBankOthFcPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; short term
    DebtLiabsDepExclCenBankOthFcStPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank
    DebtLiabsDepExclCenBankPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; short term
    DebtLiabsDepExclCenBankStPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; long term
    DebtLiabsDepExclCenBankUsdLtPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar
    DebtLiabsDepExclCenBankUsdPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; short term
    DebtLiabsDepExclCenBankUsdStPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; yen; long term
    DebtLiabsDepExclCenBankYenLtPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; yen
    DebtLiabsDepExclCenBankYenPos,
    /// U.S. debt liabilities; deposit-taking institutions except central bank; yen; short term
    DebtLiabsDepExclCenBankYenStPos,
    /// U.S. debt liabilities; euro; long term
    DebtLiabsEuroLtPos,
    /// U.S. debt liabilities; euro
    DebtLiabsEuroPos,
    /// U.S. debt liabilities; euro; short term
    DebtLiabsEuroStPos,
    /// U.S. debt liabilities; foreign currency; long term
    DebtLiabsFcLtPos,
    /// U.S. debt liabilities; foreign currency
    DebtLiabsFcPos,
    /// U.S. debt liabilities; foreign currency; short term
    DebtLiabsFcStPos,
    /// U.S. debt liabilities; general government; euro; long term
    DebtLiabsGenGovtEuroLtPos,
    /// U.S. debt liabilities; general government; euro
    DebtLiabsGenGovtEuroPos,
    /// U.S. debt liabilities; general government; euro; short term
    DebtLiabsGenGovtEuroStPos,
    /// U.S. debt liabilities; general government; foreign currency; long term
    DebtLiabsGenGovtFcLtPos,
    /// U.S. debt liabilities; general government; foreign currency
    DebtLiabsGenGovtFcPos,
    /// U.S. debt liabilities; general government; foreign currency; short term
    DebtLiabsGenGovtFcStPos,
    /// U.S. debt liabilities; general government; long term
    DebtLiabsGenGovtLtPos,
    /// U.S. debt liabilities; general government; other foreign currency; long term
    DebtLiabsGenGovtOthFcLtPos,
    /// U.S. debt liabilities; general government; other foreign currency
    DebtLiabsGenGovtOthFcPos,
    /// U.S. debt liabilities; general government; other foreign currency; short term
    DebtLiabsGenGovtOthFcStPos,
    /// U.S. debt liabilities; general government
    DebtLiabsGenGovtPos,
    /// U.S. debt liabilities; general government; short term
    DebtLiabsGenGovtStPos,
    /// U.S. debt liabilities; general government; U.S. dollar; long term
    DebtLiabsGenGovtUsdLtPos,
    /// U.S. debt liabilities; general government; U.S. dollar
    DebtLiabsGenGovtUsdPos,
    /// U.S. debt liabilities; general government; U.S. dollar; short term
    DebtLiabsGenGovtUsdStPos,
    /// U.S. debt liabilities; general government; yen; long term
    DebtLiabsGenGovtYenLtPos,
    /// U.S. debt liabilities; general government; yen
    DebtLiabsGenGovtYenPos,
    /// U.S. debt liabilities; general government; yen; short term
    DebtLiabsGenGovtYenStPos,
    /// U.S. debt liabilities; long term
    DebtLiabsLtPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; euro; long term
    DebtLiabsNonFinExclGenGovtEuroLtPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; euro
    DebtLiabsNonFinExclGenGovtEuroPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; euro; short term
    DebtLiabsNonFinExclGenGovtEuroStPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; long term
    DebtLiabsNonFinExclGenGovtFcLtPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; foreign currency
    DebtLiabsNonFinExclGenGovtFcPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; short term
    DebtLiabsNonFinExclGenGovtFcStPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; long term
    DebtLiabsNonFinExclGenGovtLtPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; long term
    DebtLiabsNonFinExclGenGovtOthFcLtPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency
    DebtLiabsNonFinExclGenGovtOthFcPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; short term
    DebtLiabsNonFinExclGenGovtOthFcStPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government
    DebtLiabsNonFinExclGenGovtPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; short term
    DebtLiabsNonFinExclGenGovtStPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; long term
    DebtLiabsNonFinExclGenGovtUsdLtPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar
    DebtLiabsNonFinExclGenGovtUsdPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; short term
    DebtLiabsNonFinExclGenGovtUsdStPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; yen; long term
    DebtLiabsNonFinExclGenGovtYenLtPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; yen
    DebtLiabsNonFinExclGenGovtYenPos,
    /// U.S. debt liabilities; nonfinancial institutions except general government; yen; short term
    DebtLiabsNonFinExclGenGovtYenStPos,
    /// U.S. debt liabilities; other foreign currency; long term
    DebtLiabsOthFcLtPos,
    /// U.S. debt liabilities; other foreign currency
    DebtLiabsOthFcPos,
    /// U.S. debt liabilities; other foreign currency; short term
    DebtLiabsOthFcStPos,
    /// U.S. debt liabilities; other financial institutions; euro; long term
    DebtLiabsOthFinEuroLtPos,
    /// U.S. debt liabilities; other financial institutions; euro
    DebtLiabsOthFinEuroPos,
    /// U.S. debt liabilities; other financial institutions; euro; short term
    DebtLiabsOthFinEuroStPos,
    /// U.S. debt liabilities; other financial institutions; foreign currency; long term
    DebtLiabsOthFinFcLtPos,
    /// U.S. debt liabilities; other financial institutions; foreign currency
    DebtLiabsOthFinFcPos,
    /// U.S. debt liabilities; other financial institutions; foreign currency; short term
    DebtLiabsOthFinFcStPos,
    /// U.S. debt liabilities; other financial institutions; long term
    DebtLiabsOthFinLtPos,
    /// U.S. debt liabilities; other financial institutions; other foreign currency; long term
    DebtLiabsOthFinOthFcLtPos,
    /// U.S. debt liabilities; other financial institutions; other foreign currency
    DebtLiabsOthFinOthFcPos,
    /// U.S. debt liabilities; other financial institutions; other foreign currency; short term
    DebtLiabsOthFinOthFcStPos,
    /// U.S. debt liabilities; other financial institutions
    DebtLiabsOthFinPos,
    /// U.S. debt liabilities; other financial institutions; short term
    DebtLiabsOthFinStPos,
    /// U.S. debt liabilities; other financial institutions; U.S. dollar; long term
    DebtLiabsOthFinUsdLtPos,
    /// U.S. debt liabilities; other financial institutions; U.S. dollar
    DebtLiabsOthFinUsdPos,
    /// U.S. debt liabilities; other financial institutions; U.S. dollar; short term
    DebtLiabsOthFinUsdStPos,
    /// U.S. debt liabilities; other financial institutions; yen; long term
    DebtLiabsOthFinYenLtPos,
    /// U.S. debt liabilities; other financial institutions; yen
    DebtLiabsOthFinYenPos,
    /// U.S. debt liabilities; other financial institutions; yen; short term
    DebtLiabsOthFinYenStPos,
    /// U.S. debt liabilities
    DebtLiabsPos,
    /// U.S. debt liabilities; short term
    DebtLiabsStPos,
    /// U.S. debt liabilities; U.S. dollar; long term
    DebtLiabsUsdLtPos,
    /// U.S. debt liabilities; U.S. dollar
    DebtLiabsUsdPos,
    /// U.S. debt liabilities; U.S. dollar; short term
    DebtLiabsUsdStPos,
    /// U.S. debt liabilities; yen; long term
    DebtLiabsYenLtPos,
    /// U.S. debt liabilities; yen
    DebtLiabsYenPos,
    /// U.S. debt liabilities; yen; short term
    DebtLiabsYenStPos,
    /// U.S. assets; portfolio investment; debt securities (Change in position attributable to changes in volume and valuation n.i.e.)
    DebtSecAssetsChgPosNie,
    /// U.S. assets; portfolio investment; debt securities (Change in position not attributable to financial-account transactions)
    DebtSecAssetsChgPosOth,
    /// U.S. assets; portfolio investment; debt securities (Change in position attributable to price changes)
    DebtSecAssetsChgPosPrice,
    /// U.S. assets; portfolio investment; debt securities (Change in position attributable to financial-account transactions)
    DebtSecAssetsChgPosTrans,
    /// U.S. assets; portfolio investment; debt securities (Change in position attributable to exchange-rate changes)
    DebtSecAssetsChgPosXRate,
    /// U.S. assets; portfolio investment; debt securities (Change in position)
    DebtSecAssetsChgPos,
    /// U.S. assets; portfolio investment; debt securities
    DebtSecAssetsPos,
    /// U.S. liabilities; portfolio investment; debt securities (Change in position attributable to changes in volume and valuation n.i.e.)
    DebtSecLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment; debt securities (Change in position not attributable to financial-account transactions)
    DebtSecLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment; debt securities (Change in position attributable to price changes)
    DebtSecLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment; debt securities (Change in position attributable to financial-account transactions)
    DebtSecLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment; debt securities (Change in position attributable to exchange-rate changes)
    DebtSecLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment; debt securities (Change in position)
    DebtSecLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment; debt securities
    DebtSecLiabsFoaPos,
    /// U.S. liabilities; portfolio investment; debt securities
    DebtSecLiabsPos,
    /// U.S. assets; direct investment at market value, asset/liability basis (Change in position attributable to changes in volume and valuation n.i.e.)
    DiInvAssetsChgPosNie,
    /// U.S. assets; direct investment at market value, asset/liability basis (Change in position not attributable to financial-account transactions)
    DiInvAssetsChgPosOth,
    /// U.S. assets; direct investment at market value, asset/liability basis (Change in position attributable to price changes)
    DiInvAssetsChgPosPrice,
    /// U.S. assets; direct investment at market value, asset/liability basis (Change in position attributable to financial-account transactions)
    DiInvAssetsChgPosTrans,
    /// U.S. assets; direct investment at market value, asset/liability basis (Change in position attributable to exchange-rate changes)
    DiInvAssetsChgPosXRate,
    /// U.S. assets; direct investment at market value, asset/liability basis (Change in position)
    DiInvAssetsChgPos,
    /// U.S. assets; direct investment at current cost, asset/liability basis
    DiInvAssetsCurrCostPos,
    /// U.S. assets; direct investment; adjustment to revalue equity from historical cost to market value
    DiInvAssetsHistCostToMarketValueAdjPos,
    /// U.S. assets; direct investment at market value, asset/liability basis; Non-SPEs
    DiInvAssetsNonSpePos,
    /// U.S. assets; direct investment at market value, asset/liability basis
    DiInvAssetsPos,
    /// U.S. assets; direct investment at market value, asset/liability basis; SPEs
    DiInvAssetsSpePos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position attributable to changes in volume and valuation n.i.e.)
    DiInvDebtInstAssetsChgPosNie,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position not attributable to financial-account transactions)
    DiInvDebtInstAssetsChgPosOth,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position attributable to price changes)
    DiInvDebtInstAssetsChgPosPrice,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position attributable to financial-account transactions)
    DiInvDebtInstAssetsChgPosTrans,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position attributable to exchange-rate changes)
    DiInvDebtInstAssetsChgPosXRate,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position)
    DiInvDebtInstAssetsChgPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; euro; long term
    DiInvDebtInstAssetsEuroLtPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; euro
    DiInvDebtInstAssetsEuroPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; euro; short term
    DiInvDebtInstAssetsEuroStPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; long term
    DiInvDebtInstAssetsFcLtPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency
    DiInvDebtInstAssetsFcPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; short term
    DiInvDebtInstAssetsFcStPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; long term
    DiInvDebtInstAssetsLtPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; Non-SPEs
    DiInvDebtInstAssetsNonSpePos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; long term
    DiInvDebtInstAssetsOthFcLtPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency
    DiInvDebtInstAssetsOthFcPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; short term
    DiInvDebtInstAssetsOthFcStPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments
    DiInvDebtInstAssetsPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; SPEs
    DiInvDebtInstAssetsSpePos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; short term
    DiInvDebtInstAssetsStPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; long term
    DiInvDebtInstAssetsUsdLtPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar
    DiInvDebtInstAssetsUsdPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; short term
    DiInvDebtInstAssetsUsdStPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; yen; long term
    DiInvDebtInstAssetsYenLtPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; yen
    DiInvDebtInstAssetsYenPos,
    /// U.S. assets; direct investment, asset/liability basis; debt instruments; yen; short term
    DiInvDebtInstAssetsYenStPos,
    /// Inward direct investment (foreign direct investment in the United States), directional basis; debt instruments
    DiInvDebtInstInwardPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position attributable to changes in volume and valuation n.i.e.)
    DiInvDebtInstLiabsChgPosNie,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position not attributable to financial-account transactions)
    DiInvDebtInstLiabsChgPosOth,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position attributable to price changes)
    DiInvDebtInstLiabsChgPosPrice,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position attributable to financial-account transactions)
    DiInvDebtInstLiabsChgPosTrans,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position attributable to exchange-rate changes)
    DiInvDebtInstLiabsChgPosXRate,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position)
    DiInvDebtInstLiabsChgPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; long term
    DiInvDebtInstLiabsEuroLtPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro
    DiInvDebtInstLiabsEuroPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; short term
    DiInvDebtInstLiabsEuroStPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; long term
    DiInvDebtInstLiabsFcLtPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency
    DiInvDebtInstLiabsFcPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; short term
    DiInvDebtInstLiabsFcStPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; long term
    DiInvDebtInstLiabsLtPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; Non-SPEs
    DiInvDebtInstLiabsNonSpePos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; long term
    DiInvDebtInstLiabsOthFcLtPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency
    DiInvDebtInstLiabsOthFcPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; short term
    DiInvDebtInstLiabsOthFcStPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments
    DiInvDebtInstLiabsPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; SPEs
    DiInvDebtInstLiabsSpePos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; short term
    DiInvDebtInstLiabsStPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; long term
    DiInvDebtInstLiabsUsdLtPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar
    DiInvDebtInstLiabsUsdPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; short term
    DiInvDebtInstLiabsUsdStPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; long term
    DiInvDebtInstLiabsYenLtPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen
    DiInvDebtInstLiabsYenPos,
    /// U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; short term
    DiInvDebtInstLiabsYenStPos,
    /// Outward direct investment (U.S. direct investment abroad), directional basis; debt instruments
    DiInvDebtInstOutwardPos,
    /// U.S. non-SPE affiliates' debt asset position in their foreign parent groups
    DiInvDebtInstUsAffiliatesClaimsByNonSpePos,
    /// U.S. SPE affiliates' debt asset position in their foreign parent groups
    DiInvDebtInstUsAffiliatesClaimsBySpePos,
    /// Direct investment; debt instruments; U.S. affiliates' claims; Non-SPEs
    DiInvDebtInstUsAffiliatesClaimsNonSpePos,
    /// Direct investment; debt instruments; U.S. affiliates' claims
    DiInvDebtInstUsAffiliatesClaimsPos,
    /// Direct investment; debt instruments; U.S. affiliates' claims; SPEs
    DiInvDebtInstUsAffiliatesClaimsSpePos,
    /// Direct investment; debt instruments; U.S. affiliates' liabilites; Non-SPEs
    DiInvDebtInstUsAffiliatesLiabsNonSpePos,
    /// Direct investment; debt instruments; U.S. affiliates' liabilites
    DiInvDebtInstUsAffiliatesLiabsPos,
    /// Direct investment; debt instruments; U.S. affiliates' liabilites; SPEs
    DiInvDebtInstUsAffiliatesLiabsSpePos,
    /// Direct investment; debt instruments; U.S. parents' claims; Non-SPEs
    DiInvDebtInstUsParentsClaimsNonSpePos,
    /// Direct investment; debt instruments; U.S. parents' claims
    DiInvDebtInstUsParentsClaimsPos,
    /// Direct investment; debt instruments; U.S. parents' claims; SPEs
    DiInvDebtInstUsParentsClaimsSpePos,
    /// U.S. parents' debt liability position in their foreign non-SPE affiliates
    DiInvDebtInstUsParentsLiabsInNonSpePos,
    /// U.S. parents' debt liability position in their foreign SPE affiliates
    DiInvDebtInstUsParentsLiabsInSpePos,
    /// Direct investment; debt instruments; U.S. parents' liabilites; Non-SPEs
    DiInvDebtInstUsParentsLiabsNonSpePos,
    /// Direct investment; debt instruments; U.S. parents' liabilites
    DiInvDebtInstUsParentsLiabsPos,
    /// Direct investment; debt instruments; U.S. parents' liabilites; SPEs
    DiInvDebtInstUsParentsLiabsSpePos,
    /// Direct investment; adjustments to convert to directional basis
    DiInvDirectionalBasisAdjPos,
    /// U.S. assets; direct investment at market value; equity (Change in position attributable to changes in volume and valuation n.i.e.)
    DiInvEquityAssetsChgPosNie,
    /// U.S. assets; direct investment at market value; equity (Change in position not attributable to financial-account transactions)
    DiInvEquityAssetsChgPosOth,
    /// U.S. assets; direct investment at market value; equity (Change in position attributable to price changes)
    DiInvEquityAssetsChgPosPrice,
    /// U.S. assets; direct investment at market value; equity (Change in position attributable to financial-account transactions)
    DiInvEquityAssetsChgPosTrans,
    /// U.S. assets; direct investment at market value; equity (Change in position attributable to exchange-rate changes)
    DiInvEquityAssetsChgPosXRate,
    /// U.S. assets; direct investment at market value; equity (Change in position)
    DiInvEquityAssetsChgPos,
    /// U.S. assets; direct investment at current cost; equity
    DiInvEquityAssetsCurrCostPos,
    /// U.S. assets; direct investment at historical cost; equity
    DiInvEquityAssetsHistCostPos,
    /// U.S. assets; direct investment at market value; equity; Non-SPEs
    DiInvEquityAssetsNonSpePos,
    /// U.S. assets; direct investment at market value; equity
    DiInvEquityAssetsPos,
    /// U.S. assets; direct investment at market value; equity; SPEs
    DiInvEquityAssetsSpePos,
    /// U.S. liabilities; direct investment at market value; equity
    DiInvEquityInwardPos,
    /// U.S. liabilities; direct investment at market value; equity (Change in position attributable to changes in volume and valuation n.i.e.)
    DiInvEquityLiabsChgPosNie,
    /// U.S. liabilities; direct investment at market value; equity (Change in position not attributable to financial-account transactions)
    DiInvEquityLiabsChgPosOth,
    /// U.S. liabilities; direct investment at market value; equity (Change in position attributable to price changes)
    DiInvEquityLiabsChgPosPrice,
    /// U.S. liabilities; direct investment at market value; equity (Change in position attributable to financial-account transactions)
    DiInvEquityLiabsChgPosTrans,
    /// U.S. liabilities; direct investment at market value; equity (Change in position attributable to exchange-rate changes)
    DiInvEquityLiabsChgPosXRate,
    /// U.S. liabilities; direct investment at market value; equity (Change in position)
    DiInvEquityLiabsChgPos,
    /// U.S. liabilities; direct investment at current cost; equity
    DiInvEquityLiabsCurrCostPos,
    /// U.S. liabilities; direct investment at historical cost; equity
    DiInvEquityLiabsHistCostPos,
    /// U.S. liabilities; direct investment at market value; equity; Non-SPEs
    DiInvEquityLiabsNonSpePos,
    /// U.S. liabilities; direct investment at market value; equity
    DiInvEquityLiabsPos,
    /// U.S. liabilities; direct investment at market value; equity; SPEs
    DiInvEquityLiabsSpePos,
    /// Inward direct investment (foreign direct investment in the United States) at current cost, directional basis
    DiInvInwardCurrCostPos,
    /// Inward direct investment (foreign direct investment in the United States) at historical cost, directional basis
    DiInvInwardHistCostPos,
    /// Inward direct investment (foreign direct investment in the United States) at market value, directional basis
    DiInvInwardMarketValuePos,
    /// U.S. liabilities; direct investment at market value, asset/liability basis (Change in position attributable to changes in volume and valuation n.i.e.)
    DiInvLiabsChgPosNie,
    /// U.S. liabilities; direct investment at market value, asset/liability basis (Change in position not attributable to financial-account transactions)
    DiInvLiabsChgPosOth,
    /// U.S. liabilities; direct investment at market value, asset/liability basis (Change in position attributable to price changes)
    DiInvLiabsChgPosPrice,
    /// U.S. liabilities; direct investment at market value, asset/liability basis (Change in position attributable to financial-account transactions)
    DiInvLiabsChgPosTrans,
    /// U.S. liabilities; direct investment at market value, asset/liability basis (Change in position attributable to exchange-rate changes)
    DiInvLiabsChgPosXRate,
    /// U.S. liabilities; direct investment at market value, asset/liability basis (Change in position)
    DiInvLiabsChgPos,
    /// U.S. liabilities; direct investment at current cost, asset/liability basis
    DiInvLiabsCurrCostPos,
    /// U.S. liabilities; direct investment; adjustment to revalue equity from historical cost to market value
    DiInvLiabsHistCostToMarketValueAdjPos,
    /// U.S. liabilities; direct investment at market value, asset/liability basis; Non-SPEs
    DiInvLiabsNonSpePos,
    /// U.S. liabilities; direct investment at market value, asset/liability basis
    DiInvLiabsPos,
    /// U.S. liabilities; direct investment at market value, asset/liability basis; SPEs
    DiInvLiabsSpePos,
    /// Outward direct investment (U.S. direct investment abroad) at current cost, directional basis
    DiInvOutwardCurrCostPos,
    /// Outward direct investment (U.S. direct investment abroad) at historical cost, directional basis
    DiInvOutwardHistCostPos,
    /// Outward direct investment (U.S. direct investment abroad) at market value, directional basis
    DiInvOutwardMarketValuePos,
    /// U.S. assets; portfolio investment; equity and investment fund shares (Change in position attributable to changes in volume and valuation n.i.e.)
    EquityAndInvFundSharesAssetsChgPosNie,
    /// U.S. assets; portfolio investment; equity and investment fund shares (Change in position not attributable to financial-account transactions)
    EquityAndInvFundSharesAssetsChgPosOth,
    /// U.S. assets; portfolio investment; equity and investment fund shares (Change in position attributable to price changes)
    EquityAndInvFundSharesAssetsChgPosPrice,
    /// U.S. assets; portfolio investment; equity and investment fund shares (Change in position attributable to financial-account transactions)
    EquityAndInvFundSharesAssetsChgPosTrans,
    /// U.S. assets; portfolio investment; equity and investment fund shares (Change in position attributable to exchange-rate changes)
    EquityAndInvFundSharesAssetsChgPosXRate,
    /// U.S. assets; portfolio investment; equity and investment fund shares (Change in position)
    EquityAndInvFundSharesAssetsChgPos,
    /// U.S. assets; portfolio investment; equity and investment fund shares
    EquityAndInvFundSharesAssetsPos,
    /// U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position attributable to changes in volume and valuation n.i.e.)
    EquityAndInvFundSharesLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position not attributable to financial-account transactions)
    EquityAndInvFundSharesLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position attributable to price changes)
    EquityAndInvFundSharesLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position attributable to financial-account transactions)
    EquityAndInvFundSharesLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position attributable to exchange-rate changes)
    EquityAndInvFundSharesLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position)
    EquityAndInvFundSharesLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment; equity and investment fund shares
    EquityAndInvFundSharesLiabsFoaPos,
    /// U.S. liabilities; portfolio investment; equity and investment fund shares
    EquityAndInvFundSharesLiabsPos,
    /// U.S. assets (Change in position attributable to changes in volume and valuation n.i.e.)
    FinAssetsChgPosNie,
    /// U.S. assets (Change in position not attributable to financial-account transactions)
    FinAssetsChgPosOth,
    /// U.S. assets (Change in position attributable to price changes)
    FinAssetsChgPosPrice,
    /// U.S. assets (Change in position attributable to financial-account transactions)
    FinAssetsChgPosTrans,
    /// U.S. assets (Change in position attributable to exchange-rate changes)
    FinAssetsChgPosXRate,
    /// U.S. assets (Change in position)
    FinAssetsChgPos,
    /// U.S. assets excluding financial derivatives (Change in position attributable to changes in volume and valuation n.i.e.)
    FinAssetsExclFinDerivChgPosNie,
    /// U.S. assets excluding financial derivatives (Change in position not attributable to financial-account transactions)
    FinAssetsExclFinDerivChgPosOth,
    /// U.S. assets excluding financial derivatives (Change in position attributable to price changes)
    FinAssetsExclFinDerivChgPosPrice,
    /// U.S. assets excluding financial derivatives (Change in position attributable to financial-account transactions)
    FinAssetsExclFinDerivChgPosTrans,
    /// U.S. assets excluding financial derivatives (Change in position attributable to exchange-rate changes)
    FinAssetsExclFinDerivChgPosXRate,
    /// U.S. assets excluding financial derivatives (Change in position)
    FinAssetsExclFinDerivChgPos,
    /// U.S. assets excluding financial derivatives
    FinAssetsExclFinDerivPos,
    /// U.S. assets
    FinAssetsPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivAssetsChgPosNie,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position not attributable to financial-account transactions)
    FinDerivAssetsChgPosOth,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position attributable to price changes)
    FinDerivAssetsChgPosPrice,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position attributable to financial-account transactions)
    FinDerivAssetsChgPosTrans,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position attributable to exchange-rate changes)
    FinDerivAssetsChgPosXRate,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position)
    FinDerivAssetsChgPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value
    FinDerivAssetsPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivExchTradedAssetsChgPosNie,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position not attributable to financial-account transactions)
    FinDerivExchTradedAssetsChgPosOth,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position attributable to price changes)
    FinDerivExchTradedAssetsChgPosPrice,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position attributable to financial-account transactions)
    FinDerivExchTradedAssetsChgPosTrans,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position attributable to exchange-rate changes)
    FinDerivExchTradedAssetsChgPosXRate,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position)
    FinDerivExchTradedAssetsChgPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts
    FinDerivExchTradedAssetsPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivExchTradedLiabsChgPosNie,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position not attributable to financial-account transactions)
    FinDerivExchTradedLiabsChgPosOth,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position attributable to price changes)
    FinDerivExchTradedLiabsChgPosPrice,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position attributable to financial-account transactions)
    FinDerivExchTradedLiabsChgPosTrans,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position attributable to exchange-rate changes)
    FinDerivExchTradedLiabsChgPosXRate,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position)
    FinDerivExchTradedLiabsChgPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts
    FinDerivExchTradedLiabsPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivForExAssetsChgPosNie,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position not attributable to financial-account transactions)
    FinDerivForExAssetsChgPosOth,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to price changes)
    FinDerivForExAssetsChgPosPrice,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to financial-account transactions)
    FinDerivForExAssetsChgPosTrans,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to exchange-rate changes)
    FinDerivForExAssetsChgPosXRate,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position)
    FinDerivForExAssetsChgPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts
    FinDerivForExAssetsPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivForExLiabsChgPosNie,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position not attributable to financial-account transactions)
    FinDerivForExLiabsChgPosOth,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to price changes)
    FinDerivForExLiabsChgPosPrice,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to financial-account transactions)
    FinDerivForExLiabsChgPosTrans,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to exchange-rate changes)
    FinDerivForExLiabsChgPosXRate,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position)
    FinDerivForExLiabsChgPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts
    FinDerivForExLiabsPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivLiabsChgPosNie,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position not attributable to financial-account transactions)
    FinDerivLiabsChgPosOth,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position attributable to price changes)
    FinDerivLiabsChgPosPrice,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position attributable to financial-account transactions)
    FinDerivLiabsChgPosTrans,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position attributable to exchange-rate changes)
    FinDerivLiabsChgPosXRate,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position)
    FinDerivLiabsChgPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value
    FinDerivLiabsPos,
    /// U.S. net international investment position; financial derivatives other than reserves (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivNetChgPosNie,
    /// U.S. net international investment position; financial derivatives other than reserves (Change in position not attributable to financial-account transactions)
    FinDerivNetChgPosOth,
    /// U.S. net international investment position; financial derivatives other than reserves (Change in position attributable to price changes)
    FinDerivNetChgPosPrice,
    /// U.S. net international investment position; financial derivatives other than reserves (Change in position attributable to financial-account transactions)
    FinDerivNetChgPosTrans,
    /// U.S. net international investment position; financial derivatives other than reserves (Change in position attributable to exchange-rate changes)
    FinDerivNetChgPosXRate,
    /// U.S. net international investment position; financial derivatives other than reserves (Change in position)
    FinDerivNetChgPos,
    /// U.S. net international investment position; financial derivatives other than reserves
    FinDerivNetPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivOtcAssetsChgPosNie,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position not attributable to financial-account transactions)
    FinDerivOtcAssetsChgPosOth,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position attributable to price changes)
    FinDerivOtcAssetsChgPosPrice,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position attributable to financial-account transactions)
    FinDerivOtcAssetsChgPosTrans,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position attributable to exchange-rate changes)
    FinDerivOtcAssetsChgPosXRate,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position)
    FinDerivOtcAssetsChgPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts
    FinDerivOtcAssetsPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivOtcLiabsChgPosNie,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position not attributable to financial-account transactions)
    FinDerivOtcLiabsChgPosOth,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position attributable to price changes)
    FinDerivOtcLiabsChgPosPrice,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position attributable to financial-account transactions)
    FinDerivOtcLiabsChgPosTrans,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position attributable to exchange-rate changes)
    FinDerivOtcLiabsChgPosXRate,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position)
    FinDerivOtcLiabsChgPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts
    FinDerivOtcLiabsPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivOthAssetsChgPosNie,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position not attributable to financial-account transactions)
    FinDerivOthAssetsChgPosOth,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position attributable to price changes)
    FinDerivOthAssetsChgPosPrice,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position attributable to financial-account transactions)
    FinDerivOthAssetsChgPosTrans,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position attributable to exchange-rate changes)
    FinDerivOthAssetsChgPosXRate,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position)
    FinDerivOthAssetsChgPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts
    FinDerivOthAssetsPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivOthLiabsChgPosNie,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position not attributable to financial-account transactions)
    FinDerivOthLiabsChgPosOth,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position attributable to price changes)
    FinDerivOthLiabsChgPosPrice,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position attributable to financial-account transactions)
    FinDerivOthLiabsChgPosTrans,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position attributable to exchange-rate changes)
    FinDerivOthLiabsChgPosXRate,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position)
    FinDerivOthLiabsChgPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts
    FinDerivOthLiabsPos,
    /// U.S. assets; other reserve assets; financial derivatives (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivReserveAssetsChgPosNie,
    /// U.S. assets; other reserve assets; financial derivatives (Change in position not attributable to financial-account transactions)
    FinDerivReserveAssetsChgPosOth,
    /// U.S. assets; other reserve assets; financial derivatives (Change in position attributable to price changes)
    FinDerivReserveAssetsChgPosPrice,
    /// U.S. assets; other reserve assets; financial derivatives (Change in position attributable to financial-account transactions)
    FinDerivReserveAssetsChgPosTrans,
    /// U.S. assets; other reserve assets; financial derivatives (Change in position attributable to exchange-rate changes)
    FinDerivReserveAssetsChgPosXRate,
    /// U.S. assets; other reserve assets; financial derivatives (Change in position)
    FinDerivReserveAssetsChgPos,
    /// U.S. assets; other reserve assets; financial derivatives
    FinDerivReserveAssetsPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivSingleCurrAssetsChgPosNie,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position not attributable to financial-account transactions)
    FinDerivSingleCurrAssetsChgPosOth,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to price changes)
    FinDerivSingleCurrAssetsChgPosPrice,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to financial-account transactions)
    FinDerivSingleCurrAssetsChgPosTrans,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to exchange-rate changes)
    FinDerivSingleCurrAssetsChgPosXRate,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position)
    FinDerivSingleCurrAssetsChgPos,
    /// U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts
    FinDerivSingleCurrAssetsPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to changes in volume and valuation n.i.e.)
    FinDerivSingleCurrLiabsChgPosNie,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position not attributable to financial-account transactions)
    FinDerivSingleCurrLiabsChgPosOth,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to price changes)
    FinDerivSingleCurrLiabsChgPosPrice,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to financial-account transactions)
    FinDerivSingleCurrLiabsChgPosTrans,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to exchange-rate changes)
    FinDerivSingleCurrLiabsChgPosXRate,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position)
    FinDerivSingleCurrLiabsChgPos,
    /// U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts
    FinDerivSingleCurrLiabsPos,
    /// U.S. liabilities (Change in position attributable to changes in volume and valuation n.i.e.)
    FinLiabsChgPosNie,
    /// U.S. liabilities (Change in position not attributable to financial-account transactions)
    FinLiabsChgPosOth,
    /// U.S. liabilities (Change in position attributable to price changes)
    FinLiabsChgPosPrice,
    /// U.S. liabilities (Change in position attributable to financial-account transactions)
    FinLiabsChgPosTrans,
    /// U.S. liabilities (Change in position attributable to exchange-rate changes)
    FinLiabsChgPosXRate,
    /// U.S. liabilities (Change in position)
    FinLiabsChgPos,
    /// U.S. liabilities excluding financial derivatives (Change in position attributable to changes in volume and valuation n.i.e.)
    FinLiabsExclFinDerivChgPosNie,
    /// U.S. liabilities excluding financial derivatives (Change in position not attributable to financial-account transactions)
    FinLiabsExclFinDerivChgPosOth,
    /// U.S. liabilities excluding financial derivatives (Change in position attributable to price changes)
    FinLiabsExclFinDerivChgPosPrice,
    /// U.S. liabilities excluding financial derivatives (Change in position attributable to financial-account transactions)
    FinLiabsExclFinDerivChgPosTrans,
    /// U.S. liabilities excluding financial derivatives (Change in position attributable to exchange-rate changes)
    FinLiabsExclFinDerivChgPosXRate,
    /// U.S. liabilities excluding financial derivatives (Change in position)
    FinLiabsExclFinDerivChgPos,
    /// U.S. liabilities excluding financial derivatives
    FinLiabsExclFinDerivPos,
    /// U.S. liabilities to foreign official agencies
    FinLiabsFoaPos,
    /// U.S. liabilities
    FinLiabsPos,
    /// U.S. assets; reserve assets; monetary gold (Change in position attributable to changes in volume and valuation n.i.e.)
    GoldReserveAssetsChgPosNie,
    /// U.S. assets; reserve assets; monetary gold (Change in position not attributable to financial-account transactions)
    GoldReserveAssetsChgPosOth,
    /// U.S. assets; reserve assets; monetary gold (Change in position attributable to price changes)
    GoldReserveAssetsChgPosPrice,
    /// U.S. assets; reserve assets; monetary gold (Change in position attributable to financial-account transactions)
    GoldReserveAssetsChgPosTrans,
    /// U.S. assets; reserve assets; monetary gold (Change in position attributable to exchange-rate changes)
    GoldReserveAssetsChgPosXRate,
    /// U.S. assets; reserve assets; monetary gold (Change in position)
    GoldReserveAssetsChgPos,
    /// U.S. assets; reserve assets; monetary gold
    GoldReserveAssetsPos,
    /// U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position attributable to changes in volume and valuation n.i.e.)
    ImfReserveAssetsChgPosNie,
    /// U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position not attributable to financial-account transactions)
    ImfReserveAssetsChgPosOth,
    /// U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position attributable to price changes)
    ImfReserveAssetsChgPosPrice,
    /// U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position attributable to financial-account transactions)
    ImfReserveAssetsChgPosTrans,
    /// U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position attributable to exchange-rate changes)
    ImfReserveAssetsChgPosXRate,
    /// U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position)
    ImfReserveAssetsChgPos,
    /// U.S. assets; reserve assets; reserve position in the International Monetary Fund
    ImfReserveAssetsPos,
    /// U.S. assets; other investment; insurance technical reserves (Change in position attributable to changes in volume and valuation n.i.e.)
    InsTechReservesAssetsChgPosNie,
    /// U.S. assets; other investment; insurance technical reserves (Change in position not attributable to financial-account transactions)
    InsTechReservesAssetsChgPosOth,
    /// U.S. assets; other investment; insurance technical reserves (Change in position attributable to price changes)
    InsTechReservesAssetsChgPosPrice,
    /// U.S. assets; other investment; insurance technical reserves (Change in position attributable to financial-account transactions)
    InsTechReservesAssetsChgPosTrans,
    /// U.S. assets; other investment; insurance technical reserves (Change in position attributable to exchange-rate changes)
    InsTechReservesAssetsChgPosXRate,
    /// U.S. assets; other investment; insurance technical reserves (Change in position)
    InsTechReservesAssetsChgPos,
    /// U.S. assets; other investment; insurance technical reserves
    InsTechReservesAssetsPos,
    /// U.S. liabilities; other investment; insurance technical reserves (Change in position attributable to changes in volume and valuation n.i.e.)
    InsTechReservesLiabsChgPosNie,
    /// U.S. liabilities; other investment; insurance technical reserves (Change in position not attributable to financial-account transactions)
    InsTechReservesLiabsChgPosOth,
    /// U.S. liabilities; other investment; insurance technical reserves (Change in position attributable to price changes)
    InsTechReservesLiabsChgPosPrice,
    /// U.S. liabilities; other investment; insurance technical reserves (Change in position attributable to financial-account transactions)
    InsTechReservesLiabsChgPosTrans,
    /// U.S. liabilities; other investment; insurance technical reserves (Change in position attributable to exchange-rate changes)
    InsTechReservesLiabsChgPosXRate,
    /// U.S. liabilities; other investment; insurance technical reserves (Change in position)
    InsTechReservesLiabsChgPos,
    /// U.S. liabilities; other investment; insurance technical reserves
    InsTechReservesLiabsPos,
    /// U.S. assets; other investment; loans (Change in position attributable to changes in volume and valuation n.i.e.)
    LoansAssetsChgPosNie,
    /// U.S. assets; other investment; loans (Change in position not attributable to financial-account transactions)
    LoansAssetsChgPosOth,
    /// U.S. assets; other investment; loans (Change in position attributable to price changes)
    LoansAssetsChgPosPrice,
    /// U.S. assets; other investment; loans (Change in position attributable to financial-account transactions)
    LoansAssetsChgPosTrans,
    /// U.S. assets; other investment; loans (Change in position attributable to exchange-rate changes)
    LoansAssetsChgPosXRate,
    /// U.S. assets; other investment; loans (Change in position)
    LoansAssetsChgPos,
    /// U.S. assets; other investment; loans
    LoansAssetsPos,
    /// U.S. liabilities; other investment; loans (Change in position attributable to changes in volume and valuation n.i.e.)
    LoansLiabsChgPosNie,
    /// U.S. liabilities; other investment; loans (Change in position not attributable to financial-account transactions)
    LoansLiabsChgPosOth,
    /// U.S. liabilities; other investment; loans (Change in position attributable to price changes)
    LoansLiabsChgPosPrice,
    /// U.S. liabilities; other investment; loans (Change in position attributable to financial-account transactions)
    LoansLiabsChgPosTrans,
    /// U.S. liabilities; other investment; loans (Change in position attributable to exchange-rate changes)
    LoansLiabsChgPosXRate,
    /// U.S. liabilities; other investment; loans (Change in position)
    LoansLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; other investment; loans
    LoansLiabsFoaPos,
    /// U.S. liabilities; other investment; loans
    LoansLiabsPos,
    /// U.S. assets; portfolio investment; long-term debt securities (Change in position attributable to changes in volume and valuation n.i.e.)
    LtDebtSecAssetsChgPosNie,
    /// U.S. assets; portfolio investment; long-term debt securities (Change in position not attributable to financial-account transactions)
    LtDebtSecAssetsChgPosOth,
    /// U.S. assets; portfolio investment; long-term debt securities (Change in position attributable to price changes)
    LtDebtSecAssetsChgPosPrice,
    /// U.S. assets; portfolio investment; long-term debt securities (Change in position attributable to financial-account transactions)
    LtDebtSecAssetsChgPosTrans,
    /// U.S. assets; portfolio investment; long-term debt securities (Change in position attributable to exchange-rate changes)
    LtDebtSecAssetsChgPosXRate,
    /// U.S. assets; portfolio investment; long-term debt securities (Change in position)
    LtDebtSecAssetsChgPos,
    /// U.S. assets; portfolio investment; long-term debt securities
    LtDebtSecAssetsPos,
    /// U.S. liabilities; portfolio investment; long-term debt securities (Change in position attributable to changes in volume and valuation n.i.e.)
    LtDebtSecLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment; long-term debt securities (Change in position not attributable to financial-account transactions)
    LtDebtSecLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment; long-term debt securities (Change in position attributable to price changes)
    LtDebtSecLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment; long-term debt securities (Change in position attributable to financial-account transactions)
    LtDebtSecLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment; long-term debt securities (Change in position attributable to exchange-rate changes)
    LtDebtSecLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment; long-term debt securities (Change in position)
    LtDebtSecLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities
    LtDebtSecLiabsFoaPos,
    /// U.S. liabilities; portfolio investment; long-term debt securities
    LtDebtSecLiabsPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities; Treasury bonds and notes
    LtDebtSecTreasLiabsFoaPos,
    /// U.S. net international investment position (Change in position attributable to changes in volume and valuation n.i.e.)
    NetChgPosNie,
    /// U.S. net international investment position (Change in position not attributable to financial-account transactions)
    NetChgPosOth,
    /// U.S. net international investment position (Change in position attributable to price changes)
    NetChgPosPrice,
    /// U.S. net international investment position (Change in position attributable to financial-account transactions)
    NetChgPosTrans,
    /// U.S. net international investment position (Change in position attributable to exchange-rate changes)
    NetChgPosXRate,
    /// U.S. net international investment position (Change in position)
    NetChgPos,
    /// U.S. net international investment position excluding financial derivatives (Change in position attributable to changes in volume and valuation n.i.e.)
    NetExclFinDerivChgPosNie,
    /// U.S. net international investment position excluding financial derivatives (Change in position not attributable to financial-account transactions)
    NetExclFinDerivChgPosOth,
    /// U.S. net international investment position excluding financial derivatives (Change in position attributable to price changes)
    NetExclFinDerivChgPosPrice,
    /// U.S. net international investment position excluding financial derivatives (Change in position attributable to financial-account transactions)
    NetExclFinDerivChgPosTrans,
    /// U.S. net international investment position excluding financial derivatives (Change in position attributable to exchange-rate changes)
    NetExclFinDerivChgPosXRate,
    /// U.S. net international investment position excluding financial derivatives (Change in position)
    NetExclFinDerivChgPos,
    /// U.S. net international investment position excluding financial derivatives
    NetExclFinDerivPos,
    /// U.S. net international investment position
    NetPos,
    /// U.S. assets; other reserve assets; other claims (Change in position attributable to changes in volume and valuation n.i.e.)
    OthClmReserveAssetsChgPosNie,
    /// U.S. assets; other reserve assets; other claims (Change in position not attributable to financial-account transactions)
    OthClmReserveAssetsChgPosOth,
    /// U.S. assets; other reserve assets; other claims (Change in position attributable to price changes)
    OthClmReserveAssetsChgPosPrice,
    /// U.S. assets; other reserve assets; other claims (Change in position attributable to financial-account transactions)
    OthClmReserveAssetsChgPosTrans,
    /// U.S. assets; other reserve assets; other claims (Change in position attributable to exchange-rate changes)
    OthClmReserveAssetsChgPosXRate,
    /// U.S. assets; other reserve assets; other claims (Change in position)
    OthClmReserveAssetsChgPos,
    /// U.S. assets; other reserve assets; other claims
    OthClmReserveAssetsPos,
    /// U.S. assets; other investment; other equity (Change in position attributable to changes in volume and valuation n.i.e.)
    OthEquityAssetsChgPosNie,
    /// U.S. assets; other investment; other equity (Change in position not attributable to financial-account transactions)
    OthEquityAssetsChgPosOth,
    /// U.S. assets; other investment; other equity (Change in position attributable to price changes)
    OthEquityAssetsChgPosPrice,
    /// U.S. assets; other investment; other equity (Change in position attributable to financial-account transactions)
    OthEquityAssetsChgPosTrans,
    /// U.S. assets; other investment; other equity (Change in position attributable to exchange-rate changes)
    OthEquityAssetsChgPosXRate,
    /// U.S. assets; other investment; other equity (Change in position)
    OthEquityAssetsChgPos,
    /// U.S. assets; other investment; other equity
    OthEquityAssetsPos,
    /// U.S. liabilities; other investment; other equity (Change in position attributable to changes in volume and valuation n.i.e.)
    OthEquityLiabsChgPosNie,
    /// U.S. liabilities; other investment; other equity (Change in position not attributable to financial-account transactions)
    OthEquityLiabsChgPosOth,
    /// U.S. liabilities; other investment; other equity (Change in position attributable to price changes)
    OthEquityLiabsChgPosPrice,
    /// U.S. liabilities; other investment; other equity (Change in position attributable to financial-account transactions)
    OthEquityLiabsChgPosTrans,
    /// U.S. liabilities; other investment; other equity (Change in position attributable to exchange-rate changes)
    OthEquityLiabsChgPosXRate,
    /// U.S. liabilities; other investment; other equity (Change in position)
    OthEquityLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; other investment; other equity
    OthEquityLiabsFoaPos,
    /// U.S. liabilities; other investment; other equity
    OthEquityLiabsPos,
    /// U.S. assets; other investment (Change in position attributable to changes in volume and valuation n.i.e.)
    OthInvAssetsChgPosNie,
    /// U.S. assets; other investment (Change in position not attributable to financial-account transactions)
    OthInvAssetsChgPosOth,
    /// U.S. assets; other investment (Change in position attributable to price changes)
    OthInvAssetsChgPosPrice,
    /// U.S. assets; other investment (Change in position attributable to financial-account transactions)
    OthInvAssetsChgPosTrans,
    /// U.S. assets; other investment (Change in position attributable to exchange-rate changes)
    OthInvAssetsChgPosXRate,
    /// U.S. assets; other investment (Change in position)
    OthInvAssetsChgPos,
    /// U.S. assets; other investment
    OthInvAssetsPos,
    /// U.S. liabilities; other investment (Change in position attributable to changes in volume and valuation n.i.e.)
    OthInvLiabsChgPosNie,
    /// U.S. liabilities; other investment (Change in position not attributable to financial-account transactions)
    OthInvLiabsChgPosOth,
    /// U.S. liabilities; other investment (Change in position attributable to price changes)
    OthInvLiabsChgPosPrice,
    /// U.S. liabilities; other investment (Change in position attributable to financial-account transactions)
    OthInvLiabsChgPosTrans,
    /// U.S. liabilities; other investment (Change in position attributable to exchange-rate changes)
    OthInvLiabsChgPosXRate,
    /// U.S. liabilities; other investment (Change in position)
    OthInvLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; other investment
    OthInvLiabsFoaPos,
    /// U.S. liabilities; other investment
    OthInvLiabsPos,
    /// U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position attributable to changes in volume and valuation n.i.e.)
    OthLtDebtSecLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position not attributable to financial-account transactions)
    OthLtDebtSecLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position attributable to price changes)
    OthLtDebtSecLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position attributable to financial-account transactions)
    OthLtDebtSecLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position attributable to exchange-rate changes)
    OthLtDebtSecLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position)
    OthLtDebtSecLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities excluding Treasury bonds and notes
    OthLtDebtSecLiabsFoaPos,
    /// U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes
    OthLtDebtSecLiabsPos,
    /// U.S. assets; other reserve assets (Change in position attributable to changes in volume and valuation n.i.e.)
    OthReserveAssetsChgPosNie,
    /// U.S. assets; other reserve assets (Change in position not attributable to financial-account transactions)
    OthReserveAssetsChgPosOth,
    /// U.S. assets; other reserve assets (Change in position attributable to price changes)
    OthReserveAssetsChgPosPrice,
    /// U.S. assets; other reserve assets (Change in position attributable to financial-account transactions)
    OthReserveAssetsChgPosTrans,
    /// U.S. assets; other reserve assets (Change in position attributable to exchange-rate changes)
    OthReserveAssetsChgPosXRate,
    /// U.S. assets; other reserve assets (Change in position)
    OthReserveAssetsChgPos,
    /// U.S. assets; other reserve assets
    OthReserveAssetsPos,
    /// U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position attributable to changes in volume and valuation n.i.e.)
    OthStDebtSecLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position not attributable to financial-account transactions)
    OthStDebtSecLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position attributable to price changes)
    OthStDebtSecLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position attributable to financial-account transactions)
    OthStDebtSecLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position attributable to exchange-rate changes)
    OthStDebtSecLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position)
    OthStDebtSecLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities excluding Treasury bills and certificates
    OthStDebtSecLiabsFoaPos,
    /// U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates
    OthStDebtSecLiabsPos,
    /// U.S. assets; portfolio investment (Change in position attributable to changes in volume and valuation n.i.e.)
    PfInvAssetsChgPosNie,
    /// U.S. assets; portfolio investment (Change in position not attributable to financial-account transactions)
    PfInvAssetsChgPosOth,
    /// U.S. assets; portfolio investment (Change in position attributable to price changes)
    PfInvAssetsChgPosPrice,
    /// U.S. assets; portfolio investment (Change in position attributable to financial-account transactions)
    PfInvAssetsChgPosTrans,
    /// U.S. assets; portfolio investment (Change in position attributable to exchange-rate changes)
    PfInvAssetsChgPosXRate,
    /// U.S. assets; portfolio investment (Change in position)
    PfInvAssetsChgPos,
    /// U.S. assets; portfolio investment
    PfInvAssetsPos,
    /// U.S. liabilities; portfolio investment (Change in position attributable to changes in volume and valuation n.i.e.)
    PfInvLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment (Change in position not attributable to financial-account transactions)
    PfInvLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment (Change in position attributable to price changes)
    PfInvLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment (Change in position attributable to financial-account transactions)
    PfInvLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment (Change in position attributable to exchange-rate changes)
    PfInvLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment (Change in position)
    PfInvLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment
    PfInvLiabsFoaPos,
    /// U.S. liabilities; portfolio investment
    PfInvLiabsPos,
    /// U.S. assets; reserve assets; central bank; long term
    ReserveAssetsCenBankLtPos,
    /// U.S. assets; reserve assets; central bank; not in special drawing rights basket; long term
    ReserveAssetsCenBankNotSdrBasketLtPos,
    /// U.S. assets; reserve assets; central bank; not in special drawing rights basket
    ReserveAssetsCenBankNotSdrBasketPos,
    /// U.S. assets; reserve assets; central bank; not in special drawing rights basket; short term
    ReserveAssetsCenBankNotSdrBasketStPos,
    /// U.S. assets; reserve assets; central bank
    ReserveAssetsCenBankPos,
    /// U.S. assets; reserve assets; central bank; in special drawing rights basket; long term
    ReserveAssetsCenBankSdrBasketLtPos,
    /// U.S. assets; reserve assets; central bank; in special drawing rights basket
    ReserveAssetsCenBankSdrBasketPos,
    /// U.S. assets; reserve assets; central bank; in special drawing rights basket; short term
    ReserveAssetsCenBankSdrBasketStPos,
    /// U.S. assets; reserve assets; central bank; short term
    ReserveAssetsCenBankStPos,
    /// U.S. assets; reserve assets (Change in position attributable to changes in volume and valuation n.i.e.)
    ReserveAssetsChgPosNie,
    /// U.S. assets; reserve assets (Change in position not attributable to financial-account transactions)
    ReserveAssetsChgPosOth,
    /// U.S. assets; reserve assets (Change in position attributable to price changes)
    ReserveAssetsChgPosPrice,
    /// U.S. assets; reserve assets (Change in position attributable to financial-account transactions)
    ReserveAssetsChgPosTrans,
    /// U.S. assets; reserve assets (Change in position attributable to exchange-rate changes)
    ReserveAssetsChgPosXRate,
    /// U.S. assets; reserve assets (Change in position)
    ReserveAssetsChgPos,
    /// U.S. assets; reserve assets; general government; long term
    ReserveAssetsGenGovtLtPos,
    /// U.S. assets; reserve assets; general government; not in special drawing rights basket; long term
    ReserveAssetsGenGovtNotSdrBasketLtPos,
    /// U.S. assets; reserve assets; general government; not in special drawing rights basket
    ReserveAssetsGenGovtNotSdrBasketPos,
    /// U.S. assets; reserve assets; general government; not in special drawing rights basket; short term
    ReserveAssetsGenGovtNotSdrBasketStPos,
    /// U.S. assets; reserve assets; general government
    ReserveAssetsGenGovtPos,
    /// U.S. assets; reserve assets; general government; in special drawing rights basket; long term
    ReserveAssetsGenGovtSdrBasketLtPos,
    /// U.S. assets; reserve assets; general government; in special drawing rights basket
    ReserveAssetsGenGovtSdrBasketPos,
    /// U.S. assets; reserve assets; general government; in special drawing rights basket; short term
    ReserveAssetsGenGovtSdrBasketStPos,
    /// U.S. assets; reserve assets; general government; short term
    ReserveAssetsGenGovtStPos,
    /// U.S. assets; reserve assets; long term
    ReserveAssetsLtPos,
    /// U.S. assets; reserve assets; not in special drawing rights basket; long term
    ReserveAssetsNotSdrBasketLtPos,
    /// U.S. assets; reserve assets; not in special drawing rights basket
    ReserveAssetsNotSdrBasketPos,
    /// U.S. assets; reserve assets; not in special drawing rights basket; short term
    ReserveAssetsNotSdrBasketStPos,
    /// U.S. assets; reserve assets
    ReserveAssetsPos,
    /// U.S. assets; reserve assets; in special drawing rights basket; long term
    ReserveAssetsSdrBasketLtPos,
    /// U.S. assets; reserve assets; in special drawing rights basket
    ReserveAssetsSdrBasketPos,
    /// U.S. assets; reserve assets; in special drawing rights basket; short term
    ReserveAssetsSdrBasketStPos,
    /// U.S. assets; reserve assets; short term
    ReserveAssetsStPos,
    /// U.S. liabilities; other investment; special drawing rights allocations (Change in position attributable to changes in volume and valuation n.i.e.)
    SdrAllocLiabsChgPosNie,
    /// U.S. liabilities; other investment; special drawing rights allocations (Change in position not attributable to financial-account transactions)
    SdrAllocLiabsChgPosOth,
    /// U.S. liabilities; other investment; special drawing rights allocations (Change in position attributable to price changes)
    SdrAllocLiabsChgPosPrice,
    /// U.S. liabilities; other investment; special drawing rights allocations (Change in position attributable to financial-account transactions)
    SdrAllocLiabsChgPosTrans,
    /// U.S. liabilities; other investment; special drawing rights allocations (Change in position attributable to exchange-rate changes)
    SdrAllocLiabsChgPosXRate,
    /// U.S. liabilities; other investment; special drawing rights allocations (Change in position)
    SdrAllocLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; other investment; special drawing rights allocations
    SdrAllocLiabsFoaPos,
    /// U.S. liabilities; other investment; special drawing rights allocations
    SdrAllocLiabsPos,
    /// U.S. assets; reserve assets; special drawing rights (Change in position attributable to changes in volume and valuation n.i.e.)
    SdrReserveAssetsChgPosNie,
    /// U.S. assets; reserve assets; special drawing rights (Change in position not attributable to financial-account transactions)
    SdrReserveAssetsChgPosOth,
    /// U.S. assets; reserve assets; special drawing rights (Change in position attributable to price changes)
    SdrReserveAssetsChgPosPrice,
    /// U.S. assets; reserve assets; special drawing rights (Change in position attributable to financial-account transactions)
    SdrReserveAssetsChgPosTrans,
    /// U.S. assets; reserve assets; special drawing rights (Change in position attributable to exchange-rate changes)
    SdrReserveAssetsChgPosXRate,
    /// U.S. assets; reserve assets; special drawing rights (Change in position)
    SdrReserveAssetsChgPos,
    /// U.S. assets; reserve assets; special drawing rights
    SdrReserveAssetsPos,
    /// U.S. assets; other reserve assets; securities (Change in position attributable to changes in volume and valuation n.i.e.)
    SecReserveAssetsChgPosNie,
    /// U.S. assets; other reserve assets; securities (Change in position not attributable to financial-account transactions)
    SecReserveAssetsChgPosOth,
    /// U.S. assets; other reserve assets; securities (Change in position attributable to price changes)
    SecReserveAssetsChgPosPrice,
    /// U.S. assets; other reserve assets; securities (Change in position attributable to financial-account transactions)
    SecReserveAssetsChgPosTrans,
    /// U.S. assets; other reserve assets; securities (Change in position attributable to exchange-rate changes)
    SecReserveAssetsChgPosXRate,
    /// U.S. assets; other reserve assets; securities (Change in position)
    SecReserveAssetsChgPos,
    /// U.S. assets; other reserve assets; securities
    SecReserveAssetsPos,
    /// U.S. assets; portfolio investment; short-term debt securities (Change in position attributable to changes in volume and valuation n.i.e.)
    StDebtSecAssetsChgPosNie,
    /// U.S. assets; portfolio investment; short-term debt securities (Change in position not attributable to financial-account transactions)
    StDebtSecAssetsChgPosOth,
    /// U.S. assets; portfolio investment; short-term debt securities (Change in position attributable to price changes)
    StDebtSecAssetsChgPosPrice,
    /// U.S. assets; portfolio investment; short-term debt securities (Change in position attributable to financial-account transactions)
    StDebtSecAssetsChgPosTrans,
    /// U.S. assets; portfolio investment; short-term debt securities (Change in position attributable to exchange-rate changes)
    StDebtSecAssetsChgPosXRate,
    /// U.S. assets; portfolio investment; short-term debt securities (Change in position)
    StDebtSecAssetsChgPos,
    /// U.S. assets; portfolio investment; short-term debt securities
    StDebtSecAssetsPos,
    /// U.S. liabilities; portfolio investment; short-term debt securities (Change in position attributable to changes in volume and valuation n.i.e.)
    StDebtSecLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment; short-term debt securities (Change in position not attributable to financial-account transactions)
    StDebtSecLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment; short-term debt securities (Change in position attributable to price changes)
    StDebtSecLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment; short-term debt securities (Change in position attributable to financial-account transactions)
    StDebtSecLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment; short-term debt securities (Change in position attributable to exchange-rate changes)
    StDebtSecLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment; short-term debt securities (Change in position)
    StDebtSecLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities
    StDebtSecLiabsFoaPos,
    /// U.S. liabilities; portfolio investment; short-term debt securities
    StDebtSecLiabsPos,
    /// U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities; Treasury bills and certificates
    StDebtSecTreasLiabsFoaPos,
    /// U.S. assets; other investment; trade credit and advances (Change in position attributable to changes in volume and valuation n.i.e.)
    TrdCredAndAdvAssetsChgPosNie,
    /// U.S. assets; other investment; trade credit and advances (Change in position not attributable to financial-account transactions)
    TrdCredAndAdvAssetsChgPosOth,
    /// U.S. assets; other investment; trade credit and advances (Change in position attributable to price changes)
    TrdCredAndAdvAssetsChgPosPrice,
    /// U.S. assets; other investment; trade credit and advances (Change in position attributable to financial-account transactions)
    TrdCredAndAdvAssetsChgPosTrans,
    /// U.S. assets; other investment; trade credit and advances (Change in position attributable to exchange-rate changes)
    TrdCredAndAdvAssetsChgPosXRate,
    /// U.S. assets; other investment; trade credit and advances (Change in position)
    TrdCredAndAdvAssetsChgPos,
    /// U.S. assets; other investment; trade credit and advances
    TrdCredAndAdvAssetsPos,
    /// U.S. liabilities; other investment; trade credit and advances (Change in position attributable to changes in volume and valuation n.i.e.)
    TrdCredAndAdvLiabsChgPosNie,
    /// U.S. liabilities; other investment; trade credit and advances (Change in position not attributable to financial-account transactions)
    TrdCredAndAdvLiabsChgPosOth,
    /// U.S. liabilities; other investment; trade credit and advances (Change in position attributable to price changes)
    TrdCredAndAdvLiabsChgPosPrice,
    /// U.S. liabilities; other investment; trade credit and advances (Change in position attributable to financial-account transactions)
    TrdCredAndAdvLiabsChgPosTrans,
    /// U.S. liabilities; other investment; trade credit and advances (Change in position attributable to exchange-rate changes)
    TrdCredAndAdvLiabsChgPosXRate,
    /// U.S. liabilities; other investment; trade credit and advances (Change in position)
    TrdCredAndAdvLiabsChgPos,
    /// U.S. liabilities to foreign official agencies; other investment; trade credit and advances
    TrdCredAndAdvLiabsFoaPos,
    /// U.S. liabilities; other investment; trade credit and advances
    TrdCredAndAdvLiabsPos,
    /// U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position attributable to changes in volume and valuation n.i.e.)
    TreasBillsAndCertsLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position not attributable to financial-account transactions)
    TreasBillsAndCertsLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position attributable to price changes)
    TreasBillsAndCertsLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position attributable to financial-account transactions)
    TreasBillsAndCertsLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position attributable to exchange-rate changes)
    TreasBillsAndCertsLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position)
    TreasBillsAndCertsLiabsChgPos,
    /// U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates
    TreasBillsAndCertsLiabsPos,
    /// U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position attributable to changes in volume and valuation n.i.e.)
    TreasBondsAndNotesLiabsChgPosNie,
    /// U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position not attributable to financial-account transactions)
    TreasBondsAndNotesLiabsChgPosOth,
    /// U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position attributable to price changes)
    TreasBondsAndNotesLiabsChgPosPrice,
    /// U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position attributable to financial-account transactions)
    TreasBondsAndNotesLiabsChgPosTrans,
    /// U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position attributable to exchange-rate changes)
    TreasBondsAndNotesLiabsChgPosXRate,
    /// U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position)
    TreasBondsAndNotesLiabsChgPos,
    /// U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes
    TreasBondsAndNotesLiabsPos,
}

impl TimeSeriesRaw {
    /// Returns all variant names as a sorted set of strings.
    pub fn variants() -> std::collections::BTreeSet<String> {
        use strum::IntoEnumIterator;
        
        Self::iter()
            .map(|variant| variant.to_string())
            .collect()
    }

    /// Returns all variant names with Position suffix removed, as a sorted set of strings.
    pub fn variants_without_position() -> std::collections::BTreeSet<String> {
        Self::iter()
            .map(|variant| {
                let variant_str = variant.to_string();
                let (_position, remainder) = Position::lex_from_right(&variant_str);
                remainder.to_string()
            })
            .collect()
    }

    /// Returns all variant names with IipCurrency and Position suffixes removed, as a sorted set of strings.
    pub fn variants_without_currency_and_position() -> std::collections::BTreeSet<String> {
        Self::iter()
            .map(|variant| {
                let variant_str = variant.to_string();
                
                // Remove Position from the end
                let (_position, remainder) = Position::lex_from_right(&variant_str);
                
                // Remove IipCurrency from the end of what's left
                let (_currency, remainder) = IipCurrency::lex_from_right(remainder);
                
                remainder.to_string()
            })
            .collect()
    }

    /// Returns all variant names with IipEntity, IipCurrency and Position suffixes removed, as a sorted set of strings.
    pub fn variants_without_entity_currency_and_position() -> std::collections::BTreeSet<String> {
        use strum::IntoEnumIterator;
        
        Self::iter()
            .map(|variant| {
                let variant_str = variant.to_string();
                
                // Remove Position from the end
                let (_position, remainder) = Position::lex_from_right(&variant_str);
                
                // Remove IipCurrency from the end of what's left
                let (_currency, remainder) = IipCurrency::lex_from_right(remainder);
                
                // Remove IipEntity from the end of what's left
                let (_entity, remainder) = IipEntity::lex_from_right(remainder);
                
                remainder.to_string()
            })
            .collect()
    }

    /// Returns all variant names with IipClass, IipEntity, IipCurrency and Position suffixes removed, as a sorted set of strings.
    pub fn variants_without_class_entity_currency_and_position() -> std::collections::BTreeSet<String> {
        use strum::IntoEnumIterator;
        
        Self::iter()
            .map(|variant| {
                let variant_str = variant.to_string();
                
                // Remove Position from the end
                let (_position, remainder) = Position::lex_from_right(&variant_str);
                
                // Remove IipCurrency from the end of what's left
                let (_currency, remainder) = IipCurrency::lex_from_right(remainder);
                
                // Remove IipEntity from the end of what's left
                let (_entity, remainder) = IipEntity::lex_from_right(remainder);
                
                // Remove IipClass from the end of what's left
                let (_class, remainder) = IipClass::lex_from_right(remainder);
                
                remainder.to_string()
            })
            .collect()
    }

    /// Returns the description for this time series code.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::CurrAndDepAssetsChgPosNie => "U.S. assets; other investment; currency and deposits (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::CurrAndDepAssetsChgPosOth => "U.S. assets; other investment; currency and deposits (Change in position not attributable to financial-account transactions); annual",
            Self::CurrAndDepAssetsChgPosPrice => "U.S. assets; other investment; currency and deposits (Change in position attributable to price changes); annual",
            Self::CurrAndDepAssetsChgPosTrans => "U.S. assets; other investment; currency and deposits (Change in position attributable to financial-account transactions); annual",
            Self::CurrAndDepAssetsChgPosXRate => "U.S. assets; other investment; currency and deposits (Change in position attributable to exchange-rate changes); annual",
            Self::CurrAndDepAssetsChgPos => "U.S. assets; other investment; currency and deposits (Change in position); annual",
            Self::CurrAndDepAssetsPos => "U.S. assets; other investment; currency and deposits; annual",
            Self::CurrAndDepLiabsChgPosNie => "U.S. liabilities; other investment; currency and deposits (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::CurrAndDepLiabsChgPosOth => "U.S. liabilities; other investment; currency and deposits (Change in position not attributable to financial-account transactions); annual",
            Self::CurrAndDepLiabsChgPosPrice => "U.S. liabilities; other investment; currency and deposits (Change in position attributable to price changes); annual",
            Self::CurrAndDepLiabsChgPosTrans => "U.S. liabilities; other investment; currency and deposits (Change in position attributable to financial-account transactions); annual",
            Self::CurrAndDepLiabsChgPosXRate => "U.S. liabilities; other investment; currency and deposits (Change in position attributable to exchange-rate changes); annual",
            Self::CurrAndDepLiabsChgPos => "U.S. liabilities; other investment; currency and deposits (Change in position); annual",
            Self::CurrAndDepLiabsFoaPos => "U.S. liabilities to foreign official agencies; other investment; currency and deposits; annual",
            Self::CurrAndDepLiabsPos => "U.S. liabilities; other investment; currency and deposits; annual",
            Self::CurrAndDepReserveAssetsChgPosNie => "U.S. assets; other reserve assets; currency and deposits (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::CurrAndDepReserveAssetsChgPosOth => "U.S. assets; other reserve assets; currency and deposits (Change in position not attributable to financial-account transactions); annual",
            Self::CurrAndDepReserveAssetsChgPosPrice => "U.S. assets; other reserve assets; currency and deposits (Change in position attributable to price changes); annual",
            Self::CurrAndDepReserveAssetsChgPosTrans => "U.S. assets; other reserve assets; currency and deposits (Change in position attributable to financial-account transactions); annual",
            Self::CurrAndDepReserveAssetsChgPosXRate => "U.S. assets; other reserve assets; currency and deposits (Change in position attributable to exchange-rate changes); annual",
            Self::CurrAndDepReserveAssetsChgPos => "U.S. assets; other reserve assets; currency and deposits (Change in position); annual",
            Self::CurrAndDepReserveAssetsPos => "U.S. assets; other reserve assets; currency and deposits; annual",
            Self::DebtAssetsExclReserveCenBankEuroLtPos => "U.S. debt assets except reserve assets; central bank; euro; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankEuroPos => "U.S. debt assets except reserve assets; central bank; euro; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankEuroStPos => "U.S. debt assets except reserve assets; central bank; euro; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankFcLtPos => "U.S. debt assets except reserve assets; central bank; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankFcPos => "U.S. debt assets except reserve assets; central bank; foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankFcStPos => "U.S. debt assets except reserve assets; central bank; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankLtPos => "U.S. debt assets except reserve assets; central bank; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankOthFcLtPos => "U.S. debt assets except reserve assets; central bank; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankOthFcPos => "U.S. debt assets except reserve assets; central bank; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankOthFcStPos => "U.S. debt assets except reserve assets; central bank; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankPos => "U.S. debt assets except reserve assets; central bank; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankStPos => "U.S. debt assets except reserve assets; central bank; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankUsdLtPos => "U.S. debt assets except reserve assets; central bank; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankUsdPos => "U.S. debt assets except reserve assets; central bank; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankUsdStPos => "U.S. debt assets except reserve assets; central bank; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankYenLtPos => "U.S. debt assets except reserve assets; central bank; yen; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankYenPos => "U.S. debt assets except reserve assets; central bank; yen; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveCenBankYenStPos => "U.S. debt assets except reserve assets; central bank; yen; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankEuroLtPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankEuroPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankEuroStPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; euro; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankFcLtPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankFcPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankFcStPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankLtPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankOthFcLtPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankOthFcPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankOthFcStPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankStPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankUsdLtPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankUsdPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankUsdStPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankYenLtPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankYenPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveDepExclCenBankYenStPos => "U.S. debt assets except reserve assets; deposit-taking institutions except central bank; yen; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveEuroLtPos => "U.S. debt assets except reserve assets; euro; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveEuroPos => "U.S. debt assets except reserve assets; euro; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveEuroStPos => "U.S. debt assets except reserve assets; euro; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveFcLtPos => "U.S. debt assets except reserve assets; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveFcPos => "U.S. debt assets except reserve assets; foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveFcStPos => "U.S. debt assets except reserve assets; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtEuroLtPos => "U.S. debt assets except reserve assets; general government; euro; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtEuroPos => "U.S. debt assets except reserve assets; general government; euro; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtEuroStPos => "U.S. debt assets except reserve assets; general government; euro; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtFcLtPos => "U.S. debt assets except reserve assets; general government; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtFcPos => "U.S. debt assets except reserve assets; general government; foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtFcStPos => "U.S. debt assets except reserve assets; general government; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtLtPos => "U.S. debt assets except reserve assets; general government; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtOthFcLtPos => "U.S. debt assets except reserve assets; general government; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtOthFcPos => "U.S. debt assets except reserve assets; general government; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtOthFcStPos => "U.S. debt assets except reserve assets; general government; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtPos => "U.S. debt assets except reserve assets; general government; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtStPos => "U.S. debt assets except reserve assets; general government; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtUsdLtPos => "U.S. debt assets except reserve assets; general government; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtUsdPos => "U.S. debt assets except reserve assets; general government; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtUsdStPos => "U.S. debt assets except reserve assets; general government; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtYenLtPos => "U.S. debt assets except reserve assets; general government; yen; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtYenPos => "U.S. debt assets except reserve assets; general government; yen; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveGenGovtYenStPos => "U.S. debt assets except reserve assets; general government; yen; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveLtPos => "U.S. debt assets except reserve assets; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtEuroLtPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtEuroPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtEuroStPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; euro; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtFcLtPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtFcPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtFcStPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtLtPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtOthFcLtPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtOthFcPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtOthFcStPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtStPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtUsdLtPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtUsdPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtUsdStPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtYenLtPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtYenPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveNonFinExclGenGovtYenStPos => "U.S. debt assets except reserve assets; nonfinancial institutions except general government; yen; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFcLtPos => "U.S. debt assets except reserve assets; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFcPos => "U.S. debt assets except reserve assets; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFcStPos => "U.S. debt assets except reserve assets; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinEuroLtPos => "U.S. debt assets except reserve assets; other financial institutions; euro; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinEuroPos => "U.S. debt assets except reserve assets; other financial institutions; euro; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinEuroStPos => "U.S. debt assets except reserve assets; other financial institutions; euro; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinFcLtPos => "U.S. debt assets except reserve assets; other financial institutions; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinFcPos => "U.S. debt assets except reserve assets; other financial institutions; foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinFcStPos => "U.S. debt assets except reserve assets; other financial institutions; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinLtPos => "U.S. debt assets except reserve assets; other financial institutions; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinOthFcLtPos => "U.S. debt assets except reserve assets; other financial institutions; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinOthFcPos => "U.S. debt assets except reserve assets; other financial institutions; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinOthFcStPos => "U.S. debt assets except reserve assets; other financial institutions; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinPos => "U.S. debt assets except reserve assets; other financial institutions; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinStPos => "U.S. debt assets except reserve assets; other financial institutions; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinUsdLtPos => "U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinUsdPos => "U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinUsdStPos => "U.S. debt assets except reserve assets; other financial institutions; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinYenLtPos => "U.S. debt assets except reserve assets; other financial institutions; yen; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinYenPos => "U.S. debt assets except reserve assets; other financial institutions; yen; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveOthFinYenStPos => "U.S. debt assets except reserve assets; other financial institutions; yen; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReservePos => "U.S. debt assets except reserve assets; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveStPos => "U.S. debt assets except reserve assets; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveUsdLtPos => "U.S. debt assets except reserve assets; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveUsdPos => "U.S. debt assets except reserve assets; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveUsdStPos => "U.S. debt assets except reserve assets; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveYenLtPos => "U.S. debt assets except reserve assets; yen; long term; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveYenPos => "U.S. debt assets except reserve assets; yen; quarterly not seasonally adjusted",
            Self::DebtAssetsExclReserveYenStPos => "U.S. debt assets except reserve assets; yen; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankEuroLtPos => "U.S. debt liabilities; central bank; euro; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankEuroPos => "U.S. debt liabilities; central bank; euro; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankEuroStPos => "U.S. debt liabilities; central bank; euro; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankFcLtPos => "U.S. debt liabilities; central bank; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankFcPos => "U.S. debt liabilities; central bank; foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankFcStPos => "U.S. debt liabilities; central bank; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankLtPos => "U.S. debt liabilities; central bank; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankOthFcLtPos => "U.S. debt liabilities; central bank; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankOthFcPos => "U.S. debt liabilities; central bank; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankOthFcStPos => "U.S. debt liabilities; central bank; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankPos => "U.S. debt liabilities; central bank; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankStPos => "U.S. debt liabilities; central bank; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankUsdLtPos => "U.S. debt liabilities; central bank; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankUsdPos => "U.S. debt liabilities; central bank; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankUsdStPos => "U.S. debt liabilities; central bank; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankYenLtPos => "U.S. debt liabilities; central bank; yen; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankYenPos => "U.S. debt liabilities; central bank; yen; quarterly not seasonally adjusted",
            Self::DebtLiabsCenBankYenStPos => "U.S. debt liabilities; central bank; yen; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankEuroLtPos => "U.S. debt liabilities; deposit-taking institutions except central bank; euro; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankEuroPos => "U.S. debt liabilities; deposit-taking institutions except central bank; euro; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankEuroStPos => "U.S. debt liabilities; deposit-taking institutions except central bank; euro; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankFcLtPos => "U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankFcPos => "U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankFcStPos => "U.S. debt liabilities; deposit-taking institutions except central bank; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankLtPos => "U.S. debt liabilities; deposit-taking institutions except central bank; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankOthFcLtPos => "U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankOthFcPos => "U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankOthFcStPos => "U.S. debt liabilities; deposit-taking institutions except central bank; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankPos => "U.S. debt liabilities; deposit-taking institutions except central bank; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankStPos => "U.S. debt liabilities; deposit-taking institutions except central bank; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankUsdLtPos => "U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankUsdPos => "U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankUsdStPos => "U.S. debt liabilities; deposit-taking institutions except central bank; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankYenLtPos => "U.S. debt liabilities; deposit-taking institutions except central bank; yen; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankYenPos => "U.S. debt liabilities; deposit-taking institutions except central bank; yen; quarterly not seasonally adjusted",
            Self::DebtLiabsDepExclCenBankYenStPos => "U.S. debt liabilities; deposit-taking institutions except central bank; yen; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsEuroLtPos => "U.S. debt liabilities; euro; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsEuroPos => "U.S. debt liabilities; euro; quarterly not seasonally adjusted",
            Self::DebtLiabsEuroStPos => "U.S. debt liabilities; euro; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsFcLtPos => "U.S. debt liabilities; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsFcPos => "U.S. debt liabilities; foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsFcStPos => "U.S. debt liabilities; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtEuroLtPos => "U.S. debt liabilities; general government; euro; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtEuroPos => "U.S. debt liabilities; general government; euro; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtEuroStPos => "U.S. debt liabilities; general government; euro; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtFcLtPos => "U.S. debt liabilities; general government; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtFcPos => "U.S. debt liabilities; general government; foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtFcStPos => "U.S. debt liabilities; general government; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtLtPos => "U.S. debt liabilities; general government; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtOthFcLtPos => "U.S. debt liabilities; general government; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtOthFcPos => "U.S. debt liabilities; general government; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtOthFcStPos => "U.S. debt liabilities; general government; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtPos => "U.S. debt liabilities; general government; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtStPos => "U.S. debt liabilities; general government; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtUsdLtPos => "U.S. debt liabilities; general government; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtUsdPos => "U.S. debt liabilities; general government; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtUsdStPos => "U.S. debt liabilities; general government; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtYenLtPos => "U.S. debt liabilities; general government; yen; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtYenPos => "U.S. debt liabilities; general government; yen; quarterly not seasonally adjusted",
            Self::DebtLiabsGenGovtYenStPos => "U.S. debt liabilities; general government; yen; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsLtPos => "U.S. debt liabilities; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtEuroLtPos => "U.S. debt liabilities; nonfinancial institutions except general government; euro; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtEuroPos => "U.S. debt liabilities; nonfinancial institutions except general government; euro; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtEuroStPos => "U.S. debt liabilities; nonfinancial institutions except general government; euro; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtFcLtPos => "U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtFcPos => "U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtFcStPos => "U.S. debt liabilities; nonfinancial institutions except general government; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtLtPos => "U.S. debt liabilities; nonfinancial institutions except general government; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtOthFcLtPos => "U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtOthFcPos => "U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtOthFcStPos => "U.S. debt liabilities; nonfinancial institutions except general government; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtPos => "U.S. debt liabilities; nonfinancial institutions except general government; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtStPos => "U.S. debt liabilities; nonfinancial institutions except general government; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtUsdLtPos => "U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtUsdPos => "U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtUsdStPos => "U.S. debt liabilities; nonfinancial institutions except general government; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtYenLtPos => "U.S. debt liabilities; nonfinancial institutions except general government; yen; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtYenPos => "U.S. debt liabilities; nonfinancial institutions except general government; yen; quarterly not seasonally adjusted",
            Self::DebtLiabsNonFinExclGenGovtYenStPos => "U.S. debt liabilities; nonfinancial institutions except general government; yen; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFcLtPos => "U.S. debt liabilities; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFcPos => "U.S. debt liabilities; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFcStPos => "U.S. debt liabilities; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinEuroLtPos => "U.S. debt liabilities; other financial institutions; euro; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinEuroPos => "U.S. debt liabilities; other financial institutions; euro; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinEuroStPos => "U.S. debt liabilities; other financial institutions; euro; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinFcLtPos => "U.S. debt liabilities; other financial institutions; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinFcPos => "U.S. debt liabilities; other financial institutions; foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinFcStPos => "U.S. debt liabilities; other financial institutions; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinLtPos => "U.S. debt liabilities; other financial institutions; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinOthFcLtPos => "U.S. debt liabilities; other financial institutions; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinOthFcPos => "U.S. debt liabilities; other financial institutions; other foreign currency; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinOthFcStPos => "U.S. debt liabilities; other financial institutions; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinPos => "U.S. debt liabilities; other financial institutions; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinStPos => "U.S. debt liabilities; other financial institutions; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinUsdLtPos => "U.S. debt liabilities; other financial institutions; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinUsdPos => "U.S. debt liabilities; other financial institutions; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinUsdStPos => "U.S. debt liabilities; other financial institutions; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinYenLtPos => "U.S. debt liabilities; other financial institutions; yen; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinYenPos => "U.S. debt liabilities; other financial institutions; yen; quarterly not seasonally adjusted",
            Self::DebtLiabsOthFinYenStPos => "U.S. debt liabilities; other financial institutions; yen; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsPos => "U.S. debt liabilities; quarterly not seasonally adjusted",
            Self::DebtLiabsStPos => "U.S. debt liabilities; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsUsdLtPos => "U.S. debt liabilities; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsUsdPos => "U.S. debt liabilities; U.S. dollar; quarterly not seasonally adjusted",
            Self::DebtLiabsUsdStPos => "U.S. debt liabilities; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DebtLiabsYenLtPos => "U.S. debt liabilities; yen; long term; quarterly not seasonally adjusted",
            Self::DebtLiabsYenPos => "U.S. debt liabilities; yen; quarterly not seasonally adjusted",
            Self::DebtLiabsYenStPos => "U.S. debt liabilities; yen; short term; quarterly not seasonally adjusted",
            Self::DebtSecAssetsChgPosNie => "U.S. assets; portfolio investment; debt securities (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::DebtSecAssetsChgPosOth => "U.S. assets; portfolio investment; debt securities (Change in position not attributable to financial-account transactions); annual",
            Self::DebtSecAssetsChgPosPrice => "U.S. assets; portfolio investment; debt securities (Change in position attributable to price changes); annual",
            Self::DebtSecAssetsChgPosTrans => "U.S. assets; portfolio investment; debt securities (Change in position attributable to financial-account transactions); annual",
            Self::DebtSecAssetsChgPosXRate => "U.S. assets; portfolio investment; debt securities (Change in position attributable to exchange-rate changes); annual",
            Self::DebtSecAssetsChgPos => "U.S. assets; portfolio investment; debt securities (Change in position); annual",
            Self::DebtSecAssetsPos => "U.S. assets; portfolio investment; debt securities; annual",
            Self::DebtSecLiabsChgPosNie => "U.S. liabilities; portfolio investment; debt securities (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::DebtSecLiabsChgPosOth => "U.S. liabilities; portfolio investment; debt securities (Change in position not attributable to financial-account transactions); annual",
            Self::DebtSecLiabsChgPosPrice => "U.S. liabilities; portfolio investment; debt securities (Change in position attributable to price changes); annual",
            Self::DebtSecLiabsChgPosTrans => "U.S. liabilities; portfolio investment; debt securities (Change in position attributable to financial-account transactions); annual",
            Self::DebtSecLiabsChgPosXRate => "U.S. liabilities; portfolio investment; debt securities (Change in position attributable to exchange-rate changes); annual",
            Self::DebtSecLiabsChgPos => "U.S. liabilities; portfolio investment; debt securities (Change in position); annual",
            Self::DebtSecLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; debt securities; annual",
            Self::DebtSecLiabsPos => "U.S. liabilities; portfolio investment; debt securities; annual",
            Self::DiInvAssetsChgPosNie => "U.S. assets; direct investment at market value, asset/liability basis (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::DiInvAssetsChgPosOth => "U.S. assets; direct investment at market value, asset/liability basis (Change in position not attributable to financial-account transactions); annual",
            Self::DiInvAssetsChgPosPrice => "U.S. assets; direct investment at market value, asset/liability basis (Change in position attributable to price changes); annual",
            Self::DiInvAssetsChgPosTrans => "U.S. assets; direct investment at market value, asset/liability basis (Change in position attributable to financial-account transactions); annual",
            Self::DiInvAssetsChgPosXRate => "U.S. assets; direct investment at market value, asset/liability basis (Change in position attributable to exchange-rate changes); annual",
            Self::DiInvAssetsChgPos => "U.S. assets; direct investment at market value, asset/liability basis (Change in position); annual",
            Self::DiInvAssetsCurrCostPos => "U.S. assets; direct investment at current cost, asset/liability basis; annual",
            Self::DiInvAssetsHistCostToMarketValueAdjPos => "U.S. assets; direct investment; adjustment to revalue equity from historical cost to market value; annual",
            Self::DiInvAssetsNonSpePos => "U.S. assets; direct investment at market value, asset/liability basis; Non-SPEs; annual",
            Self::DiInvAssetsPos => "U.S. assets; direct investment at market value, asset/liability basis; annual",
            Self::DiInvAssetsSpePos => "U.S. assets; direct investment at market value, asset/liability basis; SPEs; annual",
            Self::DiInvDebtInstAssetsChgPosNie => "U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::DiInvDebtInstAssetsChgPosOth => "U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position not attributable to financial-account transactions); annual",
            Self::DiInvDebtInstAssetsChgPosPrice => "U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position attributable to price changes); annual",
            Self::DiInvDebtInstAssetsChgPosTrans => "U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position attributable to financial-account transactions); annual",
            Self::DiInvDebtInstAssetsChgPosXRate => "U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position attributable to exchange-rate changes); annual",
            Self::DiInvDebtInstAssetsChgPos => "U.S. assets; direct investment, asset/liability basis; debt instruments (Change in position); annual",
            Self::DiInvDebtInstAssetsEuroLtPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; euro; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsEuroPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; euro; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsEuroStPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; euro; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsFcLtPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsFcPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsFcStPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsLtPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsNonSpePos => "U.S. assets; direct investment, asset/liability basis; debt instruments; Non-SPEs; annual",
            Self::DiInvDebtInstAssetsOthFcLtPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsOthFcPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsOthFcStPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; annual",
            Self::DiInvDebtInstAssetsSpePos => "U.S. assets; direct investment, asset/liability basis; debt instruments; SPEs; annual",
            Self::DiInvDebtInstAssetsStPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsUsdLtPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsUsdPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsUsdStPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsYenLtPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; yen; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsYenPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; yen; quarterly not seasonally adjusted",
            Self::DiInvDebtInstAssetsYenStPos => "U.S. assets; direct investment, asset/liability basis; debt instruments; yen; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstInwardPos => "Inward direct investment (foreign direct investment in the United States), directional basis; debt instruments; annual",
            Self::DiInvDebtInstLiabsChgPosNie => "U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::DiInvDebtInstLiabsChgPosOth => "U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position not attributable to financial-account transactions); annual",
            Self::DiInvDebtInstLiabsChgPosPrice => "U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position attributable to price changes); annual",
            Self::DiInvDebtInstLiabsChgPosTrans => "U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position attributable to financial-account transactions); annual",
            Self::DiInvDebtInstLiabsChgPosXRate => "U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position attributable to exchange-rate changes); annual",
            Self::DiInvDebtInstLiabsChgPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments (Change in position); annual",
            Self::DiInvDebtInstLiabsEuroLtPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsEuroPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsEuroStPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; euro; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsFcLtPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsFcPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsFcStPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; foreign currency; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsLtPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsNonSpePos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; Non-SPEs; annual",
            Self::DiInvDebtInstLiabsOthFcLtPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsOthFcPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsOthFcStPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; other foreign currency; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; annual",
            Self::DiInvDebtInstLiabsSpePos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; SPEs; annual",
            Self::DiInvDebtInstLiabsStPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsUsdLtPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsUsdPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsUsdStPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; U.S. dollar; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsYenLtPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; long term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsYenPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; quarterly not seasonally adjusted",
            Self::DiInvDebtInstLiabsYenStPos => "U.S. liabilities; direct investment, asset/liability basis; debt instruments; yen; short term; quarterly not seasonally adjusted",
            Self::DiInvDebtInstOutwardPos => "Outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; annual",
            Self::DiInvDebtInstUsAffiliatesClaimsByNonSpePos => "U.S. non-SPE affiliates' debt asset position in their foreign parent groups; annual",
            Self::DiInvDebtInstUsAffiliatesClaimsBySpePos => "U.S. SPE affiliates' debt asset position in their foreign parent groups; annual",
            Self::DiInvDebtInstUsAffiliatesClaimsNonSpePos => "Direct investment; debt instruments; U.S. affiliates' claims; Non-SPEs; annual",
            Self::DiInvDebtInstUsAffiliatesClaimsPos => "Direct investment; debt instruments; U.S. affiliates' claims; annual",
            Self::DiInvDebtInstUsAffiliatesClaimsSpePos => "Direct investment; debt instruments; U.S. affiliates' claims; SPEs; annual",
            Self::DiInvDebtInstUsAffiliatesLiabsNonSpePos => "Direct investment; debt instruments; U.S. affiliates' liabilites; Non-SPEs; annual",
            Self::DiInvDebtInstUsAffiliatesLiabsPos => "Direct investment; debt instruments; U.S. affiliates' liabilites; annual",
            Self::DiInvDebtInstUsAffiliatesLiabsSpePos => "Direct investment; debt instruments; U.S. affiliates' liabilites; SPEs; annual",
            Self::DiInvDebtInstUsParentsClaimsNonSpePos => "Direct investment; debt instruments; U.S. parents' claims; Non-SPEs; annual",
            Self::DiInvDebtInstUsParentsClaimsPos => "Direct investment; debt instruments; U.S. parents' claims; annual",
            Self::DiInvDebtInstUsParentsClaimsSpePos => "Direct investment; debt instruments; U.S. parents' claims; SPEs; annual",
            Self::DiInvDebtInstUsParentsLiabsInNonSpePos => "U.S. parents' debt liability position in their foreign non-SPE affiliates; annual",
            Self::DiInvDebtInstUsParentsLiabsInSpePos => "U.S. parents' debt liability position in their foreign SPE affiliates; annual",
            Self::DiInvDebtInstUsParentsLiabsNonSpePos => "Direct investment; debt instruments; U.S. parents' liabilites; Non-SPEs; annual",
            Self::DiInvDebtInstUsParentsLiabsPos => "Direct investment; debt instruments; U.S. parents' liabilites; annual",
            Self::DiInvDebtInstUsParentsLiabsSpePos => "Direct investment; debt instruments; U.S. parents' liabilites; SPEs; annual",
            Self::DiInvDirectionalBasisAdjPos => "Direct investment; adjustments to convert to directional basis; annual",
            Self::DiInvEquityAssetsChgPosNie => "U.S. assets; direct investment at market value; equity (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::DiInvEquityAssetsChgPosOth => "U.S. assets; direct investment at market value; equity (Change in position not attributable to financial-account transactions); annual",
            Self::DiInvEquityAssetsChgPosPrice => "U.S. assets; direct investment at market value; equity (Change in position attributable to price changes); annual",
            Self::DiInvEquityAssetsChgPosTrans => "U.S. assets; direct investment at market value; equity (Change in position attributable to financial-account transactions); annual",
            Self::DiInvEquityAssetsChgPosXRate => "U.S. assets; direct investment at market value; equity (Change in position attributable to exchange-rate changes); annual",
            Self::DiInvEquityAssetsChgPos => "U.S. assets; direct investment at market value; equity (Change in position); annual",
            Self::DiInvEquityAssetsCurrCostPos => "U.S. assets; direct investment at current cost; equity; annual",
            Self::DiInvEquityAssetsHistCostPos => "U.S. assets; direct investment at historical cost; equity; annual",
            Self::DiInvEquityAssetsNonSpePos => "U.S. assets; direct investment at market value; equity; Non-SPEs; annual",
            Self::DiInvEquityAssetsPos => "U.S. assets; direct investment at market value; equity; annual",
            Self::DiInvEquityAssetsSpePos => "U.S. assets; direct investment at market value; equity; SPEs; annual",
            Self::DiInvEquityInwardPos => "U.S. liabilities; direct investment at market value; equity; annual",
            Self::DiInvEquityLiabsChgPosNie => "U.S. liabilities; direct investment at market value; equity (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::DiInvEquityLiabsChgPosOth => "U.S. liabilities; direct investment at market value; equity (Change in position not attributable to financial-account transactions); annual",
            Self::DiInvEquityLiabsChgPosPrice => "U.S. liabilities; direct investment at market value; equity (Change in position attributable to price changes); annual",
            Self::DiInvEquityLiabsChgPosTrans => "U.S. liabilities; direct investment at market value; equity (Change in position attributable to financial-account transactions); annual",
            Self::DiInvEquityLiabsChgPosXRate => "U.S. liabilities; direct investment at market value; equity (Change in position attributable to exchange-rate changes); annual",
            Self::DiInvEquityLiabsChgPos => "U.S. liabilities; direct investment at market value; equity (Change in position); annual",
            Self::DiInvEquityLiabsCurrCostPos => "U.S. liabilities; direct investment at current cost; equity; annual",
            Self::DiInvEquityLiabsHistCostPos => "U.S. liabilities; direct investment at historical cost; equity; annual",
            Self::DiInvEquityLiabsNonSpePos => "U.S. liabilities; direct investment at market value; equity; Non-SPEs; annual",
            Self::DiInvEquityLiabsPos => "U.S. liabilities; direct investment at market value; equity; annual",
            Self::DiInvEquityLiabsSpePos => "U.S. liabilities; direct investment at market value; equity; SPEs; annual",
            Self::DiInvInwardCurrCostPos => "Inward direct investment (foreign direct investment in the United States) at current cost, directional basis; annual",
            Self::DiInvInwardHistCostPos => "Inward direct investment (foreign direct investment in the United States) at historical cost, directional basis; annual",
            Self::DiInvInwardMarketValuePos => "Inward direct investment (foreign direct investment in the United States) at market value, directional basis; annual",
            Self::DiInvLiabsChgPosNie => "U.S. liabilities; direct investment at market value, asset/liability basis (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::DiInvLiabsChgPosOth => "U.S. liabilities; direct investment at market value, asset/liability basis (Change in position not attributable to financial-account transactions); annual",
            Self::DiInvLiabsChgPosPrice => "U.S. liabilities; direct investment at market value, asset/liability basis (Change in position attributable to price changes); annual",
            Self::DiInvLiabsChgPosTrans => "U.S. liabilities; direct investment at market value, asset/liability basis (Change in position attributable to financial-account transactions); annual",
            Self::DiInvLiabsChgPosXRate => "U.S. liabilities; direct investment at market value, asset/liability basis (Change in position attributable to exchange-rate changes); annual",
            Self::DiInvLiabsChgPos => "U.S. liabilities; direct investment at market value, asset/liability basis (Change in position); annual",
            Self::DiInvLiabsCurrCostPos => "U.S. liabilities; direct investment at current cost, asset/liability basis; annual",
            Self::DiInvLiabsHistCostToMarketValueAdjPos => "U.S. liabilities; direct investment; adjustment to revalue equity from historical cost to market value; annual",
            Self::DiInvLiabsNonSpePos => "U.S. liabilities; direct investment at market value, asset/liability basis; Non-SPEs; annual",
            Self::DiInvLiabsPos => "U.S. liabilities; direct investment at market value, asset/liability basis; annual",
            Self::DiInvLiabsSpePos => "U.S. liabilities; direct investment at market value, asset/liability basis; SPEs; annual",
            Self::DiInvOutwardCurrCostPos => "Outward direct investment (U.S. direct investment abroad) at current cost, directional basis; annual",
            Self::DiInvOutwardHistCostPos => "Outward direct investment (U.S. direct investment abroad) at historical cost, directional basis; annual",
            Self::DiInvOutwardMarketValuePos => "Outward direct investment (U.S. direct investment abroad) at market value, directional basis; annual",
            Self::EquityAndInvFundSharesAssetsChgPosNie => "U.S. assets; portfolio investment; equity and investment fund shares (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::EquityAndInvFundSharesAssetsChgPosOth => "U.S. assets; portfolio investment; equity and investment fund shares (Change in position not attributable to financial-account transactions); annual",
            Self::EquityAndInvFundSharesAssetsChgPosPrice => "U.S. assets; portfolio investment; equity and investment fund shares (Change in position attributable to price changes); annual",
            Self::EquityAndInvFundSharesAssetsChgPosTrans => "U.S. assets; portfolio investment; equity and investment fund shares (Change in position attributable to financial-account transactions); annual",
            Self::EquityAndInvFundSharesAssetsChgPosXRate => "U.S. assets; portfolio investment; equity and investment fund shares (Change in position attributable to exchange-rate changes); annual",
            Self::EquityAndInvFundSharesAssetsChgPos => "U.S. assets; portfolio investment; equity and investment fund shares (Change in position); annual",
            Self::EquityAndInvFundSharesAssetsPos => "U.S. assets; portfolio investment; equity and investment fund shares; annual",
            Self::EquityAndInvFundSharesLiabsChgPosNie => "U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::EquityAndInvFundSharesLiabsChgPosOth => "U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position not attributable to financial-account transactions); annual",
            Self::EquityAndInvFundSharesLiabsChgPosPrice => "U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position attributable to price changes); annual",
            Self::EquityAndInvFundSharesLiabsChgPosTrans => "U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position attributable to financial-account transactions); annual",
            Self::EquityAndInvFundSharesLiabsChgPosXRate => "U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position attributable to exchange-rate changes); annual",
            Self::EquityAndInvFundSharesLiabsChgPos => "U.S. liabilities; portfolio investment; equity and investment fund shares (Change in position); annual",
            Self::EquityAndInvFundSharesLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; equity and investment fund shares; annual",
            Self::EquityAndInvFundSharesLiabsPos => "U.S. liabilities; portfolio investment; equity and investment fund shares; annual",
            Self::FinAssetsChgPosNie => "U.S. assets (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinAssetsChgPosOth => "U.S. assets (Change in position not attributable to financial-account transactions); annual",
            Self::FinAssetsChgPosPrice => "U.S. assets (Change in position attributable to price changes); annual",
            Self::FinAssetsChgPosTrans => "U.S. assets (Change in position attributable to financial-account transactions); annual",
            Self::FinAssetsChgPosXRate => "U.S. assets (Change in position attributable to exchange-rate changes); annual",
            Self::FinAssetsChgPos => "U.S. assets (Change in position); annual",
            Self::FinAssetsExclFinDerivChgPosNie => "U.S. assets excluding financial derivatives (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinAssetsExclFinDerivChgPosOth => "U.S. assets excluding financial derivatives (Change in position not attributable to financial-account transactions); annual",
            Self::FinAssetsExclFinDerivChgPosPrice => "U.S. assets excluding financial derivatives (Change in position attributable to price changes); annual",
            Self::FinAssetsExclFinDerivChgPosTrans => "U.S. assets excluding financial derivatives (Change in position attributable to financial-account transactions); annual",
            Self::FinAssetsExclFinDerivChgPosXRate => "U.S. assets excluding financial derivatives (Change in position attributable to exchange-rate changes); annual",
            Self::FinAssetsExclFinDerivChgPos => "U.S. assets excluding financial derivatives (Change in position); annual",
            Self::FinAssetsExclFinDerivPos => "U.S. assets excluding financial derivatives; annual",
            Self::FinAssetsPos => "U.S. assets; annual",
            Self::FinDerivAssetsChgPosNie => "U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivAssetsChgPosOth => "U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivAssetsChgPosPrice => "U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position attributable to price changes); annual",
            Self::FinDerivAssetsChgPosTrans => "U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivAssetsChgPosXRate => "U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivAssetsChgPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value (Change in position); annual",
            Self::FinDerivAssetsPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; annual",
            Self::FinDerivExchTradedAssetsChgPosNie => "U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivExchTradedAssetsChgPosOth => "U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivExchTradedAssetsChgPosPrice => "U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position attributable to price changes); annual",
            Self::FinDerivExchTradedAssetsChgPosTrans => "U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivExchTradedAssetsChgPosXRate => "U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivExchTradedAssetsChgPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts (Change in position); annual",
            Self::FinDerivExchTradedAssetsPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; exchange-traded contracts; annual",
            Self::FinDerivExchTradedLiabsChgPosNie => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivExchTradedLiabsChgPosOth => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivExchTradedLiabsChgPosPrice => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position attributable to price changes); annual",
            Self::FinDerivExchTradedLiabsChgPosTrans => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivExchTradedLiabsChgPosXRate => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivExchTradedLiabsChgPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts (Change in position); annual",
            Self::FinDerivExchTradedLiabsPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; exchange-traded contracts; annual",
            Self::FinDerivForExAssetsChgPosNie => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivForExAssetsChgPosOth => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivForExAssetsChgPosPrice => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to price changes); annual",
            Self::FinDerivForExAssetsChgPosTrans => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivForExAssetsChgPosXRate => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivForExAssetsChgPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts (Change in position); annual",
            Self::FinDerivForExAssetsPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; foreign exchange contracts; annual",
            Self::FinDerivForExLiabsChgPosNie => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivForExLiabsChgPosOth => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivForExLiabsChgPosPrice => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to price changes); annual",
            Self::FinDerivForExLiabsChgPosTrans => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivForExLiabsChgPosXRate => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivForExLiabsChgPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts (Change in position); annual",
            Self::FinDerivForExLiabsPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; foreign exchange contracts; annual",
            Self::FinDerivLiabsChgPosNie => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivLiabsChgPosOth => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivLiabsChgPosPrice => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position attributable to price changes); annual",
            Self::FinDerivLiabsChgPosTrans => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivLiabsChgPosXRate => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivLiabsChgPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value (Change in position); annual",
            Self::FinDerivLiabsPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; annual",
            Self::FinDerivNetChgPosNie => "U.S. net international investment position; financial derivatives other than reserves (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivNetChgPosOth => "U.S. net international investment position; financial derivatives other than reserves (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivNetChgPosPrice => "U.S. net international investment position; financial derivatives other than reserves (Change in position attributable to price changes); annual",
            Self::FinDerivNetChgPosTrans => "U.S. net international investment position; financial derivatives other than reserves (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivNetChgPosXRate => "U.S. net international investment position; financial derivatives other than reserves (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivNetChgPos => "U.S. net international investment position; financial derivatives other than reserves (Change in position); annual",
            Self::FinDerivNetPos => "U.S. net international investment position; financial derivatives other than reserves; annual",
            Self::FinDerivOtcAssetsChgPosNie => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivOtcAssetsChgPosOth => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivOtcAssetsChgPosPrice => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position attributable to price changes); annual",
            Self::FinDerivOtcAssetsChgPosTrans => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivOtcAssetsChgPosXRate => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivOtcAssetsChgPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts (Change in position); annual",
            Self::FinDerivOtcAssetsPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; annual",
            Self::FinDerivOtcLiabsChgPosNie => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivOtcLiabsChgPosOth => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivOtcLiabsChgPosPrice => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position attributable to price changes); annual",
            Self::FinDerivOtcLiabsChgPosTrans => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivOtcLiabsChgPosXRate => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivOtcLiabsChgPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts (Change in position); annual",
            Self::FinDerivOtcLiabsPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; annual",
            Self::FinDerivOthAssetsChgPosNie => "U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivOthAssetsChgPosOth => "U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivOthAssetsChgPosPrice => "U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position attributable to price changes); annual",
            Self::FinDerivOthAssetsChgPosTrans => "U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivOthAssetsChgPosXRate => "U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivOthAssetsChgPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts (Change in position); annual",
            Self::FinDerivOthAssetsPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; other over-the-counter contracts; annual",
            Self::FinDerivOthLiabsChgPosNie => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivOthLiabsChgPosOth => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivOthLiabsChgPosPrice => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position attributable to price changes); annual",
            Self::FinDerivOthLiabsChgPosTrans => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivOthLiabsChgPosXRate => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivOthLiabsChgPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts (Change in position); annual",
            Self::FinDerivOthLiabsPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; other over-the-counter contracts; annual",
            Self::FinDerivReserveAssetsChgPosNie => "U.S. assets; other reserve assets; financial derivatives (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivReserveAssetsChgPosOth => "U.S. assets; other reserve assets; financial derivatives (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivReserveAssetsChgPosPrice => "U.S. assets; other reserve assets; financial derivatives (Change in position attributable to price changes); annual",
            Self::FinDerivReserveAssetsChgPosTrans => "U.S. assets; other reserve assets; financial derivatives (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivReserveAssetsChgPosXRate => "U.S. assets; other reserve assets; financial derivatives (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivReserveAssetsChgPos => "U.S. assets; other reserve assets; financial derivatives (Change in position); annual",
            Self::FinDerivReserveAssetsPos => "U.S. assets; other reserve assets; financial derivatives; annual",
            Self::FinDerivSingleCurrAssetsChgPosNie => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivSingleCurrAssetsChgPosOth => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivSingleCurrAssetsChgPosPrice => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to price changes); annual",
            Self::FinDerivSingleCurrAssetsChgPosTrans => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivSingleCurrAssetsChgPosXRate => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivSingleCurrAssetsChgPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position); annual",
            Self::FinDerivSingleCurrAssetsPos => "U.S. assets; financial derivatives other than reserves, gross positive fair value; over-the-counter contracts; single-currency interest rate contracts; annual",
            Self::FinDerivSingleCurrLiabsChgPosNie => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinDerivSingleCurrLiabsChgPosOth => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position not attributable to financial-account transactions); annual",
            Self::FinDerivSingleCurrLiabsChgPosPrice => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to price changes); annual",
            Self::FinDerivSingleCurrLiabsChgPosTrans => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to financial-account transactions); annual",
            Self::FinDerivSingleCurrLiabsChgPosXRate => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position attributable to exchange-rate changes); annual",
            Self::FinDerivSingleCurrLiabsChgPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts (Change in position); annual",
            Self::FinDerivSingleCurrLiabsPos => "U.S. liabilities; financial derivatives other than reserves, gross negative fair value; over-the-counter contracts; single-currency interest rate contracts; annual",
            Self::FinLiabsChgPosNie => "U.S. liabilities (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinLiabsChgPosOth => "U.S. liabilities (Change in position not attributable to financial-account transactions); annual",
            Self::FinLiabsChgPosPrice => "U.S. liabilities (Change in position attributable to price changes); annual",
            Self::FinLiabsChgPosTrans => "U.S. liabilities (Change in position attributable to financial-account transactions); annual",
            Self::FinLiabsChgPosXRate => "U.S. liabilities (Change in position attributable to exchange-rate changes); annual",
            Self::FinLiabsChgPos => "U.S. liabilities (Change in position); annual",
            Self::FinLiabsExclFinDerivChgPosNie => "U.S. liabilities excluding financial derivatives (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::FinLiabsExclFinDerivChgPosOth => "U.S. liabilities excluding financial derivatives (Change in position not attributable to financial-account transactions); annual",
            Self::FinLiabsExclFinDerivChgPosPrice => "U.S. liabilities excluding financial derivatives (Change in position attributable to price changes); annual",
            Self::FinLiabsExclFinDerivChgPosTrans => "U.S. liabilities excluding financial derivatives (Change in position attributable to financial-account transactions); annual",
            Self::FinLiabsExclFinDerivChgPosXRate => "U.S. liabilities excluding financial derivatives (Change in position attributable to exchange-rate changes); annual",
            Self::FinLiabsExclFinDerivChgPos => "U.S. liabilities excluding financial derivatives (Change in position); annual",
            Self::FinLiabsExclFinDerivPos => "U.S. liabilities excluding financial derivatives; annual",
            Self::FinLiabsFoaPos => "U.S. liabilities to foreign official agencies; annual",
            Self::FinLiabsPos => "U.S. liabilities; annual",
            Self::GoldReserveAssetsChgPosNie => "U.S. assets; reserve assets; monetary gold (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::GoldReserveAssetsChgPosOth => "U.S. assets; reserve assets; monetary gold (Change in position not attributable to financial-account transactions); annual",
            Self::GoldReserveAssetsChgPosPrice => "U.S. assets; reserve assets; monetary gold (Change in position attributable to price changes); annual",
            Self::GoldReserveAssetsChgPosTrans => "U.S. assets; reserve assets; monetary gold (Change in position attributable to financial-account transactions); annual",
            Self::GoldReserveAssetsChgPosXRate => "U.S. assets; reserve assets; monetary gold (Change in position attributable to exchange-rate changes); annual",
            Self::GoldReserveAssetsChgPos => "U.S. assets; reserve assets; monetary gold (Change in position); annual",
            Self::GoldReserveAssetsPos => "U.S. assets; reserve assets; monetary gold; annual",
            Self::ImfReserveAssetsChgPosNie => "U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::ImfReserveAssetsChgPosOth => "U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position not attributable to financial-account transactions); annual",
            Self::ImfReserveAssetsChgPosPrice => "U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position attributable to price changes); annual",
            Self::ImfReserveAssetsChgPosTrans => "U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position attributable to financial-account transactions); annual",
            Self::ImfReserveAssetsChgPosXRate => "U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position attributable to exchange-rate changes); annual",
            Self::ImfReserveAssetsChgPos => "U.S. assets; reserve assets; reserve position in the International Monetary Fund (Change in position); annual",
            Self::ImfReserveAssetsPos => "U.S. assets; reserve assets; reserve position in the International Monetary Fund; annual",
            Self::InsTechReservesAssetsChgPosNie => "U.S. assets; other investment; insurance technical reserves (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::InsTechReservesAssetsChgPosOth => "U.S. assets; other investment; insurance technical reserves (Change in position not attributable to financial-account transactions); annual",
            Self::InsTechReservesAssetsChgPosPrice => "U.S. assets; other investment; insurance technical reserves (Change in position attributable to price changes); annual",
            Self::InsTechReservesAssetsChgPosTrans => "U.S. assets; other investment; insurance technical reserves (Change in position attributable to financial-account transactions); annual",
            Self::InsTechReservesAssetsChgPosXRate => "U.S. assets; other investment; insurance technical reserves (Change in position attributable to exchange-rate changes); annual",
            Self::InsTechReservesAssetsChgPos => "U.S. assets; other investment; insurance technical reserves (Change in position); annual",
            Self::InsTechReservesAssetsPos => "U.S. assets; other investment; insurance technical reserves; annual",
            Self::InsTechReservesLiabsChgPosNie => "U.S. liabilities; other investment; insurance technical reserves (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::InsTechReservesLiabsChgPosOth => "U.S. liabilities; other investment; insurance technical reserves (Change in position not attributable to financial-account transactions); annual",
            Self::InsTechReservesLiabsChgPosPrice => "U.S. liabilities; other investment; insurance technical reserves (Change in position attributable to price changes); annual",
            Self::InsTechReservesLiabsChgPosTrans => "U.S. liabilities; other investment; insurance technical reserves (Change in position attributable to financial-account transactions); annual",
            Self::InsTechReservesLiabsChgPosXRate => "U.S. liabilities; other investment; insurance technical reserves (Change in position attributable to exchange-rate changes); annual",
            Self::InsTechReservesLiabsChgPos => "U.S. liabilities; other investment; insurance technical reserves (Change in position); annual",
            Self::InsTechReservesLiabsPos => "U.S. liabilities; other investment; insurance technical reserves; annual",
            Self::LoansAssetsChgPosNie => "U.S. assets; other investment; loans (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::LoansAssetsChgPosOth => "U.S. assets; other investment; loans (Change in position not attributable to financial-account transactions); annual",
            Self::LoansAssetsChgPosPrice => "U.S. assets; other investment; loans (Change in position attributable to price changes); annual",
            Self::LoansAssetsChgPosTrans => "U.S. assets; other investment; loans (Change in position attributable to financial-account transactions); annual",
            Self::LoansAssetsChgPosXRate => "U.S. assets; other investment; loans (Change in position attributable to exchange-rate changes); annual",
            Self::LoansAssetsChgPos => "U.S. assets; other investment; loans (Change in position); annual",
            Self::LoansAssetsPos => "U.S. assets; other investment; loans; annual",
            Self::LoansLiabsChgPosNie => "U.S. liabilities; other investment; loans (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::LoansLiabsChgPosOth => "U.S. liabilities; other investment; loans (Change in position not attributable to financial-account transactions); annual",
            Self::LoansLiabsChgPosPrice => "U.S. liabilities; other investment; loans (Change in position attributable to price changes); annual",
            Self::LoansLiabsChgPosTrans => "U.S. liabilities; other investment; loans (Change in position attributable to financial-account transactions); annual",
            Self::LoansLiabsChgPosXRate => "U.S. liabilities; other investment; loans (Change in position attributable to exchange-rate changes); annual",
            Self::LoansLiabsChgPos => "U.S. liabilities; other investment; loans (Change in position); annual",
            Self::LoansLiabsFoaPos => "U.S. liabilities to foreign official agencies; other investment; loans; annual",
            Self::LoansLiabsPos => "U.S. liabilities; other investment; loans; annual",
            Self::LtDebtSecAssetsChgPosNie => "U.S. assets; portfolio investment; long-term debt securities (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::LtDebtSecAssetsChgPosOth => "U.S. assets; portfolio investment; long-term debt securities (Change in position not attributable to financial-account transactions); annual",
            Self::LtDebtSecAssetsChgPosPrice => "U.S. assets; portfolio investment; long-term debt securities (Change in position attributable to price changes); annual",
            Self::LtDebtSecAssetsChgPosTrans => "U.S. assets; portfolio investment; long-term debt securities (Change in position attributable to financial-account transactions); annual",
            Self::LtDebtSecAssetsChgPosXRate => "U.S. assets; portfolio investment; long-term debt securities (Change in position attributable to exchange-rate changes); annual",
            Self::LtDebtSecAssetsChgPos => "U.S. assets; portfolio investment; long-term debt securities (Change in position); annual",
            Self::LtDebtSecAssetsPos => "U.S. assets; portfolio investment; long-term debt securities; annual",
            Self::LtDebtSecLiabsChgPosNie => "U.S. liabilities; portfolio investment; long-term debt securities (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::LtDebtSecLiabsChgPosOth => "U.S. liabilities; portfolio investment; long-term debt securities (Change in position not attributable to financial-account transactions); annual",
            Self::LtDebtSecLiabsChgPosPrice => "U.S. liabilities; portfolio investment; long-term debt securities (Change in position attributable to price changes); annual",
            Self::LtDebtSecLiabsChgPosTrans => "U.S. liabilities; portfolio investment; long-term debt securities (Change in position attributable to financial-account transactions); annual",
            Self::LtDebtSecLiabsChgPosXRate => "U.S. liabilities; portfolio investment; long-term debt securities (Change in position attributable to exchange-rate changes); annual",
            Self::LtDebtSecLiabsChgPos => "U.S. liabilities; portfolio investment; long-term debt securities (Change in position); annual",
            Self::LtDebtSecLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities; annual",
            Self::LtDebtSecLiabsPos => "U.S. liabilities; portfolio investment; long-term debt securities; annual",
            Self::LtDebtSecTreasLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities; Treasury bonds and notes; annual",
            Self::NetChgPosNie => "U.S. net international investment position (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::NetChgPosOth => "U.S. net international investment position (Change in position not attributable to financial-account transactions); annual",
            Self::NetChgPosPrice => "U.S. net international investment position (Change in position attributable to price changes); annual",
            Self::NetChgPosTrans => "U.S. net international investment position (Change in position attributable to financial-account transactions); annual",
            Self::NetChgPosXRate => "U.S. net international investment position (Change in position attributable to exchange-rate changes); annual",
            Self::NetChgPos => "U.S. net international investment position (Change in position); annual",
            Self::NetExclFinDerivChgPosNie => "U.S. net international investment position excluding financial derivatives (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::NetExclFinDerivChgPosOth => "U.S. net international investment position excluding financial derivatives (Change in position not attributable to financial-account transactions); annual",
            Self::NetExclFinDerivChgPosPrice => "U.S. net international investment position excluding financial derivatives (Change in position attributable to price changes); annual",
            Self::NetExclFinDerivChgPosTrans => "U.S. net international investment position excluding financial derivatives (Change in position attributable to financial-account transactions); annual",
            Self::NetExclFinDerivChgPosXRate => "U.S. net international investment position excluding financial derivatives (Change in position attributable to exchange-rate changes); annual",
            Self::NetExclFinDerivChgPos => "U.S. net international investment position excluding financial derivatives (Change in position); annual",
            Self::NetExclFinDerivPos => "U.S. net international investment position excluding financial derivatives; annual",
            Self::NetPos => "U.S. net international investment position; annual",
            Self::OthClmReserveAssetsChgPosNie => "U.S. assets; other reserve assets; other claims (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::OthClmReserveAssetsChgPosOth => "U.S. assets; other reserve assets; other claims (Change in position not attributable to financial-account transactions); annual",
            Self::OthClmReserveAssetsChgPosPrice => "U.S. assets; other reserve assets; other claims (Change in position attributable to price changes); annual",
            Self::OthClmReserveAssetsChgPosTrans => "U.S. assets; other reserve assets; other claims (Change in position attributable to financial-account transactions); annual",
            Self::OthClmReserveAssetsChgPosXRate => "U.S. assets; other reserve assets; other claims (Change in position attributable to exchange-rate changes); annual",
            Self::OthClmReserveAssetsChgPos => "U.S. assets; other reserve assets; other claims (Change in position); annual",
            Self::OthClmReserveAssetsPos => "U.S. assets; other reserve assets; other claims; annual",
            Self::OthEquityAssetsChgPosNie => "U.S. assets; other investment; other equity (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::OthEquityAssetsChgPosOth => "U.S. assets; other investment; other equity (Change in position not attributable to financial-account transactions); annual",
            Self::OthEquityAssetsChgPosPrice => "U.S. assets; other investment; other equity (Change in position attributable to price changes); annual",
            Self::OthEquityAssetsChgPosTrans => "U.S. assets; other investment; other equity (Change in position attributable to financial-account transactions); annual",
            Self::OthEquityAssetsChgPosXRate => "U.S. assets; other investment; other equity (Change in position attributable to exchange-rate changes); annual",
            Self::OthEquityAssetsChgPos => "U.S. assets; other investment; other equity (Change in position); annual",
            Self::OthEquityAssetsPos => "U.S. assets; other investment; other equity; annual",
            Self::OthEquityLiabsChgPosNie => "U.S. liabilities; other investment; other equity (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::OthEquityLiabsChgPosOth => "U.S. liabilities; other investment; other equity (Change in position not attributable to financial-account transactions); annual",
            Self::OthEquityLiabsChgPosPrice => "U.S. liabilities; other investment; other equity (Change in position attributable to price changes); annual",
            Self::OthEquityLiabsChgPosTrans => "U.S. liabilities; other investment; other equity (Change in position attributable to financial-account transactions); annual",
            Self::OthEquityLiabsChgPosXRate => "U.S. liabilities; other investment; other equity (Change in position attributable to exchange-rate changes); annual",
            Self::OthEquityLiabsChgPos => "U.S. liabilities; other investment; other equity (Change in position); annual",
            Self::OthEquityLiabsFoaPos => "U.S. liabilities to foreign official agencies; other investment; other equity; annual",
            Self::OthEquityLiabsPos => "U.S. liabilities; other investment; other equity; annual",
            Self::OthInvAssetsChgPosNie => "U.S. assets; other investment (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::OthInvAssetsChgPosOth => "U.S. assets; other investment (Change in position not attributable to financial-account transactions); annual",
            Self::OthInvAssetsChgPosPrice => "U.S. assets; other investment (Change in position attributable to price changes); annual",
            Self::OthInvAssetsChgPosTrans => "U.S. assets; other investment (Change in position attributable to financial-account transactions); annual",
            Self::OthInvAssetsChgPosXRate => "U.S. assets; other investment (Change in position attributable to exchange-rate changes); annual",
            Self::OthInvAssetsChgPos => "U.S. assets; other investment (Change in position); annual",
            Self::OthInvAssetsPos => "U.S. assets; other investment; annual",
            Self::OthInvLiabsChgPosNie => "U.S. liabilities; other investment (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::OthInvLiabsChgPosOth => "U.S. liabilities; other investment (Change in position not attributable to financial-account transactions); annual",
            Self::OthInvLiabsChgPosPrice => "U.S. liabilities; other investment (Change in position attributable to price changes); annual",
            Self::OthInvLiabsChgPosTrans => "U.S. liabilities; other investment (Change in position attributable to financial-account transactions); annual",
            Self::OthInvLiabsChgPosXRate => "U.S. liabilities; other investment (Change in position attributable to exchange-rate changes); annual",
            Self::OthInvLiabsChgPos => "U.S. liabilities; other investment (Change in position); annual",
            Self::OthInvLiabsFoaPos => "U.S. liabilities to foreign official agencies; other investment; annual",
            Self::OthInvLiabsPos => "U.S. liabilities; other investment; annual",
            Self::OthLtDebtSecLiabsChgPosNie => "U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::OthLtDebtSecLiabsChgPosOth => "U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position not attributable to financial-account transactions); annual",
            Self::OthLtDebtSecLiabsChgPosPrice => "U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position attributable to price changes); annual",
            Self::OthLtDebtSecLiabsChgPosTrans => "U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position attributable to financial-account transactions); annual",
            Self::OthLtDebtSecLiabsChgPosXRate => "U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position attributable to exchange-rate changes); annual",
            Self::OthLtDebtSecLiabsChgPos => "U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes (Change in position); annual",
            Self::OthLtDebtSecLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; long-term debt securities excluding Treasury bonds and notes; annual",
            Self::OthLtDebtSecLiabsPos => "U.S. liabilities; portfolio investment; long-term debt securities excluding Treasury bonds and notes; annual",
            Self::OthReserveAssetsChgPosNie => "U.S. assets; other reserve assets (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::OthReserveAssetsChgPosOth => "U.S. assets; other reserve assets (Change in position not attributable to financial-account transactions); annual",
            Self::OthReserveAssetsChgPosPrice => "U.S. assets; other reserve assets (Change in position attributable to price changes); annual",
            Self::OthReserveAssetsChgPosTrans => "U.S. assets; other reserve assets (Change in position attributable to financial-account transactions); annual",
            Self::OthReserveAssetsChgPosXRate => "U.S. assets; other reserve assets (Change in position attributable to exchange-rate changes); annual",
            Self::OthReserveAssetsChgPos => "U.S. assets; other reserve assets (Change in position); annual",
            Self::OthReserveAssetsPos => "U.S. assets; other reserve assets; annual",
            Self::OthStDebtSecLiabsChgPosNie => "U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::OthStDebtSecLiabsChgPosOth => "U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position not attributable to financial-account transactions); annual",
            Self::OthStDebtSecLiabsChgPosPrice => "U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position attributable to price changes); annual",
            Self::OthStDebtSecLiabsChgPosTrans => "U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position attributable to financial-account transactions); annual",
            Self::OthStDebtSecLiabsChgPosXRate => "U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position attributable to exchange-rate changes); annual",
            Self::OthStDebtSecLiabsChgPos => "U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates (Change in position); annual",
            Self::OthStDebtSecLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities excluding Treasury bills and certificates; annual",
            Self::OthStDebtSecLiabsPos => "U.S. liabilities; portfolio investment; short-term debt securities excluding Treasury bills and certificates; annual",
            Self::PfInvAssetsChgPosNie => "U.S. assets; portfolio investment (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::PfInvAssetsChgPosOth => "U.S. assets; portfolio investment (Change in position not attributable to financial-account transactions); annual",
            Self::PfInvAssetsChgPosPrice => "U.S. assets; portfolio investment (Change in position attributable to price changes); annual",
            Self::PfInvAssetsChgPosTrans => "U.S. assets; portfolio investment (Change in position attributable to financial-account transactions); annual",
            Self::PfInvAssetsChgPosXRate => "U.S. assets; portfolio investment (Change in position attributable to exchange-rate changes); annual",
            Self::PfInvAssetsChgPos => "U.S. assets; portfolio investment (Change in position); annual",
            Self::PfInvAssetsPos => "U.S. assets; portfolio investment; annual",
            Self::PfInvLiabsChgPosNie => "U.S. liabilities; portfolio investment (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::PfInvLiabsChgPosOth => "U.S. liabilities; portfolio investment (Change in position not attributable to financial-account transactions); annual",
            Self::PfInvLiabsChgPosPrice => "U.S. liabilities; portfolio investment (Change in position attributable to price changes); annual",
            Self::PfInvLiabsChgPosTrans => "U.S. liabilities; portfolio investment (Change in position attributable to financial-account transactions); annual",
            Self::PfInvLiabsChgPosXRate => "U.S. liabilities; portfolio investment (Change in position attributable to exchange-rate changes); annual",
            Self::PfInvLiabsChgPos => "U.S. liabilities; portfolio investment (Change in position); annual",
            Self::PfInvLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; annual",
            Self::PfInvLiabsPos => "U.S. liabilities; portfolio investment; annual",
            Self::ReserveAssetsCenBankLtPos => "U.S. assets; reserve assets; central bank; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsCenBankNotSdrBasketLtPos => "U.S. assets; reserve assets; central bank; not in special drawing rights basket; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsCenBankNotSdrBasketPos => "U.S. assets; reserve assets; central bank; not in special drawing rights basket; quarterly not seasonally adjusted",
            Self::ReserveAssetsCenBankNotSdrBasketStPos => "U.S. assets; reserve assets; central bank; not in special drawing rights basket; short term; quarterly not seasonally adjusted",
            Self::ReserveAssetsCenBankPos => "U.S. assets; reserve assets; central bank; quarterly not seasonally adjusted",
            Self::ReserveAssetsCenBankSdrBasketLtPos => "U.S. assets; reserve assets; central bank; in special drawing rights basket; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsCenBankSdrBasketPos => "U.S. assets; reserve assets; central bank; in special drawing rights basket; quarterly not seasonally adjusted",
            Self::ReserveAssetsCenBankSdrBasketStPos => "U.S. assets; reserve assets; central bank; in special drawing rights basket; short term; quarterly not seasonally adjusted",
            Self::ReserveAssetsCenBankStPos => "U.S. assets; reserve assets; central bank; short term; quarterly not seasonally adjusted",
            Self::ReserveAssetsChgPosNie => "U.S. assets; reserve assets (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::ReserveAssetsChgPosOth => "U.S. assets; reserve assets (Change in position not attributable to financial-account transactions); annual",
            Self::ReserveAssetsChgPosPrice => "U.S. assets; reserve assets (Change in position attributable to price changes); annual",
            Self::ReserveAssetsChgPosTrans => "U.S. assets; reserve assets (Change in position attributable to financial-account transactions); annual",
            Self::ReserveAssetsChgPosXRate => "U.S. assets; reserve assets (Change in position attributable to exchange-rate changes); annual",
            Self::ReserveAssetsChgPos => "U.S. assets; reserve assets (Change in position); annual",
            Self::ReserveAssetsGenGovtLtPos => "U.S. assets; reserve assets; general government; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsGenGovtNotSdrBasketLtPos => "U.S. assets; reserve assets; general government; not in special drawing rights basket; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsGenGovtNotSdrBasketPos => "U.S. assets; reserve assets; general government; not in special drawing rights basket; quarterly not seasonally adjusted",
            Self::ReserveAssetsGenGovtNotSdrBasketStPos => "U.S. assets; reserve assets; general government; not in special drawing rights basket; short term; quarterly not seasonally adjusted",
            Self::ReserveAssetsGenGovtPos => "U.S. assets; reserve assets; general government; quarterly not seasonally adjusted",
            Self::ReserveAssetsGenGovtSdrBasketLtPos => "U.S. assets; reserve assets; general government; in special drawing rights basket; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsGenGovtSdrBasketPos => "U.S. assets; reserve assets; general government; in special drawing rights basket; quarterly not seasonally adjusted",
            Self::ReserveAssetsGenGovtSdrBasketStPos => "U.S. assets; reserve assets; general government; in special drawing rights basket; short term; quarterly not seasonally adjusted",
            Self::ReserveAssetsGenGovtStPos => "U.S. assets; reserve assets; general government; short term; quarterly not seasonally adjusted",
            Self::ReserveAssetsLtPos => "U.S. assets; reserve assets; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsNotSdrBasketLtPos => "U.S. assets; reserve assets; not in special drawing rights basket; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsNotSdrBasketPos => "U.S. assets; reserve assets; not in special drawing rights basket; quarterly not seasonally adjusted",
            Self::ReserveAssetsNotSdrBasketStPos => "U.S. assets; reserve assets; not in special drawing rights basket; short term; quarterly not seasonally adjusted",
            Self::ReserveAssetsPos => "U.S. assets; reserve assets; annual",
            Self::ReserveAssetsSdrBasketLtPos => "U.S. assets; reserve assets; in special drawing rights basket; long term; quarterly not seasonally adjusted",
            Self::ReserveAssetsSdrBasketPos => "U.S. assets; reserve assets; in special drawing rights basket; quarterly not seasonally adjusted",
            Self::ReserveAssetsSdrBasketStPos => "U.S. assets; reserve assets; in special drawing rights basket; short term; quarterly not seasonally adjusted",
            Self::ReserveAssetsStPos => "U.S. assets; reserve assets; short term; quarterly not seasonally adjusted",
            Self::SdrAllocLiabsChgPosNie => "U.S. liabilities; other investment; special drawing rights allocations (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::SdrAllocLiabsChgPosOth => "U.S. liabilities; other investment; special drawing rights allocations (Change in position not attributable to financial-account transactions); annual",
            Self::SdrAllocLiabsChgPosPrice => "U.S. liabilities; other investment; special drawing rights allocations (Change in position attributable to price changes); annual",
            Self::SdrAllocLiabsChgPosTrans => "U.S. liabilities; other investment; special drawing rights allocations (Change in position attributable to financial-account transactions); annual",
            Self::SdrAllocLiabsChgPosXRate => "U.S. liabilities; other investment; special drawing rights allocations (Change in position attributable to exchange-rate changes); annual",
            Self::SdrAllocLiabsChgPos => "U.S. liabilities; other investment; special drawing rights allocations (Change in position); annual",
            Self::SdrAllocLiabsFoaPos => "U.S. liabilities to foreign official agencies; other investment; special drawing rights allocations; annual",
            Self::SdrAllocLiabsPos => "U.S. liabilities; other investment; special drawing rights allocations; annual",
            Self::SdrReserveAssetsChgPosNie => "U.S. assets; reserve assets; special drawing rights (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::SdrReserveAssetsChgPosOth => "U.S. assets; reserve assets; special drawing rights (Change in position not attributable to financial-account transactions); annual",
            Self::SdrReserveAssetsChgPosPrice => "U.S. assets; reserve assets; special drawing rights (Change in position attributable to price changes); annual",
            Self::SdrReserveAssetsChgPosTrans => "U.S. assets; reserve assets; special drawing rights (Change in position attributable to financial-account transactions); annual",
            Self::SdrReserveAssetsChgPosXRate => "U.S. assets; reserve assets; special drawing rights (Change in position attributable to exchange-rate changes); annual",
            Self::SdrReserveAssetsChgPos => "U.S. assets; reserve assets; special drawing rights (Change in position); annual",
            Self::SdrReserveAssetsPos => "U.S. assets; reserve assets; special drawing rights; annual",
            Self::SecReserveAssetsChgPosNie => "U.S. assets; other reserve assets; securities (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::SecReserveAssetsChgPosOth => "U.S. assets; other reserve assets; securities (Change in position not attributable to financial-account transactions); annual",
            Self::SecReserveAssetsChgPosPrice => "U.S. assets; other reserve assets; securities (Change in position attributable to price changes); annual",
            Self::SecReserveAssetsChgPosTrans => "U.S. assets; other reserve assets; securities (Change in position attributable to financial-account transactions); annual",
            Self::SecReserveAssetsChgPosXRate => "U.S. assets; other reserve assets; securities (Change in position attributable to exchange-rate changes); annual",
            Self::SecReserveAssetsChgPos => "U.S. assets; other reserve assets; securities (Change in position); annual",
            Self::SecReserveAssetsPos => "U.S. assets; other reserve assets; securities; annual",
            Self::StDebtSecAssetsChgPosNie => "U.S. assets; portfolio investment; short-term debt securities (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::StDebtSecAssetsChgPosOth => "U.S. assets; portfolio investment; short-term debt securities (Change in position not attributable to financial-account transactions); annual",
            Self::StDebtSecAssetsChgPosPrice => "U.S. assets; portfolio investment; short-term debt securities (Change in position attributable to price changes); annual",
            Self::StDebtSecAssetsChgPosTrans => "U.S. assets; portfolio investment; short-term debt securities (Change in position attributable to financial-account transactions); annual",
            Self::StDebtSecAssetsChgPosXRate => "U.S. assets; portfolio investment; short-term debt securities (Change in position attributable to exchange-rate changes); annual",
            Self::StDebtSecAssetsChgPos => "U.S. assets; portfolio investment; short-term debt securities (Change in position); annual",
            Self::StDebtSecAssetsPos => "U.S. assets; portfolio investment; short-term debt securities; annual",
            Self::StDebtSecLiabsChgPosNie => "U.S. liabilities; portfolio investment; short-term debt securities (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::StDebtSecLiabsChgPosOth => "U.S. liabilities; portfolio investment; short-term debt securities (Change in position not attributable to financial-account transactions); annual",
            Self::StDebtSecLiabsChgPosPrice => "U.S. liabilities; portfolio investment; short-term debt securities (Change in position attributable to price changes); annual",
            Self::StDebtSecLiabsChgPosTrans => "U.S. liabilities; portfolio investment; short-term debt securities (Change in position attributable to financial-account transactions); annual",
            Self::StDebtSecLiabsChgPosXRate => "U.S. liabilities; portfolio investment; short-term debt securities (Change in position attributable to exchange-rate changes); annual",
            Self::StDebtSecLiabsChgPos => "U.S. liabilities; portfolio investment; short-term debt securities (Change in position); annual",
            Self::StDebtSecLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities; annual",
            Self::StDebtSecLiabsPos => "U.S. liabilities; portfolio investment; short-term debt securities; annual",
            Self::StDebtSecTreasLiabsFoaPos => "U.S. liabilities to foreign official agencies; portfolio investment; short-term debt securities; Treasury bills and certificates; annual",
            Self::TrdCredAndAdvAssetsChgPosNie => "U.S. assets; other investment; trade credit and advances (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::TrdCredAndAdvAssetsChgPosOth => "U.S. assets; other investment; trade credit and advances (Change in position not attributable to financial-account transactions); annual",
            Self::TrdCredAndAdvAssetsChgPosPrice => "U.S. assets; other investment; trade credit and advances (Change in position attributable to price changes); annual",
            Self::TrdCredAndAdvAssetsChgPosTrans => "U.S. assets; other investment; trade credit and advances (Change in position attributable to financial-account transactions); annual",
            Self::TrdCredAndAdvAssetsChgPosXRate => "U.S. assets; other investment; trade credit and advances (Change in position attributable to exchange-rate changes); annual",
            Self::TrdCredAndAdvAssetsChgPos => "U.S. assets; other investment; trade credit and advances (Change in position); annual",
            Self::TrdCredAndAdvAssetsPos => "U.S. assets; other investment; trade credit and advances; annual",
            Self::TrdCredAndAdvLiabsChgPosNie => "U.S. liabilities; other investment; trade credit and advances (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::TrdCredAndAdvLiabsChgPosOth => "U.S. liabilities; other investment; trade credit and advances (Change in position not attributable to financial-account transactions); annual",
            Self::TrdCredAndAdvLiabsChgPosPrice => "U.S. liabilities; other investment; trade credit and advances (Change in position attributable to price changes); annual",
            Self::TrdCredAndAdvLiabsChgPosTrans => "U.S. liabilities; other investment; trade credit and advances (Change in position attributable to financial-account transactions); annual",
            Self::TrdCredAndAdvLiabsChgPosXRate => "U.S. liabilities; other investment; trade credit and advances (Change in position attributable to exchange-rate changes); annual",
            Self::TrdCredAndAdvLiabsChgPos => "U.S. liabilities; other investment; trade credit and advances (Change in position); annual",
            Self::TrdCredAndAdvLiabsFoaPos => "U.S. liabilities to foreign official agencies; other investment; trade credit and advances; annual",
            Self::TrdCredAndAdvLiabsPos => "U.S. liabilities; other investment; trade credit and advances; annual",
            Self::TreasBillsAndCertsLiabsChgPosNie => "U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::TreasBillsAndCertsLiabsChgPosOth => "U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position not attributable to financial-account transactions); annual",
            Self::TreasBillsAndCertsLiabsChgPosPrice => "U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position attributable to price changes); annual",
            Self::TreasBillsAndCertsLiabsChgPosTrans => "U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position attributable to financial-account transactions); annual",
            Self::TreasBillsAndCertsLiabsChgPosXRate => "U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position attributable to exchange-rate changes); annual",
            Self::TreasBillsAndCertsLiabsChgPos => "U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates (Change in position); annual",
            Self::TreasBillsAndCertsLiabsPos => "U.S. liabilities; portfolio investment; short-term debt securities; Treasury bills and certificates; annual",
            Self::TreasBondsAndNotesLiabsChgPosNie => "U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position attributable to changes in volume and valuation n.i.e.); annual",
            Self::TreasBondsAndNotesLiabsChgPosOth => "U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position not attributable to financial-account transactions); annual",
            Self::TreasBondsAndNotesLiabsChgPosPrice => "U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position attributable to price changes); annual",
            Self::TreasBondsAndNotesLiabsChgPosTrans => "U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position attributable to financial-account transactions); annual",
            Self::TreasBondsAndNotesLiabsChgPosXRate => "U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position attributable to exchange-rate changes); annual",
            Self::TreasBondsAndNotesLiabsChgPos => "U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes (Change in position); annual",
            Self::TreasBondsAndNotesLiabsPos => "U.S. liabilities; portfolio investment; long-term debt securities; Treasury bonds and notes; annual",
        }
    }

    /// Converts a time series code string to a TimeSeries variant.
    /// 
    /// The code should be in the format "TSI_Iip{VariantName}_A" or "TSI_Iip{VariantName}_QNSA".
    /// Returns None if the code format is invalid or doesn't match any variant.
    pub fn from_code(code: &str) -> Option<Self> {
        use std::str::FromStr;
        
        let stripped = code.strip_prefix("TSI_Iip")?;
        let stripped = stripped
            .strip_suffix("_A")
            .or_else(|| stripped.strip_suffix("_QNSA"))?;
        
        Self::from_str(stripped).ok()
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct TimeSeries {
    asset: AssetKind,
    class: Option<IipClass>,
    entity: Option<IipEntity>,
    currency: Option<IipCurrency>,
    position: Position,
}

impl TimeSeries {
    /// Attempts to lex a TimeSeries from the input string.
    /// 
    /// The expected format is: {AssetKind}{IipClass?}{IipEntity?}{IipCurrency?}{Position}
    /// 
    /// Returns a tuple of (Option<Self>, remaining_str) where:
    /// - Option<Self> is Some(TimeSeries) if all required components were found, None otherwise
    /// - remaining_str is the unconsumed portion of the input string
    pub fn lex(input: &str) -> (Option<Self>, &str) {
        // Parse AssetKind (required)
        let (asset, remainder) = AssetKind::lex(input);
        let asset = match asset {
            Some(a) => a,
            None => return (None, input),
        };
        
        // Parse IipClass (optional)
        let (class, remainder) = IipClass::lex(remainder);
        
        // Parse IipEntity (optional)
        let (entity, remainder) = IipEntity::lex(remainder);
        
        // Parse IipCurrency (optional)
        let (currency, remainder) = IipCurrency::lex(remainder);
        
        // Parse Position (required) - need to parse from the end
        // For now, try to parse Position from what's left
        let (position, remainder) = Position::lex(remainder);
        let position = match position {
            Some(p) => p,
            None => return (None, input),
        };
        
        // If there's still remainder, the parse wasn't complete
        if !remainder.is_empty() {
            return (None, input);
        }
        
        let series = TimeSeries {
            asset,
            class,
            entity,
            currency,
            position,
        };
        
        (Some(series), remainder)
    }
}

impl FromStr for TimeSeries {
    type Err = Lex;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip the TSI_Iip prefix
        let s = s.strip_prefix("TSI_Iip")
            .ok_or_else(|| {
                let error = LexDetail::new(s.to_owned(), line!(), file!().to_string());
                tracing::debug!("{}", error.to_string());
                Lex::Prefix(error)
            })?;
        
        // Strip the frequency suffix (_A or _QNSA)
        let s = s.strip_suffix("_A")
            .or_else(|| s.strip_suffix("_QNSA"))
            .ok_or_else(|| {
                let error = LexDetail::new(s.to_owned(), line!(), file!().to_string());
                tracing::debug!("{}", error.to_string());
                Lex::Suffix(error)
            })?;
        
        // Lex the remainder
        let (series, remainder) = TimeSeries::lex(s);
        
        match series {
            Some(ts) if remainder.is_empty() => Ok(ts),
            Some(_) => {
                let error = LexDetail::new(remainder.to_owned(), line!(), file!().to_owned());
                tracing::debug!("{}", error.to_string());
                Err(Lex::Incomplete(error))
            },
            None => {
                let error = LexDetail::new(s.to_owned(), line!(), file!().to_owned());
                tracing::debug!("{}", error.to_string());
                Err(Lex::Empty(error))
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("Lex error parsing {input} at line {line} in {file}")]
pub struct LexDetail {
    input: String,
    line: u32,
    file: String,
}

impl std::error::Error for LexDetail {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
pub enum Lex {
    Prefix(LexDetail),
    Suffix(LexDetail),
    Incomplete(LexDetail),
    Empty(LexDetail),
}

impl std::error::Error for Lex {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Prefix(detail) => Some(detail),
            Self::Suffix(detail) => Some(detail),
            Self::Incomplete(detail) => Some(detail),
            Self::Empty(detail) => Some(detail),
        }
    }
}
