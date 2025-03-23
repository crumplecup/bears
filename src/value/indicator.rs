use crate::{BeaErr, KeyMissing, ParameterFields, ParameterValueTable, ParameterValueTableVariant};
use std::str::FromStr;

/// Valid values for the Indicator parameter.
#[derive(
    Debug,
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
pub enum Indicator {
    /// Capital account balance
    BalCapAcct,
    /// Current account balance
    BalCurrAcct,
    /// Balance on goods
    BalGds,
    /// Balance on goods and services
    BalGdsServ,
    /// Balance on primary income
    BalPrimInc,
    /// Balance on secondary income
    BalSecInc,
    /// Balance on services
    BalServ,
    /// Capital transfers; payments and other debt
    CapTransPayAndOthDeb,
    /// Capital transfers; receipts and other credit
    CapTransRecAndOthCred,
    /// Compensation of employees; payments
    CompOfEmplPay,
    /// Compensation of employees; receipts
    CompOfEmplRec,
    /// Currency and deposits; assets
    CurrAndDepAssets,
    /// Currency and deposits; assets; central bank
    CurrAndDepAssetsCentralBank,
    /// Currency and deposits; assets; deposit-taking institutions
    CurrAndDepAssetsDepTaking,
    /// Currency and deposits; assets; other financial and nonfinancial institutions
    CurrAndDepAssetsOthFinNonFin,
    /// Currency and deposits; liabilities
    CurrAndDepLiabs,
    /// Currency and deposits; liabilities; central bank
    CurrAndDepLiabsCentralBank,
    /// Currency and deposits; liabilities; deposit-taking institutions
    CurrAndDepLiabsDepTaking,
    /// Currency and deposits; liabilities; financial account
    CurrAndDepLiabsFoa,
    /// Currency and deposits; liabilities; other financial and nonfinancial institutions
    CurrAndDepLiabsOthFinNonFin,
    /// Currency and deposits; reserve assets
    CurrAndDepReserveAssets,
    /// Current assets
    CurrAssets,
    /// Current liabilities
    CurrLiabs,
    /// Current liabilities; central bank
    CurrLiabsCentralBank,
    /// Debt securities; assets
    DebtSecAssets,
    /// Debt securities; assets; deposit-taking institutions
    DebtSecAssetsDepTaking,
    /// Debt securities; assets; nonfinancial institutions
    DebtSecAssetsNonFin,
    /// Debt securities; assets; other financial institutions
    DebtSecAssetsOthFin,
    /// Debt securities; federal-sponsored agency income payments
    DebtSecFedSponsorAgencyIncPay,
    /// Debt securities; federal-sponsored agency liabilities
    DebtSecFedSponsorAgencyLiabs,
    /// Debt securities; income payments
    DebtSecIncPay,
    /// Debt securities; income payments; deposit-taking institutions
    DebtSecIncPayDepTaking,
    /// Debt securities; income payments; general government
    DebtSecIncPayGenGovt,
    /// Debt securities; income payments; nonfinancial institutions
    DebtSecIncPayNonFin,
    /// Debt securities; income payments; other financial institutions
    DebtSecIncPayOthFin,
    /// Debt securities; income receipts
    DebtSecIncRec,
    /// Debt securities; income receipts; deposit-taking institutions
    DebtSecIncRecDepTaking,
    /// Debt securities; income receipts; nonfinancial institutions
    DebtSecIncRecNonFin,
    /// Debt securities; income receipts; other financial institutions
    DebtSecIncRecOthFin,
    /// Debt securities; liabilities
    DebtSecLiabs,
    /// Debt securities; liabilities; deposit-taking institutions
    DebtSecLiabsDepTaking,
    /// Debt securities; liabilities; financial account
    DebtSecLiabsFoa,
    /// Debt securities; liabilities; general government
    DebtSecLiabsGenGovt,
    /// Debt securities; liabilities; nonfinancial institutions
    DebtSecLiabsNonFin,
    /// Debt securities; liabilities; other financial institutions
    DebtSecLiabsOthFin,
    /// Debt securities; other than federal-sponsored agency income payments; other financial institutions
    DebtSecOthThanFedSponsorAgencyIncPayOthFin,
    /// Debt securities; other than federal-sponsored agency liabilities; other financial institutions
    DebtSecOthThanFedSponsorAgencyLiabsOthFin,
    /// Debt securities; Treasury income payments
    DebtSecTreasIncPay,
    /// Debt securities; Treasury liabilities
    DebtSecTreasLiabs,
    /// Deposits; assets
    DepAssets,
    /// Deposits; assets; central bank
    DepAssetsCentralBank,
    /// Deposits; assets; deposit-taking institutions
    DepAssetsDepTaking,
    /// Deposits; assets; other financial and nonfinancial institutions
    DepAssetsOthFinNonFin,
    /// Deposits; liabilities
    DepLiabs,
    /// Deposits; liabilities; central bank
    DepLiabsCentralBank,
    /// Deposits; liabilities; deposit-taking institutions
    DepLiabsDepTaking,
    /// Deposits; liabilities; other financial and nonfinancial institutions
    DepLiabsOthFinNonFin,
    /// Deposits; repurchase liabilities; deposit-taking institutions
    DepRepurchaseLiabsDepTaking,
    /// Deposits; resale assets; deposit-taking institutions
    DepResaleAssetsDepTaking,
    /// Direct investment; assets
    DiInvAssets,
    /// Direct investment; assets; non-speculative
    DiInvAssetsNonSpe,
    /// Direct investment; assets; speculative
    DiInvAssetsSpe,
    /// Direct investment; current cost adjustments; assets
    DiInvCurrCostAdjAssets,
    /// Direct investment; current cost adjustments; income payments
    DiInvCurrCostAdjIncPay,
    /// Direct investment; current cost adjustments; income receipts
    DiInvCurrCostAdjIncRec,
    /// Direct investment; current cost adjustments; liabilities
    DiInvCurrCostAdjLiabs,
    /// Direct investment; debt instruments; assets
    DiInvDebtInstAssets,
    /// Direct investment; debt instruments; assets; non-speculative
    DiInvDebtInstAssetsNonSpe,
    /// Direct investment; debt instruments; assets; speculative
    DiInvDebtInstAssetsSpe,
    /// Direct investment; debt instruments; inward
    DiInvDebtInstInward,
    /// Direct investment; debt instruments; inward; by industry
    DiInvDebtInstInwardByInd,
    /// Direct investment; debt instruments; inward; finance and insurance
    DiInvDebtInstInwardFinAndIns,
    /// Direct investment; debt instruments; inward; manufacturing
    DiInvDebtInstInwardMnfctr,
    /// Direct investment; debt instruments; inward; other industries
    DiInvDebtInstInwardOthInd,
    /// Direct investment; debt instruments; inward; wholesale trade
    DiInvDebtInstInwardWhlslTrd,
    /// Direct investment; debt instruments; liabilities
    DiInvDebtInstLiabs,
    /// Direct investment; debt instruments; liabilities; non-speculative
    DiInvDebtInstLiabsNonSpe,
    /// Direct investment; debt instruments; liabilities; speculative
    DiInvDebtInstLiabsSpe,
    /// Direct investment; debt instruments; outward
    DiInvDebtInstOutward,
    /// Direct investment; debt instruments; outward; by industry
    DiInvDebtInstOutwardByInd,
    /// Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; finance (including depository institutions) and insurance
    DiInvDebtInstOutwardFinAndIns,
    /// Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; holding companies except bank holding companies
    DiInvDebtInstOutwardHoldExcBank,
    /// Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; manufacturing
    DiInvDebtInstOutwardMnfctr,
    /// Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; other industries (those not listed under acquisition of assets in table 6.1)
    DiInvDebtInstOutwardOthInd,
    /// Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; wholesale trade
    DiInvDebtInstOutwardWhlslTrd,
    /// Financial transactions for direct investment; debt instruments; U.S. affiliates' claims
    DiInvDebtInstUsAffiliatesClaims,
    /// Financial transactions for direct investment; U.S. non-SPE affiliates' debt claims transactions with their foreign parent groups
    DiInvDebtInstUsAffiliatesClaimsByNonSpe,
    /// Financial transactions for direct investment; U.S. SPE affiliates' debt claims transactions with their foreign parent groups
    DiInvDebtInstUsAffiliatesClaimsBySpe,
    /// Financial transactions for direct investment; debt instruments; U.S. affiliates' claims; Non-SPEs
    DiInvDebtInstUsAffiliatesClaimsNonSpe,
    /// Financial transactions for direct investment; debt instruments; U.S. affiliates' claims; SPEs
    DiInvDebtInstUsAffiliatesClaimsSpe,
    /// Financial transactions for direct investment; debt instruments; U.S. affiliates' liabilities
    DiInvDebtInstUsAffiliatesLiabs,
    /// Financial transactions for direct investment; debt instruments; U.S. affiliates' liabilities; Non-SPEs
    DiInvDebtInstUsAffiliatesLiabsNonSpe,
    /// Financial transactions for direct investment; debt instruments; U.S. affiliates' liabilities; SPEs
    DiInvDebtInstUsAffiliatesLiabsSpe,
    /// Financial transactions for direct investment; debt instruments; U.S. parents' claims
    DiInvDebtInstUsParentsClaims,
    /// Financial transactions for direct investment; debt instruments; U.S. parents' claims; Non-SPEs
    DiInvDebtInstUsParentsClaimsNonSpe,
    /// Financial transactions for direct investment; debt instruments; U.S. parents' claims; SPEs
    DiInvDebtInstUsParentsClaimsSpe,
    /// Financial transactions for direct investment; debt instruments; U.S. parents' liabilities
    DiInvDebtInstUsParentsLiabs,
    /// Financial transactions for direct investment; debt instruments; U.S. parents' liabilities; Non-SPEs
    DiInvDebtInstUsParentsLiabsNonSpe,
    /// Financial transactions for direct investment; debt instruments; U.S. parents' liabilities; SPEs
    DiInvDebtInstUsParentsLiabsSpe,
    /// Financial transactions for direct investment; U.S. parents' debt liabilities transactions with their foreign non-SPE affiliates
    DiInvDebtInstUsParentsLiabsWithNonSpe,
    /// Financial transactions for direct investment; U.S. parents' debt liabilities transactions with their foreign SPE affiliates
    DiInvDebtInstUsParentsLiabsWithSpe,
    /// Financial transactions for direct investment; adjustments to convert to directional basis
    DiInvDirectionalBasisAdj,
    /// Direct investment income payments; adjustments to convert to directional basis
    DiInvDirectionalBasisAdjIncPay,
    /// Direct investment income receipts; adjustments to convert to directional basis
    DiInvDirectionalBasisAdjIncRec,
    /// Direct investment income on liabilities; dividends and withdrawals
    DiInvDivWithdrawIncPay,
    /// Direct investment income on liabilities; dividends and withdrawals; Non-SPEs
    DiInvDivWithdrawIncPayNonSpe,
    /// Direct investment income on liabilities; dividends and withdrawals; SPEs
    DiInvDivWithdrawIncPaySpe,
    /// Direct investment income on assets; dividends and withdrawals
    DiInvDivWithdrawIncRec,
    /// Direct investment income on assets; dividends and withdrawals; Non-SPEs
    DiInvDivWithdrawIncRecNonSpe,
    /// Direct investment income on assets; dividends and withdrawals; SPEs
    DiInvDivWithdrawIncRecSpe,
    /// Net U.S. acquisition of direct investment assets; equity
    DiInvEquityAssets,
    /// Net U.S. acquisition of direct investment assets; equity; Non-SPEs
    DiInvEquityAssetsNonSpe,
    /// Net U.S. acquisition of direct investment assets; equity; SPEs
    DiInvEquityAssetsSpe,
    /// Direct investment income on liabilities; equity
    DiInvEquityIncPay,
    /// Direct investment income on liabilities; equity; Non-SPEs
    DiInvEquityIncPayNonSpe,
    /// Direct investment income on liabilities; equity; SPEs
    DiInvEquityIncPaySpe,
    /// Direct investment income on assets; equity
    DiInvEquityIncRec,
    /// Direct investment income on assets; equity; Non-SPEs
    DiInvEquityIncRecNonSpe,
    /// Direct investment income on assets; equity; SPEs
    DiInvEquityIncRecSpe,
    /// Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; finance (including depository institutions) and insurance
    DiInvEquityIntIncRecFinAndIns,
    /// Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; holding companies except bank holding companies
    DiInvEquityIntIncRecHoldExcBank,
    /// Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; manufacturing
    DiInvEquityIntIncRecMnfctr,
    /// Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; manufacturing; other industries (those not listed under receipts in table 4.2)
    DiInvEquityIntIncRecOthInd,
    /// Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; wholesale trade
    DiInvEquityIntIncRecWhlslTrd,
    /// Net U.S. incurrence of direct investment liabilities; equity
    DiInvEquityLiabs,
    /// Net U.S. incurrence of direct investment liabilities; equity; Non-SPEs
    DiInvEquityLiabsNonSpe,
    /// Net U.S. incurrence of direct investment liabilities; equity; SPEs
    DiInvEquityLiabsSpe,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings
    DiInvEquityOthThanReinvestEarnAssets,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; finance (including depository institutions) and insurance
    DiInvEquityOthThanReinvestEarnAssetsFinAndIns,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; holding companies except bank holding companies
    DiInvEquityOthThanReinvestEarnAssetsHoldExcBank,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; manufacturing
    DiInvEquityOthThanReinvestEarnAssetsMnfctr,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; Non-SPEs
    DiInvEquityOthThanReinvestEarnAssetsNonSpe,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; other industries (those not listed under acquisition of assets in table 6.1)
    DiInvEquityOthThanReinvestEarnAssetsOthInd,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; SPEs
    DiInvEquityOthThanReinvestEarnAssetsSpe,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; wholesale trade
    DiInvEquityOthThanReinvestEarnAssetsWhlslTrd,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; decreases
    DiInvEquityOthThanReinvestEarnDecAssets,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; decreases
    DiInvEquityOthThanReinvestEarnDecLiabs,
    /// Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; increases
    DiInvEquityOthThanReinvestEarnIncAssets,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; increases
    DiInvEquityOthThanReinvestEarnIncLiabs,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings
    DiInvEquityOthThanReinvestEarnLiabs,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; finance (including depository institutions) and insurance
    DiInvEquityOthThanReinvestEarnLiabsFinAndIns,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; other industries (those not listed under incurrence of liabilities in table 6.1)
    DiInvEquityOthThanReinvestEarnLiabsHoldExcBank,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; manufacturing
    DiInvEquityOthThanReinvestEarnLiabsMnfctr,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; Non-SPEs
    DiInvEquityOthThanReinvestEarnLiabsNonSpe,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; SPEs
    DiInvEquityOthThanReinvestEarnLiabsSpe,
    /// Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; wholesale trade
    DiInvEquityOthThanReinvestEarnLiabsWhlslTrd,
    /// Direct investment income without current cost adjustment on liabilities; equity
    DiInvEquityWithoutCurrCostAdjIncPay,
    /// Direct investment income without current cost adjustment on liabilities; equity; finance (including depository institutions) and insurance
    DiInvEquityWithoutCurrCostAdjIncPayFinAndIns,
    /// Direct investment income without current cost adjustment on liabilities; equity; manufacturing
    DiInvEquityWithoutCurrCostAdjIncPayMnfctr,
    /// Direct investment income without current cost adjustment on liabilities; equity; other industries (those not listed under receipts in table 4.2)
    DiInvEquityWithoutCurrCostAdjIncPayOthInd,
    /// Direct investment income without current cost adjustment on liabilities; equity; wholesale trade
    DiInvEquityWithoutCurrCostAdjIncPayWhlslTrd,
    /// Direct investment income without current cost adjustment on assets; equity
    DiInvEquityWithoutCurrCostAdjIncRec,
    /// Direct investment income without current cost adjustment on assets; equity; finance (including depository institutions) and insurance
    DiInvEquityWithoutCurrCostAdjIncRecFinAndIns,
    /// Direct investment income without current cost adjustment on assets; equity; holding companies except bank holding companies
    DiInvEquityWithoutCurrCostAdjIncRecHoldExcBank,
    /// Direct investment income without current cost adjustment on assets; equity; manufacturing
    DiInvEquityWithoutCurrCostAdjIncRecMnfctr,
    /// Direct investment income without current cost adjustment on assets; equity; manufacturing; other industries (those not listed under receipts in table 4.2)
    DiInvEquityWithoutCurrCostAdjIncRecOthInd,
    /// Direct investment income without current cost adjustment on assets; equity; wholesale trade
    DiInvEquityWithoutCurrCostAdjIncRecWhlslTrd,
    /// Direct investment income on liabilities, asset/liability basis
    DiInvIncPay,
    /// Direct investment income on liabilities, asset/liability basis; Non-SPEs
    DiInvIncPayNonSpe,
    /// Direct investment income on liabilities, asset/liability basis; SPEs
    DiInvIncPaySpe,
    /// Direct investment income on assets, asset/liability basis
    DiInvIncRec,
    /// Direct investment income on assets, asset/liability basis; Non-SPEs
    DiInvIncRecNonSpe,
    /// Direct investment income on assets, asset/liability basis; SPEs
    DiInvIncRecSpe,
    /// Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments
    DiInvIntIncInward,
    /// Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments; finance (including depository institutions) and insurance
    DiInvIntIncInwardFinAndIns,
    /// Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments; manufacturing
    DiInvIntIncInwardMnfctr,
    /// Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments; other industries (those not listed under receipts in table 4.2)
    DiInvIntIncInwardOthInd,
    /// Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments; wholesale trade
    DiInvIntIncInwardWhlslTrd,
    /// Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts
    DiInvIntIncOutward,
    /// Direct investment income on liabilities, asset/liability basis; interest
    DiInvIntIncPay,
    /// Direct investment income on liabilities, asset/liability basis; interest; Non-SPEs
    DiInvIntIncPayNonSpe,
    /// Direct investment income on liabilities, asset/liability basis; interest; SPEs
    DiInvIntIncPaySpe,
    /// Direct investment income on assets, asset/liability basis; interest
    DiInvIntIncRec,
    /// Direct investment income on assets, asset/liability basis; interest; Non-SPEs
    DiInvIntIncRecNonSpe,
    /// Direct investment income on assets, asset/liability basis; interest; SPEs
    DiInvIntIncRecSpe,
    /// Direct investment income; U.S. affiliates' interest payments
    DiInvIntUsAffiliatesIncPay,
    /// Direct investment income; U.S. affiliates' interest payments; Non-SPEs
    DiInvIntUsAffiliatesIncPayNonSpe,
    /// Direct investment income; U.S. affiliates' interest payments; SPEs
    DiInvIntUsAffiliatesIncPaySpe,
    /// Direct investment income; U.S. affiliates' interest receipts
    DiInvIntUsAffiliatesIncRec,
    /// Direct investment income; U.S. affiliates' receipts from their foreign parent groups; Non-SPEs
    DiInvIntUsAffiliatesIncRecNonSpe,
    /// Direct investment income; U.S. non-SPE affiliates' interest receipts from their foreign parent groups
    DiInvIntUsAffiliatesIncRecOfNonSpe,
    /// Direct investment income; U.S. non-SPE affiliates' interest receipts from their foreign parent groups
    DiInvIntUsAffiliatesIncRecOfSpe,
    /// Direct investment income; U.S. affiliates' receipts from their foreign parent groups; SPEs
    DiInvIntUsAffiliatesIncRecSpe,
    /// Direct investment income; U.S. parents' interest payments
    DiInvIntUsParentsIncPay,
    /// Direct investment income; U.S. parents' payments to their foreign affiliates; Non-SPEs
    DiInvIntUsParentsIncPayNonSpe,
    /// Direct investment income; U.S. parents' payments to their foreign affiliates; SPEs
    DiInvIntUsParentsIncPaySpe,
    /// Direct investment income; U.S. parents' interest payments to their foreign non-SPE affiliates
    DiInvIntUsParentsIncPayToNonSpe,
    /// Direct investment income; U.S. parents' interest payments to their foreign SPE affiliates
    DiInvIntUsParentsIncPayToSpe,
    /// Direct investment income; U.S. parents' interest receipts
    DiInvIntUsParentsIncRec,
    /// Direct investment income; U.S. parents' interest receipts; Non-SPEs
    DiInvIntUsParentsIncRecNonSpe,
    /// Direct investment income; U.S. parents' interest receipts; SPEs
    DiInvIntUsParentsIncRecSpe,
    /// Financial transactions for inward direct investment (foreign direct investment in the United States), directional basis
    DiInvInwardDirectionalBasis,
    /// Net U.S. incurrence of direct investment liabilities, asset/liability basis
    DiInvLiabs,
    /// Net U.S. incurrence of direct investment liabilities, asset/liability basis; Non-SPEs
    DiInvLiabsNonSpe,
    /// Net U.S. incurrence of direct investment liabilities, asset/liability basis; SPEs
    DiInvLiabsSpe,
    /// Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis
    DiInvOutward,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings
    DiInvReinvestEarnAssets,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings; Non-SPEs
    DiInvReinvestEarnAssetsNonSpe,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings; SPEs
    DiInvReinvestEarnAssetsSpe,
    /// Direct investment income on liabilities; reinvested earnings
    DiInvReinvestEarnIncPay,
    /// Direct investment income on liabilities; reinvested earnings; Non-SPEs
    DiInvReinvestEarnIncPayNonSpe,
    /// Direct investment income on liabilities; reinvested earnings; SPEs
    DiInvReinvestEarnIncPaySpe,
    /// Direct investment income on assets; reinvested earnings
    DiInvReinvestEarnIncRec,
    /// Direct investment income on assets; reinvested earnings; Non-SPEs
    DiInvReinvestEarnIncRecNonSpe,
    /// Direct investment income on assets; reinvested earnings; SPEs
    DiInvReinvestEarnIncRecSpe,
    /// Net U.S. incurrence of direct investment liabilities; reinvestment of earnings
    DiInvReinvestEarnLiabs,
    /// Net U.S. incurrence of direct investment liabilities; reinvestment of earnings; Non-SPEs
    DiInvReinvestEarnLiabsNonSpe,
    /// Net U.S. incurrence of direct investment liabilities; reinvestment of earnings; SPEs
    DiInvReinvestEarnLiabsSpe,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment
    DiInvReinvestEarnWithoutCurrCostAdjAssets,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; finance (including depository institutions) and insurance
    DiInvReinvestEarnWithoutCurrCostAdjAssetsFinAndIns,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; holding companies except bank holding companies
    DiInvReinvestEarnWithoutCurrCostAdjAssetsHoldExcBank,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; manufacturing
    DiInvReinvestEarnWithoutCurrCostAdjAssetsMnfctr,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; other industries (those not listed under acquisition of assets in table 6.1)
    DiInvReinvestEarnWithoutCurrCostAdjAssetsOthInd,
    /// Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; wholesale trade
    DiInvReinvestEarnWithoutCurrCostAdjAssetsWhlslTrd,
    /// Direct investment income on liabilities; reinvested earnings without current-cost adjustment
    DiInvReinvestEarnWithoutCurrCostAdjIncPay,
    /// Direct investment income on assets; reinvested earnings without current-cost adjustment
    DiInvReinvestEarnWithoutCurrCostAdjIncRec,
    /// Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment
    DiInvReinvestEarnWithoutCurrCostAdjLiabs,
    /// Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment; finance (including depository institutions) and insurance
    DiInvReinvestEarnWithoutCurrCostAdjLiabsFinAndIns,
    /// Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment; manufacturing
    DiInvReinvestEarnWithoutCurrCostAdjLiabsMnfctr,
    /// Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment; other industries (those not listed under incurrence of liabilities in table 6.1)
    DiInvReinvestEarnWithoutCurrCostAdjLiabsOthInd,
    /// Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment; wholesale trade
    DiInvReinvestEarnWithoutCurrCostAdjLiabsWhlslTrd,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis
    DiInvWithoutCurrCostAdjIncInward,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; chemicals
    DiInvWithoutCurrCostAdjIncInwardChem,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; computers and electronic products
    DiInvWithoutCurrCostAdjIncInwardCompElecProd,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; depository institutions
    DiInvWithoutCurrCostAdjIncInwardDepIns,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; electrical equipment, appliances, and components
    DiInvWithoutCurrCostAdjIncInwardElectrical,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; finance (including depository institutions) and insurance
    DiInvWithoutCurrCostAdjIncInwardFinAndIns,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; finance (including depository institutions) and insurance
    DiInvWithoutCurrCostAdjIncInwardFinExclDepInsAndIns,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; food
    DiInvWithoutCurrCostAdjIncInwardFood,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; information
    DiInvWithoutCurrCostAdjIncInwardInfo,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; machinery
    DiInvWithoutCurrCostAdjIncInwardMachinery,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; manufacturing
    DiInvWithoutCurrCostAdjIncInwardMnfctr,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; other manufacturing
    DiInvWithoutCurrCostAdjIncInwardMnfctrOth,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; other industries (those not listed under receipts in table 4.2)
    DiInvWithoutCurrCostAdjIncInwardOthInd,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; other industries (excluding 8 industry groups)
    DiInvWithoutCurrCostAdjIncInwardOthIndExcl8DiInward,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; primary and fabricated metals
    DiInvWithoutCurrCostAdjIncInwardPrimFabMtls,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; professional, scientific, and technical services
    DiInvWithoutCurrCostAdjIncInwardProfSciAndTech,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; real estate and rental and leasing
    DiInvWithoutCurrCostAdjIncInwardRealEstRentLeas,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; retail trade
    DiInvWithoutCurrCostAdjIncInwardRtlTrd,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; transportation equipment
    DiInvWithoutCurrCostAdjIncInwardTransEquip,
    /// Direct investment income without current-cost adjustment on inward investment, directional basis; wholesale trade
    DiInvWithoutCurrCostAdjIncInwardWhlslTrd,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis
    DiInvWithoutCurrCostAdjIncOutward,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; chemicals
    DiInvWithoutCurrCostAdjIncOutwardChem,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; computers and electronic products
    DiInvWithoutCurrCostAdjIncOutwardCompElecProd,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; depository institutions
    DiInvWithoutCurrCostAdjIncOutwardDepIns,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; electrical equipment, appliances, and components
    DiInvWithoutCurrCostAdjIncOutwardElectrical,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; finance (including depository institutions) and insurance
    DiInvWithoutCurrCostAdjIncOutwardFinAndIns,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; finance (including depository institutions) and insurance
    DiInvWithoutCurrCostAdjIncOutwardFinExclDepInsAndIns,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; food
    DiInvWithoutCurrCostAdjIncOutwardFood,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; holding companies except bank holding companies
    DiInvWithoutCurrCostAdjIncOutwardHoldExcBank,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; information
    DiInvWithoutCurrCostAdjIncOutwardInfo,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; machinery
    DiInvWithoutCurrCostAdjIncOutwardMachinery,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; mining
    DiInvWithoutCurrCostAdjIncOutwardMining,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; manufacturing
    DiInvWithoutCurrCostAdjIncOutwardMnfctr,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; other manufacturing
    DiInvWithoutCurrCostAdjIncOutwardMnfctrOth,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; manufacturing; other industries (those not listed under receipts in table 4.2)
    DiInvWithoutCurrCostAdjIncOutwardOthInd,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; other industries (excluding 8 industry groups)
    DiInvWithoutCurrCostAdjIncOutwardOthIndExcl8DiOutward,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; primary and fabricated metals
    DiInvWithoutCurrCostAdjIncOutwardPrimFabMtls,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; professional, scientific, and technical services
    DiInvWithoutCurrCostAdjIncOutwardProfSciAndTech,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; transportation equipment
    DiInvWithoutCurrCostAdjIncOutwardTransEquip,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; wholesale trade
    DiInvWithoutCurrCostAdjIncOutwardWhlslTrd,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis
    DiInvWithoutCurrCostAdjInward,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; chemicals
    DiInvWithoutCurrCostAdjInwardChem,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; computers and electronic products
    DiInvWithoutCurrCostAdjInwardCompElecProd,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; depository institutions
    DiInvWithoutCurrCostAdjInwardDepIns,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; electrical equipment, appliances, and components
    DiInvWithoutCurrCostAdjInwardElectrical,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; finance (including depository institutions) and insurance
    DiInvWithoutCurrCostAdjInwardFinAndIns,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; finance (including depository institutions) and insurance
    DiInvWithoutCurrCostAdjInwardFinExclDepInsAndIns,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; food
    DiInvWithoutCurrCostAdjInwardFood,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; information
    DiInvWithoutCurrCostAdjInwardInfo,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; machinery
    DiInvWithoutCurrCostAdjInwardMachinery,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; manufacturing
    DiInvWithoutCurrCostAdjInwardMnfctr,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; other manufacturing
    DiInvWithoutCurrCostAdjInwardMnfctrOth,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; other industries (those not listed under incurrence of liabilities in table 6.1)
    DiInvWithoutCurrCostAdjInwardOthInd,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; other industries (excluding 8 industry groups)
    DiInvWithoutCurrCostAdjInwardOthIndExcl8DiInward,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; primary and fabricated metals
    DiInvWithoutCurrCostAdjInwardPrimFabMtls,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; professional, scientific, and technical services
    DiInvWithoutCurrCostAdjInwardProfSciAndTech,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; real estate and rental and leasing
    DiInvWithoutCurrCostAdjInwardRealEstRentLeas,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; retail trade
    DiInvWithoutCurrCostAdjInwardRtlTrd,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; transportation equipment
    DiInvWithoutCurrCostAdjInwardTransEquip,
    /// Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; wholesale trade
    DiInvWithoutCurrCostAdjInwardWhlslTrd,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis
    DiInvWithoutCurrCostAdjOutward,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; chemicals
    DiInvWithoutCurrCostAdjOutwardChem,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; computers and electronic products
    DiInvWithoutCurrCostAdjOutwardCompElecProd,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; depository institutions
    DiInvWithoutCurrCostAdjOutwardDepIns,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; electrical equipment, appliances, and components
    DiInvWithoutCurrCostAdjOutwardElectrical,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; finance (including depository institutions) and insurance
    DiInvWithoutCurrCostAdjOutwardFinAndIns,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; finance (including depository institutions) and insurance
    DiInvWithoutCurrCostAdjOutwardFinExclDepInsAndIns,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; food
    DiInvWithoutCurrCostAdjOutwardFood,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; holding companies except bank holding companies
    DiInvWithoutCurrCostAdjOutwardHoldExcBank,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; information
    DiInvWithoutCurrCostAdjOutwardInfo,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; machinery
    DiInvWithoutCurrCostAdjOutwardMachinery,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; mining
    DiInvWithoutCurrCostAdjOutwardMining,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; manufacturing
    DiInvWithoutCurrCostAdjOutwardMnfctr,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; other manufacturing
    DiInvWithoutCurrCostAdjOutwardMnfctrOth,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; other industries (those not listed under acquisition of assets in table 6.1)
    DiInvWithoutCurrCostAdjOutwardOthInd,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; other industries (excluding 8 industry groups)
    DiInvWithoutCurrCostAdjOutwardOthIndExcl8DiOutward,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; primary and fabricated metals
    DiInvWithoutCurrCostAdjOutwardPrimFabMtls,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; professional, scientific, and technical services
    DiInvWithoutCurrCostAdjOutwardProfSciAndTech,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; transportation equipment
    DiInvWithoutCurrCostAdjOutwardTransEquip,
    /// Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; wholesale trade
    DiInvWithoutCurrCostAdjOutwardWhlslTrd,
    /// Net U.S. acquisition of portfolio investment assets; equity and investment fund shares
    EquityAndInvFundSharesAssets,
    /// Net U.S. acquisition of portfolio investment assets; equity and investment fund shares; held by deposit-taking institutions except central bank
    EquityAndInvFundSharesAssetsDepTaking,
    /// Net U.S. acquisition of portfolio investment assets; equity and investment fund shares; held by nonfinancial institutions except general government
    EquityAndInvFundSharesAssetsNonFin,
    /// Net U.S. acquisition of portfolio investment assets; equity and investment fund shares; held by non-deposit-taking financial institutions
    EquityAndInvFundSharesAssetsOthFin,
    /// Portfolio investment income payments; income on equity and investment fund shares
    EquityAndInvFundSharesIncPay,
    /// Portfolio investment income payments on equity and investment fund shares; deposit-taking institutions except central bank
    EquityAndInvFundSharesIncPayDepTaking,
    /// Portfolio investment income payments on equity and investment fund shares; nonfinancial institutions except general government
    EquityAndInvFundSharesIncPayNonFin,
    /// Portfolio investment income payments on equity and investment fund shares; non-deposit-taking financial institutions
    EquityAndInvFundSharesIncPayOthFin,
    /// Portfolio investment income receipts; income on equity and investment fund shares
    EquityAndInvFundSharesIncRec,
    /// Portfolio investment income receipts on equity and investment fund shares; deposit-taking institutions except central bank
    EquityAndInvFundSharesIncRecDepTaking,
    /// Portfolio investment income receipts on equity and investment fund shares; nonfinancial institutions except general government
    EquityAndInvFundSharesIncRecNonFin,
    /// Portfolio investment income receipts on equity and investment fund shares; non-deposit-taking financial institutions
    EquityAndInvFundSharesIncRecOthFin,
    /// Net U.S. incurrence of portfolio investment liabilities; equity and investment fund shares
    EquityAndInvFundSharesLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities; equity and investment fund shares; issued by deposit-taking institutions except central bank
    EquityAndInvFundSharesLiabsDepTaking,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; equity and investment fund shares
    EquityAndInvFundSharesLiabsFoa,
    /// Net U.S. incurrence of portfolio investment liabilities; equity and investment fund shares; issued by nonfinancial institutions except general government
    EquityAndInvFundSharesLiabsNonFin,
    /// Net U.S. incurrence of portfolio investment liabilities; equity and investment fund shares; issued by non-deposit-taking financial institutions
    EquityAndInvFundSharesLiabsOthFin,
    /// Net U.S. acquisition of portfolio investment assets; equity other than investment fund shares
    EquityOthThanInvFundSharesAssets,
    /// Portfolio investment income payments; dividends on equity other than investment fund shares
    EquityOthThanInvFundSharesIncPay,
    /// Portfolio investment income receipts; dividends on equity other than investment fund shares
    EquityOthThanInvFundSharesIncRec,
    /// Net U.S. incurrence of portfolio investment liabilities; equity other than investment fund shares
    EquityOthThanInvFundSharesLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; equity excluding investment fund shares
    EquityOthThanInvFundSharesLiabsFoa,
    /// Exports of goods
    ExpGds,
    /// Exports of agricultural foods, feeds, and beverages
    ExpGdsAgFoodsFeedsAndBevs,
    /// Exports of agricultural industrial supplies and materials
    ExpGdsAgIsm,
    /// Exports of apparel, footwear, and household goods
    ExpGdsAppFootAndHouse,
    /// Exports of automotive engines and engine parts
    ExpGdsAutoEngAndEngParts,
    /// Exports of automotive vehicles, parts, and engines
    ExpGdsAutoVehPartsAndEngines,
    /// Exports of bauxite and aluminum
    ExpGdsBauxAndAlum,
    /// Exports of goods; balance of payments adjustments, net
    ExpGdsBopAdj,
    /// Exports of building materials except metals
    ExpGdsBuildMatsExcMetals,
    /// Exports of capital goods except automotive
    ExpGdsCapGoodsExclAuto,
    /// Exports of goods, Census basis
    ExpGdsCensus,
    /// Exports of chemicals except medicinals
    ExpGdsChemsExcMeds,
    /// Exports of civilian aircraft, complete, all types
    ExpGdsCivAir,
    /// Exports of civilian aircraft, engines, and parts
    ExpGdsCivAirEngAndParts,
    /// Exports of coal and related products
    ExpGdsCoalAndRelProds,
    /// Exports of computers
    ExpGdsComp,
    /// Exports of computer accessories, peripherals, and parts
    ExpGdsCompAccPeriAndParts,
    /// Exports of consumer goods except food and automotive
    ExpGdsConsGoodsExcFoodAndAuto,
    /// Exports of copper
    ExpGdsCopper,
    /// Exports of corn
    ExpGdsCorn,
    /// Exports of crude petroleum
    ExpGdsCrudePet,
    /// Exports of distilled beverages and other nonagricultural foods, feeds, and beverages
    ExpGdsDistBevAndOthNonAgFoodsFeedsAndBevs,
    /// Exports of durable consumer goods
    ExpGdsDurCons,
    /// Exports of electric-generating machinery, electric apparatus, and parts
    ExpGdsElecGenMachElecAppAndParts,
    /// Exports of energy products
    ExpGdsEnergyProd,
    /// Exports of engines and parts for civilian aircraft
    ExpGdsEngAndPartsForCivAir,
    /// Exports of fertilizers, pesticides, and insecticides
    ExpGdsFertPestAndInsect,
    /// Exports of fish and shellfish
    ExpGdsFishShellfish,
    /// Exports of foods, feeds, and beverages
    ExpGdsFoodsFeedsAndBevs,
    /// Exports of fuel oil
    ExpGdsFuelOil,
    /// Exports of goods; balance of payments adjustments, net; goods procured in U.S. ports by foreign carriers
    ExpGdsGdsProcPortsBopAdj,
    /// Exports of gem diamonds and other gemstones
    ExpGdsGemDiamAndOthGem,
    /// Exports of general merchandise
    ExpGdsGenMerch,
    /// Exports of grains and preparations
    ExpGdsGrainsPreps,
    /// Exports of hides and skins, including furskins
    ExpGdsHidesSkins,
    /// Exports of household and kitchen appliances
    ExpGdsHouseAndKitchApp,
    /// Exports of household furnishings and related products
    ExpGdsHouseFurnAndRelProds,
    /// Exports of household and kitchen appliances and other household goods
    ExpGdsHouseKitchAppAndOthHouse,
    /// Exports of industrial engines, pumps, and compressors
    ExpGdsIndEngPumpsComps,
    /// Exports of industrial inorganic chemicals
    ExpGdsIndInorgChems,
    /// Exports of industrial organic chemicals
    ExpGdsIndOrgChems,
    /// Exports of iron and steel products
    ExpGdsIronAndSteelProds,
    /// Exports of industrial supplies and materials
    ExpGdsIsm,
    /// Exports of jewelry and collectibles
    ExpGdsJewelryAndCollect,
    /// Exports of liquified petroleum gases
    ExpGdsLiqPetGases,
    /// Exports of machinery and equipment except consumer-type
    ExpGdsMachAndEquipExcCons,
    /// Exports of machine tools and metalworking machinery
    ExpGdsMachToolsMetalworkMach,
    /// Exports of measuring, testing, and control instruments
    ExpGdsMeasTestControlInst,
    /// Exports of meat products and poultry
    ExpGdsMeatProdsPoultry,
    /// Exports of medicinal, dental, and pharmaceutical products
    ExpGdsMedDentAndPharm,
    /// Exports of goods; balance of payments adjustments, net; net exports of goods under merchanting
    ExpGdsMerchantingBopAdj,
    /// Net exports of goods under merchanting
    ExpGdsMerchantingNet,
    /// Exports of metals and nonmetallic products
    ExpGdsMetalsAndNonmetProds,
    /// Exports of natural gas
    ExpGdsNaturalGas,
    /// Exports of nonagricultural foods, feeds, and beverages
    ExpGdsNonAgFoodsFeedsAndBevs,
    /// Exports of nonagricultural industrial supplies and materials
    ExpGdsNonAgIsm,
    /// Exports of nondurable consumer goods
    ExpGdsNondurCons,
    /// Exports of nonferrous metals
    ExpGdsNonferrousMetals,
    /// Exports of nonmonetary gold
    ExpGdsNonmonetaryGold,
    /// Exports of goods; balance of payments adjustments, net; nonmonetary gold
    ExpGdsNonmonGoldBopAdj,
    /// Exports of nuclear fuel and electric energy
    ExpGdsNuclearFuelAndElecEnergy,
    /// Exports of oil-drilling, mining, and construction machinery
    ExpGdsOilDrillMiningConstMach,
    /// Exports of other agricultural foods, feeds, and beverages
    ExpGdsOthAgFoodsFeedsAndBevs,
    /// Exports of other agricultural industrial supplies
    ExpGdsOthAgIsm,
    /// Exports of other automotive parts and accessories
    ExpGdsOthAutoPartsAndAcc,
    /// Exports of goods; balance of payments adjustments, net; other adjustments, net
    ExpGdsOthBopAdj,
    /// Exports of other chemicals
    ExpGdsOthChems,
    /// Exports of other durable consumer goods
    ExpGdsOthDurCons,
    /// Exports of other feeds
    ExpGdsOthFeeds,
    /// Exports of other general merchandise
    ExpGdsOthGenMerch,
    /// Exports of other household goods, including cell phones
    ExpGdsOthHouseIncCellPhones,
    /// Exports of other industrial machinery
    ExpGdsOthIndMach,
    /// Exports of other metals and nonmetallic products
    ExpGdsOthMetalsAndNonmetProds,
    /// Exports of other nondurable consumer goods
    ExpGdsOthNondurCons,
    /// Exports of other nonferrous metals
    ExpGdsOthNonferrousMetals,
    /// Exports of other nonmetals
    ExpGdsOthNonmetals,
    /// Exports of other office and business machines
    ExpGdsOthOfficeAndBusMach,
    /// Exports of other petroleum products
    ExpGdsOthPetProds,
    /// Exports of other service-industry and agricultural machinery
    ExpGdsOthServIndAndAgMach,
    /// Exports of other transportation equipment
    ExpGdsOthTransEquip,
    /// Exports of paper and paper-base stocks
    ExpGdsPaperAndPaperBaseStocks,
    /// Exports of passenger cars, new and used
    ExpGdsPassCars,
    /// Exports of petroleum and products
    ExpGdsPetAndProds,
    /// Exports of plastic materials
    ExpGdsPlasticMaterials,
    /// Exports of precious metals except nonmonetary gold
    ExpGdsPrecMetalsExcNonmonGold,
    /// Exports of goods; balance of payments adjustments, net; private gift parcel remittances
    ExpGdsPrivGiftParcelRemitBopAdj,
    /// Exports of radio and stereo equipment, including recorded media
    ExpGdsRadioAndStereoEquip,
    /// Exports of raw cotton
    ExpGdsRawCotton,
    /// Exports of goods; balance of payments adjustments, net; repair of equipment
    ExpGdsRepairEquipBopAdj,
    /// Exports of rice and other food grains
    ExpGdsRiceOthFoodGrains,
    /// Exports of scientific, hospital, and medical equipment and parts
    ExpGdsSciHospAndMedEquipAndParts,
    /// Exports of semiconductors
    ExpGdsSemiconductors,
    /// Exports of goods and services
    ExpGdsServ,
    /// Exports of goods and services and income receipts (credits)
    ExpGdsServIncRec,
    /// Exports of soybeans
    ExpGdsSoybeans,
    /// Exports of steelmaking materials
    ExpGdsSteelmakingMats,
    /// Exports of telecommunications equipment
    ExpGdsTelecomEquip,
    /// Exports of textile supplies and related materials
    ExpGdsTextileSupAndRelMats,
    /// Exports of toiletries and cosmetics
    ExpGdsToilAndCosmet,
    /// Exports of toys and sporting goods, including bicycles
    ExpGdsToysAndSport,
    /// Exports of trucks, buses, and special purpose vehicles
    ExpGdsTrucksBusesSpecPurpVeh,
    /// Exports of televisions, video receivers, and other video equipment
    ExpGdsTvsVidRecAndOthVidEquip,
    /// Exports of unmanufactured tobacco
    ExpGdsUnmanufTobacco,
    /// Exports of goods; balance of payments adjustments, net; exports under U.S. military agency sales contracts
    ExpGdsUsMilAgencyBopAdj,
    /// Exports of vegetables, fruits, nuts, and preparations
    ExpGdsVegFruitNutPreps,
    /// Exports of wheat
    ExpGdsWheat,
    /// Exports of general merchandise
    ExpGenMerch,
    /// Exports of services
    ExpServ,
    /// Exports of artistic related services
    ExpServArtisticRelated,
    /// Exports of audiovisual services
    ExpServAudVis,
    /// Charges for the use of intellectual property n.i.e.; exports
    ExpServChargesForTheUseOfIpNie,
    /// Charges for the use of intellectual property n.i.e.; exports; licenses to reproduce and/or distribute audiovisual products
    ExpServCipLicensesAudVis,
    /// Charges for the use of intellectual property n.i.e.; exports; licenses to reproduce and/or distribute computer software
    ExpServCipLicensesCompSoftware,
    /// Charges for the use of intellectual property n.i.e.; exports; franchises and trademarks licensing fees
    ExpServCipLicensesFranchisesTrademarks,
    /// Charges for the use of intellectual property n.i.e.; exports; licenses for the use of outcomes of research and development
    ExpServCipLicensesOutcomesResearchAndDev,
    /// Exports of computer services
    ExpServComp,
    /// Exports of construction services
    ExpServConst,
    /// Construction abroad
    ExpServConstAbroad,
    /// Foreign contractors' expenditures in the United States
    ExpServConstExpend,
    /// Exports of construction services
    ExpServConstruction,
    /// Exports of financial services
    ExpServFinancial,
    /// Exports of explicitly charged and other financial services
    ExpServFinancialExplicitAndOth,
    /// Exports of brokerage and market-making services
    ExpServFinBrokMarketMak,
    /// Exports of credit card and other credit-related services
    ExpServFinCredCardOthCredRelated,
    /// Exports of financial advisory and custody services
    ExpServFinFinAdvCust,
    /// Exports of financial management services
    ExpServFinFinMan,
    /// Exports of securities lending, electronic funds transfer, and other services
    ExpServFinSecLendEftOth,
    /// Exports of underwriting and private placement services
    ExpServFinUwPrivPlace,
    /// Exports of financial intermediation services indirectly measured
    ExpServFisim,
    /// Exports of government goods and services n.i.e.
    ExpServGovtGoodsAndServicesNie,
    /// Exports of information services
    ExpServInfo,
    /// Exports of insurance services
    ExpServInsurance,
    /// Exports of auxiliary insurance services
    ExpServInsuranceAuxIns,
    /// Exports of direct insurance services
    ExpServInsuranceDirect,
    /// Exports of reinsurance services
    ExpServInsuranceReins,
    /// Exports of maintenance and repair services n.i.e.
    ExpServMaintenanceAndRepairNie,
    /// Exports of manufacturing services on physical inputs owned by others
    ExpServManufacturing,
    /// Exports of other business services
    ExpServOtherBusiness,
    /// Exports of personal, cultural, and recreational services
    ExpServPersCultAndRec,
    /// Exports of other personal, cultural, and recreational services
    ExpServPersCultAndRecOth,
    /// Exports of professional and management consulting services
    ExpServProfMgmtConsult,
    /// exports of research and development services
    ExpServResearchAndDev,
    /// exports of technical, trade-related, and other business services
    ExpServTechTradeRelatedOth,
    /// exports of telecommunications services
    ExpServTelecom,
    /// Exports of telecommunications, computer, and information services
    ExpServTelecomCompAndInfo,
    /// Exports of transport services
    ExpServTransport,
    /// Exports of air transport services
    ExpServTransportAir,
    /// Exports of air freight services
    ExpServTransportAirFreight,
    /// Exports of air passenger services
    ExpServTransportAirPass,
    /// Exports of air port services
    ExpServTransportAirPort,
    /// Exports of transport services; other modes of transport
    ExpServTransportOth,
    /// Exports of sea transport services
    ExpServTransportSea,
    /// Exports of sea freight services
    ExpServTransportSeaFreight,
    /// Exports of sea port services
    ExpServTransportSeaPort,
    /// Exports of travel services (for all purposes including education)
    ExpServTravel,
    /// Exports of business travel services
    ExpServTravelBusiness,
    /// Exports of other business travel services
    ExpServTravelBusinessOth,
    /// Exports of education-related services
    ExpServTravelEducation,
    /// Exports of health-related services
    ExpServTravelHealth,
    /// Exports of personal travel services
    ExpServTravelPersonal,
    /// Exports of other personal travel services
    ExpServTravelPersonalOth,
    /// Expenditures in the U.S. by border, seasonal, and other short-term workers
    ExpServTravelShortTermWork,
    /// Net U.S. acquisition of financial assets excluding financial derivatives
    FinAssetsExclFinDeriv,
    /// Financial derivatives other than reserves, net transactions
    FinDeriv,
    /// Net U.S. acquisition of reserve assets; other; financial derivatives
    FinDerivReserveAssets,
    /// Net U.S. incurrence of liabilities excluding financial derivatives
    FinLiabsExclFinDeriv,
    /// Net U.S. incurrence of liabilities to foreign official agencies
    FinLiabsFoa,
    /// Net U.S. acquisition of reserve assets; monetary gold
    GoldReserveAssets,
    /// Net U.S. acquisition of reserve assets; reserve position in the International Monetary Fund
    ImfReserveAssets,
    /// Imports of goods
    ImpGds,
    /// Imports of agricultural foods, feeds, and beverages
    ImpGdsAgFoodsFeedsAndBevs,
    /// Imports of agricultural industrial supplies and materials
    ImpGdsAgIsm,
    /// Imports of apparel, footwear, and household goods
    ImpGdsAppFootAndHouse,
    /// Imports of automotive engines and engine parts
    ImpGdsAutoEngAndEngParts,
    /// Imports of automotive vehicles, parts, and engines
    ImpGdsAutoVehPartsAndEngines,
    /// Imports of bauxite and aluminum
    ImpGdsBauxAndAlum,
    /// Imports of goods; balance of payments adjustments, net
    ImpGdsBopAdj,
    /// Imports of building materials except metals
    ImpGdsBuildMatsExcMetals,
    /// Imports of capital goods except automotive
    ImpGdsCapGoodsExclAuto,
    /// Imports of goods, Census basis
    ImpGdsCensus,
    /// Imports of chemicals except medicinals
    ImpGdsChemsExcMeds,
    /// Imports of civilian aircraft, complete, all types
    ImpGdsCivAir,
    /// Imports of civilian aircraft, engines, and parts
    ImpGdsCivAirEngAndParts,
    /// Imports of coal and related products
    ImpGdsCoalAndRelProds,
    /// Imports of cocoa beans and sugar
    ImpGdsCocoaAndSugar,
    /// Imports of computers
    ImpGdsComp,
    /// Imports of computer accessories, peripherals, and parts
    ImpGdsCompAccPeriAndParts,
    /// Imports of consumer goods except food and automotive
    ImpGdsConsGoodsExcFoodAndAuto,
    /// Imports of crude petroleum
    ImpGdsCrudePet,
    /// Imports of distilled beverages and other nonagricultural foods, feeds, and beverages
    ImpGdsDistBevAndOthNonAgFoodsFeedsAndBevs,
    /// Imports of durable consumer goods
    ImpGdsDurCons,
    /// Imports of electric-generating machinery, electric apparatus and parts
    ImpGdsElecGenMachElecAppAndParts,
    /// Imports of energy products
    ImpGdsEnergyProds,
    /// Imports of engines and parts for civilian aircraft
    ImpGdsEngAndPartsForCivAir,
    /// Imports of fertilizers, pesticides, and insecticides
    ImpGdsFertPestAndInsect,
    /// Imports of fish and shellfish
    ImpGdsFishShellfish,
    /// Imports of foods, feeds, and beverages
    ImpGdsFoodsFeedsAndBevs,
    /// Imports of fuel oil
    ImpGdsFuelOil,
    /// Imports of goods; balance of payments adjustments, net; goods procured in foreign ports by U.S. carriers
    ImpGdsGdsProcPortsBopAdj,
    /// Imports of gem diamonds and other gemstones
    ImpGdsGemDiamAndOthGem,
    /// Imports of general merchandise
    ImpGdsGenMerch,
    /// Imports of green coffee
    ImpGdsGreenCoffee,
    /// Imports of household and kitchen appliances
    ImpGdsHouseAndKitchApp,
    /// Imports of household furnishings and related products
    ImpGdsHouseFurnAndRelProds,
    /// Imports of household and kitchen appliances and other household goods
    ImpGdsHouseKitchAppAndOthHouse,
    /// Imports of industrial engines, pumps, and compressors
    ImpGdsIndEngPumpsComps,
    /// Imports of industrial inorganic chemicals
    ImpGdsIndInorgChems,
    /// Imports of industrial organic chemicals
    ImpGdsIndOrgChems,
    /// Imports of goods; balance of payments adjustments, net; inland freight in Canada and Mexico
    ImpGdsInlandFreightCanMexBopAdj,
    /// Imports of iron and steel products
    ImpGdsIronAndSteelProds,
    /// Imports of industrial supplies and materials
    ImpGdsIsm,
    /// Imports of jewelry and collectibles
    ImpGdsJewelryAndCollect,
    /// Imports of liquified petroleum gases
    ImpGdsLiqPetGases,
    /// Imports of goods; balance of payments adjustments, net; locomotives and railcars
    ImpGdsLocoRailBopAdj,
    /// Imports of machinery and equipment except consumer-type
    ImpGdsMachAndEquipExcCons,
    /// Imports of machine tools and metalworking machinery
    ImpGdsMachToolsMetalworkMach,
    /// Imports of measuring, testing, and control instruments
    ImpGdsMeasTestControlInst,
    /// Imports of meat products and poultry
    ImpGdsMeatProdsPoultry,
    /// Imports of medicinal, dental, and pharmaceutical products
    ImpGdsMedDentAndPharm,
    /// Imports of metals and nonmetallic products
    ImpGdsMetalsAndNonmetProds,
    /// Imports of natural gas
    ImpGdsNaturalGas,
    /// Imports of nonagricultural foods, feeds, and beverages
    ImpGdsNonAgFoodsFeedsAndBevs,
    /// Imports of nonagricultural industrial supplies and materials
    ImpGdsNonAgIsm,
    /// Imports of nondurable consumer goods
    ImpGdsNondurCons,
    /// Imports of nonferrous metals
    ImpGdsNonferrousMetals,
    /// Imports of nonmonetary gold
    ImpGdsNonmonetaryGold,
    /// Imports of goods; balance of payments adjustments, net; nonmonetary gold
    ImpGdsNonmonGoldBopAdj,
    /// Imports of nuclear fuel and electric energy
    ImpGdsNuclearFuelAndElecEnergy,
    /// Imports of oil-drilling, mining, and construction machinery
    ImpGdsOilDrillMiningConstMach,
    /// Imports of other agricultural foods, feeds, and beverages
    ImpGdsOthAgFoodsFeedsAndBevs,
    /// Imports of other automotive parts and accessories
    ImpGdsOthAutoPartsAndAcc,
    /// Imports of goods; balance of payments adjustments, net; other adjustments, net
    ImpGdsOthBopAdj,
    /// Imports of other chemicals
    ImpGdsOthChems,
    /// Imports of other durable consumer goods
    ImpGdsOthDurCons,
    /// Imports of other general merchandise
    ImpGdsOthGenMerch,
    /// Imports of other household goods, including cell phones
    ImpGdsOthHouseIncCellPhones,
    /// Imports of other industrial machinery
    ImpGdsOthIndMach,
    /// Imports of other metals and nonmetallic products
    ImpGdsOthMetalsAndNonmetProds,
    /// Imports of other nondurable consumer goods
    ImpGdsOthNondurCons,
    /// Imports of other nonferrous metals
    ImpGdsOthNonferrousMetals,
    /// Imports of other nonmetals
    ImpGdsOthNonmetals,
    /// Imports of other office and business machines
    ImpGdsOthOfficeAndBusMach,
    /// Imports of other petroleum products
    ImpGdsOthPetProds,
    /// Imports of other service-industry and agricultural machinery
    ImpGdsOthServIndAndAgMach,
    /// Imports of other transportation equipment
    ImpGdsOthTransEquip,
    /// Imports of paper and paper-base stocks
    ImpGdsPaperAndPaperBaseStocks,
    /// Imports of passenger cars, new and used
    ImpGdsPassCars,
    /// Imports of petroleum and products
    ImpGdsPetAndProds,
    /// Imports of plastic materials
    ImpGdsPlasticMaterials,
    /// Imports of precious metals except nonmonetary gold
    ImpGdsPrecMetalsExcNonmonGold,
    /// Imports of radio and stereo equipment, including recorded media
    ImpGdsRadioAndStereoEquip,
    /// Imports of goods; balance of payments adjustments, net; repair of equipment
    ImpGdsRepairEquipBopAdj,
    /// Imports of scientific, hospital, and medical equipment and parts
    ImpGdsSciHospAndMedEquipAndParts,
    /// Imports of semiconductors
    ImpGdsSemiconductors,
    /// Imports of goods and services
    ImpGdsServ,
    /// Imports of goods and services and income payments (debits)
    ImpGdsServIncPay,
    /// Imports of goods; balance of payments adjustments, net; software revaluation
    ImpGdsSoftRevalBopAdj,
    /// Imports of steelmaking materials
    ImpGdsSteelmakingMats,
    /// Imports of telecommunications equipment
    ImpGdsTelecomEquip,
    /// Imports of textile supplies and related materials
    ImpGdsTextileSupAndRelMats,
    /// Imports of toiletries and cosmetics
    ImpGdsToilAndCosmet,
    /// Imports of toys and sporting goods, including bicycles
    ImpGdsToysAndSport,
    /// Imports of trucks, buses, and special purpose vehicles
    ImpGdsTrucksBusesSpecPurpVeh,
    /// Imports of televisions, video receivers, and other video equipment
    ImpGdsTvsVidRecAndOthVidEquip,
    /// Imports of goods; balance of payments adjustments, net; imports by U.S. military agencies
    ImpGdsUsMilAgencyBopAdj,
    /// Imports of vegetables, fruits, nuts, and preparations
    ImpGdsVegFruitNutPreps,
    /// Imports of wine, beer, and related products
    ImpGdsWineBeerRelProds,
    /// Imports of services
    ImpServ,
    /// Imports of artistic related services
    ImpServArtisticRelated,
    /// Imports of audiovisual services
    ImpServAudVis,
    /// Charges for the use of intellectual property n.i.e.; imports
    ImpServChargesForTheUseOfIpNie,
    /// Charges for the use of intellectual property n.i.e.; imports; licenses to reproduce and/or distribute audiovisual products
    ImpServCipLicensesAudVis,
    /// Charges for the use of intellectual property n.i.e.; imports; licenses to reproduce and/or distribute computer software
    ImpServCipLicensesCompSoftware,
    /// Charges for the use of intellectual property n.i.e.; imports; franchises and trademarks licensing fees
    ImpServCipLicensesFranchisesTrademarks,
    /// Charges for the use of intellectual property n.i.e.; imports; licenses for the use of outcomes of research and development
    ImpServCipLicensesOutcomesResearchAndDev,
    /// Imports of computer services
    ImpServComp,
    /// Imports of construction services
    ImpServConst,
    /// U.S. contractors' expenditures abroad
    ImpServConstExpend,
    /// Construction in the United States
    ImpServConstInTheUs,
    /// Imports of construction services
    ImpServConstruction,
    /// Imports of financial services
    ImpServFinancial,
    /// Imports of explicitly charged and other financial services
    ImpServFinancialExplicitAndOth,
    /// Imports of brokerage and market-making services
    ImpServFinBrokMarketMak,
    /// Imports of credit card and other credit-related services
    ImpServFinCredCardOthCredRelated,
    /// Imports of financial advisory and custody services
    ImpServFinFinAdvCust,
    /// Imports of financial management services
    ImpServFinFinMan,
    /// Imports of securities lending, electronic funds transfer, and other services
    ImpServFinSecLendEftOth,
    /// Imports of underwriting and private placement services
    ImpServFinUwPrivPlace,
    /// Imports of financial intermediation services indirectly measured
    ImpServFisim,
    /// Imports of government goods and services n.i.e.
    ImpServGovtGoodsAndServicesNie,
    /// Imports of information services
    ImpServInfo,
    /// Imports of insurance services
    ImpServInsurance,
    /// Imports of auxiliary insurance services
    ImpServInsuranceAuxIns,
    /// Imports of direct insurance services
    ImpServInsuranceDirect,
    /// Imports of reinsurance services
    ImpServInsuranceReIns,
    /// Imports of maintenance and repair services n.i.e.
    ImpServMaintenanceAndRepairNie,
    /// Imports of manufacturing services on physical inputs owned by others
    ImpServManufacturing,
    /// Imports of other business services
    ImpServOtherBusiness,
    /// Imports of personal, cultural, and recreational services
    ImpServPersCultAndRec,
    /// Imports of other personal, cultural, and recreational services
    ImpServPersCultAndRecOth,
    /// Imports of professional and management consulting services
    ImpServProfMgmtConsult,
    /// Research and development services imports
    ImpServResearchAndDev,
    /// Other technical, trade-related, and other business services imports
    ImpServTechTradeRelatedOth,
    /// Telecommunications services imports
    ImpServTelecom,
    /// Telecommunications, computer, and information services imports
    ImpServTelecomCompAndInfo,
    /// Transport services imports
    ImpServTransport,
    /// Air transport services imports
    ImpServTransportAir,
    /// Air freight transport services imports
    ImpServTransportAirFreight,
    /// Air passenger transport services imports
    ImpServTransportAirPass,
    /// Air transport services imports; port services
    ImpServTransportAirPort,
    /// Other transport services imports
    ImpServTransportOth,
    /// Sea transport services imports
    ImpServTransportSea,
    /// Sea freight transport services imports
    ImpServTransportSeaFreight,
    /// Sea transport services imports; port services
    ImpServTransportSeaPort,
    /// Travel services imports
    ImpServTravel,
    /// Business travel services imports
    ImpServTravelBusiness,
    /// Other business travel services imports
    ImpServTravelBusinessOth,
    /// Education-related travel services imports
    ImpServTravelEducation,
    /// Health-related travel services imports
    ImpServTravelHealth,
    /// Personal travel services imports
    ImpServTravelPersonal,
    /// Other personal travel services imports
    ImpServTravelPersonalOth,
    /// Short-term work-related travel services imports
    ImpServTravelShortTermWork,
    /// Insurance losses paid
    InsLossesPaid,
    /// Insurance losses recovered
    InsLossesRecovered,
    /// Insurance premiums paid
    InsPremiumsPaid,
    /// Insurance premiums received
    InsPremiumsReceived,
    /// Insurance technical reserves; assets
    InsTechReservesAssets,
    /// Insurance technical reserves; assets; other financial and nonfinancial institutions
    InsTechReservesAssetsOthFinNonFin,
    /// Insurance technical reserves; liabilities
    InsTechReservesLiabs,
    /// Insurance technical reserves; liabilities; other financial and nonfinancial institutions
    InsTechReservesLiabsOthFinNonFin,
    /// Investment fund shares; assets
    InvFundSharesAssets,
    /// Investment fund shares; income payments
    InvFundSharesIncPay,
    /// Investment fund shares; income receipts
    InvFundSharesIncRec,
    /// Investment fund shares; liabilities
    InvFundSharesLiabs,
    /// Investment fund shares; liabilities; financial account
    InvFundSharesLiabsFoa,
    /// Investment income payments
    InvIncPay,
    /// Investment income receipts
    InvIncRec,
    /// Investment income receipts; reinvested earnings
    InvRaIncRec,
    /// Investment income receipts; reinvested earnings; interest income
    InvRaIntIncRec,
    /// Loans; assets
    LoansAssets,
    /// Loans; assets; deposit-taking institutions
    LoansAssetsDepTaking,
    /// Loans; assets; general government
    LoansAssetsGenGovt,
    /// Loans; assets; other financial and nonfinancial institutions
    LoansAssetsOthFinNonFin,
    /// Loans; liabilities
    LoansLiabs,
    /// Loans; liabilities; deposit-taking institutions
    LoansLiabsDepTaking,
    /// Loans; liabilities; financial account
    LoansLiabsFoa,
    /// Loans; liabilities; other financial and nonfinancial institutions
    LoansLiabsOthFinNonFin,
    /// Loans; repurchase liabilities; other financial and nonfinancial institutions
    LoansRepurchaseLiabsOthFinNonFin,
    /// Loans; resale assets; other financial and nonfinancial institutions
    LoansResaleAssetsOthFinNonFin,
    /// Long-term debt securities; assets
    LtDebtSecAssets,
    /// Long-term debt securities; assets; deposit-taking institutions
    LtDebtSecAssetsDepTaking,
    /// Long-term debt securities; assets; nonfinancial institutions
    LtDebtSecAssetsNonFin,
    /// Long-term debt securities; assets; other financial institutions
    LtDebtSecAssetsOthFin,
    /// Long-term debt securities; corporate assets
    LtDebtSecCorpAssets,
    /// Long-term debt securities; corporate liabilities
    LtDebtSecCorpLiabs,
    /// Long-term debt securities; corporate liabilities; financial account
    LtDebtSecCorpLiabsFoa,
    /// Long-term debt securities; federal-sponsored agency income payments
    LtDebtSecFedSponsorAgencyIncPay,
    /// Long-term debt securities; federal-sponsored agency liabilities
    LtDebtSecFedSponsorAgencyLiabs,
    /// Long-term debt securities; federal-sponsored agency liabilities; financial account
    LtDebtSecFedSponsorAgencyLiabsFoa,
    /// Long-term debt securities; government assets
    LtDebtSecGovtAssets,
    /// Long-term debt securities; income payments
    LtDebtSecIncPay,
    /// Long-term debt securities; income payments; deposit-taking institutions
    LtDebtSecIncPayDepTaking,
    /// Long-term debt securities; income payments; nonfinancial institutions
    LtDebtSecIncPayNonFin,
    /// Long-term debt securities; income receipts
    LtDebtSecIncRec,
    /// Long-term debt securities; income receipts; deposit-taking institutions
    LtDebtSecIncRecDepTaking,
    /// Long-term debt securities; income receipts; nonfinancial institutions
    LtDebtSecIncRecNonFin,
    /// Long-term debt securities; income receipts; other financial institutions
    LtDebtSecIncRecOthFin,
    /// Long-term debt securities; liabilities
    LtDebtSecLiabs,
    /// Long-term debt securities; liabilities; deposit-taking institutions
    LtDebtSecLiabsDepTaking,
    /// Long-term debt securities; liabilities; financial account
    LtDebtSecLiabsFoa,
    /// Long-term debt securities; liabilities; nonfinancial institutions
    LtDebtSecLiabsNonFin,
    /// Long-term debt securities; negative CD assets
    LtDebtSecNegCdAssets,
    /// Long-term debt securities; negative CD liabilities
    LtDebtSecNegCdLiabs,
    /// Long-term debt securities; negative CD liabilities; financial account
    LtDebtSecNegCdLiabsFoa,
    /// Long-term debt securities; other than federal-sponsored agency income payments; other financial institutions
    LtDebtSecOthThanFedSponsorAgencyIncPayOthFin,
    /// Long-term debt securities; other than federal-sponsored agency liabilities; other financial institutions
    LtDebtSecOthThanFedSponsorAgencyLiabsOthFin,
    /// Portfolio investment income payments; interest on state and local government long-term securities
    LtDebtSecStateLocalGovtIncPay,
    /// Net U.S. incurrence of portfolio investment liabilities; state and local government long-term securities
    LtDebtSecStateLocalGovtLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; state and local government long-term securities
    LtDebtSecStateLocalGovtLiabsFoa,
    /// Portfolio investment income payments; interest on long-term U.S. Treasury securities
    LtDebtSecTreasIncPay,
    /// Net U.S. incurrence of portfolio investment liabilities; Treasury bonds and notes
    LtDebtSecTreasLiabs,
    /// Net U.S. acquisition of other investment assets; long-term deposits
    LtDepAssets,
    /// Net U.S. acquistion of other investment assets; long-term deposits; held by deposit-taking institutions except central bank
    LtDepAssetsDepTaking,
    /// Net U.S. acquistion of other investment assets; long-term deposits; held by non-deposit-taking institutions and nonfinancial institutions except general government
    LtDepAssetsOthFinNonFin,
    /// Net U.S. incurrence of other investment liabilities; long-term deposits
    LtDepLiabs,
    /// Net U.S. incurrence of other investment liabilities; long-term deposits; issued by deposit-taking institutions except central bank
    LtDepLiabsDepTaking,
    /// Net U.S. incurrence of other investment liabilities; long-term deposits; issued by non-deposit-taking institutions and nonfinancial institutions except general government
    LtDepLiabsOthFinNonFin,
    /// Net U.S. acquisition of other investment assets; long-term loans
    LtLoansAssets,
    /// Net U.S. acquistion of other investment assets; long-term loans; held by deposit-taking institutions except central bank
    LtLoansAssetsDepTaking,
    /// Net U.S. acquistion of other investment assets; long-term loans; held by general government
    LtLoansAssetsGenGovt,
    /// Net U.S. acquistion of other investment assets; long-term loans; held by non-deposit-taking institutions and nonfinancial institutions except general government
    LtLoansAssetsOthFinNonFin,
    /// Net U.S. incurrence of other investment liabilities; long-term loans
    LtLoansLiabs,
    /// Net U.S. incurrence of other investment liabilities; long-term loans; issued by deposit-taking institutions except central bank
    LtLoansLiabsDepTaking,
    /// Net U.S. incurrence of other investment liabilities; long-term loans; issued by non-deposit-taking institutions and nonfinancial institutions except general government
    LtLoansLiabsOthFinNonFin,
    /// Net U.S. acquisition of other investment assets; long-term trade credit and advances
    LtTrdCredAndAdvAssets,
    /// Net U.S. acquistion of other investment assets; long-term trade credit and advances; held by non-deposit-taking institutions and nonfinancial institutions except general government
    LtTrdCredAndAdvAssetsOthFinNonFin,
    /// Net U.S. incurrence of other investment liabilities; long-term trade credit and advances
    LtTrdCredAndAdvLiabs,
    /// Net U.S. incurrence of other investment liabilities; long-term trade credit and advances; issued by general government
    LtTrdCredAndAdvLiabsGenGovt,
    /// Net U.S. incurrence of other investment liabilities; long-term trade credit and advances; issued by non-deposit-taking institutions and nonfinancial institutions except general government
    LtTrdCredAndAdvLiabsOthFinNonFin,
    /// Net lending (+) or net borrowing (-) from current- and capital-account transactions
    NetLendBorrCurrCapAcct,
    /// Net lending (+) or net borrowing (-) from financial-account transactions
    NetLendBorrFinAcct,
    /// Net U.S. acquisition of reserve assets; other; other claims
    OthClmReserveAssets,
    /// Net U.S. acquisition of other investment assets; other equity
    OthEquityAssets,
    /// Net U.S. acquistion of other investment assets; held by general government; other equity
    OthEquityAssetsGenGovt,
    /// Net U.S. incurrence of other investment liabilities; other equity
    OthEquityLiabs,
    /// Net U.S. incurrence of other investment liabilities to foreign official agencies; other equity
    OthEquityLiabsFoa,
    /// Net U.S. acquisition of other investment assets
    OthInvAssets,
    /// Net U.S. acquistion of other investment assets; held by central bank
    OthInvAssetsCentralBank,
    /// Net U.S. acquistion of other investment assets; held by deposit-taking institutions except central bank
    OthInvAssetsDepTaking,
    /// Net U.S. acquistion of other investment assets; held by general government
    OthInvAssetsGenGovt,
    /// Net U.S. acquistion of other investment assets; held by non-deposit-taking institutions and nonfinancial institutions except general government
    OthInvAssetsOthFinNonFin,
    /// Other investment income payments
    OthInvIncPay,
    /// Other investment interest income payments before adjusting for FISIM
    OthInvIncPayBeforeFisim,
    /// Other investment income payments; on liabilities issued by central bank
    OthInvIncPayCentralBank,
    /// Other investment income payments; on liabilities issued by deposit-taking institutions except central bank
    OthInvIncPayDepTaking,
    /// Other investment income payments; on liabilities issued by general government
    OthInvIncPayGenGovt,
    /// Other investment income payments; on liabilities issued by non-deposit-taking financial institutions and nonfinancial instutitions except general government
    OthInvIncPayOthFinNonFin,
    /// Other investment income receipts
    OthInvIncRec,
    /// Other investment interest income receipts before adjusting for FISIM
    OthInvIncRecBeforeFisim,
    /// Other investment income receipts; on assets held by central bank
    OthInvIncRecCentralBank,
    /// Other investment income receipts; on assets held by deposit-taking institutions except central bank
    OthInvIncRecDepTaking,
    /// Other investment income receipts; on assets held by general government
    OthInvIncRecGenGovt,
    /// Other investment income receipts; on assets held by non-deposit-taking financial institutions and nonfinancial instutitions except general government
    OthInvIncRecOthFinNonFin,
    /// Other investment income payments; income attributable to insurance policyholders
    OthInvInsPolHoldtIncPay,
    /// Other investment income payments; income attributable to insurance policyholders; on liabilities issued by non-deposit-taking institutions and nonfinancial institutions except general government
    OthInvInsPolHoldtIncPayOthFinNonFin,
    /// Other investment income receipts; income attributable to insurance policyholders
    OthInvInsPolHoldtIncRec,
    /// Other investment income receipts; income attributable to insurance policyholders; on assets held by non-deposit-taking institutions and nonfinancial institutions except general government
    OthInvInsPolHoldtIncRecOthFinNonFin,
    /// Net U.S. acquistion of other investment assets; interbank transactions
    OthInvInterbankAssets,
    /// Net U.S. incurrence of other investment liabilities; interbank transactions
    OthInvInterbankLiabsDepTaking,
    /// Other investment income payments; interest
    OthInvIntIncPay,
    /// Other investment income payments; interest; on liabilities issued by central bank
    OthInvIntIncPayCentralBank,
    /// Other investment income payments; interest; on liabilities issued by deposit-taking institutions except central bank
    OthInvIntIncPayDepTaking,
    /// Other investment income payments; interest on special drawing rights allocations
    OthInvIntIncPayGenGovt,
    /// Other investment income payments; interest; on liabilities issued by non-deposit-taking financial institutions and nonfinancial instutitions except general government
    OthInvIntIncPayOthFinNonFin,
    /// Other investment income receipts; interest
    OthInvIntIncRec,
    /// Other investment income receipts; interest; on assets held by central bank
    OthInvIntIncRecCentralBank,
    /// Other investment income receipts; interest; on assets held by deposit-taking institutions except central bank
    OthInvIntIncRecDepTaking,
    /// Other investment income receipts; interest; on assets held by general government
    OthInvIntIncRecGenGovt,
    /// Other investment income receipts; interest; on assets held by non-deposit-taking financial institutions and nonfinancial instutitions except general government
    OthInvIntIncRecOthFinNonFin,
    /// Net U.S. incurrence of other investment liabilities
    OthInvLiabs,
    /// Net U.S. incurrence of other investment liabilities; issued by central bank
    OthInvLiabsCentralBank,
    /// Net U.S. incurrence of other investment liabilities; issued by deposit-taking institutions except central bank
    OthInvLiabsDepTaking,
    /// Net U.S. incurrence of other investment liabilities to foreign official agencies
    OthInvLiabsFoa,
    /// Net U.S. incurrence of other investment liabilities; issued by general government
    OthInvLiabsGenGovt,
    /// Net U.S. incurrence of other investment liabilities; issued by non-deposit-taking institutions and nonfinancial institutions except general government
    OthInvLiabsOthFinNonFin,
    /// Net U.S. acquisition of reserve assets; other
    OthReserveAssets,
    /// Net U.S. acquisition of portfolio investment assets
    PfInvAssets,
    /// Net U.S. acquisition of portfolio investment assets; held by deposit-taking institutions except central bank
    PfInvAssetsDepTaking,
    /// Net U.S. acquisition of portfolio investment assets; held by nonfinancial institutions except general government
    PfInvAssetsNonFin,
    /// Net U.S. acquisition of portfolio investment assets; held by non-deposit-taking financial institutions
    PfInvAssetsOthFin,
    /// Portfolio investment income payments
    PfInvIncPay,
    /// Portfolio investment income payments; deposit-taking institutions except central bank
    PfInvIncPayDepTaking,
    /// Portfolio investment income payments; general government
    PfInvIncPayGenGovt,
    /// Portfolio investment income payments; nonfinancial institutions except general government
    PfInvIncPayNonFin,
    /// Portfolio investment income payments; non-deposit-taking financial institutions
    PfInvIncPayOthFin,
    /// Portfolio investment income receipts
    PfInvIncRec,
    /// Portfolio investment income receipts; deposit-taking institutions except central bank
    PfInvIncRecDepTaking,
    /// Portfolio investment income receipts; nonfinancial institutions except general government
    PfInvIncRecNonFin,
    /// Portfolio investment income receipts; non-deposit-taking financial institutions
    PfInvIncRecOthFin,
    /// Net U.S. incurrence of portfolio investment liabilities
    PfInvLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities; issued by deposit-taking institutions except central bank
    PfInvLiabsDepTaking,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies
    PfInvLiabsFoa,
    /// Net U.S. incurrence of portfolio investment liabilities; issued by general government
    PfInvLiabsGenGovt,
    /// Net U.S. incurrence of portfolio investment liabilities; issued by nonfinancial institutions except general government
    PfInvLiabsNonFin,
    /// Net U.S. incurrence of portfolio investment liabilities; issued by non-deposit-taking financial institutions
    PfInvLiabsOthFin,
    /// Primary income payments
    PrimIncPay,
    /// Primary income receipts
    PrimIncRec,
    /// Net U.S. acquisition of reserve assets
    ReserveAssets,
    /// Net U.S. incurrence of other investment liabilities; special drawing rights allocations
    SdrAllocLiabs,
    /// Net U.S. incurrence of other investment liabilities to foreign official agencies; special drawing rights allocations
    SdrAllocLiabsFoa,
    /// Net U.S. acquisition of reserve assets; special drawing rights
    SdrReserveAssets,
    /// Seasonal adjustment discrepancy
    SeasAdjDisc,
    /// Secondary income (current transfer) payments
    SecIncPay,
    /// Secondary income (current transfer) payments; private transfers; charitable donations
    SecIncPayCharitableDonations,
    /// Secondary income (current transfer) payments; private transfers; transfers to foreign students
    SecIncPayForeignStudents,
    /// Secondary income (current transfer) payments; general government transfers
    SecIncPayGenGovt,
    /// Secondary income (current transfer) payments; general government transfers; contributions to international organizations
    SecIncPayGenGovtContribIntOrg,
    /// Secondary income (current transfer) payments; general government transfers; international cooperation
    SecIncPayGenGovtIntCoop,
    /// Secondary income (current transfer) payments; other general government transfers
    SecIncPayGenGovtOth,
    /// Secondary income (current transfer) payments; general government transfers; social benefits
    SecIncPayGenGovtSocialBenefits,
    /// Secondary income (current transfer) payments; private transfers; insurance-related transfers
    SecIncPayInsuranceRelated,
    /// Secondary income (current transfer) payments; other private transfers
    SecIncPayOthPrivateTransfer,
    /// Secondary income (current transfer) payments; private transfers; personal transfers
    SecIncPayPersonal,
    /// Secondary income (current transfer) payments; private transfers
    SecIncPayPrivate,
    /// Secondary income (current transfer) payments; private transfers; fines and penalties
    SecIncPayPrivateFinesPenalties,
    /// Secondary income (current transfer) payments; private transfers; taxes on income, wealth, etc.
    SecIncPayTaxesIncomeWealth,
    /// Secondary income (current transfer) receipts
    SecIncRec,
    /// Secondary income (current transfer) receipts; general government transfers
    SecIncRecGenGovt,
    /// Secondary income (current transfer) receipts; general government transfers; fines and penalties
    SecIncRecGenGovtFinesPenalties,
    /// Secondary income (current transfer) receipts; general government transfers; international cooperation
    SecIncRecGenGovtIntCoop,
    /// Secondary income (current transfer) receipts; other general government transfers
    SecIncRecGenGovtOth,
    /// Secondary income (current transfer) receipts; general government transfers; taxes on income, wealth, etc.
    SecIncRecGenGovtTaxesIncomeWealth,
    /// Secondary income (current transfer) receipts; insurance-related transfers
    SecIncRecInsuranceRelated,
    /// Secondary income (current transfer) receipts; other private transfers
    SecIncRecOthPrivateTransfer,
    /// Secondary income (current transfer) receipts; private transfers
    SecIncRecPrivate,
    /// Secondary income (current transfer) receipts; fines and penalties
    SecIncRecPrivateFinesPenalties,
    /// Net U.S. acquisition of reserve assets; other; securities
    SecReserveAssets,
    /// Statistical discrepancy
    StatDisc,
    /// Net U.S. acquisition of portfolio investment assets; short-term debt securities
    StDebtSecAssets,
    /// Net U.S. acquisition of portfolio investment assets; short-term debt securities; held by deposit-taking institutions except central bank
    StDebtSecAssetsDepTaking,
    /// Net U.S. acquisition of portfolio investment assets; short-term debt securities; held by nonfinancial institutions except general government
    StDebtSecAssetsNonFin,
    /// Net U.S. acquisition of portfolio investment assets; short-term debt securities; held by non-deposit-taking financial institutions
    StDebtSecAssetsOthFin,
    /// Net U.S. incurrence of portfolio investment liabilities; commercial paper and other short-term debt securities (those not listed in table 7.1)
    StDebtSecCommPaperAndOthLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; commercial paper and other short-term debt securities (those not listed in table 9.1)
    StDebtSecCommPaperAndOthLiabsFoa,
    /// Net U.S. acquisition of portfolio investment assets; short-term commercial paper
    StDebtSecCommPaperAssets,
    /// Portfolio investment income payments; interest on short-term federally sponsored agency securties
    StDebtSecFedSponsorAgencyIncPay,
    /// Net U.S. incurrence of portfolio investment liabilities; short-term federally sponsored agency securities
    StDebtSecFedSponsorAgencyLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; short-term federally sponsored agency securities
    StDebtSecFedSponsorAgencyLiabsFoa,
    /// Portfolio investment income payments; interest on short-term debt securities
    StDebtSecIncPay,
    /// Portfolio investment income payments; interest on short-term debt securities; deposit-taking institutions except central bank
    StDebtSecIncPayDepTaking,
    /// Portfolio investment income payments; interest on short-term debt securities; nonfinancial institutions except general government
    StDebtSecIncPayNonFin,
    /// Portfolio investment income receipts; interest on short-term debt securities
    StDebtSecIncRec,
    /// Portfolio investment income receipts; interest on short-term debt securities; deposit-taking institutions except central bank
    StDebtSecIncRecDepTaking,
    /// Portfolio investment income receipts; interest on short-term debt securities; nonfinancial institutions except general government
    StDebtSecIncRecNonFin,
    /// Portfolio investment income receipts; interest on short-term debt securities; non-deposit-taking financial institutions
    StDebtSecIncRecOthFin,
    /// Net U.S. incurrence of portfolio investment liabilities; short-term debt securities
    StDebtSecLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities; short-term debt securities; issued by deposit-taking institutions except central bank
    StDebtSecLiabsDepTaking,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; short-term debt securities
    StDebtSecLiabsFoa,
    /// Net U.S. incurrence of portfolio investment liabilities; short-term debt securities; issued by nonfinancial institutions except general government
    StDebtSecLiabsNonFin,
    /// Net U.S. acquisition of portfolio investment assets; short-term negotiable certificates of deposit
    StDebtSecNegCdAssets,
    /// Net U.S. incurrence of portfolio investment liabilities; short-term negotiable certificates of deposit
    StDebtSecNegCdLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; short-term negotiable certificates of deposit
    StDebtSecNegCdLiabsFoa,
    /// Net U.S. acquisition of portfolio investment assets; short-term debt securities other than negotiable certificates of deposit and commercial paper
    StDebtSecOthAssets,
    /// Portfolio investment income payments; interest on short-term debt securities other than federally sponsored agency securties; non-deposit-taking financial institutions
    StDebtSecOthThanFedSponsorAgencyIncPayOthFin,
    /// Net U.S. incurrence of portfolio investment liabilities; short-term debt securities other than federally sponsored agency securities; issued by non-deposit-taking financial institutions
    StDebtSecOthThanFedSponsorAgencyLiabsOthFin,
    /// Portfolio investment income payments; interest on short-term U.S. Treasury securities
    StDebtSecTreasIncPay,
    /// Net U.S. incurrence of portfolio investment liabilities; Treasury bills and certificates
    StDebtSecTreasLiabs,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; Treasury bills and certificates
    StDebtSecTreasLiabsFoa,
    /// Net U.S. acquisition of other investment assets; short-term deposits
    StDepAssets,
    /// Net U.S. acquistion of other investment assets; short-term deposits; held by central bank
    StDepAssetsCentralBank,
    /// Net U.S. acquistion of other investment assets; short-term deposits; held by deposit-taking institutions except central bank
    StDepAssetsDepTaking,
    /// Net U.S. acquistion of other investment assets; short-term deposits; held by non-deposit-taking institutions and nonfinancial institutions except general government
    StDepAssetsOthFinNonFin,
    /// Net U.S. incurrence of other investment liabilities; short-term deposits
    StDepLiabs,
    /// Net U.S. incurrence of other investment liabilities; short-term deposits; issued by central bank
    StDepLiabsCentralBank,
    /// Net U.S. incurrence of other investment liabilities; short-term deposits; issued by deposit-taking institutions except central bank
    StDepLiabsDepTaking,
    /// Net U.S. incurrence of other investment liabilities; short-term deposits; issued by non-deposit-taking institutions and nonfinancial institutions except general government
    StDepLiabsOthFinNonFin,
    /// Net U.S. acquisition of other investment assets; short-term loans
    StLoansAssets,
    /// Net U.S. acquistion of other investment assets; short-term loans; held by deposit-taking institutions except central bank
    StLoansAssetsDepTaking,
    /// Net U.S. acquistion of other investment assets; short-term loans; held by non-deposit-taking institutions and nonfinancial institutions except general government
    StLoansAssetsOthFinNonFin,
    /// Net U.S. incurrence of other investment liabilities; short-term loans
    StLoansLiabs,
    /// Net U.S. incurrence of other investment liabilities; short-term loans; issued by deposit-taking institutions except central bank
    StLoansLiabsDepTaking,
    /// Net U.S. incurrence of other investment liabilities; short-term loans; issued by non-deposit-taking institutions and nonfinancial institutions except general government
    StLoansLiabsOthFinNonFin,
    /// Net U.S. acquisition of other investment assets; short-term trade credit and advances
    StTrdCredAndAdvAssets,
    /// Net U.S. acquistion of other investment assets; short-term trade credit and advances; held by non-deposit-taking institutions and nonfinancial institutions except general government
    StTrdCredAndAdvAssetsOthFinNonFin,
    /// Net U.S. incurrence of other investment liabilities; short-term trade credit and advances
    StTrdCredAndAdvLiabs,
    /// Net U.S. incurrence of other investment liabilities; short-term trade credit and advances; issued by non-deposit-taking institutions and nonfinancial institutions except general government
    StTrdCredAndAdvLiabsOthFinNonFin,
    /// Net U.S. acquisition of other investment assets; trade credit and advances
    TrdCredAndAdvAssets,
    /// Net U.S. acquistion of other investment assets; trade credit and advances; held by non-deposit-taking institutions and nonfinancial institutions except general government
    TrdCredAndAdvAssetsOthFinNonFin,
    /// Net U.S. incurrence of other investment liabilities; trade credit and advances
    TrdCredAndAdvLiabs,
    /// Net U.S. incurrence of other investment liabilities to foreign official agencies; trade credit and advances
    TrdCredAndAdvLiabsFoa,
    /// Net U.S. incurrence of other investment liabilities; trade credit and advances; issued by general government
    TrdCredAndAdvLiabsGenGovt,
    /// Net U.S. incurrence of other investment liabilities; trade credit and advances; issued by non-deposit-taking institutions and nonfinancial institutions except general government
    TrdCredAndAdvLiabsOthFinNonFin,
    /// Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; Treasury bonds and notes
    TreasBondsAndNotesLiabsFoa,
    /// Direct investment income on inward investment (foreign direct investment in the United States), directional basis
    #[display("TSI_ItaDiInvIncInward")]
    TsiItaDiInvIncInward,
    /// Direct investment income on outward investment (U.S. direct investment abroad), directional basis
    #[display("TSI_ItaDiInvIncOutward")]
    TsiItaDiInvIncOutward,
    /// Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis
    #[display("TSI_ItaDiInvWithoutCurrCostAdjIncOutward")]
    TsiItaDiInvWithoutCurrCostAdjIncOutward,
}

