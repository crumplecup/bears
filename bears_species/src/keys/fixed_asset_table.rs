use crate::{
    BeaErr, DeriveFromStr, NipaTable, ParameterFields, ParameterName, ParameterValueTable,
    ParameterValueTableVariant,
};

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
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIter,
    derive_more::Display,
    derive_more::FromStr,
    derive_new::new,
)]
pub enum FixedAssetTable {
    /// Table 1.1. Current-Cost Net Stock of Fixed Assets and Consumer Durable Goods (A)
    #[default]
    #[display("FAAt101")]
    Faat101,
    /// Table 1.2. Chain-Type Quantity Indexes for Net Stock of Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt102")]
    Faat102,
    /// Table 1.3. Current-Cost Depreciation of Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt103")]
    Faat103,
    /// Table 1.4. Chain-Type Quantity Indexes for Depreciation of Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt104")]
    Faat104,
    /// Table 1.5. Investment in Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt105")]
    Faat105,
    /// Table 1.6. Chain-Type Quantity Indexes for Investment in Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt106")]
    Faat106,
    /// Table 1.7. Current-Cost Other Changes in Volume of Assets for Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt107")]
    Faat107,
    /// Table 1.8. Historical-Cost Other Changes in Volume of Assets for Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt108")]
    Faat108,
    /// Table 1.9. Current-Cost Average Age at Yearend of Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt109")]
    Faat109,
    /// Table 2.1. Current-Cost Net Stock of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt201")]
    Faat201,
    /// Table 2.2. Chain-Type Quantity Indexes for Net Stock of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt202")]
    Faat202,
    /// Table 2.3. Historical-Cost Net Stock of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt203")]
    Faat203,
    /// Table 2.4. Current-Cost Depreciation of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt204")]
    Faat204,
    /// Table 2.5. Chain-Type Quantity Indexes for Depreciation of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt205")]
    Faat205,
    /// Table 2.6. Historical-Cost Depreciation of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt206")]
    Faat206,
    /// Table 2.7. Investment in Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt207")]
    Faat207,
    /// Table 2.8. Chain-Type Quantity indexes for Investment in Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt208")]
    Faat208,
    /// Table 2.9. Current-Cost Average Age at Yearend of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt209")]
    Faat209,
    /// Table 2.10. Historical-Cost Average Age at Yearend of Private Fixed Assets,  Equipment, Structures, and Intellectual Property Products by Type (A)
    #[display("FAAt210")]
    Faat210,
    /// Table 3.1E. Current-Cost Net Stock of Private Equipment by Industry (A)
    #[display("FAAt301E")]
    Faat301E,
    /// Table 3.1ESI. Current-Cost Net Stock of Private Fixed Assets by Industry (A)
    #[display("FAAt301ESI")]
    Faat301Esi,
    /// Table 3.1I. Current-Cost Net Stock of Intellectual Property Products by Industry (A)
    #[display("FAAt301I")]
    Faat301I,
    /// Table 3.1S. Current-Cost net Stock of Private Structures by Industry (A)
    #[display("FAAt301S")]
    Faat301S,
    /// Table 3.2E. Chain-Type Quantity Indexes for Net Stock of Private Equipment by Industry (A)
    #[display("FAAt302E")]
    Faat302E,
    /// Table 3.2ESI. Chain-Type Quantity Indexes for Net Stock of Private Fixed Assets by Industry (A)
    #[display("FAAt302ESI")]
    Faat302Esi,
    /// Table 3.2I. Chain-Type Quantity Indexes for Net Stock of Intellectual Property Products by Industry (A)
    #[display("FAAt302I")]
    Faat302I,
    /// Table 3.2S. Chain-Type Quantity Indexes for Net Stock of Private Structures by Industry (A)
    #[display("FAAt302S")]
    Faat302S,
    /// Table 3.3E. Historical-Cost Net Stock of Private Equipment by Industry (A)
    #[display("FAAt303E")]
    Faat303E,
    /// Table 3.3ESI. Historical-Cost Net Stock of Private Fixed Assets by Industry (A)
    #[display("FAAt303ESI")]
    Faat303Esi,
    /// Table 3.3I. Historical-Cost Net Stock of Private Intellectual Property Products by Industry (A)
    #[display("FAAt303I")]
    Faat303I,
    /// Table 3.3S. Historical-Cost net Stock of Private Structures by Industry (A)
    #[display("FAAt303S")]
    Faat303S,
    /// Table 3.4E. Current-Cost Depreciation of Private Equipment by Industry (A)
    #[display("FAAt304E")]
    Faat304E,
    /// Table 3.4ESI. Current-Cost Depreciation of Private Fixed Assets by Industry (A)
    #[display("FAAt304ESI")]
    Faat304Esi,
    /// Table 3.4I. Current-Cost Depreciation of Private Intellectual Property Products by Industry (A)
    #[display("FAAt304I")]
    Faat304I,
    /// Table 3.4S. Current-Cost Depreciation of Private Structures by Industry (A)
    #[display("FAAt304S")]
    Faat304S,
    /// Table 3.5E. Chain-Type Quantity Indexes for Depreciation of Private Equipment by Industry (A)
    #[display("FAAt305E")]
    Faat305E,
    /// Table 3.5ESI. Chain-Type Quantity Indexes for Depreciation of Private Fixed Assets by Industry (A)
    #[display("FAAt305ESI")]
    Faat305Esi,
    /// Table 3.5I. Chain-Type Quantity Indexes for Depreciation of Private Intellectual Property Products by Industry (A)
    #[display("FAAt305I")]
    Faat305I,
    /// Table 3.5S. Chain-Type Quantity Indexes for Depreciation of Private Structures by Industry (A)
    #[display("FAAt305S")]
    Faat305S,
    /// Table 3.6E. Historical-Cost Depreciation of Private Equipment by Industry (A)
    #[display("FAAt306E")]
    Faat306E,
    /// Table 3.6ESI. Historical-Cost Depreciation of Private Fixed Assets by Industry (A)
    #[display("FAAt306ESI")]
    Faat306Esi,
    /// Table 3.6I. Historical-Cost Depreciation of Private Intellectual Property Products by Industry (A)
    #[display("FAAt306I")]
    Faat306I,
    /// Table 3.6S. Historical-Cost Depreciation of Private Structures by Industry (A)
    #[display("FAAt306S")]
    Faat306S,
    /// Table 3.7E. Investment in Private Equipment by Industry (A)
    #[display("FAAt307E")]
    Faat307E,
    /// Table 3.7ESI. Investment in Private Fixed Assets by Industry (A)
    #[display("FAAt307ESI")]
    Faat307Esi,
    /// Table 3.7I. Investment in Private Intellectual Property Products by Industry (A)
    #[display("FAAt307I")]
    Faat307I,
    /// Table 3.7S. Investment in Private Structures by Industry (A)
    #[display("FAAt307S")]
    Faat307S,
    /// Table 3.8E. Chain-Type Quantity Indexes for Investment in Private Equipment by Industry (A)
    #[display("FAAt308E")]
    Faat308E,
    /// Table 3.8ESI. Chain-Type Quantity Indexes for Investment in Private Fixed Assets by Industry (A)
    #[display("FAAt308ESI")]
    Faat308Esi,
    /// Table 3.8I. Chain-Type Quantity Indexes for Investment in Private Intellectual Property Products by Industry (A)
    #[display("FAAt308I")]
    Faat308I,
    /// Table 3.8S. Chain-Type Quantity Indexes for Investment in Private Structures by  Industry (A)
    #[display("FAAt308S")]
    Faat308S,
    /// Table 3.9E. Current-Cost Average Age at Yearend of Private Equipment by Industry (A)
    #[display("FAAt309E")]
    Faat309E,
    /// Table 3.9ESI. Current-Cost Average Age at Yearend of Private Fixed Assets by Industry (A)
    #[display("FAAt309ESI")]
    Faat309Esi,
    /// Table 3.9I. Current-Cost Average Age at Yearend of Private Intellectual Property Products by Industry (A)
    #[display("FAAt309I")]
    Faat309I,
    /// Table 3.9S. Current-Cost Average Age at Yearend of Private Structures by Industry (A)
    #[display("FAAt309S")]
    Faat309S,
    /// Table 3.10E. Historical-Cost Average Age at Yearend of Private Equipment by Industry (A)
    #[display("FAAt310E")]
    Faat310E,
    /// Table 3.10ESI. Historical-Cost Average Age at Yearend of Private Fixed Assets by Industry (A)
    #[display("FAAt310ESI")]
    Faat310Esi,
    /// Table 3.10I. Historical-Cost Average Age at Yearend of Private Intellectual Property Products by Industry (A)
    #[display("FAAt310I")]
    Faat310I,
    /// Table 3.10S. Historical-Cost Average Age at Yearend of Private Structures by Industry (A)
    #[display("FAAt310S")]
    Faat310S,
    /// Table 4.1. Current-Cost Net Stock of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt401")]
    Faat401,
    /// Table 4.2. Chain-Type Quantity Indexes for Net Stock of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt402")]
    Faat402,
    /// Table 4.3. Historical-Cost Net Stock of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt403")]
    Faat403,
    /// Table 4.4. Current-Cost Depreciation of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt404")]
    Faat404,
    /// Table 4.5. Chain-Type Quantity Indexes for Depreciation of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt405")]
    Faat405,
    /// Table 4.6. Historical-Cost Depreciation of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt406")]
    Faat406,
    /// Table 4.7. Investment in Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt407")]
    Faat407,
    /// Table 4.8. Chain-Type Quantity Indexes for Investment in Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt408")]
    Faat408,
    /// Table 4.9. Current-Cost Average Age at Yearend of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt409")]
    Faat409,
    /// Table 4.10. Historical-Cost Average Age at Yearend of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt410")]
    Faat410,
    /// Table 5.1. Current-Cost Net Stock of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt501")]
    Faat501,
    /// Table 5.2. Chain-Type Quantity Indexes for Net Stock of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt502")]
    Faat502,
    /// Table 5.3. Historical-Cost Net Stock of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt503")]
    Faat503,
    /// Table 5.4. Current-Cost Depreciation of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt504")]
    Faat504,
    /// Table 5.5. Chain-Type Quantity Indexes for Depreciation of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt505")]
    Faat505,
    /// Table 5.6. Historical-Cost Depreciation of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt506")]
    Faat506,
    /// Table 5.7. Investment in Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt507")]
    Faat507,
    /// Table 5.8. Chain-Type Quantity Indexes for Investment in Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt508")]
    Faat508,
    /// Table 5.9. Current-Cost Average Age at Yearend of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt509")]
    Faat509,
    /// Table 5.10. Historical-Cost Average Age at Yearend of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)
    #[display("FAAt510")]
    Faat510,
    /// Table 6.1. Current-Cost Net Stock of Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt601")]
    Faat601,
    /// Table 6.2. Chain-type Quantity Indexes for Net Stock of Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt602")]
    Faat602,
    /// Table 6.3. Historical-Cost Net Stock of Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt603")]
    Faat603,
    /// Table 6.4. Current-Cost Depreciation of Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt604")]
    Faat604,
    /// Table 6.5. Chain-Type Quantity Indexes for Depreciation of Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt605")]
    Faat605,
    /// Table 6.6. Historical-Cost Depreciation of Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt606")]
    Faat606,
    /// Table 6.7. Investment in Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt607")]
    Faat607,
    /// Table 6.8. Chain-Type Quantity Indexes for Investment in Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt608")]
    Faat608,
    /// Table 6.9. Current-Cost Average Age at Yearend of Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt609")]
    Faat609,
    /// Table 6.10. Historical-Cost Average Age at Yearend of Private Fixed Assets by Industry Group and Legal Form of Organization (A)
    #[display("FAAt610")]
    Faat610,
    /// Table 7.1. Current-Cost Net Stock of Government Fixed Assets (A)
    #[display("FAAt701")]
    Faat701,
    /// Table 7.2. Chain-Type Quantity Indexes for Net Stock of Government Fixed Assets (A)
    #[display("FAAt702")]
    Faat702,
    /// Table 7.3. Current-Cost Depreciation of Government Fixed Assets (A)
    #[display("FAAt703")]
    Faat703,
    /// Table 7.4. Chain-Type Quantity Indexes for Depreciation of Government Fixed Assets (A)
    #[display("FAAt704")]
    Faat704,
    /// Table 7.5. Investment in Government Fixed Assets (A)
    #[display("FAAt705")]
    Faat705,
    /// Table 7.6. Chain-Type Quantity Indexes for Investment in Government Fixed Assets (A)
    #[display("FAAt706")]
    Faat706,
    /// Table 7.7. Current-Cost Average Age at Yearend of Government Fixed Assets (A)
    #[display("FAAt707")]
    Faat707,
    /// Table 8.1. Current-Cost Net Stock of Consumer Durable Goods (A)
    #[display("FAAt801")]
    Faat801,
    /// Table 8.2. Chain-Type Quantity Indexes for Net Stock of Consumer Durable Goods (A)
    #[display("FAAt802")]
    Faat802,
    /// Table 8.3. Historical-Cost Net Stock of Consumer Durable Goods (A)
    #[display("FAAt803")]
    Faat803,
    /// Table 8.4. Current-Cost Depreciation of Consumer Durable Goods (A)
    #[display("FAAt804")]
    Faat804,
    /// Table 8.5. Chain-Type Quantity Indexes for Depreciation of Consumer Durable Goods (A)
    #[display("FAAt805")]
    Faat805,
    /// Table 8.6. Historical-Cost Depreciation of Consumer Durable Goods (A)
    #[display("FAAt806")]
    Faat806,
    /// Table 8.7. Investment in Consumer Durable Goods (A)
    #[display("FAAt807")]
    Faat807,
    /// Table 8.8. Chain-Type Quantity Indexes for Investment in Consumer Durable Goods (A)
    #[display("FAAt808")]
    Faat808,
    /// Table 8.9. Current-Cost Average Age at Yearend of Consumer Durable Goods (A)
    #[display("FAAt809")]
    Faat809,
    /// Table 8.10. Historical-Cost Average Age at Yearend of Consumer Durable Goods (A)
    #[display("FAAt810")]
    Faat810,
    /// Table 9.1. Real Net Stock of Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt901")]
    Faat901,
    /// Table 9.2. Real Depreciation of Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt902")]
    Faat902,
    /// Table 9.3. Real Investment in Fixed Assets and Consumer Durable Goods (A)
    #[display("FAAt903")]
    Faat903,
}

