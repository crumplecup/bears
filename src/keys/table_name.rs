use crate::{
    BeaErr, DeriveFromStr, NipaTable, ParameterFields, ParameterValueTable,
    ParameterValueTableVariant,
};
use std::str::FromStr;

/// TODO: Used in Fixed Assets and Regional key sets.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    derive_new::new,
    derive_getters::Getters,
    serde::Deserialize,
    serde::Serialize,
)]
#[display("{}", self.name)]
pub struct TableName {
    name: String,
    description: String,
}

impl From<&NipaTable> for TableName {
    fn from(value: &NipaTable) -> Self {
        Self::new(value.table_name().into(), value.description().into())
    }
}

impl From<&ParameterFields> for TableName {
    fn from(value: &ParameterFields) -> Self {
        Self::new(value.key().into(), value.desc().into())
    }
}

impl TryFrom<&ParameterValueTable> for TableName {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::NipaTable(tab) => Ok(TableName::from(tab)),
            ParameterValueTable::ParameterFields(tab) => Ok(TableName::from(tab)),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("NipaTable or ParameterFields needed, found {other:#?}"),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}

/// Represents the various table types available in the National Income and Product Accounts (NIPA).
/// Each variant corresponds to a specific NIPA table with a unique identifier.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
)]
pub enum NipaTableName {
    /// Table 1.1.1. Percent Change From Preceding Period in Real Gross Domestic Product (A) (Q)
    T10101,
    /// Table 1.1.2. Contributions to Percent Change in Real Gross Domestic Product (A) (Q)
    T10102,
    /// Table 1.1.3. Real Gross Domestic Product, Quantity Indexes (A) (Q)
    T10103,
    /// Table 1.1.4. Price Indexes for Gross Domestic Product (A) (Q)
    T10104,
    /// Table 1.1.5. Gross Domestic Product (A) (Q)
    T10105,
    /// Table 1.1.6. Real Gross Domestic Product, Chained Dollars (A) (Q)
    T10106,
    /// Table 1.1.7. Percent Change From Preceding Period in Prices for Gross Domestic Product (A) (Q)
    T10107,
    /// Table 1.1.8. Contributions to Percent Change in the Gross Domestic Product Price Index (A) (Q)
    T10108,
    /// Table 1.1.9. Implicit Price Deflators for Gross Domestic Product (A) (Q)
    T10109,
    /// Table 1.1.10. Percentage Shares of Gross Domestic Product (A) (Q)
    T10110,
    /// Table 1.1.11. Real Gross Domestic Product: Percent Change From Quarter One Year Ago (Q)
    T10111,
    /// Table 1.2.1. Percent Change From Preceding Period in Real Gross Domestic Product by Major Type of Product (A) (Q)
    T10201,
    /// Table 1.2.2. Contributions to Percent Change in Real Gross Domestic Product by Major Type of Product (A) (Q)
    T10202,
    /// Table 1.2.3. Real Gross Domestic Product by Major Type of Product, Quantity Indexes (A) (Q)
    T10203,
    /// Table 1.2.4. Price Indexes for Gross Domestic Product by Major Type of Product (A) (Q)
    T10204,
    /// Table 1.2.5. Gross Domestic Product by Major Type of Product (A) (Q)
    T10205,
    /// Table 1.2.6. Real Gross Domestic Product by Major Type of Product, Chained Dollars (A) (Q)
    T10206,
    /// Table 1.3.1. Percent Change From Preceding Period in Real Gross Value Added by Sector (A) (Q)
    T10301,
    /// Table 1.3.3. Real Gross Value Added by Sector, Quantity Indexes (A) (Q)
    T10303,
    /// Table 1.3.4. Price Indexes for Gross Value Added by Sector (A) (Q)
    T10304,
    /// Table 1.3.5. Gross Value Added by Sector (A) (Q)
    T10305,
    /// Table 1.3.6. Real Gross Value Added by Sector, Chained Dollars (A) (Q)
    T10306,
    /// Table 1.4.1. Percent Change From Preceding Period in Real Gross Domestic Product, Real Gross Domestic Purchases, and Real Final Sales to Domestic Purchasers (A) (Q)
    T10401,
    /// Table 1.4.3. Real Gross Domestic Product, Real Gross Domestic Purchases, and Real Final Sales to Domestic Purchasers, Quantity Indexes (A) (Q)
    T10403,
    /// Table 1.4.4. Price Indexes for Gross Domestic Product, Gross Domestic Purchases, and Final Sales to Domestic Purchasers (A) (Q)
    T10404,
    /// Table 1.4.5. Relation of Gross Domestic Product, Gross Domestic Purchases, and Final Sales to Domestic Purchasers (A) (Q)
    T10405,
    /// Table 1.4.6. Relation of Real Gross Domestic Product, Real Gross Domestic Purchases, and Real Final Sales to Domestic Purchasers, Chained Dollars (A) (Q)
    T10406,
    /// Table 1.5.1. Percent Change From Preceding Period in Real Gross Domestic Product, Expanded Detail (A) (Q)
    T10501,
    /// Table 1.5.2. Contributions to Percent Change in Real Gross Domestic Product, Expanded Detail (A) (Q)
    T10502,
    /// Table 1.5.3. Real Gross Domestic Product, Expanded Detail, Quantity Indexes (A) (Q)
    T10503,
    /// Table 1.5.4. Price Indexes for Gross Domestic Product, Expanded Detail (A) (Q)
    T10504,
    /// Table 1.5.5. Gross Domestic Product, Expanded Detail (A) (Q)
    T10505,
    /// Table 1.5.6. Real Gross Domestic Product, Expanded Detail, Chained Dollars (A) (Q)
    T10506,
    /// Table 1.6.4. Price Indexes for Gross Domestic Purchases (A) (Q)
    T10604,
    /// Table 1.6.7. Percent Change From Preceding Period in Prices for Gross Domestic Purchases (A) (Q)
    T10607,
    /// Table 1.6.8. Contributions to Percent Change in the Gross Domestic Purchases Price Index (A) (Q)
    T10608,
    /// Table 1.7.1. Percent Change From Preceding Period in Real Gross Domestic Product, Real Gross National Product, and Real Net National Product (A) (Q)
    T10701,
    /// Table 1.7.3. Real Gross Domestic Product, Real Gross National Product, and Real Net National Product, Quantity Indexes (A) (Q)
    T10703,
    /// Table 1.7.4. Price Indexes for Gross Domestic Product, Gross National Product, and Net National Product (A) (Q)
    T10704,
    /// Table 1.7.5. Relation of Gross Domestic Product, Gross National Product, Net National Product, National Income, and Personal Income (A) (Q)
    T10705,
    /// Table 1.7.6. Relation of Real Gross Domestic Product, Real Gross National Product, and Real Net National Product, Chained Dollars (A) (Q)
    T10706,
    /// Table 1.8.3. Command-Basis Real Gross Domestic Product and Gross National Product, Quantity Indexes (A) (Q)
    T10803,
    /// Table 1.8.6. Command-Basis Real Gross Domestic Product and Gross National Product, Chained Dollars (A) (Q)
    T10806,
    /// Table 1.9.3. Real Net Value Added by Sector, Quantity Indexes (A)
    T10903,
    /// Table 1.9.4. Price Indexes for Net Value Added by Sector (A)
    T10904,
    /// Table 1.9.5. Net Value Added by Sector (A)
    T10905,
    /// Table 1.9.6. Real Net Value Added by Sector, Chained Dollars (A)
    T10906,
    /// Table 1.10. Gross Domestic Income by Type of Income (A) (Q)
    T11000,
    /// Table 1.11. Percentage Shares of Gross Domestic Income (A)
    T11100,
    /// Table 1.12. National Income by Type of Income (A) (Q)
    T11200,
    /// Table 1.13. National Income by Sector, Legal Form of Organization, and Type of Income (A)
    T11300,
    /// Table 1.14. Gross Value Added of Domestic Corporate Business in Current Dollars and Gross Value Added of Nonfinancial Domestic Corporate Business in Current and Chained Dollars (A) (Q)
    T11400,
    /// Table 1.15. Price, Costs, and Profit Per Unit of Real Gross Value Added of Nonfinancial Domestic Corporate Business (A) (Q)
    T11500,
    /// Table 1.16. Sources and Uses of Private Enterprise Income (A)
    T11600,
    /// Table 1.17.1. Percent Change From Preceding Period in Real Gross Domestic Product, Real Gross Domestic Income, and Other Major NIPA Aggregates (A) (Q)
    T11701,
    /// Table 1.17.5. Gross Domestic Product, Gross Domestic Income, and Other Major NIPA Aggregates (A) (Q)
    T11705,
    /// Table 1.17.6. Real Gross Domestic Product, Real Gross Domestic Income, and Other Major NIPA Aggregates, Chained Dollars (A) (Q)
    T11706,
    /// Table 2.1. Personal Income and Its Disposition (A) (Q)
    T20100,
    /// Table 2.2A. Wages and Salaries by Industry (A) (Q)
    T20200A,
    /// Table 2.2B. Wages and Salaries by Industry (A) (Q)
    T20200B,
    /// Table 2.3.1. Percent Change From Preceding Period in Real Personal Consumption Expenditures by Major Type of Product (A) (Q)
    T20301,
    /// Table 2.3.2. Contributions to Percent Change in Real Personal Consumption Expenditures by Major Type of Product (A) (Q)
    T20302,
    /// Table 2.3.3. Real Personal Consumption Expenditures by Major Type of Product, Quantity Indexes (A) (Q)
    T20303,
    /// Table 2.3.4. Price Indexes for Personal Consumption Expenditures by Major Type of Product (A) (Q)
    T20304,
    /// Table 2.3.5. Personal Consumption Expenditures by Major Type of Product (A) (Q)
    T20305,
    /// Table 2.3.6. Real Personal Consumption Expenditures by Major Type of Product, Chained Dollars (A) (Q)
    T20306,
    /// Table 2.3.7. Percent Change From Preceding Period in Prices for Personal Consumption Expenditures by Major Type of Product (A) (Q)
    T20307,
    /// Table 2.3.8. Contributions to Percent Change in Prices for Personal Consumption Expenditures by Major Type of Product (A) (Q)
    T20308,
    /// Table 2.3.11. Real Personal Consumption Expenditures by Major Type of Product: Percent Change from Quarter One Year Ago (Q)
    T20311,
    /// Table 2.4.1. Percent Change From Preceding Period in Real Personal Consumption Expenditures by Type of Product (A) (Q)
    T20401,
    /// Table 2.4.2. Contributions to Percent Change in Real Personal Consumption Expenditures by Type of Product (A) (Q)
    T20402,
    /// Table 2.4.3. Real Personal Consumption Expenditures by Type of Product, Quantity Indexes (A) (Q)
    T20403,
    /// Table 2.4.4. Price Indexes for Personal Consumption Expenditures by Type of Product (A) (Q)
    T20404,
    /// Table 2.4.5. Personal Consumption Expenditures by Type of Product (A) (Q)
    T20405,
    /// Table 2.4.6. Real Personal Consumption Expenditures by Type of Product, Chained Dollars (A) (Q)
    T20406,
    /// Table 2.4.7. Percent Change From Preceding Period in Prices for Personal Consumption Expenditures by Type of Product (A) (Q)
    T20407,
    /// Table 2.4.8. Contributions to Percent Change in Prices for Personal Consumption Expenditures by Type of Product (A) (Q)
    T20408,
    /// Table 2.5.3. Real Personal Consumption Expenditures by Function, Quantity Indexes (A)
    T20503,
    /// Table 2.5.4. Price Indexes for Personal Consumption Expenditures by Function (A)
    T20504,
    /// Table 2.5.5. Personal Consumption Expenditures by Function (A)
    T20505,
    /// Table 2.5.6. Real Personal Consumption Expenditures by Function, Chained Dollars (A)
    T20506,
    /// Table 2.6. Personal Income and Its Disposition, Monthly (M)
    T20600,
    /// Table 2.7A. Wages and Salaries by Industry, Monthly (M)
    T20700A,
    /// Table 2.7B. Wages and Salaries by Industry, Monthly (M)
    T20700B,
    /// Table 2.8.1. Percent Change From Preceding Period in Real Personal Consumption Expenditures by Major Type of Product, Monthly (M)
    T20801,
    /// Table 2.8.2. Contributions to Percent Change in Real Personal Consumption Expenditures by Major Type of Product, Monthly (M)
    T20802,
    /// Table 2.8.3. Real Personal Consumption Expenditures by Major Type of Product, Monthly, Quantity Indexes (M)
    T20803,
    /// Table 2.8.4. Price Indexes for Personal Consumption Expenditures by Major Type of Product, Monthly (M)
    T20804,
    /// Table 2.8.5. Personal Consumption Expenditures by Major Type of Product, Monthly (M)
    T20805,
    /// Table 2.8.6. Real Personal Consumption Expenditures by Major Type of Product, Monthly, Chained Dollars (M)
    T20806,
    /// Table 2.8.7. Percent Change From Preceding Period in Prices for Personal Consumption Expenditures by Major Type of Product, Monthly (M)
    T20807,
    /// Table 2.8.8. Contributions to Percent Change in Prices for Personal Consumption Expenditures by Major Type of Product, Monthly (M)
    T20808,
    /// Table 2.8.11. Real Personal Consumption Expenditures by Major Type of Product: Percent Change from Month One Year Ago (M)
    T20811,
    /// Table 2.9. Personal Income and Its Disposition by Households and by Nonprofit Institutions Serving Households (A)
    T20900,
    /// Table 2.10. Distributions of Personal and Disposable Income for Households (A)
    T21000,
    /// Table 3.1. Government Current Receipts and Expenditures (A) (Q)
    T30100,
    /// Table 3.2. Federal Government Current Receipts and Expenditures (A) (Q)
    T30200,
    /// Table 3.3. State and Local Government Current Receipts and Expenditures (A) (Q)
    T30300,
    /// Table 3.4. Personal Current Tax Receipts (A)
    T30400,
    /// Table 3.5. Taxes on Production and Imports (A)
    T30500,
    /// Table 3.6. Contributions for Government Social Insurance (A)
    T30600,
    /// Table 3.7. Government Current Transfer Receipts (A)
    T30700,
    /// Table 3.8. Current Surplus of Government Enterprises (A)
    T30800,
    /// Table 3.9.1. Percent Change From Preceding Period in Real Government Consumption Expenditures and Gross Investment (A) (Q)
    T30901,
    /// Table 3.9.2. Contributions to Percent Change in Real Government Consumption Expenditures and Gross Investment (A) (Q)
    T30902,
    /// Table 3.9.3. Real Government Consumption Expenditures and Gross Investment, Quantity Indexes (A) (Q)
    T30903,
    /// Table 3.9.4. Price Indexes for Government Consumption Expenditures and Gross Investment (A) (Q)
    T30904,
    /// Table 3.9.5. Government Consumption Expenditures and Gross Investment (A) (Q)
    T30905,
    /// Table 3.9.6. Real Government Consumption Expenditures and Gross Investment, Chained Dollars (A) (Q)
    T30906,
    /// Table 3.10.1. Percent Change From Preceding Period in Real Government Consumption Expenditures and General Government Gross Output (A) (Q)
    T31001,
    /// Table 3.10.3. Real Government Consumption Expenditures and General Government Gross Output, Quantity Indexes (A) (Q)
    T31003,
    /// Table 3.10.4. Price Indexes for Government Consumption Expenditures and General Government Gross Output (A) (Q)
    T31004,
    /// Table 3.10.5. Government Consumption Expenditures and General Government Gross Output (A) (Q)
    T31005,
    /// Table 3.10.6. Real Government Consumption Expenditures and General Government Gross Output, Chained Dollars (A) (Q)
    T31006,
    /// Table 3.11.1. Percent Change From Preceding Period in Real National Defense Consumption Expenditures and Gross Investment by Type (A) (Q)
    T31101,
    /// Table 3.11.2. Contributions to Percent Change in National Defense Consumption Expenditures and Gross Investment by Type (A) (Q)
    T31102,
    /// Table 3.11.3. Real National Defense Consumption Expenditures and Gross Investment by Type, Quantity Indexes (A) (Q)
    T31103,
    /// Table 3.11.4. Price Indexes for National Defense Consumption Expenditures and Gross Investment by Type (A) (Q)
    T31104,
    /// Table 3.11.5. National Defense Consumption Expenditures and Gross Investment by Type (A) (Q)
    T31105,
    /// Table 3.11.6. Real National Defense Consumption Expenditures and Gross Investment by Type, Chained Dollars (A) (Q)
    T31106,
    /// Table 3.12. Government Social Benefits (A)
    T31200,
    /// Table 3.13. Subsidies (A)
    T31300,
    /// Table 3.14. Government Social Insurance Funds Current Receipts and Expenditures (A)
    T31400,
    /// Table 3.15.1. Percent Change From Preceding Period in Real Government Consumption Expenditures and Gross Investment by Function (A)
    T31501,
    /// Table 3.15.2. Contributions to Percent Change in Real Government Consumption Expenditures and Gross Investment by Function (A)
    T31502,
    /// Table 3.15.3. Real Government Consumption Expenditures and Gross Investment by Function, Quantity Indexes (A)
    T31503,
    /// Table 3.15.4. Price Indexes for Government Consumption Expenditures and Gross Investment by Function (A)
    T31504,
    /// Table 3.15.5. Government Consumption Expenditures and Gross Investment by Function (A)
    T31505,
    /// Table 3.15.6. Real Government Consumption Expenditures and Gross Investment by Function, Chained Dollars (A)
    T31506,
    /// Table 3.16. Government Current Expenditures by Function (A)
    T31600,
    /// Table 3.17. Selected Government Current and Capital Expenditures by Function (A)
    T31700,
    /// Table 3.18A. Relation of Federal Government Current Receipts and Expenditures in the National Income and Product Accounts to the Consolidated Cash Statement, Fiscal Years and Quarters (A) (Q)
    T31800A,
    /// Table 3.18B. Relation of Federal Government Current Receipts and Expenditures in the National Income and Product Accounts to the Budget, Fiscal Years and Quarters (A) (Q)
    T31800B,
    /// Table 3.19. Relation of State and Local Government Current Receipts and Expenditures in the National Income and Product Accounts to Census Bureau 'Government Finances' Data, Fiscal Years (A)
    T31900,
    /// Table 3.20. State Government Current Receipts and Expenditures (A)
    T32000,
    /// Table 3.21. Local Government Current Receipts and Expenditures (A)
    T32100,
    /// Table 4.1. Foreign Transactions in the National Income and Product Accounts (A) (Q)
    T40100,
    /// Table 4.2.1. Percent Change From Preceding Period in Real Exports and in Real Imports of Goods and Services by Type of Product (A) (Q)
    T40201,
    /// Table 4.2.2. Contributions to Percent Change in Real Exports and Real Imports of Goods and Services by Type of Product (A) (Q)
    T40202,
    /// Table 4.2.3A. Real Exports and Imports of Goods and Services by Type of Product, Quantity Indexes (A) (Q)
    T40203A,
    /// Table 4.2.3B. Real Exports and Imports of Goods and Services by Type of Product, Quantity Indexes (A) (Q)
    T40203B,
    /// Table 4.2.4A. Price Indexes for Exports and Imports of Goods and Services by Type of Product (A) (Q)
    T40204A,
    /// Table 4.2.4B. Price Indexes for Exports and Imports of Goods and Services by Type of Product (A) (Q)
    T40204B,
    /// Table 4.2.5A. Exports and Imports of Goods and Services by Type of Product (A) (Q)
    T40205A,
    /// Table 4.2.5B. Exports and Imports of Goods and Services by Type of Product (A) (Q)
    T40205B,
    /// Table 4.2.6B. Real Exports and Imports of Goods and Services by Type of Product, Chained Dollars (A) (Q)
    T40206B,
    /// Table 4.3A. Relation of Foreign Transactions in the National Income and Product Accounts to the Corresponding Items in the International Transactions Accounts (A)
    T4030A,
    /// Table 4.3B. Relation of Foreign Transactions in the National Income and Product Accounts to the Corresponding Items in the International Transactions Accounts (A)
    T4030B,
    /// Table 4.3C. Relation of Foreign Transactions in the National Income and Product Accounts to the Corresponding Items in the International Transactions Accounts (A) (Q)
    T4030C,
    /// Table 5.1. Saving and Investment by Sector (A) (Q)
    T50100,
    /// Table 5.2.3. Real Gross and Net Domestic Investment by Major Type, Quantity Indexes (A)
    T50203,
    /// Table 5.2.5. Gross and Net Domestic Investment by Major Type (A)
    T50205,
    /// Table 5.2.6. Real Gross and Net Domestic Investment by Major Type, Chained dollars (A)
    T50206,
    /// Table 5.3.1. Percent Change From Preceding Period in Real Private Fixed Investment by Type (A) (Q)
    T50301,
    /// Table 5.3.2. Contributions to Percent Change in Real Private Fixed Investment by Type (A) (Q)
    T50302,
    /// Table 5.3.3. Real Private Fixed Investment by Type, Quantity Indexes (A) (Q)
    T50303,
    /// Table 5.3.4. Price Indexes for Private Fixed Investment by Type (A) (Q)
    T50304,
    /// Table 5.3.5. Private Fixed Investment by Type (A) (Q)
    T50305,
    /// Table 5.3.6. Real Private Fixed Investment by Type, Chained Dollars (A) (Q)
    T50306,
    /// Table 5.4.1. Percent Change From Preceding Period in Real Private Fixed Investment in Structures by Type (A)
    T50401,
    /// Table 5.4.2. Contributions to Percent Change in Real Private Fixed Investment in Structures by Type (A)
    T50402,
    /// Table 5.4.3. Real Private Fixed Investment in Structures by Type, Quantity Indexes (A)
    T50403,
    /// Table 5.4.4. Price Indexes for Private Fixed Investment in Structures by Type (A)
    T50404,
    /// Table 5.4.5. Private Fixed Investment in Structures by Type (A)
    T50405,
    /// Table 5.4.6. Real Private Fixed Investment in Structures by Type, Chained Dollars (A)
    T50406,
    /// Table 5.5.1. Percent Change From Preceding Period in Real Private Fixed Investment in Equipment by Type (A)
    T50501,
    /// Table 5.5.2. Contributions to Percent Change in Real Private Fixed Investment in Equipment by Type (A)
    T50502,
    /// Table 5.5.3. Real Private Fixed Investment in Equipment by Type, Quantity Indexes (A)
    T50503,
    /// Table 5.5.4. Price Indexes for Private Fixed Investment in Equipment by Type (A)
    T50504,
    /// Table 5.5.5. Private Fixed Investment in Equipment by Type (A)
    T50505,
    /// Table 5.5.6. Real Private Fixed Investment in Equipment by Type, Chained Dollars (A)
    T50506,
    /// Table 5.6.1. Percent Change From Preceding Period in Real Private Fixed Investment in Intellectual Property Products by Type (A)
    T50601,
    /// Table 5.6.2. Contributions to Percent Change in Private Fixed Investment in Intellectual Property Products by Type (A)
    T50602,
    /// Table 5.6.3. Real Private Fixed Investment in Intellectual Property Products by Type, Quantity Indexes (A)
    T50603,
    /// Table 5.6.4. Price Indexes for Private Fixed Investment in Intellectual Property Products by Type (A)
    T50604,
    /// Table 5.6.5. Private Fixed Investment in Intellectual Property Products by Type (A)
    T50605,
    /// Table 5.6.6. Real Private Fixed Investment in Intellectual Property Products by Type, Chained Dollars (A)
    T50606,
    /// Table 5.7.5A. Change in Private Inventories by Industry (A) (Q)
    T50705A,
    /// Table 5.7.5B. Change in Private Inventories by Industry (A) (Q)
    T50705B,
    /// Table 5.7.6A. Change in Real Private Inventories by Industry, Chained Dollars (A) (Q)
    T50706A,
    /// Table 5.7.6B. Change in Real Private Inventories by Industry, Chained Dollars (A) (Q)
    T50706B,
    /// Table 5.8.5A. Private Inventories and Domestic Final Sales of Business by Industry (Q)
    T50805A,
    /// Table 5.8.5B. Private Inventories and Domestic Final Sales by Industry (Q)
    T50805B,
    /// Table 5.8.6A. Real Private Inventories and Real Domestic Final Sales of Business by Industry, Chained Dollars (Q)
    T50806A,
    /// Table 5.8.6B. Real Private Inventories and Real Domestic Final Sales by Industry, Chained Dollars (Q)
    T50806B,
    /// Table 5.8.9A. Implicit Price Deflators for Private Inventories by Industry (Q)
    T50809A,
    /// Table 5.8.9B. Implicit Price Deflators for Private Inventories by Industry (Q)
    T50809B,
    /// Table 5.9.3. Real Gross Government Fixed Investment by Type, Quantity Indexes (A)
    T50903,
    /// Table 5.9.4. Price Indexes for Gross Government Fixed Investment by Type (A)
    T50904,
    /// Table 5.9.5. Gross Government Fixed Investment by Type (A)
    T50905,
    /// Table 5.9.6. Real Gross Government Fixed Investment by Type, Chained Dollars (A)
    T50906,
    /// Table 5.10. Changes in Net Stock of Produced Assets (Fixed Assets and Inventories) (A)
    T51000,
    /// Table 5.11. Capital Transfers Paid and Received, by Sector and by Type (A)
    T51100,
    /// Table 6.1B. National Income Without Capital Consumption Adjustment by Industry (A) (Q)
    T60100B,
    /// Table 6.1C. National Income Without Capital Consumption Adjustment by Industry (A) (Q)
    T60100C,
    /// Table 6.1D. National Income Without Capital Consumption Adjustment by Industry (A) (Q)
    T60100D,
    /// Table 6.2A. Compensation of Employees by Industry (A)
    T60200A,
    /// Table 6.2B. Compensation of Employees by Industry (A)
    T60200B,
    /// Table 6.2C. Compensation of Employees by Industry (A)
    T60200C,
    /// Table 6.2D. Compensation of Employees by Industry (A)
    T60200D,
    /// Table 6.3A. Wages and Salaries by Industry (A)
    T60300A,
    /// Table 6.3B. Wages and Salaries by Industry (A)
    T60300B,
    /// Table 6.3C. Wages and Salaries by Industry (A)
    T60300C,
    /// Table 6.3D. Wages and Salaries by Industry (A)
    T60300D,
    /// Table 6.4A. Full-Time and Part-Time Employees by Industry (A)
    T60400A,
    /// Table 6.4B. Full-Time and Part-Time Employees by Industry (A)
    T60400B,
    /// Table 6.4C. Full-Time and Part-Time Employees by Industry (A)
    T60400C,
    /// Table 6.4D. Full-Time and Part-Time Employees by Industry (A)
    T60400D,
    /// Table 6.5A. Full-Time Equivalent Employees by Industry (A)
    T60500A,
    /// Table 6.5B. Full-Time Equivalent Employees by Industry (A)
    T60500B,
    /// Table 6.5C. Full-Time Equivalent Employees by Industry (A)
    T60500C,
    /// Table 6.5D. Full-Time Equivalent Employees by Industry (A)
    T60500D,
    /// Table 6.6A. Wages and Salaries Per Full-Time Equivalent Employee by Industry (A)
    T60600A,
    /// Table 6.6B. Wages and Salaries Per Full-Time Equivalent Employee by Industry (A)
    T60600B,
    /// Table 6.6C. Wages and Salaries Per Full-Time Equivalent Employee by Industry (A)
    T60600C,
    /// Table 6.6D. Wages and Salaries Per Full-Time Equivalent Employee by Industry (A)
    T60600D,
    /// Table 6.10B. Employer Contributions for Government Social Insurance by Industry (A)
    T61000B,
    /// Table 6.10C. Employer Contributions for Government Social Insurance by Industry (A)
    T61000C,
    /// Table 6.10D. Employer Contributions for Government Social Insurance by Industry (A)
    T61000D,
    /// Table 6.11A. Employer Contributions for Employee Pension and Insurance Funds by Industry and by Type (A)
    T61100A,
    /// Table 6.11B. Employer Contributions for Employee Pension and Insurance Funds by Industry and by Type (A)
    T61100B,
    /// Table 6.11C. Employer Contributions for Employee Pension and Insurance Funds by Industry and by Type (A)
    T61100C,
    /// Table 6.11D. Employer Contributions for Employee Pension and Insurance Funds by Industry and by Type (A)
    T61100D,
    /// Table 6.12A. Nonfarm Proprietors' Income by Industry (A)
    T61200A,
    /// Table 6.12B. Nonfarm Proprietors' Income by Industry (A)
    T61200B,
    /// Table 6.12C. Nonfarm Proprietors' Income by Industry (A)
    T61200C,
    /// Table 6.12D. Nonfarm Proprietors' Income by Industry (A)
    T61200D,
    /// Table 6.13A. Noncorporate Capital Consumption Allowances by Industry (A)
    T61300A,
    /// Table 6.13B. Noncorporate Capital Consumption Allowances by Industry (A)
    T61300B,
    /// Table 6.13C. Noncorporate Capital Consumption Allowances by Industry (A)
    T61300C,
    /// Table 6.13D. Noncorporate Capital Consumption Allowances by Industry (A)
    T61300D,
    /// Table 6.14A. Inventory Valuation Adjustment to Nonfarm Incomes by Legal Form of Organization and by Industry (A)
    T61400A,
    /// Table 6.14B. Inventory Valuation Adjustment to Nonfarm Incomes by Legal Form of Organization and by Industry (A)
    T61400B,
    /// Table 6.14C. Inventory Valuation Adjustment to Nonfarm Incomes by Legal Form of Organization and by Industry (A)
    T61400C,
    /// Table 6.14D. Inventory Valuation Adjustment to Nonfarm Incomes by Legal Form of Organization and by Industry (A)
    T61400D,
    /// Table 6.15A. Net Interest by Industry (A)
    T61500A,
    /// Table 6.15B. Net Interest by Industry (A)
    T61500B,
    /// Table 6.15C. Net Interest by Industry (A)
    T61500C,
    /// Table 6.15D. Net Interest by Industry (A)
    T61500D,
    /// Table 6.16A. Corporate Profits by Industry (A)
    T61600A,
    /// Table 6.16B. Corporate Profits by Industry (A) (Q)
    T61600B,
    /// Table 6.16C. Corporate Profits by Industry (A) (Q)
    T61600C,
    /// Table 6.16D. Corporate Profits by Industry (A) (Q)
    T61600D,
    /// Table 6.17A. Corporate Profits Before Tax by Industry (A)
    T61700A,
    /// Table 6.17B. Corporate Profits Before Tax by Industry (A)
    T61700B,
    /// Table 6.17C. Corporate Profits Before Tax by Industry (A)
    T61700C,
    /// Table 6.17D. Corporate Profits Before Tax by Industry (A)
    T61700D,
    /// Table 6.18A. Taxes on Corporate Income by Industry (A)
    T61800A,
    /// Table 6.18B. Taxes on Corporate Income by Industry (A)
    T61800B,
    /// Table 6.18C. Taxes on Corporate Income by Industry (A)
    T61800C,
    /// Table 6.18D. Taxes on Corporate Income by Industry (A)
    T61800D,
    /// Table 6.19A. Corporate Profits After Tax by Industry (A)
    T61900A,
    /// Table 6.19B. Corporate Profits After Tax by Industry (A)
    T61900B,
    /// Table 6.19C. Corporate Profits After Tax by Industry (A)
    T61900C,
    /// Table 6.19D. Corporate Profits After Tax by Industry (A)
    T61900D,
    /// Table 6.20A. Net Corporate Dividend Payments by Industry (A)
    T62000A,
    /// Table 6.20B. Net Corporate Dividend Payments by Industry (A)
    T62000B,
    /// Table 6.20C. Net Corporate Dividend Payments by Industry (A)
    T62000C,
    /// Table 6.20D. Net Corporate Dividend Payments by Industry (A)
    T62000D,
    /// Table 6.21A. Undistributed Corporate Profits by Industry (A)
    T62100A,
    /// Table 6.21B. Undistributed Corporate Profits by Industry (A)
    T62100B,
    /// Table 6.21C. Undistributed Corporate Profits by Industry (A)
    T62100C,
    /// Table 6.21D. Undistributed Corporate Profits by Industry (A)
    T62100D,
    /// Table 6.22A. Corporate Capital Consumption Allowances by Industry (A)
    T62200A,
    /// Table 6.22B. Corporate Capital Consumption Allowances by Industry (A)
    T62200B,
    /// Table 6.22C. Corporate Capital Consumption Allowances by Industry (A)
    T62200C,
    /// Table 6.22D. Corporate Capital Consumption Allowances by Industry (A)
    T62200D,
    /// Table 7.1. Selected Per Capita Product and Income Series in Current and Chained Dollars (A) (Q)
    T70100,
    /// Table 7.2.1A. Percent Change From Preceding Period in Real Auto Output (A) (Q)
    T70201A,
    /// Table 7.2.1B. Percent Change From Preceding Period in Real Motor Vehicle Output (A) (Q)
    T70201B,
    /// Table 7.2.3A. Real Auto Output, Quantity Indexes (A) (Q)
    T70203A,
    /// Table 7.2.3B. Real Motor Vehicle Output, Quantity Indexes (A) (Q)
    T70203B,
    /// Table 7.2.4A. Price Indexes for Auto Output (A) (Q)
    T70204A,
    /// Table 7.2.4B. Price Indexes for Motor Vehicle Output (A) (Q)
    T70204B,
    /// Table 7.2.5A. Auto Output (A) (Q)
    T70205A,
    /// Table 7.2.5B. Motor Vehicle Output (A) (Q)
    T70205B,
    /// Table 7.2.6B. Real Motor Vehicle Output, Chained Dollars (A) (Q)
    T70206B,
    /// Table 7.3.3. Real Farm Sector Output, Real Gross Value Added, and Real Net Value Added, Quantity Indexes (A)
    T70303,
    /// Table 7.3.4. Price Indexes for Farm Sector Output, Gross Value Added, and Net Value Added (A)
    T70304,
    /// Table 7.3.5. Farm Sector Output, Gross Value Added, and Net Value Added (A)
    T70305,
    /// Table 7.3.6. Real Farm Sector Output, Real Gross Value Added, and Real Net Value Added, Chained Dollars (A)
    T70306,
    /// Table 7.4.3. Real Housing Sector Output, Real Gross Value Added, and Real Net Value Added, Quantity Indexes (A)
    T70403,
    /// Table 7.4.4. Price Indexes for Housing Sector Output, Gross Value Added, and Net Value Added (A)
    T70404,
    /// Table 7.4.5. Housing Sector Output, Gross Value Added, and Net Value Added (A)
    T70405,
    /// Table 7.4.6. Real Housing Sector Output, Real Gross Value Added, and Real Net Value Added, Chained Dollars (A)
    T70406,
    /// Table 7.5. Consumption of Fixed Capital by Legal Form of Organization and Type of Income (A) (Q)
    T70500,
    /// Table 7.6. Capital Consumption Adjustment by Legal Form of Organization and Type of Adjustment (A)
    T70600,
    /// Table 7.7. Business Current Transfer Payments by Type (A)
    T70700,
    /// Table 7.8. Supplements to Wages and Salaries by Type (A)
    T70800,
    /// Table 7.9. Rental Income of Persons by Legal Form of Organization and by Type of Income (A)
    T70900,
    /// Table 7.10. Dividends Paid and Received by Sector (A)
    T71000,
    /// Table 7.11. Interest Paid and Received by Sector and Legal Form of Organization (A)
    T71100,
    /// Table 7.13. Relation of Consumption of Fixed Capital in the National Income and Product Accounts to Depreciation and Amortization as Published by the Internal Revenue Service (A)
    T71300,
    /// Table 7.14. Relation of Nonfarm Proprietors' Income in the National Income and Product Accounts to Corresponding Measures as Published by the Internal Revenue Service (A)
    T71400,
    /// Table 7.15. Relation of Net Farm Income in the National Income and Product Accounts to Net Farm Income as Published by the U.S. Department of Agriculture (A)
    T71500,
    /// Table 7.16. Relation of Corporate Profits, Taxes, and Dividends in the National Income and Product Accounts to Corresponding Measures as Published by the Internal Revenue Service (A)
    T71600,
    /// Table 7.17. Relation of Monetary Interest Paid and Received in the National Income and Product Accounts to Corresponding Measures as Published by the Internal Revenue Service (A)
    T71700,
    /// Table 7.18. Relation of Wages and Salaries in the National Income and Product Accounts to Wages and Salaries as Published by the Bureau of Labor Statistics (A)
    T71800,
    /// Table 7.19. Comparison of Income and Outlays of Nonprofit Institutions Serving Households with Revenue and Expenses as Published by the Internal Revenue Service (A)
    T71900,
    /// Table 7.20. Transactions of Defined Benefit and Defined Contribution Pension Plans (A)
    T72000,
    /// Table 7.21. Transactions of Defined Benefit Pension Plans (A)
    T72100,
    /// Table 7.22. Transactions of Private Defined Benefit Pension Plans (A)
    T72200,
    /// Table 7.23. Transactions of Federal Government Defined Benefit Pension Plans (A)
    T72300,
    /// Table 7.24. Transactions of State and Local Government Defined Benefit Pension Plans (A)
    T72400,
    /// Table 7.25. Transactions of Defined Contribution Pension Plans (A)
    T72500,
    /// Table 8.1.3. Real Gross Domestic Product, Quantity Indexes, Not Seasonally Adjusted (Q)
    T80103,
    /// Table 8.1.4. Price Indexes for Gross Domestic Product, Not Seasonally Adjusted (Q)
    T80104,
    /// Table 8.1.5. Gross Domestic Product, Not Seasonally Adjusted (Q)
    T80105,
    /// Table 8.1.6. Real Gross Domestic Product, Chained Dollars, Not Seasonally Adjusted (Q)
    T80106,
    /// Table 8.1.11. Real Gross Domestic Product: Percent Change From Quarter One Year Ago, Not Seasonally Adjusted (Q)
    T80111,
    /// Table 8.2. Gross Domestic Income by Type of Income, Not Seasonally Adjusted (Q)
    T80200,
    /// Table 8.3. Federal Government Current Receipts and Expenditures, Not Seasonally Adjusted (Q)
    T80300,
    /// Table 8.4. State and Local Government Current Receipts and Expenditures, Not Seasonally Adjusted (Q)
    T80400,
}