impl Indicator {
    /// Converts BEA parameter value strings into a variant of `Indicator`, handling special cases
    /// where the BEA key is not strictly camel cased.
    pub fn from_value(value: &str) -> Result<Self, KeyMissing> {
        match Indicator::from_str(value) {
            Ok(result) => Ok(result),
            Err(_) => {
                let result = match value {
                    "TSI_ItaDiInvIncInward" => Self::TsiItaDiInvIncInward,
                    "TSI_ItaDiInvIncOutward" => Self::TsiItaDiInvIncOutward,
                    "TSI_ItaDiInvWithoutCurrCostAdjIncOutward" => {
                        Self::TsiItaDiInvWithoutCurrCostAdjIncOutward
                    }
                    _ => {
                        return Err(KeyMissing::new(
                            value.to_owned(),
                            line!(),
                            file!().to_owned(),
                        ));
                    }
                };
                Ok(result)
            }
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::BalCapAcct => "Capital account balance",
            Self::BalCurrAcct => "Current account balance",
            Self::BalGds => "Balance on goods",
            Self::BalGdsServ => "Balance on goods and services",
            Self::BalPrimInc => "Balance on primary income",
            Self::BalSecInc => "Balance on secondary income",
            Self::BalServ => "Balance on services",
            Self::CapTransPayAndOthDeb => "Capital transfers; payments and other debt",
            Self::CapTransRecAndOthCred => "Capital transfers; receipts and other credit",
            Self::CompOfEmplPay => "Compensation of employees; payments",
            Self::CompOfEmplRec => "Compensation of employees; receipts",
            Self::CurrAndDepAssets => "Currency and deposits; assets",
            Self::CurrAndDepAssetsCentralBank => "Currency and deposits; assets; central bank",
            Self::CurrAndDepAssetsDepTaking => {
                "Currency and deposits; assets; deposit-taking institutions"
            }
            Self::CurrAndDepAssetsOthFinNonFin => {
                "Currency and deposits; assets; other financial and nonfinancial institutions"
            }
            Self::CurrAndDepLiabs => "Currency and deposits; liabilities",
            Self::CurrAndDepLiabsCentralBank => "Currency and deposits; liabilities; central bank",
            Self::CurrAndDepLiabsDepTaking => {
                "Currency and deposits; liabilities; deposit-taking institutions"
            }
            Self::CurrAndDepLiabsFoa => "Currency and deposits; liabilities; financial account",
            Self::CurrAndDepLiabsOthFinNonFin => {
                "Currency and deposits; liabilities; other financial and nonfinancial institutions"
            }
            Self::CurrAndDepReserveAssets => "Currency and deposits; reserve assets",
            Self::CurrAssets => "Current assets",
            Self::CurrLiabs => "Current liabilities",
            Self::CurrLiabsCentralBank => "Current liabilities; central bank",
            Self::DebtSecAssets => "Debt securities; assets",
            Self::DebtSecAssetsDepTaking => "Debt securities; assets; deposit-taking institutions",
            Self::DebtSecAssetsNonFin => "Debt securities; assets; nonfinancial institutions",
            Self::DebtSecAssetsOthFin => "Debt securities; assets; other financial institutions",
            Self::DebtSecFedSponsorAgencyIncPay => {
                "Debt securities; federal-sponsored agency income payments"
            }
            Self::DebtSecFedSponsorAgencyLiabs => {
                "Debt securities; federal-sponsored agency liabilities"
            }
            Self::DebtSecIncPay => "Debt securities; income payments",
            Self::DebtSecIncPayDepTaking => {
                "Debt securities; income payments; deposit-taking institutions"
            }
            Self::DebtSecIncPayGenGovt => "Debt securities; income payments; general government",
            Self::DebtSecIncPayNonFin => {
                "Debt securities; income payments; nonfinancial institutions"
            }
            Self::DebtSecIncPayOthFin => {
                "Debt securities; income payments; other financial institutions"
            }
            Self::DebtSecIncRec => "Debt securities; income receipts",
            Self::DebtSecIncRecDepTaking => {
                "Debt securities; income receipts; deposit-taking institutions"
            }
            Self::DebtSecIncRecNonFin => {
                "Debt securities; income receipts; nonfinancial institutions"
            }
            Self::DebtSecIncRecOthFin => {
                "Debt securities; income receipts; other financial institutions"
            }
            Self::DebtSecLiabs => "Debt securities; liabilities",
            Self::DebtSecLiabsDepTaking => {
                "Debt securities; liabilities; deposit-taking institutions"
            }
            Self::DebtSecLiabsFoa => "Debt securities; liabilities; financial account",
            Self::DebtSecLiabsGenGovt => "Debt securities; liabilities; general government",
            Self::DebtSecLiabsNonFin => "Debt securities; liabilities; nonfinancial institutions",
            Self::DebtSecLiabsOthFin => {
                "Debt securities; liabilities; other financial institutions"
            }
            Self::DebtSecOthThanFedSponsorAgencyIncPayOthFin => {
                "Debt securities; other than federal-sponsored agency income payments; other financial institutions"
            }
            Self::DebtSecOthThanFedSponsorAgencyLiabsOthFin => {
                "Debt securities; other than federal-sponsored agency liabilities; other financial institutions"
            }
            Self::DebtSecTreasIncPay => "Debt securities; Treasury income payments",
            Self::DebtSecTreasLiabs => "Debt securities; Treasury liabilities",
            Self::DepAssets => "Deposits; assets",
            Self::DepAssetsCentralBank => "Deposits; assets; central bank",
            Self::DepAssetsDepTaking => "Deposits; assets; deposit-taking institutions",
            Self::DepAssetsOthFinNonFin => {
                "Deposits; assets; other financial and nonfinancial institutions"
            }
            Self::DepLiabs => "Deposits; liabilities",
            Self::DepLiabsCentralBank => "Deposits; liabilities; central bank",
            Self::DepLiabsDepTaking => "Deposits; liabilities; deposit-taking institutions",
            Self::DepLiabsOthFinNonFin => {
                "Deposits; liabilities; other financial and nonfinancial institutions"
            }
            Self::DepRepurchaseLiabsDepTaking => {
                "Deposits; repurchase liabilities; deposit-taking institutions"
            }
            Self::DepResaleAssetsDepTaking => {
                "Deposits; resale assets; deposit-taking institutions"
            }
            Self::DiInvAssets => "Direct investment; assets",
            Self::DiInvAssetsNonSpe => "Direct investment; assets; non-speculative",
            Self::DiInvAssetsSpe => "Direct investment; assets; speculative",
            Self::DiInvCurrCostAdjAssets => "Direct investment; current cost adjustments; assets",
            Self::DiInvCurrCostAdjIncPay => {
                "Direct investment; current cost adjustments; income payments"
            }
            Self::DiInvCurrCostAdjIncRec => {
                "Direct investment; current cost adjustments; income receipts"
            }
            Self::DiInvCurrCostAdjLiabs => {
                "Direct investment; current cost adjustments; liabilities"
            }
            Self::DiInvDebtInstAssets => "Direct investment; debt instruments; assets",
            Self::DiInvDebtInstAssetsNonSpe => {
                "Direct investment; debt instruments; assets; non-speculative"
            }
            Self::DiInvDebtInstAssetsSpe => {
                "Direct investment; debt instruments; assets; speculative"
            }
            Self::DiInvDebtInstInward => "Direct investment; debt instruments; inward",
            Self::DiInvDebtInstInwardByInd => {
                "Direct investment; debt instruments; inward; by industry"
            }
            Self::DiInvDebtInstInwardFinAndIns => {
                "Direct investment; debt instruments; inward; finance and insurance"
            }
            Self::DiInvDebtInstInwardMnfctr => {
                "Direct investment; debt instruments; inward; manufacturing"
            }
            Self::DiInvDebtInstInwardOthInd => {
                "Direct investment; debt instruments; inward; other industries"
            }
            Self::DiInvDebtInstInwardWhlslTrd => {
                "Direct investment; debt instruments; inward; wholesale trade"
            }
            Self::DiInvDebtInstLiabs => "Direct investment; debt instruments; liabilities",
            Self::DiInvDebtInstLiabsNonSpe => {
                "Direct investment; debt instruments; liabilities; non-speculative"
            }
            Self::DiInvDebtInstLiabsSpe => {
                "Direct investment; debt instruments; liabilities; speculative"
            }
            Self::DiInvDebtInstOutward => "Direct investment; debt instruments; outward",
            Self::DiInvDebtInstOutwardByInd => {
                "Direct investment; debt instruments; outward; by industry"
            }
            Self::DiInvDebtInstOutwardFinAndIns => {
                "Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; finance (including depository institutions) and insurance"
            }
            Self::DiInvDebtInstOutwardHoldExcBank => {
                "Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; holding companies except bank holding companies"
            }
            Self::DiInvDebtInstOutwardMnfctr => {
                "Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; manufacturing"
            }
            Self::DiInvDebtInstOutwardOthInd => {
                "Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; other industries (those not listed under acquisition of assets in table 6.1)"
            }
            Self::DiInvDebtInstOutwardWhlslTrd => {
                "Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis; debt instruments; wholesale trade"
            }
            Self::DiInvDebtInstUsAffiliatesClaims => {
                "Financial transactions for direct investment; debt instruments; U.S. affiliates' claims"
            }
            Self::DiInvDebtInstUsAffiliatesClaimsByNonSpe => {
                "Financial transactions for direct investment; U.S. non-SPE affiliates' debt claims transactions with their foreign parent groups"
            }
            Self::DiInvDebtInstUsAffiliatesClaimsBySpe => {
                "Financial transactions for direct investment; U.S. SPE affiliates' debt claims transactions with their foreign parent groups"
            }
            Self::DiInvDebtInstUsAffiliatesClaimsNonSpe => {
                "Financial transactions for direct investment; debt instruments; U.S. affiliates' claims; Non-SPEs"
            }
            Self::DiInvDebtInstUsAffiliatesClaimsSpe => {
                "Financial transactions for direct investment; debt instruments; U.S. affiliates' claims; SPEs"
            }
            Self::DiInvDebtInstUsAffiliatesLiabs => {
                "Financial transactions for direct investment; debt instruments; U.S. affiliates' liabilities"
            }
            Self::DiInvDebtInstUsAffiliatesLiabsNonSpe => {
                "Financial transactions for direct investment; debt instruments; U.S. affiliates' liabilities; Non-SPEs"
            }
            Self::DiInvDebtInstUsAffiliatesLiabsSpe => {
                "Financial transactions for direct investment; debt instruments; U.S. affiliates' liabilities; SPEs"
            }
            Self::DiInvDebtInstUsParentsClaims => {
                "Financial transactions for direct investment; debt instruments; U.S. parents' claims"
            }
            Self::DiInvDebtInstUsParentsClaimsNonSpe => {
                "Financial transactions for direct investment; debt instruments; U.S. parents' claims; Non-SPEs"
            }
            Self::DiInvDebtInstUsParentsClaimsSpe => {
                "Financial transactions for direct investment; debt instruments; U.S. parents' claims; SPEs"
            }
            Self::DiInvDebtInstUsParentsLiabs => {
                "Financial transactions for direct investment; debt instruments; U.S. parents' liabilities"
            }
            Self::DiInvDebtInstUsParentsLiabsNonSpe => {
                "Financial transactions for direct investment; debt instruments; U.S. parents' liabilities; Non-SPEs"
            }
            Self::DiInvDebtInstUsParentsLiabsSpe => {
                "Financial transactions for direct investment; debt instruments; U.S. parents' liabilities; SPEs"
            }
            Self::DiInvDebtInstUsParentsLiabsWithNonSpe => {
                "Financial transactions for direct investment; U.S. parents' debt liabilities transactions with their foreign non-SPE affiliates"
            }
            Self::DiInvDebtInstUsParentsLiabsWithSpe => {
                "Financial transactions for direct investment; U.S. parents' debt liabilities transactions with their foreign SPE affiliates"
            }
            Self::DiInvDirectionalBasisAdj => {
                "Financial transactions for direct investment; adjustments to convert to directional basis"
            }
            Self::DiInvDirectionalBasisAdjIncPay => {
                "Direct investment income payments; adjustments to convert to directional basis"
            }
            Self::DiInvDirectionalBasisAdjIncRec => {
                "Direct investment income receipts; adjustments to convert to directional basis"
            }
            Self::DiInvDivWithdrawIncPay => {
                "Direct investment income on liabilities; dividends and withdrawals"
            }
            Self::DiInvDivWithdrawIncPayNonSpe => {
                "Direct investment income on liabilities; dividends and withdrawals; Non-SPEs"
            }
            Self::DiInvDivWithdrawIncPaySpe => {
                "Direct investment income on liabilities; dividends and withdrawals; SPEs"
            }
            Self::DiInvDivWithdrawIncRec => {
                "Direct investment income on assets; dividends and withdrawals"
            }
            Self::DiInvDivWithdrawIncRecNonSpe => {
                "Direct investment income on assets; dividends and withdrawals; Non-SPEs"
            }
            Self::DiInvDivWithdrawIncRecSpe => {
                "Direct investment income on assets; dividends and withdrawals; SPEs"
            }
            Self::DiInvEquityAssets => "Net U.S. acquisition of direct investment assets; equity",
            Self::DiInvEquityAssetsNonSpe => {
                "Net U.S. acquisition of direct investment assets; equity; Non-SPEs"
            }
            Self::DiInvEquityAssetsSpe => {
                "Net U.S. acquisition of direct investment assets; equity; SPEs"
            }
            Self::DiInvEquityIncPay => "Direct investment income on liabilities; equity",
            Self::DiInvEquityIncPayNonSpe => {
                "Direct investment income on liabilities; equity; Non-SPEs"
            }
            Self::DiInvEquityIncPaySpe => "Direct investment income on liabilities; equity; SPEs",
            Self::DiInvEquityIncRec => "Direct investment income on assets; equity",
            Self::DiInvEquityIncRecNonSpe => "Direct investment income on assets; equity; Non-SPEs",
            Self::DiInvEquityIncRecSpe => "Direct investment income on assets; equity; SPEs",
            Self::DiInvEquityIntIncRecFinAndIns => {
                "Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; finance (including depository institutions) and insurance"
            }
            Self::DiInvEquityIntIncRecHoldExcBank => {
                "Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; holding companies except bank holding companies"
            }
            Self::DiInvEquityIntIncRecMnfctr => {
                "Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; manufacturing"
            }
            Self::DiInvEquityIntIncRecOthInd => {
                "Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; manufacturing; other industries (those not listed under receipts in table 4.2)"
            }
            Self::DiInvEquityIntIncRecWhlslTrd => {
                "Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts; wholesale trade"
            }
            Self::DiInvEquityLiabs => {
                "Net U.S. incurrence of direct investment liabilities; equity"
            }
            Self::DiInvEquityLiabsNonSpe => {
                "Net U.S. incurrence of direct investment liabilities; equity; Non-SPEs"
            }
            Self::DiInvEquityLiabsSpe => {
                "Net U.S. incurrence of direct investment liabilities; equity; SPEs"
            }
            Self::DiInvEquityOthThanReinvestEarnAssets => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings"
            }
            Self::DiInvEquityOthThanReinvestEarnAssetsFinAndIns => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; finance (including depository institutions) and insurance"
            }
            Self::DiInvEquityOthThanReinvestEarnAssetsHoldExcBank => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; holding companies except bank holding companies"
            }
            Self::DiInvEquityOthThanReinvestEarnAssetsMnfctr => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; manufacturing"
            }
            Self::DiInvEquityOthThanReinvestEarnAssetsNonSpe => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; Non-SPEs"
            }
            Self::DiInvEquityOthThanReinvestEarnAssetsOthInd => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; other industries (those not listed under acquisition of assets in table 6.1)"
            }
            Self::DiInvEquityOthThanReinvestEarnAssetsSpe => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; SPEs"
            }
            Self::DiInvEquityOthThanReinvestEarnAssetsWhlslTrd => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; wholesale trade"
            }
            Self::DiInvEquityOthThanReinvestEarnDecAssets => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; decreases"
            }
            Self::DiInvEquityOthThanReinvestEarnDecLiabs => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; decreases"
            }
            Self::DiInvEquityOthThanReinvestEarnIncAssets => {
                "Net U.S. acquisition of direct investment assets; equity other than reinvestment of earnings; increases"
            }
            Self::DiInvEquityOthThanReinvestEarnIncLiabs => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; increases"
            }
            Self::DiInvEquityOthThanReinvestEarnLiabs => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings"
            }
            Self::DiInvEquityOthThanReinvestEarnLiabsFinAndIns => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; finance (including depository institutions) and insurance"
            }
            Self::DiInvEquityOthThanReinvestEarnLiabsHoldExcBank => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; other industries (those not listed under incurrence of liabilities in table 6.1)"
            }
            Self::DiInvEquityOthThanReinvestEarnLiabsMnfctr => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; manufacturing"
            }
            Self::DiInvEquityOthThanReinvestEarnLiabsNonSpe => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; Non-SPEs"
            }
            Self::DiInvEquityOthThanReinvestEarnLiabsSpe => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; SPEs"
            }
            Self::DiInvEquityOthThanReinvestEarnLiabsWhlslTrd => {
                "Net U.S. incurrence of direct investment liabilities; equity other than reinvestment of earnings; wholesale trade"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncPay => {
                "Direct investment income without current cost adjustment on liabilities; equity"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncPayFinAndIns => {
                "Direct investment income without current cost adjustment on liabilities; equity; finance (including depository institutions) and insurance"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncPayMnfctr => {
                "Direct investment income without current cost adjustment on liabilities; equity; manufacturing"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncPayOthInd => {
                "Direct investment income without current cost adjustment on liabilities; equity; other industries (those not listed under receipts in table 4.2)"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncPayWhlslTrd => {
                "Direct investment income without current cost adjustment on liabilities; equity; wholesale trade"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncRec => {
                "Direct investment income without current cost adjustment on assets; equity"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncRecFinAndIns => {
                "Direct investment income without current cost adjustment on assets; equity; finance (including depository institutions) and insurance"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncRecHoldExcBank => {
                "Direct investment income without current cost adjustment on assets; equity; holding companies except bank holding companies"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncRecMnfctr => {
                "Direct investment income without current cost adjustment on assets; equity; manufacturing"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncRecOthInd => {
                "Direct investment income without current cost adjustment on assets; equity; manufacturing; other industries (those not listed under receipts in table 4.2)"
            }
            Self::DiInvEquityWithoutCurrCostAdjIncRecWhlslTrd => {
                "Direct investment income without current cost adjustment on assets; equity; wholesale trade"
            }
            Self::DiInvIncPay => "Direct investment income on liabilities, asset/liability basis",
            Self::DiInvIncPayNonSpe => {
                "Direct investment income on liabilities, asset/liability basis; Non-SPEs"
            }
            Self::DiInvIncPaySpe => {
                "Direct investment income on liabilities, asset/liability basis; SPEs"
            }
            Self::DiInvIncRec => "Direct investment income on assets, asset/liability basis",
            Self::DiInvIncRecNonSpe => {
                "Direct investment income on assets, asset/liability basis; Non-SPEs"
            }
            Self::DiInvIncRecSpe => {
                "Direct investment income on assets, asset/liability basis; SPEs"
            }
            Self::DiInvIntIncInward => {
                "Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments"
            }
            Self::DiInvIntIncInwardFinAndIns => {
                "Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments; finance (including depository institutions) and insurance"
            }
            Self::DiInvIntIncInwardMnfctr => {
                "Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments; manufacturing"
            }
            Self::DiInvIntIncInwardOthInd => {
                "Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments; other industries (those not listed under receipts in table 4.2)"
            }
            Self::DiInvIntIncInwardWhlslTrd => {
                "Direct investment income on inward investment (foreign direct investment in the United States), directional basis; interest, net payments; wholesale trade"
            }
            Self::DiInvIntIncOutward => {
                "Direct investment income on outward investment (U.S. direct investment abroad), directional basis; interest, net receipts"
            }
            Self::DiInvIntIncPay => {
                "Direct investment income on liabilities, asset/liability basis; interest"
            }
            Self::DiInvIntIncPayNonSpe => {
                "Direct investment income on liabilities, asset/liability basis; interest; Non-SPEs"
            }
            Self::DiInvIntIncPaySpe => {
                "Direct investment income on liabilities, asset/liability basis; interest; SPEs"
            }
            Self::DiInvIntIncRec => {
                "Direct investment income on assets, asset/liability basis; interest"
            }
            Self::DiInvIntIncRecNonSpe => {
                "Direct investment income on assets, asset/liability basis; interest; Non-SPEs"
            }
            Self::DiInvIntIncRecSpe => {
                "Direct investment income on assets, asset/liability basis; interest; SPEs"
            }
            Self::DiInvIntUsAffiliatesIncPay => {
                "Direct investment income; U.S. affiliates' interest payments"
            }
            Self::DiInvIntUsAffiliatesIncPayNonSpe => {
                "Direct investment income; U.S. affiliates' interest payments; Non-SPEs"
            }
            Self::DiInvIntUsAffiliatesIncPaySpe => {
                "Direct investment income; U.S. affiliates' interest payments; SPEs"
            }
            Self::DiInvIntUsAffiliatesIncRec => {
                "Direct investment income; U.S. affiliates' interest receipts"
            }
            Self::DiInvIntUsAffiliatesIncRecNonSpe => {
                "Direct investment income; U.S. affiliates' receipts from their foreign parent groups; Non-SPEs"
            }
            Self::DiInvIntUsAffiliatesIncRecOfNonSpe => {
                "Direct investment income; U.S. non-SPE affiliates' interest receipts from their foreign parent groups"
            }
            Self::DiInvIntUsAffiliatesIncRecOfSpe => {
                "Direct investment income; U.S. non-SPE affiliates' interest receipts from their foreign parent groups"
            }
            Self::DiInvIntUsAffiliatesIncRecSpe => {
                "Direct investment income; U.S. affiliates' receipts from their foreign parent groups; SPEs"
            }
            Self::DiInvIntUsParentsIncPay => {
                "Direct investment income; U.S. parents' interest payments"
            }
            Self::DiInvIntUsParentsIncPayNonSpe => {
                "Direct investment income; U.S. parents' payments to their foreign affiliates; Non-SPEs"
            }
            Self::DiInvIntUsParentsIncPaySpe => {
                "Direct investment income; U.S. parents' payments to their foreign affiliates; SPEs"
            }
            Self::DiInvIntUsParentsIncPayToNonSpe => {
                "Direct investment income; U.S. parents' interest payments to their foreign non-SPE affiliates"
            }
            Self::DiInvIntUsParentsIncPayToSpe => {
                "Direct investment income; U.S. parents' interest payments to their foreign SPE affiliates"
            }
            Self::DiInvIntUsParentsIncRec => {
                "Direct investment income; U.S. parents' interest receipts"
            }
            Self::DiInvIntUsParentsIncRecNonSpe => {
                "Direct investment income; U.S. parents' interest receipts; Non-SPEs"
            }
            Self::DiInvIntUsParentsIncRecSpe => {
                "Direct investment income; U.S. parents' interest receipts; SPEs"
            }
            Self::DiInvInwardDirectionalBasis => {
                "Financial transactions for inward direct investment (foreign direct investment in the United States), directional basis"
            }
            Self::DiInvLiabs => {
                "Net U.S. incurrence of direct investment liabilities, asset/liability basis"
            }
            Self::DiInvLiabsNonSpe => {
                "Net U.S. incurrence of direct investment liabilities, asset/liability basis; Non-SPEs"
            }
            Self::DiInvLiabsSpe => {
                "Net U.S. incurrence of direct investment liabilities, asset/liability basis; SPEs"
            }
            Self::DiInvOutward => {
                "Financial transactions for outward direct investment (U.S. direct investment abroad), directional basis"
            }
            Self::DiInvReinvestEarnAssets => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings"
            }
            Self::DiInvReinvestEarnAssetsNonSpe => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings; Non-SPEs"
            }
            Self::DiInvReinvestEarnAssetsSpe => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings; SPEs"
            }
            Self::DiInvReinvestEarnIncPay => {
                "Direct investment income on liabilities; reinvested earnings"
            }
            Self::DiInvReinvestEarnIncPayNonSpe => {
                "Direct investment income on liabilities; reinvested earnings; Non-SPEs"
            }
            Self::DiInvReinvestEarnIncPaySpe => {
                "Direct investment income on liabilities; reinvested earnings; SPEs"
            }
            Self::DiInvReinvestEarnIncRec => {
                "Direct investment income on assets; reinvested earnings"
            }
            Self::DiInvReinvestEarnIncRecNonSpe => {
                "Direct investment income on assets; reinvested earnings; Non-SPEs"
            }
            Self::DiInvReinvestEarnIncRecSpe => {
                "Direct investment income on assets; reinvested earnings; SPEs"
            }
            Self::DiInvReinvestEarnLiabs => {
                "Net U.S. incurrence of direct investment liabilities; reinvestment of earnings"
            }
            Self::DiInvReinvestEarnLiabsNonSpe => {
                "Net U.S. incurrence of direct investment liabilities; reinvestment of earnings; Non-SPEs"
            }
            Self::DiInvReinvestEarnLiabsSpe => {
                "Net U.S. incurrence of direct investment liabilities; reinvestment of earnings; SPEs"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjAssets => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjAssetsFinAndIns => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; finance (including depository institutions) and insurance"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjAssetsHoldExcBank => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; holding companies except bank holding companies"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjAssetsMnfctr => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; manufacturing"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjAssetsOthInd => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; other industries (those not listed under acquisition of assets in table 6.1)"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjAssetsWhlslTrd => {
                "Net U.S. acquisition of direct investment assets; reinvestment of earnings without current-cost adjustment; wholesale trade"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjIncPay => {
                "Direct investment income on liabilities; reinvested earnings without current-cost adjustment"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjIncRec => {
                "Direct investment income on assets; reinvested earnings without current-cost adjustment"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjLiabs => {
                "Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjLiabsFinAndIns => {
                "Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment; finance (including depository institutions) and insurance"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjLiabsMnfctr => {
                "Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment; manufacturing"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjLiabsOthInd => {
                "Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment; other industries (those not listed under incurrence of liabilities in table 6.1)"
            }
            Self::DiInvReinvestEarnWithoutCurrCostAdjLiabsWhlslTrd => {
                "Net U.S. incurrence of direct investment liabilities; reinvestment of earnings without current-cost adjustment; wholesale trade"
            }
            Self::DiInvWithoutCurrCostAdjIncInward => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardChem => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; chemicals"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardCompElecProd => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; computers and electronic products"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardDepIns => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; depository institutions "
            }
            Self::DiInvWithoutCurrCostAdjIncInwardElectrical => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; electrical equipment, appliances, and components"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardFinAndIns => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; finance (including depository institutions) and insurance"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardFinExclDepInsAndIns => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; finance (including depository institutions) and insurance"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardFood => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; food"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardInfo => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; information"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardMachinery => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; machinery"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardMnfctr => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; manufacturing"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardMnfctrOth => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; other manufacturing"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardOthInd => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; other industries (those not listed under receipts in table 4.2)"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardOthIndExcl8DiInward => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; other industries (excluding 8 industry groups)"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardPrimFabMtls => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; primary and fabricated metals"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardProfSciAndTech => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; professional, scientific, and technical services"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardRealEstRentLeas => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; real estate and rental and leasing"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardRtlTrd => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; retail trade"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardTransEquip => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; transportation equipment"
            }
            Self::DiInvWithoutCurrCostAdjIncInwardWhlslTrd => {
                "Direct investment income without current-cost adjustment on inward investment, directional basis; wholesale trade"
            }
            Self::DiInvWithoutCurrCostAdjIncOutward => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardChem => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; chemicals"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardCompElecProd => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; computers and electronic products"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardDepIns => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; depository institutions "
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardElectrical => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; electrical equipment, appliances, and components"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardFinAndIns => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; finance (including depository institutions) and insurance"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardFinExclDepInsAndIns => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; finance (including depository institutions) and insurance"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardFood => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; food"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardHoldExcBank => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; holding companies except bank holding companies"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardInfo => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; information"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardMachinery => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; machinery"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardMining => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; mining"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardMnfctr => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; manufacturing"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardMnfctrOth => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; other manufacturing"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardOthInd => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; manufacturing; other industries (those not listed under receipts in table 4.2)"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardOthIndExcl8DiOutward => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; other industries (excluding 8 industry groups)"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardPrimFabMtls => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; primary and fabricated metals"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardProfSciAndTech => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; professional, scientific, and technical services"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardTransEquip => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; transportation equipment"
            }
            Self::DiInvWithoutCurrCostAdjIncOutwardWhlslTrd => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis; wholesale trade"
            }
            Self::DiInvWithoutCurrCostAdjInward => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis"
            }
            Self::DiInvWithoutCurrCostAdjInwardChem => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; chemicals"
            }
            Self::DiInvWithoutCurrCostAdjInwardCompElecProd => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; computers and electronic products"
            }
            Self::DiInvWithoutCurrCostAdjInwardDepIns => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; depository institutions "
            }
            Self::DiInvWithoutCurrCostAdjInwardElectrical => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; electrical equipment, appliances, and components"
            }
            Self::DiInvWithoutCurrCostAdjInwardFinAndIns => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; finance (including depository institutions) and insurance"
            }
            Self::DiInvWithoutCurrCostAdjInwardFinExclDepInsAndIns => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; finance (including depository institutions) and insurance"
            }
            Self::DiInvWithoutCurrCostAdjInwardFood => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; food"
            }
            Self::DiInvWithoutCurrCostAdjInwardInfo => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; information"
            }
            Self::DiInvWithoutCurrCostAdjInwardMachinery => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; machinery"
            }
            Self::DiInvWithoutCurrCostAdjInwardMnfctr => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; manufacturing"
            }
            Self::DiInvWithoutCurrCostAdjInwardMnfctrOth => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; other manufacturing"
            }
            Self::DiInvWithoutCurrCostAdjInwardOthInd => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; other industries (those not listed under incurrence of liabilities in table 6.1)"
            }
            Self::DiInvWithoutCurrCostAdjInwardOthIndExcl8DiInward => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; other industries (excluding 8 industry groups)"
            }
            Self::DiInvWithoutCurrCostAdjInwardPrimFabMtls => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; primary and fabricated metals"
            }
            Self::DiInvWithoutCurrCostAdjInwardProfSciAndTech => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; professional, scientific, and technical services"
            }
            Self::DiInvWithoutCurrCostAdjInwardRealEstRentLeas => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; real estate and rental and leasing"
            }
            Self::DiInvWithoutCurrCostAdjInwardRtlTrd => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; retail trade"
            }
            Self::DiInvWithoutCurrCostAdjInwardTransEquip => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; transportation equipment"
            }
            Self::DiInvWithoutCurrCostAdjInwardWhlslTrd => {
                "Financial transactions without current-cost adjustment for inward direct investment (foreign direct investment in the United States), directional basis; wholesale trade"
            }
            Self::DiInvWithoutCurrCostAdjOutward => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis"
            }
            Self::DiInvWithoutCurrCostAdjOutwardChem => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; chemicals"
            }
            Self::DiInvWithoutCurrCostAdjOutwardCompElecProd => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; computers and electronic products"
            }
            Self::DiInvWithoutCurrCostAdjOutwardDepIns => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; depository institutions "
            }
            Self::DiInvWithoutCurrCostAdjOutwardElectrical => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; electrical equipment, appliances, and components"
            }
            Self::DiInvWithoutCurrCostAdjOutwardFinAndIns => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; finance (including depository institutions) and insurance"
            }
            Self::DiInvWithoutCurrCostAdjOutwardFinExclDepInsAndIns => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; finance (including depository institutions) and insurance"
            }
            Self::DiInvWithoutCurrCostAdjOutwardFood => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; food"
            }
            Self::DiInvWithoutCurrCostAdjOutwardHoldExcBank => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; holding companies except bank holding companies"
            }
            Self::DiInvWithoutCurrCostAdjOutwardInfo => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; information"
            }
            Self::DiInvWithoutCurrCostAdjOutwardMachinery => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; machinery"
            }
            Self::DiInvWithoutCurrCostAdjOutwardMining => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; mining"
            }
            Self::DiInvWithoutCurrCostAdjOutwardMnfctr => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; manufacturing"
            }
            Self::DiInvWithoutCurrCostAdjOutwardMnfctrOth => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; other manufacturing"
            }
            Self::DiInvWithoutCurrCostAdjOutwardOthInd => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; other industries (those not listed under acquisition of assets in table 6.1)"
            }
            Self::DiInvWithoutCurrCostAdjOutwardOthIndExcl8DiOutward => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; other industries (excluding 8 industry groups)"
            }
            Self::DiInvWithoutCurrCostAdjOutwardPrimFabMtls => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; primary and fabricated metals"
            }
            Self::DiInvWithoutCurrCostAdjOutwardProfSciAndTech => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; professional, scientific, and technical services"
            }
            Self::DiInvWithoutCurrCostAdjOutwardTransEquip => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; transportation equipment"
            }
            Self::DiInvWithoutCurrCostAdjOutwardWhlslTrd => {
                "Financial transactions without current-cost adjustment for outward direct investment (U.S. direct investment abroad), directional basis; wholesale trade"
            }
            Self::EquityAndInvFundSharesAssets => {
                "Net U.S. acquisition of portfolio investment assets; equity and investment fund shares"
            }
            Self::EquityAndInvFundSharesAssetsDepTaking => {
                "Net U.S. acquisition of portfolio investment assets; equity and investment fund shares; held by deposit-taking institutions except central bank"
            }
            Self::EquityAndInvFundSharesAssetsNonFin => {
                "Net U.S. acquisition of portfolio investment assets; equity and investment fund shares; held by nonfinancial institutions except general government"
            }
            Self::EquityAndInvFundSharesAssetsOthFin => {
                "Net U.S. acquisition of portfolio investment assets; equity and investment fund shares; held by non-deposit-taking financial institutions"
            }
            Self::EquityAndInvFundSharesIncPay => {
                "Portfolio investment income payments; income on equity and investment fund shares"
            }
            Self::EquityAndInvFundSharesIncPayDepTaking => {
                "Portfolio investment income payments on equity and investment fund shares; deposit-taking institutions except central bank"
            }
            Self::EquityAndInvFundSharesIncPayNonFin => {
                "Portfolio investment income payments on equity and investment fund shares; nonfinancial institutions except general government"
            }
            Self::EquityAndInvFundSharesIncPayOthFin => {
                "Portfolio investment income payments on equity and investment fund shares; non-deposit-taking financial institutions"
            }
            Self::EquityAndInvFundSharesIncRec => {
                "Portfolio investment income receipts; income on equity and investment fund shares"
            }
            Self::EquityAndInvFundSharesIncRecDepTaking => {
                "Portfolio investment income receipts on equity and investment fund shares; deposit-taking institutions except central bank"
            }
            Self::EquityAndInvFundSharesIncRecNonFin => {
                "Portfolio investment income receipts on equity and investment fund shares; nonfinancial institutions except general government"
            }
            Self::EquityAndInvFundSharesIncRecOthFin => {
                "Portfolio investment income receipts on equity and investment fund shares; non-deposit-taking financial institutions"
            }
            Self::EquityAndInvFundSharesLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; equity and investment fund shares"
            }
            Self::EquityAndInvFundSharesLiabsDepTaking => {
                "Net U.S. incurrence of portfolio investment liabilities; equity and investment fund shares; issued by deposit-taking institutions except central bank"
            }
            Self::EquityAndInvFundSharesLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; equity and investment fund shares"
            }
            Self::EquityAndInvFundSharesLiabsNonFin => {
                "Net U.S. incurrence of portfolio investment liabilities; equity and investment fund shares; issued by nonfinancial institutions except general government"
            }
            Self::EquityAndInvFundSharesLiabsOthFin => {
                "Net U.S. incurrence of portfolio investment liabilities; equity and investment fund shares; issued by non-deposit-taking financial institutions"
            }
            Self::EquityOthThanInvFundSharesAssets => {
                "Net U.S. acquisition of portfolio investment assets; equity other than investment fund shares"
            }
            Self::EquityOthThanInvFundSharesIncPay => {
                "Portfolio investment income payments; dividends on equity other than investment fund shares"
            }
            Self::EquityOthThanInvFundSharesIncRec => {
                "Portfolio investment income receipts; dividends on equity other than investment fund shares"
            }
            Self::EquityOthThanInvFundSharesLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; equity other than investment fund shares"
            }
            Self::EquityOthThanInvFundSharesLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; equity excluding investment fund shares"
            }
            Self::ExpGds => "Exports of goods",
            Self::ExpGdsAgFoodsFeedsAndBevs => {
                "Exports of agricultural foods, feeds, and beverages"
            }
            Self::ExpGdsAgIsm => "Exports of agricultural industrial supplies and materials",
            Self::ExpGdsAppFootAndHouse => "Exports of apparel, footwear, and household goods",
            Self::ExpGdsAutoEngAndEngParts => "Exports of automotive engines and engine parts",
            Self::ExpGdsAutoVehPartsAndEngines => {
                "Exports of automotive vehicles, parts, and engines"
            }
            Self::ExpGdsBauxAndAlum => "Exports of bauxite and aluminum",
            Self::ExpGdsBopAdj => "Exports of goods; balance of payments adjustments, net",
            Self::ExpGdsBuildMatsExcMetals => "Exports of building materials except metals",
            Self::ExpGdsCapGoodsExclAuto => "Exports of capital goods except automotive",
            Self::ExpGdsCensus => "Exports of goods, Census basis",
            Self::ExpGdsChemsExcMeds => "Exports of chemicals except medicinals",
            Self::ExpGdsCivAir => "Exports of civilian aircraft, complete, all types",
            Self::ExpGdsCivAirEngAndParts => "Exports of civilian aircraft, engines, and parts",
            Self::ExpGdsCoalAndRelProds => "Exports of coal and related products",
            Self::ExpGdsComp => "Exports of computers",
            Self::ExpGdsCompAccPeriAndParts => {
                "Exports of computer accessories, peripherals, and parts"
            }
            Self::ExpGdsConsGoodsExcFoodAndAuto => {
                "Exports of consumer goods except food and automotive"
            }
            Self::ExpGdsCopper => "Exports of copper",
            Self::ExpGdsCorn => "Exports of corn",
            Self::ExpGdsCrudePet => "Exports of crude petroleum",
            Self::ExpGdsDistBevAndOthNonAgFoodsFeedsAndBevs => {
                "Exports of distilled beverages and other nonagricultural foods, feeds, and beverages"
            }
            Self::ExpGdsDurCons => "Exports of durable consumer goods",
            Self::ExpGdsElecGenMachElecAppAndParts => {
                "Exports of electric-generating machinery, electric apparatus, and parts"
            }
            Self::ExpGdsEnergyProd => "Exports of energy products",
            Self::ExpGdsEngAndPartsForCivAir => {
                "Exports of engines and parts for civilian aircraft"
            }
            Self::ExpGdsFertPestAndInsect => "Exports of fertilizers, pesticides, and insecticides",
            Self::ExpGdsFishShellfish => "Exports of fish and shellfish",
            Self::ExpGdsFoodsFeedsAndBevs => "Exports of foods, feeds, and beverages",
            Self::ExpGdsFuelOil => "Exports of fuel oil",
            Self::ExpGdsGdsProcPortsBopAdj => {
                "Exports of goods; balance of payments adjustments, net; goods procured in U.S. ports by foreign carriers"
            }
            Self::ExpGdsGemDiamAndOthGem => "Exports of gem diamonds and other gemstones",
            Self::ExpGdsGenMerch => "Exports of general merchandise",
            Self::ExpGdsGrainsPreps => "Exports of grains and preparations",
            Self::ExpGdsHidesSkins => "Exports of hides and skins, including furskins",
            Self::ExpGdsHouseAndKitchApp => "Exports of household and kitchen appliances",
            Self::ExpGdsHouseFurnAndRelProds => {
                "Exports of household furnishings and related products"
            }
            Self::ExpGdsHouseKitchAppAndOthHouse => {
                "Exports of household and kitchen appliances and other household goods"
            }
            Self::ExpGdsIndEngPumpsComps => "Exports of industrial engines, pumps, and compressors",
            Self::ExpGdsIndInorgChems => "Exports of industrial inorganic chemicals",
            Self::ExpGdsIndOrgChems => "Exports of industrial organic chemicals",
            Self::ExpGdsIronAndSteelProds => "Exports of iron and steel products",
            Self::ExpGdsIsm => "Exports of industrial supplies and materials",
            Self::ExpGdsJewelryAndCollect => "Exports of jewelry and collectibles",
            Self::ExpGdsLiqPetGases => "Exports of liquified petroleum gases",
            Self::ExpGdsMachAndEquipExcCons => {
                "Exports of machinery and equipment except consumer-type"
            }
            Self::ExpGdsMachToolsMetalworkMach => {
                "Exports of machine tools and metalworking machinery"
            }
            Self::ExpGdsMeasTestControlInst => {
                "Exports of measuring, testing, and control instruments"
            }
            Self::ExpGdsMeatProdsPoultry => "Exports of meat products and poultry",
            Self::ExpGdsMedDentAndPharm => {
                "Exports of medicinal, dental, and pharmaceutical products"
            }
            Self::ExpGdsMerchantingBopAdj => {
                "Exports of goods; balance of payments adjustments, net; net exports of goods under merchanting"
            }
            Self::ExpGdsMerchantingNet => "Net exports of goods under merchanting",
            Self::ExpGdsMetalsAndNonmetProds => "Exports of metals and nonmetallic products",
            Self::ExpGdsNaturalGas => "Exports of natural gas",
            Self::ExpGdsNonAgFoodsFeedsAndBevs => {
                "Exports of nonagricultural foods, feeds, and beverages"
            }
            Self::ExpGdsNonAgIsm => "Exports of nonagricultural industrial supplies and materials",
            Self::ExpGdsNondurCons => "Exports of nondurable consumer goods",
            Self::ExpGdsNonferrousMetals => "Exports of nonferrous metals",
            Self::ExpGdsNonmonetaryGold => "Exports of nonmonetary gold",
            Self::ExpGdsNonmonGoldBopAdj => {
                "Exports of goods; balance of payments adjustments, net; nonmonetary gold"
            }
            Self::ExpGdsNuclearFuelAndElecEnergy => "Exports of nuclear fuel and electric energy",
            Self::ExpGdsOilDrillMiningConstMach => {
                "Exports of oil-drilling, mining, and construction machinery"
            }
            Self::ExpGdsOthAgFoodsFeedsAndBevs => {
                "Exports of other agricultural foods, feeds, and beverages"
            }
            Self::ExpGdsOthAgIsm => "Exports of other agricultural industrial supplies",
            Self::ExpGdsOthAutoPartsAndAcc => "Exports of other automotive parts and accessories",
            Self::ExpGdsOthBopAdj => {
                "Exports of goods; balance of payments adjustments, net; other adjustments, net"
            }
            Self::ExpGdsOthChems => "Exports of other chemicals",
            Self::ExpGdsOthDurCons => "Exports of other durable consumer goods",
            Self::ExpGdsOthFeeds => "Exports of other feeds",
            Self::ExpGdsOthGenMerch => "Exports of other general merchandise",
            Self::ExpGdsOthHouseIncCellPhones => {
                "Exports of other household goods, including cell phones"
            }
            Self::ExpGdsOthIndMach => "Exports of other industrial machinery",
            Self::ExpGdsOthMetalsAndNonmetProds => {
                "Exports of other metals and nonmetallic products"
            }
            Self::ExpGdsOthNondurCons => "Exports of other nondurable consumer goods",
            Self::ExpGdsOthNonferrousMetals => "Exports of other nonferrous metals",
            Self::ExpGdsOthNonmetals => "Exports of other nonmetals",
            Self::ExpGdsOthOfficeAndBusMach => "Exports of other office and business machines",
            Self::ExpGdsOthPetProds => "Exports of other petroleum products",
            Self::ExpGdsOthServIndAndAgMach => {
                "Exports of other service-industry and agricultural machinery"
            }
            Self::ExpGdsOthTransEquip => "Exports of other transportation equipment",
            Self::ExpGdsPaperAndPaperBaseStocks => "Exports of paper and paper-base stocks",
            Self::ExpGdsPassCars => "Exports of passenger cars, new and used",
            Self::ExpGdsPetAndProds => "Exports of petroleum and products",
            Self::ExpGdsPlasticMaterials => "Exports of plastic materials",
            Self::ExpGdsPrecMetalsExcNonmonGold => {
                "Exports of precious metals except nonmonetary gold"
            }
            Self::ExpGdsPrivGiftParcelRemitBopAdj => {
                "Exports of goods; balance of payments adjustments, net; private gift parcel remittances"
            }
            Self::ExpGdsRadioAndStereoEquip => {
                "Exports of radio and stereo equipment, including recorded media"
            }
            Self::ExpGdsRawCotton => "Exports of raw cotton",
            Self::ExpGdsRepairEquipBopAdj => {
                "Exports of goods; balance of payments adjustments, net; repair of equipment"
            }
            Self::ExpGdsRiceOthFoodGrains => "Exports of rice and other food grains",
            Self::ExpGdsSciHospAndMedEquipAndParts => {
                "Exports of scientific, hospital, and medical equipment and parts"
            }
            Self::ExpGdsSemiconductors => "Exports of semiconductors",
            Self::ExpGdsServ => "Exports of goods and services",
            Self::ExpGdsServIncRec => "Exports of goods and services and income receipts (credits)",
            Self::ExpGdsSoybeans => "Exports of soybeans",
            Self::ExpGdsSteelmakingMats => "Exports of steelmaking materials",
            Self::ExpGdsTelecomEquip => "Exports of telecommunications equipment",
            Self::ExpGdsTextileSupAndRelMats => "Exports of textile supplies and related materials",
            Self::ExpGdsToilAndCosmet => "Exports of toiletries and cosmetics",
            Self::ExpGdsToysAndSport => "Exports of toys and sporting goods, including bicycles",
            Self::ExpGdsTrucksBusesSpecPurpVeh => {
                "Exports of trucks, buses, and special purpose vehicles"
            }
            Self::ExpGdsTvsVidRecAndOthVidEquip => {
                "Exports of televisions, video receivers, and other video equipment"
            }
            Self::ExpGdsUnmanufTobacco => "Exports of unmanufactured tobacco",
            Self::ExpGdsUsMilAgencyBopAdj => {
                "Exports of goods; balance of payments adjustments, net; exports under U.S. military agency sales contracts"
            }
            Self::ExpGdsVegFruitNutPreps => "Exports of vegetables, fruits, nuts, and preparations",
            Self::ExpGdsWheat => "Exports of wheat",
            Self::ExpGenMerch => "Exports of general merchandise",
            Self::ExpServ => "Exports of services",
            Self::ExpServArtisticRelated => "Exports of artistic related services",
            Self::ExpServAudVis => "Exports of audiovisual services",
            Self::ExpServChargesForTheUseOfIpNie => {
                "Charges for the use of intellectual property n.i.e.; exports"
            }
            Self::ExpServCipLicensesAudVis => {
                "Charges for the use of intellectual property n.i.e.; exports; licenses to reproduce and/or distribute audiovisual products"
            }
            Self::ExpServCipLicensesCompSoftware => {
                "Charges for the use of intellectual property n.i.e.; exports; licenses to reproduce and/or distribute computer software"
            }
            Self::ExpServCipLicensesFranchisesTrademarks => {
                "Charges for the use of intellectual property n.i.e.; exports; franchises and trademarks licensing fees"
            }
            Self::ExpServCipLicensesOutcomesResearchAndDev => {
                "Charges for the use of intellectual property n.i.e.; exports; licenses for the use of outcomes of research and development"
            }
            Self::ExpServComp => "Exports of computer services",
            Self::ExpServConst => "Exports of construction services",
            Self::ExpServConstAbroad => "Construction abroad",
            Self::ExpServConstExpend => "Foreign contractors' expenditures in the United States",
            Self::ExpServConstruction => "Exports of construction services",
            Self::ExpServFinancial => "Exports of financial services",
            Self::ExpServFinancialExplicitAndOth => {
                "Exports of explicitly charged and other financial services"
            }
            Self::ExpServFinBrokMarketMak => "Exports of brokerage and market-making services",
            Self::ExpServFinCredCardOthCredRelated => {
                "Exports of credit card and other credit-related services"
            }
            Self::ExpServFinFinAdvCust => "Exports of financial advisory and custody services",
            Self::ExpServFinFinMan => "Exports of financial management services",
            Self::ExpServFinSecLendEftOth => {
                "Exports of securities lending, electronic funds transfer, and other services"
            }
            Self::ExpServFinUwPrivPlace => "Exports of underwriting and private placement services",
            Self::ExpServFisim => {
                "Exports of financial intermediation services indirectly measured"
            }
            Self::ExpServGovtGoodsAndServicesNie => {
                "Exports of government goods and services n.i.e."
            }
            Self::ExpServInfo => "Exports of information services",
            Self::ExpServInsurance => "Exports of insurance services",
            Self::ExpServInsuranceAuxIns => "Exports of auxiliary insurance services",
            Self::ExpServInsuranceDirect => "Exports of direct insurance services",
            Self::ExpServInsuranceReins => "Exports of reinsurance services",
            Self::ExpServMaintenanceAndRepairNie => {
                "Exports of maintenance and repair services n.i.e."
            }
            Self::ExpServManufacturing => {
                "Exports of manufacturing services on physical inputs owned by others"
            }
            Self::ExpServOtherBusiness => "Exports of other business services",
            Self::ExpServPersCultAndRec => {
                "Exports of personal, cultural, and recreational services"
            }
            Self::ExpServPersCultAndRecOth => {
                "Exports of other personal, cultural, and recreational services"
            }
            Self::ExpServProfMgmtConsult => {
                "Exports of professional and management consulting services"
            }
            Self::ExpServResearchAndDev => "exports of research and development services",
            Self::ExpServTechTradeRelatedOth => {
                "exports of technical, trade-related, and other business services"
            }
            Self::ExpServTelecom => "exports of telecommunications services",
            Self::ExpServTelecomCompAndInfo => {
                "Exports of telecommunications, computer, and information services"
            }
            Self::ExpServTransport => "Exports of transport services",
            Self::ExpServTransportAir => "Exports of air transport services",
            Self::ExpServTransportAirFreight => "Exports of air freight services",
            Self::ExpServTransportAirPass => "Exports of air passenger services",
            Self::ExpServTransportAirPort => "Exports of air port services",
            Self::ExpServTransportOth => "Exports of transport services; other modes of transport",
            Self::ExpServTransportSea => "Exports of sea transport services",
            Self::ExpServTransportSeaFreight => "Exports of sea freight services",
            Self::ExpServTransportSeaPort => "Exports of sea port services",
            Self::ExpServTravel => {
                "Exports of travel services (for all purposes including education)"
            }
            Self::ExpServTravelBusiness => "Exports of business travel services",
            Self::ExpServTravelBusinessOth => "Exports of other business travel services",
            Self::ExpServTravelEducation => "Exports of education-related services",
            Self::ExpServTravelHealth => "Exports of health-related services",
            Self::ExpServTravelPersonal => "Exports of personal travel services",
            Self::ExpServTravelPersonalOth => "Exports of other personal travel services",
            Self::ExpServTravelShortTermWork => {
                "Expenditures in the U.S. by border, seasonal, and other short-term workers"
            }
            Self::FinAssetsExclFinDeriv => {
                "Net U.S. acquisition of financial assets excluding financial derivatives"
            }
            Self::FinDeriv => "Financial derivatives other than reserves, net transactions",
            Self::FinDerivReserveAssets => {
                "Net U.S. acquisition of reserve assets; other; financial derivatives"
            }
            Self::FinLiabsExclFinDeriv => {
                "Net U.S. incurrence of liabilities excluding financial derivatives"
            }
            Self::FinLiabsFoa => "Net U.S. incurrence of liabilities to foreign official agencies",
            Self::GoldReserveAssets => "Net U.S. acquisition of reserve assets; monetary gold",
            Self::ImfReserveAssets => {
                "Net U.S. acquisition of reserve assets; reserve position in the International Monetary Fund"
            }
            Self::ImpGds => "Imports of goods",
            Self::ImpGdsAgFoodsFeedsAndBevs => {
                "Imports of agricultural foods, feeds, and beverages"
            }
            Self::ImpGdsAgIsm => "Imports of agricultural industrial supplies and materials",
            Self::ImpGdsAppFootAndHouse => "Imports of apparel, footwear, and household goods",
            Self::ImpGdsAutoEngAndEngParts => "Imports of automotive engines and engine parts",
            Self::ImpGdsAutoVehPartsAndEngines => {
                "Imports of automotive vehicles, parts, and engines"
            }
            Self::ImpGdsBauxAndAlum => "Imports of bauxite and aluminum",
            Self::ImpGdsBopAdj => "Imports of goods; balance of payments adjustments, net",
            Self::ImpGdsBuildMatsExcMetals => "Imports of building materials except metals",
            Self::ImpGdsCapGoodsExclAuto => "Imports of capital goods except automotive",
            Self::ImpGdsCensus => "Imports of goods, Census basis",
            Self::ImpGdsChemsExcMeds => "Imports of chemicals except medicinals",
            Self::ImpGdsCivAir => "Imports of civilian aircraft, complete, all types",
            Self::ImpGdsCivAirEngAndParts => "Imports of civilian aircraft, engines, and parts",
            Self::ImpGdsCoalAndRelProds => "Imports of coal and related products",
            Self::ImpGdsCocoaAndSugar => "Imports of cocoa beans and sugar",
            Self::ImpGdsComp => "Imports of computers",
            Self::ImpGdsCompAccPeriAndParts => {
                "Imports of computer accessories, peripherals, and parts"
            }
            Self::ImpGdsConsGoodsExcFoodAndAuto => {
                "Imports of consumer goods except food and automotive"
            }
            Self::ImpGdsCrudePet => "Imports of crude petroleum",
            Self::ImpGdsDistBevAndOthNonAgFoodsFeedsAndBevs => {
                "Imports of distilled beverages and other nonagricultural foods, feeds, and beverages"
            }
            Self::ImpGdsDurCons => "Imports of durable consumer goods",
            Self::ImpGdsElecGenMachElecAppAndParts => {
                "Imports of electric-generating machinery, electric apparatus and parts"
            }
            Self::ImpGdsEnergyProds => "Imports of energy products",
            Self::ImpGdsEngAndPartsForCivAir => {
                "Imports of engines and parts for civilian aircraft"
            }
            Self::ImpGdsFertPestAndInsect => "Imports of fertilizers, pesticides, and insecticides",
            Self::ImpGdsFishShellfish => "Imports of fish and shellfish",
            Self::ImpGdsFoodsFeedsAndBevs => "Imports of foods, feeds, and beverages",
            Self::ImpGdsFuelOil => "Imports of fuel oil",
            Self::ImpGdsGdsProcPortsBopAdj => {
                "Imports of goods; balance of payments adjustments, net; goods procured in foreign ports by U.S. carriers"
            }
            Self::ImpGdsGemDiamAndOthGem => "Imports of gem diamonds and other gemstones",
            Self::ImpGdsGenMerch => "Imports of general merchandise",
            Self::ImpGdsGreenCoffee => "Imports of green coffee",
            Self::ImpGdsHouseAndKitchApp => "Imports of household and kitchen appliances",
            Self::ImpGdsHouseFurnAndRelProds => {
                "Imports of household furnishings and related products"
            }
            Self::ImpGdsHouseKitchAppAndOthHouse => {
                "Imports of household and kitchen appliances and other household goods"
            }
            Self::ImpGdsIndEngPumpsComps => "Imports of industrial engines, pumps, and compressors",
            Self::ImpGdsIndInorgChems => "Imports of industrial inorganic chemicals",
            Self::ImpGdsIndOrgChems => "Imports of industrial organic chemicals",
            Self::ImpGdsInlandFreightCanMexBopAdj => {
                "Imports of goods; balance of payments adjustments, net; inland freight in Canada and Mexico"
            }
            Self::ImpGdsIronAndSteelProds => "Imports of iron and steel products",
            Self::ImpGdsIsm => "Imports of industrial supplies and materials",
            Self::ImpGdsJewelryAndCollect => "Imports of jewelry and collectibles",
            Self::ImpGdsLiqPetGases => "Imports of liquified petroleum gases",
            Self::ImpGdsLocoRailBopAdj => {
                "Imports of goods; balance of payments adjustments, net; locomotives and railcars"
            }
            Self::ImpGdsMachAndEquipExcCons => {
                "Imports of machinery and equipment except consumer-type"
            }
            Self::ImpGdsMachToolsMetalworkMach => {
                "Imports of machine tools and metalworking machinery"
            }
            Self::ImpGdsMeasTestControlInst => {
                "Imports of measuring, testing, and control instruments"
            }
            Self::ImpGdsMeatProdsPoultry => "Imports of meat products and poultry",
            Self::ImpGdsMedDentAndPharm => {
                "Imports of medicinal, dental, and pharmaceutical products"
            }
            Self::ImpGdsMetalsAndNonmetProds => "Imports of metals and nonmetallic products",
            Self::ImpGdsNaturalGas => "Imports of natural gas",
            Self::ImpGdsNonAgFoodsFeedsAndBevs => {
                "Imports of nonagricultural foods, feeds, and beverages"
            }
            Self::ImpGdsNonAgIsm => "Imports of nonagricultural industrial supplies and materials",
            Self::ImpGdsNondurCons => "Imports of nondurable consumer goods",
            Self::ImpGdsNonferrousMetals => "Imports of nonferrous metals",
            Self::ImpGdsNonmonetaryGold => "Imports of nonmonetary gold",
            Self::ImpGdsNonmonGoldBopAdj => {
                "Imports of goods; balance of payments adjustments, net; nonmonetary gold"
            }
            Self::ImpGdsNuclearFuelAndElecEnergy => "Imports of nuclear fuel and electric energy",
            Self::ImpGdsOilDrillMiningConstMach => {
                "Imports of oil-drilling, mining, and construction machinery"
            }
            Self::ImpGdsOthAgFoodsFeedsAndBevs => {
                "Imports of other agricultural foods, feeds, and beverages"
            }
            Self::ImpGdsOthAutoPartsAndAcc => "Imports of other automotive parts and accessories",
            Self::ImpGdsOthBopAdj => {
                "Imports of goods; balance of payments adjustments, net; other adjustments, net"
            }
            Self::ImpGdsOthChems => "Imports of other chemicals",
            Self::ImpGdsOthDurCons => "Imports of other durable consumer goods",
            Self::ImpGdsOthGenMerch => "Imports of other general merchandise",
            Self::ImpGdsOthHouseIncCellPhones => {
                "Imports of other household goods, including cell phones"
            }
            Self::ImpGdsOthIndMach => "Imports of other industrial machinery",
            Self::ImpGdsOthMetalsAndNonmetProds => {
                "Imports of other metals and nonmetallic products"
            }
            Self::ImpGdsOthNondurCons => "Imports of other nondurable consumer goods",
            Self::ImpGdsOthNonferrousMetals => "Imports of other nonferrous metals",
            Self::ImpGdsOthNonmetals => "Imports of other nonmetals",
            Self::ImpGdsOthOfficeAndBusMach => "Imports of other office and business machines",
            Self::ImpGdsOthPetProds => "Imports of other petroleum products",
            Self::ImpGdsOthServIndAndAgMach => {
                "Imports of other service-industry and agricultural machinery"
            }
            Self::ImpGdsOthTransEquip => "Imports of other transportation equipment",
            Self::ImpGdsPaperAndPaperBaseStocks => "Imports of paper and paper-base stocks",
            Self::ImpGdsPassCars => "Imports of passenger cars, new and used",
            Self::ImpGdsPetAndProds => "Imports of petroleum and products",
            Self::ImpGdsPlasticMaterials => "Imports of plastic materials",
            Self::ImpGdsPrecMetalsExcNonmonGold => {
                "Imports of precious metals except nonmonetary gold"
            }
            Self::ImpGdsRadioAndStereoEquip => {
                "Imports of radio and stereo equipment, including recorded media"
            }
            Self::ImpGdsRepairEquipBopAdj => {
                "Imports of goods; balance of payments adjustments, net; repair of equipment"
            }
            Self::ImpGdsSciHospAndMedEquipAndParts => {
                "Imports of scientific, hospital, and medical equipment and parts"
            }
            Self::ImpGdsSemiconductors => "Imports of semiconductors",
            Self::ImpGdsServ => "Imports of goods and services",
            Self::ImpGdsServIncPay => "Imports of goods and services and income payments (debits)",
            Self::ImpGdsSoftRevalBopAdj => {
                "Imports of goods; balance of payments adjustments, net; software revaluation"
            }
            Self::ImpGdsSteelmakingMats => "Imports of steelmaking materials",
            Self::ImpGdsTelecomEquip => "Imports of telecommunications equipment",
            Self::ImpGdsTextileSupAndRelMats => "Imports of textile supplies and related materials",
            Self::ImpGdsToilAndCosmet => "Imports of toiletries and cosmetics",
            Self::ImpGdsToysAndSport => "Imports of toys and sporting goods, including bicycles",
            Self::ImpGdsTrucksBusesSpecPurpVeh => {
                "Imports of trucks, buses, and special purpose vehicles"
            }
            Self::ImpGdsTvsVidRecAndOthVidEquip => {
                "Imports of televisions, video receivers, and other video equipment"
            }
            Self::ImpGdsUsMilAgencyBopAdj => {
                "Imports of goods; balance of payments adjustments, net; imports by U.S. military agencies"
            }
            Self::ImpGdsVegFruitNutPreps => "Imports of vegetables, fruits, nuts, and preparations",
            Self::ImpGdsWineBeerRelProds => "Imports of wine, beer, and related products",
            Self::ImpServ => "Imports of services",
            Self::ImpServArtisticRelated => "Imports of artistic related services",
            Self::ImpServAudVis => "Imports of audiovisual services",
            Self::ImpServChargesForTheUseOfIpNie => {
                "Charges for the use of intellectual property n.i.e.; imports"
            }
            Self::ImpServCipLicensesAudVis => {
                "Charges for the use of intellectual property n.i.e.; imports; licenses to reproduce and/or distribute audiovisual products"
            }
            Self::ImpServCipLicensesCompSoftware => {
                "Charges for the use of intellectual property n.i.e.; imports; licenses to reproduce and/or distribute computer software"
            }
            Self::ImpServCipLicensesFranchisesTrademarks => {
                "Charges for the use of intellectual property n.i.e.; imports; franchises and trademarks licensing fees"
            }
            Self::ImpServCipLicensesOutcomesResearchAndDev => {
                "Charges for the use of intellectual property n.i.e.; imports; licenses for the use of outcomes of research and development"
            }
            Self::ImpServComp => "Imports of computer services",
            Self::ImpServConst => "Imports of construction services",
            Self::ImpServConstExpend => "U.S. contractors' expenditures abroad",
            Self::ImpServConstInTheUs => "Construction in the United States",
            Self::ImpServConstruction => "Imports of construction services",
            Self::ImpServFinancial => "Imports of financial services",
            Self::ImpServFinancialExplicitAndOth => {
                "Imports of explicitly charged and other financial services"
            }
            Self::ImpServFinBrokMarketMak => "Imports of brokerage and market-making services",
            Self::ImpServFinCredCardOthCredRelated => {
                "Imports of credit card and other credit-related services"
            }
            Self::ImpServFinFinAdvCust => "Imports of financial advisory and custody services",
            Self::ImpServFinFinMan => "Imports of financial management services",
            Self::ImpServFinSecLendEftOth => {
                "Imports of securities lending, electronic funds transfer, and other services"
            }
            Self::ImpServFinUwPrivPlace => "Imports of underwriting and private placement services",
            Self::ImpServFisim => {
                "Imports of financial intermediation services indirectly measured"
            }
            Self::ImpServGovtGoodsAndServicesNie => {
                "Imports of government goods and services n.i.e."
            }
            Self::ImpServInfo => "Imports of information services",
            Self::ImpServInsurance => "Imports of insurance services",
            Self::ImpServInsuranceAuxIns => "Imports of auxiliary insurance services",
            Self::ImpServInsuranceDirect => "Imports of direct insurance services",
            Self::ImpServInsuranceReIns => "Imports of reinsurance services",
            Self::ImpServMaintenanceAndRepairNie => {
                "Imports of maintenance and repair services n.i.e."
            }
            Self::ImpServManufacturing => {
                "Imports of manufacturing services on physical inputs owned by others"
            }
            Self::ImpServOtherBusiness => "Imports of other business services",
            Self::ImpServPersCultAndRec => {
                "Imports of personal, cultural, and recreational services"
            }
            Self::ImpServPersCultAndRecOth => {
                "Imports of other personal, cultural, and recreational services"
            }
            Self::ImpServProfMgmtConsult => {
                "Imports of professional and management consulting services"
            }
            Self::ImpServResearchAndDev => "Research and development services imports",
            Self::ImpServTechTradeRelatedOth => {
                "Other technical, trade-related, and other business services imports"
            }
            Self::ImpServTelecom => "Telecommunications services imports",
            Self::ImpServTelecomCompAndInfo => {
                "Telecommunications, computer, and information services imports"
            }
            Self::ImpServTransport => "Transport services imports",
            Self::ImpServTransportAir => "Air transport services imports",
            Self::ImpServTransportAirFreight => "Air freight transport services imports",
            Self::ImpServTransportAirPass => "Air passenger transport services imports",
            Self::ImpServTransportAirPort => "Air transport services imports; port services",
            Self::ImpServTransportOth => "Other transport services imports",
            Self::ImpServTransportSea => "Sea transport services imports",
            Self::ImpServTransportSeaFreight => "Sea freight transport services imports",
            Self::ImpServTransportSeaPort => "Sea transport services imports; port services",
            Self::ImpServTravel => "Travel services imports",
            Self::ImpServTravelBusiness => "Business travel services imports",
            Self::ImpServTravelBusinessOth => "Other business travel services imports",
            Self::ImpServTravelEducation => "Education-related travel services imports",
            Self::ImpServTravelHealth => "Health-related travel services imports",
            Self::ImpServTravelPersonal => "Personal travel services imports",
            Self::ImpServTravelPersonalOth => "Other personal travel services imports",
            Self::ImpServTravelShortTermWork => "Short-term work-related travel services imports",
            Self::InsLossesPaid => "Insurance losses paid",
            Self::InsLossesRecovered => "Insurance losses recovered",
            Self::InsPremiumsPaid => "Insurance premiums paid",
            Self::InsPremiumsReceived => "Insurance premiums received",
            Self::InsTechReservesAssets => "Insurance technical reserves; assets",
            Self::InsTechReservesAssetsOthFinNonFin => {
                "Insurance technical reserves; assets; other financial and nonfinancial institutions"
            }
            Self::InsTechReservesLiabs => "Insurance technical reserves; liabilities",
            Self::InsTechReservesLiabsOthFinNonFin => {
                "Insurance technical reserves; liabilities; other financial and nonfinancial institutions"
            }
            Self::InvFundSharesAssets => "Investment fund shares; assets",
            Self::InvFundSharesIncPay => "Investment fund shares; income payments",
            Self::InvFundSharesIncRec => "Investment fund shares; income receipts",
            Self::InvFundSharesLiabs => "Investment fund shares; liabilities",
            Self::InvFundSharesLiabsFoa => "Investment fund shares; liabilities; financial account",
            Self::InvIncPay => "Investment income payments",
            Self::InvIncRec => "Investment income receipts",
            Self::InvRaIncRec => "Investment income receipts; reinvested earnings",
            Self::InvRaIntIncRec => {
                "Investment income receipts; reinvested earnings; interest income"
            }
            Self::LoansAssets => "Loans; assets",
            Self::LoansAssetsDepTaking => "Loans; assets; deposit-taking institutions",
            Self::LoansAssetsGenGovt => "Loans; assets; general government",
            Self::LoansAssetsOthFinNonFin => {
                "Loans; assets; other financial and nonfinancial institutions"
            }
            Self::LoansLiabs => "Loans; liabilities",
            Self::LoansLiabsDepTaking => "Loans; liabilities; deposit-taking institutions",
            Self::LoansLiabsFoa => "Loans; liabilities; financial account",
            Self::LoansLiabsOthFinNonFin => {
                "Loans; liabilities; other financial and nonfinancial institutions"
            }
            Self::LoansRepurchaseLiabsOthFinNonFin => {
                "Loans; repurchase liabilities; other financial and nonfinancial institutions"
            }
            Self::LoansResaleAssetsOthFinNonFin => {
                "Loans; resale assets; other financial and nonfinancial institutions"
            }
            Self::LtDebtSecAssets => "Long-term debt securities; assets",
            Self::LtDebtSecAssetsDepTaking => {
                "Long-term debt securities; assets; deposit-taking institutions"
            }
            Self::LtDebtSecAssetsNonFin => {
                "Long-term debt securities; assets; nonfinancial institutions"
            }
            Self::LtDebtSecAssetsOthFin => {
                "Long-term debt securities; assets; other financial institutions"
            }
            Self::LtDebtSecCorpAssets => "Long-term debt securities; corporate assets",
            Self::LtDebtSecCorpLiabs => "Long-term debt securities; corporate liabilities",
            Self::LtDebtSecCorpLiabsFoa => {
                "Long-term debt securities; corporate liabilities; financial account"
            }
            Self::LtDebtSecFedSponsorAgencyIncPay => {
                "Long-term debt securities; federal-sponsored agency income payments"
            }
            Self::LtDebtSecFedSponsorAgencyLiabs => {
                "Long-term debt securities; federal-sponsored agency liabilities"
            }
            Self::LtDebtSecFedSponsorAgencyLiabsFoa => {
                "Long-term debt securities; federal-sponsored agency liabilities; financial account"
            }
            Self::LtDebtSecGovtAssets => "Long-term debt securities; government assets",
            Self::LtDebtSecIncPay => "Long-term debt securities; income payments",
            Self::LtDebtSecIncPayDepTaking => {
                "Long-term debt securities; income payments; deposit-taking institutions"
            }
            Self::LtDebtSecIncPayNonFin => {
                "Long-term debt securities; income payments; nonfinancial institutions"
            }
            Self::LtDebtSecIncRec => "Long-term debt securities; income receipts",
            Self::LtDebtSecIncRecDepTaking => {
                "Long-term debt securities; income receipts; deposit-taking institutions"
            }
            Self::LtDebtSecIncRecNonFin => {
                "Long-term debt securities; income receipts; nonfinancial institutions"
            }
            Self::LtDebtSecIncRecOthFin => {
                "Long-term debt securities; income receipts; other financial institutions"
            }
            Self::LtDebtSecLiabs => "Long-term debt securities; liabilities",
            Self::LtDebtSecLiabsDepTaking => {
                "Long-term debt securities; liabilities; deposit-taking institutions"
            }
            Self::LtDebtSecLiabsFoa => "Long-term debt securities; liabilities; financial account",
            Self::LtDebtSecLiabsNonFin => {
                "Long-term debt securities; liabilities; nonfinancial institutions"
            }
            Self::LtDebtSecNegCdAssets => "Long-term debt securities; negative CD assets",
            Self::LtDebtSecNegCdLiabs => "Long-term debt securities; negative CD liabilities",
            Self::LtDebtSecNegCdLiabsFoa => {
                "Long-term debt securities; negative CD liabilities; financial account"
            }
            Self::LtDebtSecOthThanFedSponsorAgencyIncPayOthFin => {
                "Long-term debt securities; other than federal-sponsored agency income payments; other financial institutions"
            }
            Self::LtDebtSecOthThanFedSponsorAgencyLiabsOthFin => {
                "Long-term debt securities; other than federal-sponsored agency liabilities; other financial institutions"
            }
            Self::LtDebtSecStateLocalGovtIncPay => {
                "Portfolio investment income payments; interest on state and local government long-term securities"
            }
            Self::LtDebtSecStateLocalGovtLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; state and local government long-term securities"
            }
            Self::LtDebtSecStateLocalGovtLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; state and local government long-term securities"
            }
            Self::LtDebtSecTreasIncPay => {
                "Portfolio investment income payments; interest on long-term U.S. Treasury securities"
            }
            Self::LtDebtSecTreasLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; Treasury bonds and notes"
            }
            Self::LtDepAssets => {
                "Net U.S. acquisition of other investment assets; long-term deposits"
            }
            Self::LtDepAssetsDepTaking => {
                "Net U.S. acquistion of other investment assets; long-term deposits; held by deposit-taking institutions except central bank"
            }
            Self::LtDepAssetsOthFinNonFin => {
                "Net U.S. acquistion of other investment assets; long-term deposits; held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::LtDepLiabs => {
                "Net U.S. incurrence of other investment liabilities; long-term deposits"
            }
            Self::LtDepLiabsDepTaking => {
                "Net U.S. incurrence of other investment liabilities; long-term deposits; issued by deposit-taking institutions except central bank"
            }
            Self::LtDepLiabsOthFinNonFin => {
                "Net U.S. incurrence of other investment liabilities; long-term deposits; issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::LtLoansAssets => {
                "Net U.S. acquisition of other investment assets; long-term loans"
            }
            Self::LtLoansAssetsDepTaking => {
                "Net U.S. acquistion of other investment assets; long-term loans; held by deposit-taking institutions except central bank"
            }
            Self::LtLoansAssetsGenGovt => {
                "Net U.S. acquistion of other investment assets; long-term loans; held by general government"
            }
            Self::LtLoansAssetsOthFinNonFin => {
                "Net U.S. acquistion of other investment assets; long-term loans; held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::LtLoansLiabs => {
                "Net U.S. incurrence of other investment liabilities; long-term loans"
            }
            Self::LtLoansLiabsDepTaking => {
                "Net U.S. incurrence of other investment liabilities; long-term loans; issued by deposit-taking institutions except central bank"
            }
            Self::LtLoansLiabsOthFinNonFin => {
                "Net U.S. incurrence of other investment liabilities; long-term loans; issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::LtTrdCredAndAdvAssets => {
                "Net U.S. acquisition of other investment assets; long-term trade credit and advances"
            }
            Self::LtTrdCredAndAdvAssetsOthFinNonFin => {
                "Net U.S. acquistion of other investment assets; long-term trade credit and advances; held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::LtTrdCredAndAdvLiabs => {
                "Net U.S. incurrence of other investment liabilities; long-term trade credit and advances"
            }
            Self::LtTrdCredAndAdvLiabsGenGovt => {
                "Net U.S. incurrence of other investment liabilities; long-term trade credit and advances; issued by general government"
            }
            Self::LtTrdCredAndAdvLiabsOthFinNonFin => {
                "Net U.S. incurrence of other investment liabilities; long-term trade credit and advances; issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::NetLendBorrCurrCapAcct => {
                "Net lending (+) or net borrowing (-) from current- and capital-account transactions"
            }
            Self::NetLendBorrFinAcct => {
                "Net lending (+) or net borrowing (-) from financial-account transactions"
            }
            Self::OthClmReserveAssets => {
                "Net U.S. acquisition of reserve assets; other; other claims"
            }
            Self::OthEquityAssets => {
                "Net U.S. acquisition of other investment assets; other equity"
            }
            Self::OthEquityAssetsGenGovt => {
                "Net U.S. acquistion of other investment assets; held by general government; other equity"
            }
            Self::OthEquityLiabs => {
                "Net U.S. incurrence of other investment liabilities; other equity"
            }
            Self::OthEquityLiabsFoa => {
                "Net U.S. incurrence of other investment liabilities to foreign official agencies; other equity"
            }
            Self::OthInvAssets => "Net U.S. acquisition of other investment assets",
            Self::OthInvAssetsCentralBank => {
                "Net U.S. acquistion of other investment assets; held by central bank"
            }
            Self::OthInvAssetsDepTaking => {
                "Net U.S. acquistion of other investment assets; held by deposit-taking institutions except central bank"
            }
            Self::OthInvAssetsGenGovt => {
                "Net U.S. acquistion of other investment assets; held by general government"
            }
            Self::OthInvAssetsOthFinNonFin => {
                "Net U.S. acquistion of other investment assets; held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::OthInvIncPay => "Other investment income payments",
            Self::OthInvIncPayBeforeFisim => {
                "Other investment interest income payments before adjusting for FISIM"
            }
            Self::OthInvIncPayCentralBank => {
                "Other investment income payments; on liabilities issued by central bank"
            }
            Self::OthInvIncPayDepTaking => {
                "Other investment income payments; on liabilities issued by deposit-taking institutions except central bank"
            }
            Self::OthInvIncPayGenGovt => {
                "Other investment income payments; on liabilities issued by general government"
            }
            Self::OthInvIncPayOthFinNonFin => {
                "Other investment income payments; on liabilities issued by non-deposit-taking financial institutions and nonfinancial instutitions except general government"
            }
            Self::OthInvIncRec => "Other investment income receipts",
            Self::OthInvIncRecBeforeFisim => {
                "Other investment interest income receipts before adjusting for FISIM"
            }
            Self::OthInvIncRecCentralBank => {
                "Other investment income receipts; on assets held by central bank"
            }
            Self::OthInvIncRecDepTaking => {
                "Other investment income receipts; on assets held by deposit-taking institutions except central bank"
            }
            Self::OthInvIncRecGenGovt => {
                "Other investment income receipts; on assets held by general government"
            }
            Self::OthInvIncRecOthFinNonFin => {
                "Other investment income receipts; on assets held by non-deposit-taking financial institutions and nonfinancial instutitions except general government"
            }
            Self::OthInvInsPolHoldtIncPay => {
                "Other investment income payments; income attributable to insurance policyholders"
            }
            Self::OthInvInsPolHoldtIncPayOthFinNonFin => {
                "Other investment income payments; income attributable to insurance policyholders; on liabilities issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::OthInvInsPolHoldtIncRec => {
                "Other investment income receipts; income attributable to insurance policyholders"
            }
            Self::OthInvInsPolHoldtIncRecOthFinNonFin => {
                "Other investment income receipts; income attributable to insurance policyholders; on assets held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::OthInvInterbankAssets => {
                "Net U.S. acquistion of other investment assets; interbank transactions"
            }
            Self::OthInvInterbankLiabsDepTaking => {
                "Net U.S. incurrence of other investment liabilities; interbank transactions"
            }
            Self::OthInvIntIncPay => "Other investment income payments; interest",
            Self::OthInvIntIncPayCentralBank => {
                "Other investment income payments; interest; on liabilities issued by central bank"
            }
            Self::OthInvIntIncPayDepTaking => {
                "Other investment income payments; interest; on liabilities issued by deposit-taking institutions except central bank"
            }
            Self::OthInvIntIncPayGenGovt => {
                "Other investment income payments; interest on special drawing rights allocations"
            }
            Self::OthInvIntIncPayOthFinNonFin => {
                "Other investment income payments; interest; on liabilities issued by non-deposit-taking financial institutions and nonfinancial instutitions except general government"
            }
            Self::OthInvIntIncRec => "Other investment income receipts; interest",
            Self::OthInvIntIncRecCentralBank => {
                "Other investment income receipts; interest; on assets held by central bank"
            }
            Self::OthInvIntIncRecDepTaking => {
                "Other investment income receipts; interest; on assets held by deposit-taking institutions except central bank"
            }
            Self::OthInvIntIncRecGenGovt => {
                "Other investment income receipts; interest; on assets held by general government"
            }
            Self::OthInvIntIncRecOthFinNonFin => {
                "Other investment income receipts; interest; on assets held by non-deposit-taking financial institutions and nonfinancial instutitions except general government"
            }
            Self::OthInvLiabs => "Net U.S. incurrence of other investment liabilities",
            Self::OthInvLiabsCentralBank => {
                "Net U.S. incurrence of other investment liabilities; issued by central bank"
            }
            Self::OthInvLiabsDepTaking => {
                "Net U.S. incurrence of other investment liabilities; issued by deposit-taking institutions except central bank"
            }
            Self::OthInvLiabsFoa => {
                "Net U.S. incurrence of other investment liabilities to foreign official agencies"
            }
            Self::OthInvLiabsGenGovt => {
                "Net U.S. incurrence of other investment liabilities; issued by general government"
            }
            Self::OthInvLiabsOthFinNonFin => {
                "Net U.S. incurrence of other investment liabilities; issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::OthReserveAssets => "Net U.S. acquisition of reserve assets; other",
            Self::PfInvAssets => "Net U.S. acquisition of portfolio investment assets",
            Self::PfInvAssetsDepTaking => {
                "Net U.S. acquisition of portfolio investment assets; deposit-taking institutions"
            }
            Self::PfInvAssetsNonFin => {
                "Net U.S. acquisition of portfolio investment assets; nonfinancial institutions"
            }
            Self::PfInvAssetsOthFin => {
                "Net U.S. acquisition of portfolio investment assets; other financial institutions"
            }
            Self::PfInvIncPay => "Portfolio investment income payments",
            Self::PfInvIncPayDepTaking => {
                "Portfolio investment income payments; deposit-taking institutions"
            }
            Self::PfInvIncPayGenGovt => "Portfolio investment income payments; general government",
            Self::PfInvIncPayNonFin => {
                "Portfolio investment income payments; nonfinancial institutions"
            }
            Self::PfInvIncPayOthFin => {
                "Portfolio investment income payments; other financial institutions"
            }
            Self::PfInvIncRec => "Portfolio investment income receipts",
            Self::PfInvIncRecDepTaking => {
                "Portfolio investment income receipts; deposit-taking institutions"
            }
            Self::PfInvIncRecNonFin => {
                "Portfolio investment income receipts; nonfinancial institutions"
            }
            Self::PfInvIncRecOthFin => {
                "Portfolio investment income receipts; other financial institutions"
            }
            Self::PfInvLiabs => "Net U.S. incurrence of portfolio investment liabilities",
            Self::PfInvLiabsDepTaking => {
                "Net U.S. incurrence of portfolio investment liabilities; deposit-taking institutions"
            }
            Self::PfInvLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities; financial account"
            }
            Self::PfInvLiabsGenGovt => {
                "Net U.S. incurrence of portfolio investment liabilities; general government"
            }
            Self::PfInvLiabsNonFin => {
                "Net U.S. incurrence of portfolio investment liabilities; nonfinancial institutions"
            }
            Self::PfInvLiabsOthFin => {
                "Net U.S. incurrence of portfolio investment liabilities; other financial institutions"
            }
            Self::PrimIncPay => "Primary income payments",
            Self::PrimIncRec => "Primary income receipts",
            Self::ReserveAssets => "Reserve assets",
            Self::SdrAllocLiabs => "Special drawing rights allocations; liabilities",
            Self::SdrAllocLiabsFoa => {
                "Special drawing rights allocations; liabilities; financial account"
            }
            Self::SdrReserveAssets => "Special drawing rights; reserve assets",
            Self::SeasAdjDisc => "Seasonal adjustment discrepancy",
            Self::SecIncPay => "Secondary income (current transfer) payments",
            Self::SecIncPayCharitableDonations => {
                "Secondary income (current transfer) payments; charitable donations"
            }
            Self::SecIncPayForeignStudents => {
                "Secondary income (current transfer) payments; foreign students"
            }
            Self::SecIncPayGenGovt => {
                "Secondary income (current transfer) payments; general government transfers"
            }
            Self::SecIncPayGenGovtContribIntOrg => {
                "Secondary income (current transfer) payments; general government contributions to international organizations"
            }
            Self::SecIncPayGenGovtIntCoop => {
                "Secondary income (current transfer) payments; general government international cooperation"
            }
            Self::SecIncPayGenGovtOth => {
                "Secondary income (current transfer) payments; general government other"
            }
            Self::SecIncPayGenGovtSocialBenefits => {
                "Secondary income (current transfer) payments; general government social benefits"
            }
            Self::SecIncPayInsuranceRelated => {
                "Secondary income (current transfer) payments; insurance-related"
            }
            Self::SecIncPayOthPrivateTransfer => {
                "Secondary income (current transfer) payments; other private transfers"
            }
            Self::SecIncPayPersonal => "Secondary income (current transfer) payments; personal",
            Self::SecIncPayPrivate => "Secondary income (current transfer) payments; private",
            Self::SecIncPayPrivateFinesPenalties => {
                "Secondary income (current transfer) payments; private fines and penalties"
            }
            Self::SecIncPayTaxesIncomeWealth => {
                "Secondary income (current transfer) payments; taxes on income and wealth"
            }
            Self::SecIncRec => "Secondary income (current transfer) receipts",
            Self::SecIncRecGenGovt => {
                "Secondary income (current transfer) receipts; general government transfers"
            }
            Self::SecIncRecGenGovtFinesPenalties => {
                "Secondary income (current transfer) receipts; general government transfers; fines and penalties"
            }
            Self::SecIncRecGenGovtIntCoop => {
                "Secondary income (current transfer) receipts; general government transfers; international cooperation"
            }
            Self::SecIncRecGenGovtOth => {
                "Secondary income (current transfer) receipts; other general government transfers"
            }
            Self::SecIncRecGenGovtTaxesIncomeWealth => {
                "Secondary income (current transfer) receipts; general government transfers; taxes on income, wealth, etc."
            }
            Self::SecIncRecInsuranceRelated => {
                "Secondary income (current transfer) receipts; insurance-related transfers"
            }
            Self::SecIncRecOthPrivateTransfer => {
                "Secondary income (current transfer) receipts; other private transfers"
            }
            Self::SecIncRecPrivate => {
                "Secondary income (current transfer) receipts; private transfers"
            }
            Self::SecIncRecPrivateFinesPenalties => {
                "Secondary income (current transfer) receipts; fines and penalties"
            }
            Self::SecReserveAssets => "Net U.S. acquisition of reserve assets; other; securities",
            Self::StatDisc => "Statistical discrepancy",
            Self::StDebtSecAssets => {
                "Net U.S. acquisition of portfolio investment assets; short-term debt securities"
            }
            Self::StDebtSecAssetsDepTaking => {
                "Net U.S. acquisition of portfolio investment assets; short-term debt securities; held by deposit-taking institutions except central bank"
            }
            Self::StDebtSecAssetsNonFin => {
                "Net U.S. acquisition of portfolio investment assets; short-term debt securities; held by nonfinancial institutions except general government"
            }
            Self::StDebtSecAssetsOthFin => {
                "Net U.S. acquisition of portfolio investment assets; short-term debt securities; held by non-deposit-taking financial institutions"
            }
            Self::StDebtSecCommPaperAndOthLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; commercial paper and other short-term debt securities (those not listed in table 7.1)"
            }
            Self::StDebtSecCommPaperAndOthLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; commercial paper and other short-term debt securities (those not listed in table 9.1)"
            }
            Self::StDebtSecCommPaperAssets => {
                "Net U.S. acquisition of portfolio investment assets; short-term commercial paper"
            }
            Self::StDebtSecFedSponsorAgencyIncPay => {
                "Portfolio investment income payments; interest on short-term federally sponsored agency securties"
            }
            Self::StDebtSecFedSponsorAgencyLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; short-term federally sponsored agency securities"
            }
            Self::StDebtSecFedSponsorAgencyLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; short-term federally sponsored agency securities"
            }
            Self::StDebtSecIncPay => {
                "Portfolio investment income payments; interest on short-term debt securities"
            }
            Self::StDebtSecIncPayDepTaking => {
                "Portfolio investment income payments; interest on short-term debt securities; deposit-taking institutions except central bank"
            }
            Self::StDebtSecIncPayNonFin => {
                "Portfolio investment income payments; interest on short-term debt securities; nonfinancial institutions except general government"
            }
            Self::StDebtSecIncRec => {
                "Portfolio investment income receipts; interest on short-term debt securities"
            }
            Self::StDebtSecIncRecDepTaking => {
                "Portfolio investment income receipts; interest on short-term debt securities; deposit-taking institutions except central bank"
            }
            Self::StDebtSecIncRecNonFin => {
                "Portfolio investment income receipts; interest on short-term debt securities; nonfinancial institutions except general government"
            }
            Self::StDebtSecIncRecOthFin => {
                "Portfolio investment income receipts; interest on short-term debt securities; non-deposit-taking financial institutions"
            }
            Self::StDebtSecLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; short-term debt securities"
            }
            Self::StDebtSecLiabsDepTaking => {
                "Net U.S. incurrence of portfolio investment liabilities; short-term debt securities; issued by deposit-taking institutions except central bank"
            }
            Self::StDebtSecLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; short-term debt securities"
            }
            Self::StDebtSecLiabsNonFin => {
                "Net U.S. incurrence of portfolio investment liabilities; short-term debt securities; issued by nonfinancial institutions except general government"
            }
            Self::StDebtSecNegCdAssets => {
                "Net U.S. acquisition of portfolio investment assets; short-term negotiable certificates of deposit"
            }
            Self::StDebtSecNegCdLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; short-term negotiable certificates of deposit"
            }
            Self::StDebtSecNegCdLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; short-term negotiable certificates of deposit"
            }
            Self::StDebtSecOthAssets => {
                "Net U.S. acquisition of portfolio investment assets; short-term debt securities other than negotiable certificates of deposit and commercial paper"
            }
            Self::StDebtSecOthThanFedSponsorAgencyIncPayOthFin => {
                "Portfolio investment income payments; interest on short-term debt securities other than federally sponsored agency securties; non-deposit-taking financial institutions"
            }
            Self::StDebtSecOthThanFedSponsorAgencyLiabsOthFin => {
                "Net U.S. incurrence of portfolio investment liabilities; short-term debt securities other than federally sponsored agency securities; issued by non-deposit-taking financial institutions"
            }
            Self::StDebtSecTreasIncPay => {
                "Portfolio investment income payments; interest on short-term U.S. Treasury securities"
            }
            Self::StDebtSecTreasLiabs => {
                "Net U.S. incurrence of portfolio investment liabilities; Treasury bills and certificates"
            }
            Self::StDebtSecTreasLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; Treasury bills and certificates"
            }
            Self::StDepAssets => {
                "Net U.S. acquisition of other investment assets; short-term deposits"
            }
            Self::StDepAssetsCentralBank => {
                "Net U.S. acquistion of other investment assets; short-term deposits; held by central bank"
            }
            Self::StDepAssetsDepTaking => {
                "Net U.S. acquistion of other investment assets; short-term deposits; held by deposit-taking institutions except central bank"
            }
            Self::StDepAssetsOthFinNonFin => {
                "Net U.S. acquistion of other investment assets; short-term deposits; held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::StDepLiabs => {
                "Net U.S. incurrence of other investment liabilities; short-term deposits"
            }
            Self::StDepLiabsCentralBank => {
                "Net U.S. incurrence of other investment liabilities; short-term deposits; issued by central bank"
            }
            Self::StDepLiabsDepTaking => {
                "Net U.S. incurrence of other investment liabilities; short-term deposits; issued by deposit-taking institutions except central bank"
            }
            Self::StDepLiabsOthFinNonFin => {
                "Net U.S. incurrence of other investment liabilities; short-term deposits; issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::StLoansAssets => {
                "Net U.S. acquisition of other investment assets; short-term loans"
            }
            Self::StLoansAssetsDepTaking => {
                "Net U.S. acquistion of other investment assets; short-term loans; held by deposit-taking institutions except central bank"
            }
            Self::StLoansAssetsOthFinNonFin => {
                "Net U.S. acquistion of other investment assets; short-term loans; held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::StLoansLiabs => {
                "Net U.S. incurrence of other investment liabilities; short-term loans"
            }
            Self::StLoansLiabsDepTaking => {
                "Net U.S. incurrence of other investment liabilities; short-term loans; issued by deposit-taking institutions except central bank"
            }
            Self::StLoansLiabsOthFinNonFin => {
                "Net U.S. incurrence of other investment liabilities; short-term loans; issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::StTrdCredAndAdvAssets => {
                "Net U.S. acquisition of other investment assets; short-term trade credit and advances"
            }
            Self::StTrdCredAndAdvAssetsOthFinNonFin => {
                "Net U.S. acquistion of other investment assets; short-term trade credit and advances; held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::StTrdCredAndAdvLiabs => {
                "Net U.S. incurrence of other investment liabilities; short-term trade credit and advances"
            }
            Self::StTrdCredAndAdvLiabsOthFinNonFin => {
                "Net U.S. incurrence of other investment liabilities; short-term trade credit and advances; issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::TrdCredAndAdvAssets => {
                "Net U.S. acquisition of other investment assets; trade credit and advances"
            }
            Self::TrdCredAndAdvAssetsOthFinNonFin => {
                "Net U.S. acquistion of other investment assets; trade credit and advances; held by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::TrdCredAndAdvLiabs => {
                "Net U.S. incurrence of other investment liabilities; trade credit and advances"
            }
            Self::TrdCredAndAdvLiabsFoa => {
                "Net U.S. incurrence of other investment liabilities to foreign official agencies; trade credit and advances"
            }
            Self::TrdCredAndAdvLiabsGenGovt => {
                "Net U.S. incurrence of other investment liabilities; trade credit and advances; issued by general government"
            }
            Self::TrdCredAndAdvLiabsOthFinNonFin => {
                "Net U.S. incurrence of other investment liabilities; trade credit and advances; issued by non-deposit-taking institutions and nonfinancial institutions except general government"
            }
            Self::TreasBondsAndNotesLiabsFoa => {
                "Net U.S. incurrence of portfolio investment liabilities to foreign official agencies; Treasury bonds and notes"
            }
            Self::TsiItaDiInvIncInward => {
                "Direct investment income on inward investment (foreign direct investment in the United States), directional basis"
            }
            Self::TsiItaDiInvIncOutward => {
                "Direct investment income on outward investment (U.S. direct investment abroad), directional basis"
            }
            Self::TsiItaDiInvWithoutCurrCostAdjIncOutward => {
                "Direct investment income without current cost adjustment on outward investment (U.S. direct investment abroad), directional basis"
            }
        }
    }
}

impl TryFrom<&ParameterFields> for Indicator {
    type Error = KeyMissing;
    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        Self::from_value(value.key())
    }
}

impl TryFrom<&ParameterValueTable> for Indicator {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::ParameterFields(pf) => Ok(Self::try_from(pf)?),
            _ => {
                let error = ParameterValueTableVariant::new(
                    "ParameterFields needed".to_owned(),
                    line!(),
                    file!().to_owned(),
                );
                Err(error.into())
            }
        }
    }
}
