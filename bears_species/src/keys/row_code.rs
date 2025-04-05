use crate::{map_to_int, Naics};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    strum::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum RowCode {
    Naics(i64),
    Parent(i64),
    Region(String),
    Addendum(String),
}

impl Default for RowCode {
    fn default() -> Self {
        Self::Addendum("Empty".to_string())
    }
}

impl RowCode {
    pub fn from_value(
        value: &serde_json::Map<String, serde_json::Value>,
        title: &str,
        naics: &Naics,
    ) -> Result<Self, RowCodeMissing> {
        match map_to_int("RowCode", value) {
            // Code is present.
            Ok(code) => Ok(Self::Naics(code)),
            // Code is missing, try to determine the corrent code then error.
            Err(_) => {
                let mut naics = naics.clone();
                // Does the row name match a naics title?
                naics.retain(|v| v.title().trim().to_lowercase() == title.trim().to_lowercase());
                if naics.is_empty() {
                    // No matching title.
                    match title {
                        // title has a clear and unambiguous match
                        // 3311,Iron and Steel Mills and Ferroalloy Manufacturing ,,
                        "Iron and steel mills" => {
                            tracing::trace!("Categoring iron and steel mills.");
                            Ok(Self::Naics(3311))
                        }
                        // 3327,"Machine Shops; Turned Product; and Screw, Nut, and Bolt Manufacturing",,
                        "Machine shop products, turned products, and screws, nuts, and bolts"
                        | "Machine shops; turned products; and screws, nuts, and bolts" => {
                            tracing::trace!(
                                "Categorizing machine shop products, turned products, screws nuts and bolts."
                            );
                            Ok(Self::Naics(3327))
                        }
                        // 5171,Wired and Wireless Telecommunications (except Satellite),,
                        "Wired and wireless telecommunications carriers" => {
                            tracing::trace!(
                                "Categorizing wired and wireless telecommunications carriers."
                            );
                            Ok(Self::Naics(5171))
                        }
                        // 5222,Nondepository Credit Intermediation ,,
                        "Nondepository credit intermediation, except branches and agencies" => {
                            tracing::trace!("Categorizing nondepository credit intermediation.");
                            Ok(Self::Naics(5222))
                        }
                        // 5419,"Other Professional, Scientific, and Technical Services",,
                        "Other-Professional, scientific, and technical services" => {
                            tracing::trace!(
                                "Categorizing other professional, scientific, and technical services."
                            );
                            Ok(Self::Naics(5419))
                        }
                        // 5222,Nondepository Credit Intermediation ,,
                        "Non-depository credit intermediation, except branches and agencies" => {
                            tracing::trace!(
                                "Categorizing Non-depository credit intermediation, except branches and agencies."
                            );
                            Ok(Self::Naics(5222))
                        }
                        // 42471,Petroleum Bulk Stations and Terminals ,,
                        "Petroleum storage for hire" => {
                            tracing::trace!("Categorizing Petroleum storage for hire.");
                            Ok(Self::Naics(42471))
                        }
                        // 4599,Other Miscellaneous Retailers ,,
                        "Other-Retail trade" => {
                            tracing::trace!("Categorizing other miscellaneous retailers.");
                            Ok(Self::Naics(4599))
                        }
                        // We default to the parent code when the title does not match a specific
                        // industry.
                        // 325 is the parent code for Chemical Manufacturing
                        "Other-Chemicals" => {
                            tracing::trace!(
                                "Categorizing Other-Chemicals as Chemical Manufacturing."
                            );
                            Ok(Self::Parent(325))
                        }
                        // 333,Machinery Manufacturing,,
                        "Other-Machinery" => {
                            tracing::trace!(
                                "Categorizing Other-Machinery as Machinery Manufacturing."
                            );
                            Ok(Self::Parent(333))
                        }
                        // 334,Computer and Electronic Product Manufacturing,,
                        "Other-Computers and electronic products" => {
                            tracing::trace!(
                                "Categorizing computer and electronic product manufacturing."
                            );
                            Ok(Self::Parent(334))
                        }
                        // 336,Transportation Equipment Manufacturing,,
                        "Other-Transportation equipment" => {
                            tracing::trace!("Categorizing Transportation equipment manufacturing.");
                            Ok(Self::Parent(336))
                        }
                        // 339,Miscellaneous Manufacturing,,
                        "Other-Manufacturing" => {
                            tracing::trace!("Categorizing miscellaneous manufacturing.");
                            Ok(Self::Parent(339))
                        }
                        // 42,Wholesale Trade,,
                        "Other-Wholesale trade" => {
                            tracing::trace!("Categorizing other wholesale trade.");
                            Ok(Self::Parent(42))
                        }
                        // 51,Information,,
                        "Other-Information" => {
                            tracing::trace!("Categorizing other information.");
                            Ok(Self::Parent(51))
                        }
                        // 339,Miscellaneous Manufacturing,,
                        "Other-Other industries" => {
                            tracing::trace!(
                                "Categorizing other industries as miscellaneous manufacturing."
                            );
                            Ok(Self::Parent(339))
                        }
                        // 21,"Mining, Quarrying, and Oil and Gas Extraction",,
                        "Other-Mining" => {
                            tracing::trace!("Categorizing other mining.");
                            Ok(Self::Parent(21))
                        }
                        // 92615,"Regulation, Licensing, and Inspection of Miscellaneous Commercial Sectors ",,
                        "Fees, taxes, permits, licenses" => {
                            tracing::trace!("Categorizing addendum.");
                            Ok(Self::Parent(92615))
                        }
                        // 5132,Software Publishers,,
                        "Intellectual property rights" => {
                            tracing::trace!("Categorizing Intellectual property rights.");
                            Ok(Self::Parent(5132))
                        }
                        // 531,Real Estate,,
                        "Land" => {
                            tracing::trace!("Categorizing Land.");
                            Ok(Self::Parent(531))
                        }
                        // 23621,Industrial Building Construction,,
                        // could also be sewage treatment
                        "Plant and equipment" => {
                            tracing::trace!("Categorizing Plant and equipment.");
                            Ok(Self::Parent(23621))
                        }
                        // 3399,Other Miscellaneous Manufacturing,,
                        "Other---  All  --" => {
                            tracing::trace!("Categorizing Other---  All  --.");
                            Ok(Self::Parent(3399))
                        }
                        // 45999,All Other Miscellaneous Retailers ,,
                        "Miscellaneous retailers" => {
                            tracing::trace!("Categorizing Miscellaneous retailers.");
                            Ok(Self::Parent(45999))
                        }
                        // 3322,Cutlery and Handtool Manufacturing,,
                        "Cutlery and handtools" => {
                            tracing::trace!("Categorizing Cutlery and handtools.");
                            Ok(Self::Parent(3322))
                        }
                        // Not a valid NAICS category.
                        // Regional designations
                        "Far East:" => {
                            tracing::trace!("Categorizing Far East.");
                            Ok(Self::Region("Far East".to_string()))
                        }
                        "Far West:" => {
                            tracing::trace!("Categorizing Far West.");
                            Ok(Self::Region("Far West".to_string()))
                        }
                        "Rocky Mountains:" => {
                            tracing::trace!("Categorizing Rocky Mountains.");
                            Ok(Self::Region("Rocky Mountains".to_string()))
                        }
                        "Southwest:" => {
                            tracing::trace!("Categorizing Southwest.");
                            Ok(Self::Region("Southwest".to_string()))
                        }
                        "Southeast:" => {
                            tracing::trace!("Categorizing Southeast.");
                            Ok(Self::Region("Southeast".to_string()))
                        }
                        "Plains:" => {
                            tracing::trace!("Categorizing Plains.");
                            Ok(Self::Region("Plains".to_string()))
                        }
                        "Great Lakes:" => {
                            tracing::trace!("Categorizing Great Lakes.");
                            Ok(Self::Region("Great Lakes".to_string()))
                        }
                        "Mideast:" => {
                            tracing::trace!("Categorizing Mideast.");
                            Ok(Self::Region("Mideast".to_string()))
                        }
                        "New England:" => {
                            tracing::trace!("Categorizing New England.");
                            Ok(Self::Region("New England".to_string()))
                        }
                        "Addendum:" => {
                            tracing::trace!("Categorizing addendum.");
                            Ok(Self::Addendum("Addendum".to_string()))
                        }
                        _ => {
                            // Unknown row name variant, code indeterminate.
                            let error = RowCodeMissing::new(
                                title.to_string(),
                                line!(),
                                file!().to_string(),
                            );
                            Err(error)
                        }
                    }
                } else {
                    // Title matches.  Pull code from title.
                    let code = *naics[0].code();
                    tracing::trace!("Setting NAICS code {code} for {title}.");
                    Ok(Self::Naics(code))
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_new::new)]
#[display("MNE RowCode missing for {row} at line {line} in {file}")]
pub struct RowCodeMissing {
    row: String,
    line: u32,
    file: String,
}

impl std::error::Error for RowCodeMissing {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