/// Returns the description associated with the table name.
impl FixedAssetTable {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Faat101 => {
                "Table 1.1. Current-Cost Net Stock of Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat102 => {
                "Table 1.2. Chain-Type Quantity Indexes for Net Stock of Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat103 => {
                "Table 1.3. Current-Cost Depreciation of Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat104 => {
                "Table 1.4. Chain-Type Quantity Indexes for Depreciation of Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat105 => "Table 1.5. Investment in Fixed Assets and Consumer Durable Goods (A)",
            Self::Faat106 => {
                "Table 1.6. Chain-Type Quantity Indexes for Investment in Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat107 => {
                "Table 1.7. Current-Cost Other Changes in Volume of Assets for Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat108 => {
                "Table 1.8. Historical-Cost Other Changes in Volume of Assets for Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat109 => {
                "Table 1.9. Current-Cost Average Age at Yearend of Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat201 => {
                "Table 2.1. Current-Cost Net Stock of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat202 => {
                "Table 2.2. Chain-Type Quantity Indexes for Net Stock of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat203 => {
                "Table 2.3. Historical-Cost Net Stock of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat204 => {
                "Table 2.4. Current-Cost Depreciation of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat205 => {
                "Table 2.5. Chain-Type Quantity Indexes for Depreciation of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat206 => {
                "Table 2.6. Historical-Cost Depreciation of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat207 => {
                "Table 2.7. Investment in Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat208 => {
                "Table 2.8. Chain-Type Quantity indexes for Investment in Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat209 => {
                "Table 2.9. Current-Cost Average Age at Yearend of Private Fixed Assets, Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat210 => {
                "Table 2.10. Historical-Cost Average Age at Yearend of Private Fixed Assets,  Equipment, Structures, and Intellectual Property Products by Type (A)"
            }
            Self::Faat301E => {
                "Table 3.1E. Current-Cost Net Stock of Private Equipment by Industry (A)"
            }
            Self::Faat301Esi => {
                "Table 3.1ESI. Current-Cost Net Stock of Private Fixed Assets by Industry (A)"
            }
            Self::Faat301I => {
                "Table 3.1I. Current-Cost Net Stock of Intellectual Property Products by Industry (A)"
            }
            Self::Faat301S => {
                "Table 3.1S. Current-Cost net Stock of Private Structures by Industry (A)"
            }
            Self::Faat302E => {
                "Table 3.2E. Chain-Type Quantity Indexes for Net Stock of Private Equipment by Industry (A)"
            }
            Self::Faat302Esi => {
                "Table 3.2ESI. Chain-Type Quantity Indexes for Net Stock of Private Fixed Assets by Industry (A)"
            }
            Self::Faat302I => {
                "Table 3.2I. Chain-Type Quantity Indexes for Net Stock of Intellectual Property Products by Industry (A)"
            }
            Self::Faat302S => {
                "Table 3.2S. Chain-Type Quantity Indexes for Net Stock of Private Structures by Industry (A)"
            }
            Self::Faat303E => {
                "Table 3.3E. Historical-Cost Net Stock of Private Equipment by Industry (A)"
            }
            Self::Faat303Esi => {
                "Table 3.3ESI. Historical-Cost Net Stock of Private Fixed Assets by Industry (A)"
            }
            Self::Faat303I => {
                "Table 3.3I. Historical-Cost Net Stock of Private Intellectual Property Products by Industry (A)"
            }
            Self::Faat303S => {
                "Table 3.3S. Historical-Cost net Stock of Private Structures by Industry (A)"
            }
            Self::Faat304E => {
                "Table 3.4E. Current-Cost Depreciation of Private Equipment by Industry (A)"
            }
            Self::Faat304Esi => {
                "Table 3.4ESI. Current-Cost Depreciation of Private Fixed Assets by Industry (A)"
            }
            Self::Faat304I => {
                "Table 3.4I. Current-Cost Depreciation of Private Intellectual Property Products by Industry (A)"
            }
            Self::Faat304S => {
                "Table 3.4S. Current-Cost Depreciation of Private Structures by Industry (A)"
            }
            Self::Faat305E => {
                "Table 3.5E. Chain-Type Quantity Indexes for Depreciation of Private Equipment by Industry (A)"
            }
            Self::Faat305Esi => {
                "Table 3.5ESI. Chain-Type Quantity Indexes for Depreciation of Private Fixed Assets by Industry (A)"
            }
            Self::Faat305I => {
                "Table 3.5I. Chain-Type Quantity Indexes for Depreciation of Private Intellectual Property Products by Industry (A)"
            }
            Self::Faat305S => {
                "Table 3.5S. Chain-Type Quantity Indexes for Depreciation of Private Structures by Industry (A)"
            }
            Self::Faat306E => {
                "Table 3.6E. Historical-Cost Depreciation of Private Equipment by Industry (A)"
            }
            Self::Faat306Esi => {
                "Table 3.6ESI. Historical-Cost Depreciation of Private Fixed Assets by Industry (A)"
            }
            Self::Faat306I => {
                "Table 3.6I. Historical-Cost Depreciation of Private Intellectual Property Products by Industry (A)"
            }
            Self::Faat306S => {
                "Table 3.6S. Historical-Cost Depreciation of Private Structures by Industry (A)"
            }
            Self::Faat307E => "Table 3.7E. Investment in Private Equipment by Industry (A)",
            Self::Faat307Esi => "Table 3.7ESI. Investment in Private Fixed Assets by Industry (A)",
            Self::Faat307I => {
                "Table 3.7I. Investment in Private Intellectual Property Products by Industry (A)"
            }
            Self::Faat307S => "Table 3.7S. Investment in Private Structures by Industry (A)",
            Self::Faat308E => {
                "Table 3.8E. Chain-Type Quantity Indexes for Investment in Private Equipment by Industry (A)"
            }
            Self::Faat308Esi => {
                "Table 3.8ESI. Chain-Type Quantity Indexes for Investment in Private Fixed Assets by Industry (A)"
            }
            Self::Faat308I => {
                "Table 3.8I. Chain-Type Quantity Indexes for Investment in Private Intellectual Property Products by Industry (A)"
            }
            Self::Faat308S => {
                "Table 3.8S. Chain-Type Quantity Indexes for Investment in Private Structures by  Industry (A)"
            }
            Self::Faat309E => {
                "Table 3.9E. Current-Cost Average Age at Yearend of Private Equipment by Industry (A)"
            }
            Self::Faat309Esi => {
                "Table 3.9ESI. Current-Cost Average Age at Yearend of Private Fixed Assets by Industry (A)"
            }
            Self::Faat309I => {
                "Table 3.9I. Current-Cost Average Age at Yearend of Private Intellectual Property Products by Industry (A)"
            }
            Self::Faat309S => {
                "Table 3.9S. Current-Cost Average Age at Yearend of Private Structures by Industry (A)"
            }
            Self::Faat310E => {
                "Table 3.10E. Historical-Cost Average Age at Yearend of Private Equipment by Industry (A)"
            }
            Self::Faat310Esi => {
                "Table 3.10ESI. Historical-Cost Average Age at Yearend of Private Fixed Assets by Industry (A)"
            }
            Self::Faat310I => {
                "Table 3.10I. Historical-Cost Average Age at Yearend of Private Intellectual Property Products by Industry (A)"
            }
            Self::Faat310S => {
                "Table 3.10S. Historical-Cost Average Age at Yearend of Private Structures by Industry (A)"
            }
            Self::Faat401 => {
                "Table 4.1. Current-Cost Net Stock of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat402 => {
                "Table 4.2. Chain-Type Quantity Indexes for Net Stock of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat403 => {
                "Table 4.3. Historical-Cost Net Stock of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat404 => {
                "Table 4.4. Current-Cost Depreciation of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat405 => {
                "Table 4.5. Chain-Type Quantity Indexes for Depreciation of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat406 => {
                "Table 4.6. Historical-Cost Depreciation of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat407 => {
                "Table 4.7. Investment in Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat408 => {
                "Table 4.8. Chain-Type Quantity Indexes for Investment in Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat409 => {
                "Table 4.9. Current-Cost Average Age at Yearend of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat410 => {
                "Table 4.10. Historical-Cost Average Age at Yearend of Private Nonresidential Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat501 => {
                "Table 5.1. Current-Cost Net Stock of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat502 => {
                "Table 5.2. Chain-Type Quantity Indexes for Net Stock of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat503 => {
                "Table 5.3. Historical-Cost Net Stock of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat504 => {
                "Table 5.4. Current-Cost Depreciation of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat505 => {
                "Table 5.5. Chain-Type Quantity Indexes for Depreciation of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat506 => {
                "Table 5.6. Historical-Cost Depreciation of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat507 => {
                "Table 5.7. Investment in Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat508 => {
                "Table 5.8. Chain-Type Quantity Indexes for Investment in Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat509 => {
                "Table 5.9. Current-Cost Average Age at Yearend of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat510 => {
                "Table 5.10. Historical-Cost Average Age at Yearend of Residential Fixed Assets by Type of Owner, Legal Form of Organization, and Tenure Group (A)"
            }
            Self::Faat601 => {
                "Table 6.1. Current-Cost Net Stock of Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat602 => {
                "Table 6.2. Chain-type Quantity Indexes for Net Stock of Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat603 => {
                "Table 6.3. Historical-Cost Net Stock of Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat604 => {
                "Table 6.4. Current-Cost Depreciation of Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat605 => {
                "Table 6.5. Chain-Type Quantity Indexes for Depreciation of Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat606 => {
                "Table 6.6. Historical-Cost Depreciation of Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat607 => {
                "Table 6.7. Investment in Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat608 => {
                "Table 6.8. Chain-Type Quantity Indexes for Investment in Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat609 => {
                "Table 6.9. Current-Cost Average Age at Yearend of Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat610 => {
                "Table 6.10. Historical-Cost Average Age at Yearend of Private Fixed Assets by Industry Group and Legal Form of Organization (A)"
            }
            Self::Faat701 => "Table 7.1. Current-Cost Net Stock of Government Fixed Assets (A)",
            Self::Faat702 => {
                "Table 7.2. Chain-Type Quantity Indexes for Net Stock of Government Fixed Assets (A)"
            }
            Self::Faat703 => "Table 7.3. Current-Cost Depreciation of Government Fixed Assets (A)",
            Self::Faat704 => {
                "Table 7.4. Chain-Type Quantity Indexes for Depreciation of Government Fixed Assets (A)"
            }
            Self::Faat705 => "Table 7.5. Investment in Government Fixed Assets (A)",
            Self::Faat706 => {
                "Table 7.6. Chain-Type Quantity Indexes for Investment in Government Fixed Assets (A)"
            }
            Self::Faat707 => {
                "Table 7.7. Current-Cost Average Age at Yearend of Government Fixed Assets (A)"
            }
            Self::Faat801 => "Table 8.1. Current-Cost Net Stock of Consumer Durable Goods (A)",
            Self::Faat802 => {
                "Table 8.2. Chain-Type Quantity Indexes for Net Stock of Consumer Durable Goods (A)"
            }
            Self::Faat803 => "Table 8.3. Historical-Cost Net Stock of Consumer Durable Goods (A)",
            Self::Faat804 => "Table 8.4. Current-Cost Depreciation of Consumer Durable Goods (A)",
            Self::Faat805 => {
                "Table 8.5. Chain-Type Quantity Indexes for Depreciation of Consumer Durable Goods (A)"
            }
            Self::Faat806 => {
                "Table 8.6. Historical-Cost Depreciation of Consumer Durable Goods (A)"
            }
            Self::Faat807 => "Table 8.7. Investment in Consumer Durable Goods (A)",
            Self::Faat808 => {
                "Table 8.8. Chain-Type Quantity Indexes for Investment in Consumer Durable Goods (A)"
            }
            Self::Faat809 => {
                "Table 8.9. Current-Cost Average Age at Yearend of Consumer Durable Goods (A)"
            }
            Self::Faat810 => {
                "Table 8.10. Historical-Cost Average Age at Yearend of Consumer Durable Goods (A)"
            }
            Self::Faat901 => {
                "Table 9.1. Real Net Stock of Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat902 => {
                "Table 9.2. Real Depreciation of Fixed Assets and Consumer Durable Goods (A)"
            }
            Self::Faat903 => {
                "Table 9.3. Real Investment in Fixed Assets and Consumer Durable Goods (A)"
            }
        }
    }

    /// The String representation of a variant serves as the value for the corresponding parameter
    /// name in BEA API calls. The `params` method formats the parameter name as a key, as the
    /// variant name as the value for use in building complex API queries.
    pub fn params(&self) -> (String, String) {
        let key = ParameterName::TableName.to_string();
        let value = self.to_string();
        (key, value)
    }
}

use std::str::FromStr;
impl TryFrom<&NipaTable> for FixedAssetTable {
    type Error = BeaErr;
    fn try_from(value: &NipaTable) -> Result<Self, Self::Error> {
        Self::from_str(value.table_name()).map_err(|e| {
            DeriveFromStr::new(
                value.table_name().to_owned(),
                e,
                line!(),
                file!().to_owned(),
            )
            .into()
        })
    }
}

impl TryFrom<&ParameterFields> for FixedAssetTable {
    type Error = BeaErr;

    fn try_from(value: &ParameterFields) -> Result<Self, Self::Error> {
        Self::from_str(value.key()).map_err(|e| {
            DeriveFromStr::new(value.key().to_owned(), e, line!(), file!().to_owned()).into()
        })
    }
}

impl TryFrom<&ParameterValueTable> for FixedAssetTable {
    type Error = BeaErr;
    fn try_from(value: &ParameterValueTable) -> Result<Self, Self::Error> {
        match value {
            ParameterValueTable::NipaTable(tab) => Self::try_from(tab),
            ParameterValueTable::ParameterFields(tab) => Self::try_from(tab),
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