impl NipaTableName {
    /// Returns the full descriptive name of the table as a static string.
    ///
    /// This method provides the complete title of the NIPA table corresponding to the enum variant.
    pub fn description(&self) -> &'static str {
        match self {
            NipaTableName::T10101 => {
                "Table 1.1.1. Percent Change From Preceding Period in Real Gross Domestic Product (A) (Q)"
            }
            NipaTableName::T10102 => {
                "Table 1.1.2. Contributions to Percent Change in Real Gross Domestic Product (A) (Q)"
            }
            NipaTableName::T10103 => {
                "Table 1.1.3. Real Gross Domestic Product, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T10104 => {
                "Table 1.1.4. Price Indexes for Gross Domestic Product (A) (Q)"
            }
            NipaTableName::T10105 => "Table 1.1.5. Gross Domestic Product (A) (Q)",
            NipaTableName::T10106 => {
                "Table 1.1.6. Real Gross Domestic Product, Chained Dollars (A) (Q)"
            }
            NipaTableName::T10107 => {
                "Table 1.1.7. Percent Change From Preceding Period in Prices for Gross Domestic Product (A) (Q)"
            }
            NipaTableName::T10108 => {
                "Table 1.1.8. Contributions to Percent Change in the Gross Domestic Product Price Index (A) (Q)"
            }
            NipaTableName::T10109 => {
                "Table 1.1.9. Implicit Price Deflators for Gross Domestic Product (A) (Q)"
            }
            NipaTableName::T10110 => {
                "Table 1.1.10. Percentage Shares of Gross Domestic Product (A) (Q)"
            }
            NipaTableName::T10111 => {
                "Table 1.1.11. Real Gross Domestic Product: Percent Change From Quarter One Year Ago (Q)"
            }
            NipaTableName::T10201 => {
                "Table 1.2.1. Percent Change From Preceding Period in Real Gross Domestic Product by Major Type of Product (A) (Q)"
            }
            NipaTableName::T10202 => {
                "Table 1.2.2. Contributions to Percent Change in Real Gross Domestic Product by Major Type of Product (A) (Q)"
            }
            NipaTableName::T10203 => {
                "Table 1.2.3. Real Gross Domestic Product by Major Type of Product, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T10204 => {
                "Table 1.2.4. Price Indexes for Gross Domestic Product by Major Type of Product (A) (Q)"
            }
            NipaTableName::T10205 => {
                "Table 1.2.5. Gross Domestic Product by Major Type of Product (A) (Q)"
            }
            NipaTableName::T10206 => {
                "Table 1.2.6. Real Gross Domestic Product by Major Type of Product, Chained Dollars (A) (Q)"
            }
            NipaTableName::T10301 => {
                "Table 1.3.1. Percent Change From Preceding Period in Real Gross Value Added by Sector (A) (Q)"
            }
            NipaTableName::T10303 => {
                "Table 1.3.3. Real Gross Value Added by Sector, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T10304 => {
                "Table 1.3.4. Price Indexes for Gross Value Added by Sector (A) (Q)"
            }
            NipaTableName::T10305 => "Table 1.3.5. Gross Value Added by Sector (A) (Q)",
            NipaTableName::T10306 => {
                "Table 1.3.6. Real Gross Value Added by Sector, Chained Dollars (A) (Q)"
            }
            NipaTableName::T10401 => {
                "Table 1.4.1. Percent Change From Preceding Period in Real Gross Domestic Product, Real Gross Domestic Purchases, and Real Final Sales to Domestic Purchasers (A) (Q)"
            }
            NipaTableName::T10403 => {
                "Table 1.4.3. Real Gross Domestic Product, Real Gross Domestic Purchases, and Real Final Sales to Domestic Purchasers, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T10404 => {
                "Table 1.4.4. Price Indexes for Gross Domestic Product, Gross Domestic Purchases, and Final Sales to Domestic Purchasers (A) (Q)"
            }
            NipaTableName::T10405 => {
                "Table 1.4.5. Relation of Gross Domestic Product, Gross Domestic Purchases, and Final Sales to Domestic Purchasers (A) (Q)"
            }
            NipaTableName::T10406 => {
                "Table 1.4.6. Relation of Real Gross Domestic Product, Real Gross Domestic Purchases, and Real Final Sales to Domestic Purchasers, Chained Dollars (A) (Q)"
            }
            NipaTableName::T10501 => {
                "Table 1.5.1. Percent Change From Preceding Period in Real Gross Domestic Product, Expanded Detail (A) (Q)"
            }
            NipaTableName::T10502 => {
                "Table 1.5.2. Contributions to Percent Change in Real Gross Domestic Product, Expanded Detail (A) (Q)"
            }
            NipaTableName::T10503 => {
                "Table 1.5.3. Real Gross Domestic Product, Expanded Detail, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T10504 => {
                "Table 1.5.4. Price Indexes for Gross Domestic Product, Expanded Detail (A) (Q)"
            }
            NipaTableName::T10505 => "Table 1.5.5. Gross Domestic Product, Expanded Detail (A) (Q)",
            NipaTableName::T10506 => {
                "Table 1.5.6. Real Gross Domestic Product, Expanded Detail, Chained Dollars (A) (Q)"
            }
            NipaTableName::T10604 => {
                "Table 1.6.4. Price Indexes for Gross Domestic Purchases (A) (Q)"
            }
            NipaTableName::T10607 => {
                "Table 1.6.7. Percent Change From Preceding Period in Prices for Gross Domestic Purchases (A) (Q)"
            }
            NipaTableName::T10608 => {
                "Table 1.6.8. Contributions to Percent Change in the Gross Domestic Purchases Price Index (A) (Q)"
            }
            NipaTableName::T10701 => {
                "Table 1.7.1. Percent Change From Preceding Period in Real Gross Domestic Product, Real Gross National Product, and Real Net National Product (A) (Q)"
            }
            NipaTableName::T10703 => {
                "Table 1.7.3. Real Gross Domestic Product, Real Gross National Product, and Real Net National Product, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T10704 => {
                "Table 1.7.4. Price Indexes for Gross Domestic Product, Gross National Product, and Net National Product (A) (Q)"
            }
            NipaTableName::T10705 => {
                "Table 1.7.5. Relation of Gross Domestic Product, Gross National Product, Net National Product, National Income, and Personal Income (A) (Q)"
            }
            NipaTableName::T10706 => {
                "Table 1.7.6. Relation of Real Gross Domestic Product, Real Gross National Product, and Real Net National Product, Chained Dollars (A) (Q)"
            }
            NipaTableName::T10803 => {
                "Table 1.8.3. Command-Basis Real Gross Domestic Product and Gross National Product, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T10806 => {
                "Table 1.8.6. Command-Basis Real Gross Domestic Product and Gross National Product, Chained Dollars (A) (Q)"
            }
            NipaTableName::T10903 => {
                "Table 1.9.3. Real Net Value Added by Sector, Quantity Indexes (A)"
            }
            NipaTableName::T10904 => "Table 1.9.4. Price Indexes for Net Value Added by Sector (A)",
            NipaTableName::T10905 => "Table 1.9.5. Net Value Added by Sector (A)",
            NipaTableName::T10906 => {
                "Table 1.9.6. Real Net Value Added by Sector, Chained Dollars (A)"
            }
            NipaTableName::T11000 => "Table 1.10. Gross Domestic Income by Type of Income (A) (Q)",
            NipaTableName::T11100 => "Table 1.11. Percentage Shares of Gross Domestic Income (A)",
            NipaTableName::T11200 => "Table 1.12. National Income by Type of Income (A) (Q)",
            NipaTableName::T11300 => {
                "Table 1.13. National Income by Sector, Legal Form of Organization, and Type of Income (A)"
            }
            NipaTableName::T11400 => {
                "Table 1.14. Gross Value Added of Domestic Corporate Business in Current Dollars and Gross Value Added of Nonfinancial Domestic Corporate Business in Current and Chained Dollars (A) (Q)"
            }
            NipaTableName::T11500 => {
                "Table 1.15. Price, Costs, and Profit Per Unit of Real Gross Value Added of Nonfinancial Domestic Corporate Business (A) (Q)"
            }
            NipaTableName::T11600 => {
                "Table 1.16. Sources and Uses of Private Enterprise Income (A)"
            }
            NipaTableName::T11701 => {
                "Table 1.17.1. Percent Change From Preceding Period in Real Gross Domestic Product, Real Gross Domestic Income, and Other Major NIPA Aggregates (A) (Q)"
            }
            NipaTableName::T11705 => {
                "Table 1.17.5. Gross Domestic Product, Gross Domestic Income, and Other Major NIPA Aggregates (A) (Q)"
            }
            NipaTableName::T11706 => {
                "Table 1.17.6. Real Gross Domestic Product, Real Gross Domestic Income, and Other Major NIPA Aggregates, Chained Dollars (A) (Q)"
            }
            NipaTableName::T20100 => "Table 2.1. Personal Income and Its Disposition (A) (Q)",
            NipaTableName::T20200A => "Table 2.2A. Wages and Salaries by Industry (A) (Q)",
            NipaTableName::T20200B => "Table 2.2B. Wages and Salaries by Industry (A) (Q)",
            NipaTableName::T20301 => {
                "Table 2.3.1. Percent Change From Preceding Period in Real Personal Consumption Expenditures by Major Type of Product (A) (Q)"
            }
            NipaTableName::T20302 => {
                "Table 2.3.2. Contributions to Percent Change in Real Personal Consumption Expenditures by Major Type of Product (A) (Q)"
            }
            NipaTableName::T20303 => {
                "Table 2.3.3. Real Personal Consumption Expenditures by Major Type of Product, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T20304 => {
                "Table 2.3.4. Price Indexes for Personal Consumption Expenditures by Major Type of Product (A) (Q)"
            }
            NipaTableName::T20305 => {
                "Table 2.3.5. Personal Consumption Expenditures by Major Type of Product (A) (Q)"
            }
            NipaTableName::T20306 => {
                "Table 2.3.6. Real Personal Consumption Expenditures by Major Type of Product, Chained Dollars (A) (Q)"
            }
            NipaTableName::T20307 => {
                "Table 2.3.7. Percent Change From Preceding Period in Prices for Personal Consumption Expenditures by Major Type of Product (A) (Q)"
            }
            NipaTableName::T20308 => {
                "Table 2.3.8. Contributions to Percent Change in Prices for Personal Consumption Expenditures by Major Type of Product (A) (Q)"
            }
            NipaTableName::T20311 => {
                "Table 2.3.11. Real Personal Consumption Expenditures by Major Type of Product: Percent Change from Quarter One Year Ago (Q)"
            }
            NipaTableName::T20401 => {
                "Table 2.4.1. Percent Change From Preceding Period in Real Personal Consumption Expenditures by Type of Product (A) (Q)"
            }
            NipaTableName::T20402 => {
                "Table 2.4.2. Contributions to Percent Change in Real Personal Consumption Expenditures by Type of Product (A) (Q)"
            }
            NipaTableName::T20403 => {
                "Table 2.4.3. Real Personal Consumption Expenditures by Type of Product, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T20404 => {
                "Table 2.4.4. Price Indexes for Personal Consumption Expenditures by Type of Product (A) (Q)"
            }
            NipaTableName::T20405 => {
                "Table 2.4.5. Personal Consumption Expenditures by Type of Product (A) (Q)"
            }
            NipaTableName::T20406 => {
                "Table 2.4.6. Real Personal Consumption Expenditures by Type of Product, Chained Dollars (A) (Q)"
            }
            NipaTableName::T20407 => {
                "Table 2.4.7. Percent Change From Preceding Period in Prices for Personal Consumption Expenditures by Type of Product (A) (Q)"
            }
            NipaTableName::T20408 => {
                "Table 2.4.8. Contributions to Percent Change in Prices for Personal Consumption Expenditures by Type of Product (A) (Q)"
            }
            NipaTableName::T20503 => {
                "Table 2.5.3. Real Personal Consumption Expenditures by Function, Quantity Indexes (A)"
            }
            NipaTableName::T20504 => {
                "Table 2.5.4. Price Indexes for Personal Consumption Expenditures by Function (A)"
            }
            NipaTableName::T20505 => {
                "Table 2.5.5. Personal Consumption Expenditures by Function (A)"
            }
            NipaTableName::T20506 => {
                "Table 2.5.6. Real Personal Consumption Expenditures by Function, Chained Dollars (A)"
            }
            NipaTableName::T20600 => "Table 2.6. Personal Income and Its Disposition, Monthly (M)",
            NipaTableName::T20700A => "Table 2.7A. Wages and Salaries by Industry, Monthly (M)",
            NipaTableName::T20700B => "Table 2.7B. Wages and Salaries by Industry, Monthly (M)",
            NipaTableName::T20801 => {
                "Table 2.8.1. Percent Change From Preceding Period in Real Personal Consumption Expenditures by Major Type of Product, Monthly (M)"
            }
            NipaTableName::T20802 => {
                "Table 2.8.2. Contributions to Percent Change in Real Personal Consumption Expenditures by Major Type of Product, Monthly (M)"
            }
            NipaTableName::T20803 => {
                "Table 2.8.3. Real Personal Consumption Expenditures by Major Type of Product, Monthly, Quantity Indexes (M)"
            }
            NipaTableName::T20804 => {
                "Table 2.8.4. Price Indexes for Personal Consumption Expenditures by Major Type of Product, Monthly (M)"
            }
            NipaTableName::T20805 => {
                "Table 2.8.5. Personal Consumption Expenditures by Major Type of Product, Monthly (M)"
            }
            NipaTableName::T20806 => {
                "Table 2.8.6. Real Personal Consumption Expenditures by Major Type of Product, Monthly, Chained Dollars (M)"
            }
            NipaTableName::T20807 => {
                "Table 2.8.7. Percent Change From Preceding Period in Prices for Personal Consumption Expenditures by Major Type of Product, Monthly (M)"
            }
            NipaTableName::T20808 => {
                "Table 2.8.8. Contributions to Percent Change in Prices for Personal Consumption Expenditures by Major Type of Product, Monthly (M)"
            }
            NipaTableName::T20811 => {
                "Table 2.8.11. Real Personal Consumption Expenditures by Major Type of Product: Percent Change from Month One Year Ago (M)"
            }
            NipaTableName::T20900 => {
                "Table 2.9. Personal Income and Its Disposition by Households and by Nonprofit Institutions Serving Households (A)"
            }
            NipaTableName::T21000 => {
                "Table 2.10. Distributions of Personal and Disposable Income for Households (A)"
            }
            NipaTableName::T30100 => {
                "Table 3.1. Government Current Receipts and Expenditures (A) (Q)"
            }
            NipaTableName::T30200 => {
                "Table 3.2. Federal Government Current Receipts and Expenditures (A) (Q)"
            }
            NipaTableName::T30300 => {
                "Table 3.3. State and Local Government Current Receipts and Expenditures (A) (Q)"
            }
            NipaTableName::T30400 => "Table 3.4. Personal Current Tax Receipts (A)",
            NipaTableName::T30500 => "Table 3.5. Taxes on Production and Imports (A)",
            NipaTableName::T30600 => "Table 3.6. Contributions for Government Social Insurance (A)",
            NipaTableName::T30700 => "Table 3.7. Government Current Transfer Receipts (A)",
            NipaTableName::T30800 => "Table 3.8. Current Surplus of Government Enterprises (A)",
            NipaTableName::T30901 => {
                "Table 3.9.1. Percent Change From Preceding Period in Real Government Consumption Expenditures and Gross Investment (A) (Q)"
            }
            NipaTableName::T30902 => {
                "Table 3.9.2. Contributions to Percent Change in Real Government Consumption Expenditures and Gross Investment (A) (Q)"
            }
            NipaTableName::T30903 => {
                "Table 3.9.3. Real Government Consumption Expenditures and Gross Investment, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T30904 => {
                "Table 3.9.4. Price Indexes for Government Consumption Expenditures and Gross Investment (A) (Q)"
            }
            NipaTableName::T30905 => {
                "Table 3.9.5. Government Consumption Expenditures and Gross Investment (A) (Q)"
            }
            NipaTableName::T30906 => {
                "Table 3.9.6. Real Government Consumption Expenditures and Gross Investment, Chained Dollars (A) (Q)"
            }
            NipaTableName::T31001 => {
                "Table 3.10.1. Percent Change From Preceding Period in Real Government Consumption Expenditures and General Government Gross Output (A) (Q)"
            }
            NipaTableName::T31003 => {
                "Table 3.10.3. Real Government Consumption Expenditures and General Government Gross Output, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T31004 => {
                "Table 3.10.4. Price Indexes for Government Consumption Expenditures and General Government Gross Output (A) (Q)"
            }
            NipaTableName::T31005 => {
                "Table 3.10.5. Government Consumption Expenditures and General Government Gross Output (A) (Q)"
            }
            NipaTableName::T31006 => {
                "Table 3.10.6. Real Government Consumption Expenditures and General Government Gross Output, Chained Dollars (A) (Q)"
            }
            NipaTableName::T31101 => {
                "Table 3.11.1. Percent Change From Preceding Period in Real National Defense Consumption Expenditures and Gross Investment by Type (A) (Q)"
            }
            NipaTableName::T31102 => {
                "Table 3.11.2. Contributions to Percent Change in National Defense Consumption Expenditures and Gross Investment by Type (A) (Q)"
            }
            NipaTableName::T31103 => {
                "Table 3.11.3. Real National Defense Consumption Expenditures and Gross Investment by Type, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T31104 => {
                "Table 3.11.4. Price Indexes for National Defense Consumption Expenditures and Gross Investment by Type (A) (Q)"
            }
            NipaTableName::T31105 => {
                "Table 3.11.5. National Defense Consumption Expenditures and Gross Investment by Type (A) (Q)"
            }
            NipaTableName::T31106 => {
                "Table 3.11.6. Real National Defense Consumption Expenditures and Gross Investment by Type, Chained Dollars (A) (Q)"
            }
            NipaTableName::T31200 => "Table 3.12. Government Social Benefits (A)",
            NipaTableName::T31300 => "Table 3.13. Subsidies (A)",
            NipaTableName::T31400 => {
                "Table 3.14. Government Social Insurance Funds Current Receipts and Expenditures (A)"
            }
            NipaTableName::T31501 => {
                "Table 3.15.1. Percent Change From Preceding Period in Real Government Consumption Expenditures and Gross Investment by Function (A)"
            }
            NipaTableName::T31502 => {
                "Table 3.15.2. Contributions to Percent Change in Real Government Consumption Expenditures and Gross Investment by Function (A)"
            }
            NipaTableName::T31503 => {
                "Table 3.15.3. Real Government Consumption Expenditures and Gross Investment by Function, Quantity Indexes (A)"
            }
            NipaTableName::T31504 => {
                "Table 3.15.4. Price Indexes for Government Consumption Expenditures and Gross Investment by Function (A)"
            }
            NipaTableName::T31505 => {
                "Table 3.15.5. Government Consumption Expenditures and Gross Investment by Function (A)"
            }
            NipaTableName::T31506 => {
                "Table 3.15.6. Real Government Consumption Expenditures and Gross Investment by Function, Chained Dollars (A)"
            }
            NipaTableName::T31600 => "Table 3.16. Government Current Expenditures by Function (A)",
            NipaTableName::T31700 => {
                "Table 3.17. Selected Government Current and Capital Expenditures by Function (A)"
            }
            NipaTableName::T31800A => {
                "Table 3.18A. Relation of Federal Government Current Receipts and Expenditures in the National Income and Product Accounts to the Consolidated Cash Statement, Fiscal Years and Quarters (A) (Q)"
            }
            NipaTableName::T31800B => {
                "Table 3.18B. Relation of Federal Government Current Receipts and Expenditures in the National Income and Product Accounts to the Budget, Fiscal Years and Quarters (A) (Q)"
            }
            NipaTableName::T31900 => {
                "Table 3.19. Relation of State and Local Government Current Receipts and Expenditures in the National Income and Product Accounts to Census Bureau 'Government Finances' Data, Fiscal Years (A)"
            }
            NipaTableName::T32000 => {
                "Table 3.20. State Government Current Receipts and Expenditures (A)"
            }
            NipaTableName::T32100 => {
                "Table 3.21. Local Government Current Receipts and Expenditures (A)"
            }
            NipaTableName::T40100 => {
                "Table 4.1. Foreign Transactions in the National Income and Product Accounts (A) (Q)"
            }
            NipaTableName::T40201 => {
                "Table 4.2.1. Percent Change From Preceding Period in Real Exports and in Real Imports of Goods and Services by Type of Product (A) (Q)"
            }
            NipaTableName::T40202 => {
                "Table 4.2.2. Contributions to Percent Change in Real Exports and Real Imports of Goods and Services by Type of Product (A) (Q)"
            }
            NipaTableName::T40203A => {
                "Table 4.2.3A. Real Exports and Imports of Goods and Services by Type of Product, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T40203B => {
                "Table 4.2.3B. Real Exports and Imports of Goods and Services by Type of Product, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T40204A => {
                "Table 4.2.4A. Price Indexes for Exports and Imports of Goods and Services by Type of Product (A) (Q)"
            }
            NipaTableName::T40204B => {
                "Table 4.2.4B. Price Indexes for Exports and Imports of Goods and Services by Type of Product (A) (Q)"
            }
            NipaTableName::T40205A => {
                "Table 4.2.5A. Exports and Imports of Goods and Services by Type of Product (A) (Q)"
            }
            NipaTableName::T40205B => {
                "Table 4.2.5B. Exports and Imports of Goods and Services by Type of Product (A) (Q)"
            }
            NipaTableName::T40206B => {
                "Table 4.2.6B. Real Exports and Imports of Goods and Services by Type of Product, Chained Dollars (A) (Q)"
            }
            NipaTableName::T4030A => {
                "Table 4.3A. Relation of Foreign Transactions in the National Income and Product Accounts to the Corresponding Items in the International Transactions Accounts (A)"
            }
            NipaTableName::T4030B => {
                "Table 4.3B. Relation of Foreign Transactions in the National Income and Product Accounts to the Corresponding Items in the International Transactions Accounts (A)"
            }
            NipaTableName::T4030C => {
                "Table 4.3C. Relation of Foreign Transactions in the National Income and Product Accounts to the Corresponding Items in the International Transactions Accounts (A) (Q)"
            }
            NipaTableName::T50100 => "Table 5.1. Saving and Investment by Sector (A) (Q)",
            NipaTableName::T50203 => {
                "Table 5.2.3. Real Gross and Net Domestic Investment by Major Type, Quantity Indexes (A)"
            }
            NipaTableName::T50205 => {
                "Table 5.2.5. Gross and Net Domestic Investment by Major Type (A)"
            }
            NipaTableName::T50206 => {
                "Table 5.2.6. Real Gross and Net Domestic Investment by Major Type, Chained dollars (A)"
            }
            NipaTableName::T50301 => {
                "Table 5.3.1. Percent Change From Preceding Period in Real Private Fixed Investment by Type (A) (Q)"
            }
            NipaTableName::T50302 => {
                "Table 5.3.2. Contributions to Percent Change in Real Private Fixed Investment by Type (A) (Q)"
            }
            NipaTableName::T50303 => {
                "Table 5.3.3. Real Private Fixed Investment by Type, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T50304 => {
                "Table 5.3.4. Price Indexes for Private Fixed Investment by Type (A) (Q)"
            }
            NipaTableName::T50305 => "Table 5.3.5. Private Fixed Investment by Type (A) (Q)",
            NipaTableName::T50306 => {
                "Table 5.3.6. Real Private Fixed Investment by Type, Chained Dollars (A) (Q)"
            }
            NipaTableName::T50401 => {
                "Table 5.4.1. Percent Change From Preceding Period in Real Private Fixed Investment in Structures by Type (A)"
            }
            NipaTableName::T50402 => {
                "Table 5.4.2. Contributions to Percent Change in Real Private Fixed Investment in Structures by Type (A)"
            }
            NipaTableName::T50403 => {
                "Table 5.4.3. Real Private Fixed Investment in Structures by Type, Quantity Indexes (A)"
            }
            NipaTableName::T50404 => {
                "Table 5.4.4. Price Indexes for Private Fixed Investment in Structures by Type (A)"
            }
            NipaTableName::T50405 => {
                "Table 5.4.5. Private Fixed Investment in Structures by Type (A)"
            }
            NipaTableName::T50406 => {
                "Table 5.4.6. Real Private Fixed Investment in Structures by Type, Chained Dollars (A)"
            }
            NipaTableName::T50501 => {
                "Table 5.5.1. Percent Change From Preceding Period in Real Private Fixed Investment in Equipment by Type (A)"
            }
            NipaTableName::T50502 => {
                "Table 5.5.2. Contributions to Percent Change in Real Private Fixed Investment in Equipment by Type (A)"
            }
            NipaTableName::T50503 => {
                "Table 5.5.3. Real Private Fixed Investment in Equipment by Type, Quantity Indexes (A)"
            }
            NipaTableName::T50504 => {
                "Table 5.5.4. Price Indexes for Private Fixed Investment in Equipment by Type (A)"
            }
            NipaTableName::T50505 => {
                "Table 5.5.5. Private Fixed Investment in Equipment by Type (A)"
            }
            NipaTableName::T50506 => {
                "Table 5.5.6. Real Private Fixed Investment in Equipment by Type, Chained Dollars (A)"
            }
            NipaTableName::T50601 => {
                "Table 5.6.1. Percent Change From Preceding Period in Real Private Fixed Investment in Intellectual Property Products by Type (A)"
            }
            NipaTableName::T50602 => {
                "Table 5.6.2. Contributions to Percent Change in Private Fixed Investment in Intellectual Property Products by Type (A)"
            }
            NipaTableName::T50603 => {
                "Table 5.6.3. Real Private Fixed Investment in Intellectual Property Products by Type, Quantity Indexes (A)"
            }
            NipaTableName::T50604 => {
                "Table 5.6.4. Price Indexes for Private Fixed Investment in Intellectual Property Products by Type (A)"
            }
            NipaTableName::T50605 => {
                "Table 5.6.5. Private Fixed Investment in Intellectual Property Products by Type (A)"
            }
            NipaTableName::T50606 => {
                "Table 5.6.6. Real Private Fixed Investment in Intellectual Property Products by Type, Chained Dollars (A)"
            }
            NipaTableName::T50705A => {
                "Table 5.7.5A. Change in Private Inventories by Industry (A) (Q)"
            }
            NipaTableName::T50705B => {
                "Table 5.7.5B. Change in Private Inventories by Industry (A) (Q)"
            }
            NipaTableName::T50706A => {
                "Table 5.7.6A. Change in Real Private Inventories by Industry, Chained Dollars (A) (Q)"
            }
            NipaTableName::T50706B => {
                "Table 5.7.6B. Change in Real Private Inventories by Industry, Chained Dollars (A) (Q)"
            }
            NipaTableName::T50805A => {
                "Table 5.8.5A. Private Inventories and Domestic Final Sales of Business by Industry (Q)"
            }
            NipaTableName::T50805B => {
                "Table 5.8.5B. Private Inventories and Domestic Final Sales by Industry (Q)"
            }
            NipaTableName::T50806A => {
                "Table 5.8.6A. Real Private Inventories and Real Domestic Final Sales of Business by Industry, Chained Dollars (Q)"
            }
            NipaTableName::T50806B => {
                "Table 5.8.6B. Real Private Inventories and Real Domestic Final Sales by Industry, Chained Dollars (Q)"
            }
            NipaTableName::T50809A => {
                "Table 5.8.9A. Implicit Price Deflators for Private Inventories by Industry (Q)"
            }
            NipaTableName::T50809B => {
                "Table 5.8.9B. Implicit Price Deflators for Private Inventories by Industry (Q)"
            }
            NipaTableName::T50903 => {
                "Table 5.9.3. Real Gross Government Fixed Investment by Type, Quantity Indexes (A)"
            }
            NipaTableName::T50904 => {
                "Table 5.9.4. Price Indexes for Gross Government Fixed Investment by Type (A)"
            }
            NipaTableName::T50905 => "Table 5.9.5. Gross Government Fixed Investment by Type (A)",
            NipaTableName::T50906 => {
                "Table 5.9.6. Real Gross Government Fixed Investment by Type, Chained Dollars (A)"
            }
            NipaTableName::T51000 => {
                "Table 5.10. Changes in Net Stock of Produced Assets (Fixed Assets and Inventories) (A)"
            }
            NipaTableName::T51100 => {
                "Table 5.11. Capital Transfers Paid and Received, by Sector and by Type (A)"
            }
            NipaTableName::T60100B => {
                "Table 6.1B. National Income Without Capital Consumption Adjustment by Industry (A) (Q)"
            }
            NipaTableName::T60100C => {
                "Table 6.1C. National Income Without Capital Consumption Adjustment by Industry (A) (Q)"
            }
            NipaTableName::T60100D => {
                "Table 6.1D. National Income Without Capital Consumption Adjustment by Industry (A) (Q)"
            }
            NipaTableName::T60200A => "Table 6.2A. Compensation of Employees by Industry (A)",
            NipaTableName::T60200B => "Table 6.2B. Compensation of Employees by Industry (A)",
            NipaTableName::T60200C => "Table 6.2C. Compensation of Employees by Industry (A)",
            NipaTableName::T60200D => "Table 6.2D. Compensation of Employees by Industry (A)",
            NipaTableName::T60300A => "Table 6.3A. Wages and Salaries by Industry (A)",
            NipaTableName::T60300B => "Table 6.3B. Wages and Salaries by Industry (A)",
            NipaTableName::T60300C => "Table 6.3C. Wages and Salaries by Industry (A)",
            NipaTableName::T60300D => "Table 6.3D. Wages and Salaries by Industry (A)",
            NipaTableName::T60400A => {
                "Table 6.4A. Full-Time and Part-Time Employees by Industry (A)"
            }
            NipaTableName::T60400B => {
                "Table 6.4B. Full-Time and Part-Time Employees by Industry (A)"
            }
            NipaTableName::T60400C => {
                "Table 6.4C. Full-Time and Part-Time Employees by Industry (A)"
            }
            NipaTableName::T60400D => {
                "Table 6.4D. Full-Time and Part-Time Employees by Industry (A)"
            }
            NipaTableName::T60500A => "Table 6.5A. Full-Time Equivalent Employees by Industry (A)",
            NipaTableName::T60500B => "Table 6.5B. Full-Time Equivalent Employees by Industry (A)",
            NipaTableName::T60500C => "Table 6.5C. Full-Time Equivalent Employees by Industry (A)",
            NipaTableName::T60500D => "Table 6.5D. Full-Time Equivalent Employees by Industry (A)",
            NipaTableName::T60600A => {
                "Table 6.6A. Wages and Salaries Per Full-Time Equivalent Employee by Industry (A)"
            }
            NipaTableName::T60600B => {
                "Table 6.6B. Wages and Salaries Per Full-Time Equivalent Employee by Industry (A)"
            }
            NipaTableName::T60600C => {
                "Table 6.6C. Wages and Salaries Per Full-Time Equivalent Employee by Industry (A)"
            }
            NipaTableName::T60600D => {
                "Table 6.6D. Wages and Salaries Per Full-Time Equivalent Employee by Industry (A)"
            }
            NipaTableName::T61000B => {
                "Table 6.10B. Employer Contributions for Government Social Insurance by Industry (A)"
            }
            NipaTableName::T61000C => {
                "Table 6.10C. Employer Contributions for Government Social Insurance by Industry (A)"
            }
            NipaTableName::T61000D => {
                "Table 6.10D. Employer Contributions for Government Social Insurance by Industry (A)"
            }
            NipaTableName::T61100A => {
                "Table 6.11A. Employer Contributions for Employee Pension and Insurance Funds by Industry and by Type (A)"
            }
            NipaTableName::T61100B => {
                "Table 6.11B. Employer Contributions for Employee Pension and Insurance Funds by Industry and by Type (A)"
            }
            NipaTableName::T61100C => {
                "Table 6.11C. Employer Contributions for Employee Pension and Insurance Funds by Industry and by Type (A)"
            }
            NipaTableName::T61100D => {
                "Table 6.11D. Employer Contributions for Employee Pension and Insurance Funds by Industry and by Type (A)"
            }
            NipaTableName::T61200A => "Table 6.12A. Nonfarm Proprietors' Income by Industry (A)",
            NipaTableName::T61200B => "Table 6.12B. Nonfarm Proprietors' Income by Industry (A)",
            NipaTableName::T61200C => "Table 6.12C. Nonfarm Proprietors' Income by Industry (A)",
            NipaTableName::T61200D => "Table 6.12D. Nonfarm Proprietors' Income by Industry (A)",
            NipaTableName::T61300A => {
                "Table 6.13A. Noncorporate Capital Consumption Allowances by Industry (A)"
            }
            NipaTableName::T61300B => {
                "Table 6.13B. Noncorporate Capital Consumption Allowances by Industry (A)"
            }
            NipaTableName::T61300C => {
                "Table 6.13C. Noncorporate Capital Consumption Allowances by Industry (A)"
            }
            NipaTableName::T61300D => {
                "Table 6.13D. Noncorporate Capital Consumption Allowances by Industry (A)"
            }
            NipaTableName::T61400A => {
                "Table 6.14A. Inventory Valuation Adjustment to Nonfarm Incomes by Legal Form of Organization and by Industry (A)"
            }
            NipaTableName::T61400B => {
                "Table 6.14B. Inventory Valuation Adjustment to Nonfarm Incomes by Legal Form of Organization and by Industry (A)"
            }
            NipaTableName::T61400C => {
                "Table 6.14C. Inventory Valuation Adjustment to Nonfarm Incomes by Legal Form of Organization and by Industry (A)"
            }
            NipaTableName::T61400D => {
                "Table 6.14D. Inventory Valuation Adjustment to Nonfarm Incomes by Legal Form of Organization and by Industry (A)"
            }
            NipaTableName::T61500A => "Table 6.15A. Net Interest by Industry (A)",
            NipaTableName::T61500B => "Table 6.15B. Net Interest by Industry (A)",
            NipaTableName::T61500C => "Table 6.15C. Net Interest by Industry (A)",
            NipaTableName::T61500D => "Table 6.15D. Net Interest by Industry (A)",
            NipaTableName::T61600A => "Table 6.16A. Corporate Profits by Industry (A)",
            NipaTableName::T61600B => "Table 6.16B. Corporate Profits by Industry (A) (Q)",
            NipaTableName::T61600C => "Table 6.16C. Corporate Profits by Industry (A) (Q)",
            NipaTableName::T61600D => "Table 6.16D. Corporate Profits by Industry (A) (Q)",
            NipaTableName::T61700A => "Table 6.17A. Corporate Profits Before Tax by Industry (A)",
            NipaTableName::T61700B => "Table 6.17B. Corporate Profits Before Tax by Industry (A)",
            NipaTableName::T61700C => "Table 6.17C. Corporate Profits Before Tax by Industry (A)",
            NipaTableName::T61700D => "Table 6.17D. Corporate Profits Before Tax by Industry (A)",
            NipaTableName::T61800A => "Table 6.18A. Taxes on Corporate Income by Industry (A)",
            NipaTableName::T61800B => "Table 6.18B. Taxes on Corporate Income by Industry (A)",
            NipaTableName::T61800C => "Table 6.18C. Taxes on Corporate Income by Industry (A)",
            NipaTableName::T61800D => "Table 6.18D. Taxes on Corporate Income by Industry (A)",
            NipaTableName::T61900A => "Table 6.19A. Corporate Profits After Tax by Industry (A)",
            NipaTableName::T61900B => "Table 6.19B. Corporate Profits After Tax by Industry (A)",
            NipaTableName::T61900C => "Table 6.19C. Corporate Profits After Tax by Industry (A)",
            NipaTableName::T61900D => "Table 6.19D. Corporate Profits After Tax by Industry (A)",
            NipaTableName::T62000A => {
                "Table 6.20A. Net Corporate Dividend Payments by Industry (A)"
            }
            NipaTableName::T62000B => {
                "Table 6.20B. Net Corporate Dividend Payments by Industry (A)"
            }
            NipaTableName::T62000C => {
                "Table 6.20C. Net Corporate Dividend Payments by Industry (A)"
            }
            NipaTableName::T62000D => {
                "Table 6.20D. Net Corporate Dividend Payments by Industry (A)"
            }
            NipaTableName::T62100A => {
                "Table 6.21A. Undistributed Corporate Profits by Industry (A)"
            }
            NipaTableName::T62100B => {
                "Table 6.21B. Undistributed Corporate Profits by Industry (A)"
            }
            NipaTableName::T62100C => {
                "Table 6.21C. Undistributed Corporate Profits by Industry (A)"
            }
            NipaTableName::T62100D => {
                "Table 6.21D. Undistributed Corporate Profits by Industry (A)"
            }
            NipaTableName::T62200A => {
                "Table 6.22A. Corporate Capital Consumption Allowances by Industry (A)"
            }
            NipaTableName::T62200B => {
                "Table 6.22B. Corporate Capital Consumption Allowances by Industry (A)"
            }
            NipaTableName::T62200C => {
                "Table 6.22C. Corporate Capital Consumption Allowances by Industry (A)"
            }
            NipaTableName::T62200D => {
                "Table 6.22D. Corporate Capital Consumption Allowances by Industry (A)"
            }
            NipaTableName::T70100 => {
                "Table 7.1. Selected Per Capita Product and Income Series in Current and Chained Dollars (A) (Q)"
            }
            NipaTableName::T70201A => {
                "Table 7.2.1A. Percent Change From Preceding Period in Real Auto Output (A) (Q)"
            }
            NipaTableName::T70201B => {
                "Table 7.2.1B. Percent Change From Preceding Period in Real Motor Vehicle Output (A) (Q)"
            }
            NipaTableName::T70203A => "Table 7.2.3A. Real Auto Output, Quantity Indexes (A) (Q)",
            NipaTableName::T70203B => {
                "Table 7.2.3B. Real Motor Vehicle Output, Quantity Indexes (A) (Q)"
            }
            NipaTableName::T70204A => "Table 7.2.4A. Price Indexes for Auto Output (A) (Q)",
            NipaTableName::T70204B => {
                "Table 7.2.4B. Price Indexes for Motor Vehicle Output (A) (Q)"
            }
            NipaTableName::T70205A => "Table 7.2.5A. Auto Output (A) (Q)",
            NipaTableName::T70205B => "Table 7.2.5B. Motor Vehicle Output (A) (Q)",
            NipaTableName::T70206B => {
                "Table 7.2.6B. Real Motor Vehicle Output, Chained Dollars (A) (Q)"
            }
            NipaTableName::T70303 => {
                "Table 7.3.3. Real Farm Sector Output, Real Gross Value Added, and Real Net Value Added, Quantity Indexes (A)"
            }
            NipaTableName::T70304 => {
                "Table 7.3.4. Price Indexes for Farm Sector Output, Gross Value Added, and Net Value Added (A)"
            }
            NipaTableName::T70305 => {
                "Table 7.3.5. Farm Sector Output, Gross Value Added, and Net Value Added (A)"
            }
            NipaTableName::T70306 => {
                "Table 7.3.6. Real Farm Sector Output, Real Gross Value Added, and Real Net Value Added, Chained Dollars (A)"
            }
            NipaTableName::T70403 => {
                "Table 7.4.3. Real Housing Sector Output, Real Gross Value Added, and Real Net Value Added, Quantity Indexes (A)"
            }
            NipaTableName::T70404 => {
                "Table 7.4.4. Price Indexes for Housing Sector Output, Gross Value Added, and Net Value Added (A)"
            }
            NipaTableName::T70405 => {
                "Table 7.4.5. Housing Sector Output, Gross Value Added, and Net Value Added (A)"
            }
            NipaTableName::T70406 => {
                "Table 7.4.6. Real Housing Sector Output, Real Gross Value Added, and Real Net Value Added, Chained Dollars (A)"
            }
            NipaTableName::T70500 => {
                "Table 7.5. Consumption of Fixed Capital by Legal Form of Organization and Type of Income (A) (Q)"
            }
            NipaTableName::T70600 => {
                "Table 7.6. Capital Consumption Adjustment by Legal Form of Organization and Type of Adjustment (A)"
            }
            NipaTableName::T70700 => "Table 7.7. Business Current Transfer Payments by Type (A)",
            NipaTableName::T70800 => "Table 7.8. Supplements to Wages and Salaries by Type (A)",
            NipaTableName::T70900 => {
                "Table 7.9. Rental Income of Persons by Legal Form of Organization and by Type of Income (A)"
            }
            NipaTableName::T71000 => "Table 7.10. Dividends Paid and Received by Sector (A)",
            NipaTableName::T71100 => {
                "Table 7.11. Interest Paid and Received by Sector and Legal Form of Organization (A)"
            }
            NipaTableName::T71300 => {
                "Table 7.13. Relation of Consumption of Fixed Capital in the National Income and Product Accounts to Depreciation and Amortization as Published by the Internal Revenue Service (A)"
            }
            NipaTableName::T71400 => {
                "Table 7.14. Relation of Nonfarm Proprietors' Income in the National Income and Product Accounts to Corresponding Measures as Published by the Internal Revenue Service (A)"
            }
            NipaTableName::T71500 => {
                "Table 7.15. Relation of Net Farm Income in the National Income and Product Accounts to Net Farm Income as Published by the U.S. Department of Agriculture (A)"
            }
            NipaTableName::T71600 => {
                "Table 7.16. Relation of Corporate Profits, Taxes, and Dividends in the National Income and Product Accounts to Corresponding Measures as Published by the Internal Revenue Service (A)"
            }
            NipaTableName::T71700 => {
                "Table 7.17. Relation of Monetary Interest Paid and Received in the National Income and Product Accounts to Corresponding Measures as Published by the Internal Revenue Service (A)"
            }
            NipaTableName::T71800 => {
                "Table 7.18. Relation of Wages and Salaries in the National Income and Product Accounts to Wages and Salaries as Published by the Bureau of Labor Statistics (A)"
            }
            NipaTableName::T71900 => {
                "Table 7.19. Comparison of Income and Outlays of Nonprofit Institutions Serving Households with Revenue and Expenses as Published by the Internal Revenue Service (A)"
            }
            NipaTableName::T72000 => {
                "Table 7.20. Transactions of Defined Benefit and Defined Contribution Pension Plans (A)"
            }
            NipaTableName::T72100 => {
                "Table 7.21. Transactions of Defined Benefit Pension Plans (A)"
            }
            NipaTableName::T72200 => {
                "Table 7.22. Transactions of Private Defined Benefit Pension Plans (A)"
            }
            NipaTableName::T72300 => {
                "Table 7.23. Transactions of Federal Government Defined Benefit Pension Plans (A)"
            }
            NipaTableName::T72400 => {
                "Table 7.24. Transactions of State and Local Government Defined Benefit Pension Plans (A)"
            }
            NipaTableName::T72500 => {
                "Table 7.25. Transactions of Defined Contribution Pension Plans (A)"
            }
            NipaTableName::T80103 => {
                "Table 8.1.3. Real Gross Domestic Product, Quantity Indexes, Not Seasonally Adjusted (Q)"
            }
            NipaTableName::T80104 => {
                "Table 8.1.4. Price Indexes for Gross Domestic Product, Not Seasonally Adjusted (Q)"
            }
            NipaTableName::T80105 => {
                "Table 8.1.5. Gross Domestic Product, Not Seasonally Adjusted (Q)"
            }
            NipaTableName::T80106 => {
                "Table 8.1.6. Real Gross Domestic Product, Chained Dollars, Not Seasonally Adjusted (Q)"
            }
            NipaTableName::T80111 => {
                "Table 8.1.11. Real Gross Domestic Product: Percent Change From Quarter One Year Ago, Not Seasonally Adjusted (Q)"
            }
            NipaTableName::T80200 => {
                "Table 8.2. Gross Domestic Income by Type of Income, Not Seasonally Adjusted (Q)"
            }
            NipaTableName::T80300 => {
                "Table 8.3. Federal Government Current Receipts and Expenditures, Not Seasonally Adjusted (Q)"
            }
            NipaTableName::T80400 => {
                "Table 8.4. State and Local Government Current Receipts and Expenditures, Not Seasonally Adjusted (Q)"
            }
        }
    }
}

impl TryFrom<&NipaTable> for NipaTableName {
    type Error = DeriveFromStr;

    fn try_from(value: &NipaTable) -> Result<Self, Self::Error> {
        let key = value.table_name();
        Self::from_str(key)
            .map_err(|e| DeriveFromStr::new(key.to_owned(), e, line!(), file!().to_owned()))
    }
}

impl TryFrom<&ParameterFields> for NipaTableName {
    type Error = DeriveFromStr;

    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        let key = value.key();
        Self::from_str(key)
            .map_err(|e| DeriveFromStr::new(key.to_owned(), e, line!(), file!().to_owned()))
    }
}

impl TryFrom<&ParameterValueTable> for NipaTableName {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::NipaTable(tab) => Ok(NipaTableName::try_from(tab)?),
            ParameterValueTable::ParameterFields(tab) => Ok(NipaTableName::try_from(tab)?),
            other => {
                let error = ParameterValueTableVariant::new(
                    format!("NipaTable or ParameterFields needed, found {other:#?}"),
                    line!(),
                    file!().to_string(),
                );
                Err(error.into())
            }
        }
    }
}
