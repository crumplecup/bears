use crate::{AreaOrCountry, Naics, NaicsItems, StateKind, map_to_int};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum RowCode {
    Naics(Naics),
    Parent(Naics),
    Region(AreaOrCountry),
    State(StateKind),
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
        naics: &NaicsItems,
    ) -> Result<Self, RowCodeMissing> {
        match map_to_int("RowCode", value) {
            // Code is present.
            Ok(code) => {
                // First try to parse the row code as an area or country designation
                // 3-digit naics codes that match AOC codes will be parsed incorrectly
                if let Some(region) = AreaOrCountry::from_code(code) {
                    Ok(Self::Region(region))
                // States also have row codes, try those next
                } else if let Some(state) = StateKind::from_code(code) {
                    Ok(Self::State(state))
                } else {
                    // Not an AOC or state code, try as a NAICS code.
                    let code_str = code.to_string();
                    if let Some(naics) = Naics::from_code(&code_str) {
                        Ok(Self::Naics(naics))
                    } else {
                        // Row code is a number, but the number is not recognized, throw an error.
                        let error =
                            RowCodeMissing::new(code.to_string(), line!(), file!().to_owned());
                        Err(error)
                    }
                }
            }
            // Code is missing, try to determine the corrent code then error.
            Err(_) => {
                let mut naics = naics.clone();
                // Does the row name match a naics title?
                naics.retain(|v| v.title().trim().to_lowercase() == title.trim().to_lowercase());
                if naics.is_empty() {
                    // No matching title.
                    match title {
                        // title has a clear and unambiguous match
                        // 3118,Bakeries and Tortilla Manufacturing,BakeriesAndTortillaManufacturing
                        "Bakeries and tortilla  manufacturing" => {
                            tracing::trace!("Categoring Bakeries and tortilla  manufactoring.");
                            if let Some(naics) = Naics::from_code("3118") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 3311,Iron and Steel Mills and Ferroalloy Manufacturing ,,
                        "Iron and steel mills" => {
                            tracing::trace!("Categoring iron and steel mills.");
                            if let Some(naics) = Naics::from_code("3311") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 3327,"Machine Shops; Turned Product; and Screw, Nut, and Bolt Manufacturing",,
                        "Machine shop products, turned products, and screws, nuts, and bolts"
                        | "Machine shops; turned products; and screws, nuts, and bolts" => {
                            tracing::trace!(
                                "Categorizing machine shop products, turned products, screws nuts and bolts."
                            );
                            if let Some(naics) = Naics::from_code("3327") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 5171,Wired and Wireless Telecommunications (except Satellite),,
                        "Wired and wireless telecommunications carriers" => {
                            tracing::trace!(
                                "Categorizing wired and wireless telecommunications carriers."
                            );
                            if let Some(naics) = Naics::from_code("5171") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 517112,Wireless Telecommunications Carriers (except Satellite),WirelessTelecommunicationsCarriersExceptSatellite
                        "Wired and wireless telecommunications (except satellite)" => {
                            tracing::trace!(
                                "Categorizing Wired and wireless telecommunications (except satellite)"
                            );
                            if let Some(naics) = Naics::from_code("517112") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 516210,"Media Streaming Distribution Services, Social Networks, and Other Media Networks and Content Providers",MediaStreamingDistributionServicesSocialNetworksAndOtherMediaNetworksAndContentProviders
                        "Media streaming distribution services, social networks, and other media networks and content providers" =>
                        {
                            tracing::trace!(
                                "Categorizing Media streaming distribution services, social networks, and other media networks and content providers"
                            );
                            if let Some(naics) = Naics::from_code("516210") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 519290,Web Search Portals and All Other Information Services,WebSearchPortalsAndAllOtherInformationServices
                        "Web search portals, libraries, archives, and other information services" =>
                        {
                            tracing::trace!(
                                "Categorizing Web search portals, libraries, archives, and other information services"
                            );
                            if let Some(naics) = Naics::from_code("519290") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 5151,Radio and Television Broadcasting,RadioAndTelevisionBroadcasting
                        "Radio and television broadcasting stations" => {
                            tracing::trace!(
                                "Categorizing Radio and television broadcasting stations"
                            );
                            if let Some(naics) = Naics::from_code("5151") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }

                        // 5222,Nondepository Credit Intermediation ,,
                        "Nondepository credit intermediation, except branches and agencies" => {
                            tracing::trace!("Categorizing nondepository credit intermediation.");
                            if let Some(naics) = Naics::from_code("5222") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 5419,"Other Professional, Scientific, and Technical Services",,
                        "Other-Professional, scientific, and technical services" => {
                            tracing::trace!(
                                "Categorizing other professional, scientific, and technical services."
                            );
                            if let Some(naics) = Naics::from_code("5419") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 5222,Nondepository Credit Intermediation ,,
                        "Non-depository credit intermediation, except branches and agencies" => {
                            tracing::trace!(
                                "Categorizing Non-depository credit intermediation, except branches and agencies."
                            );
                            if let Some(naics) = Naics::from_code("5222") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 42471,Petroleum Bulk Stations and Terminals ,,
                        "Petroleum storage for hire" => {
                            tracing::trace!("Categorizing Petroleum storage for hire.");
                            if let Some(naics) = Naics::from_code("42471") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 4599,Other Miscellaneous Retailers ,,
                        "Other-Retail trade" => {
                            tracing::trace!("Categorizing other miscellaneous retailers.");
                            if let Some(naics) = Naics::from_code("4599") {
                                Ok(Self::Naics(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // We default to the parent code when the title does not match a specific
                        // industry.
                        // 325 is the parent code for Chemical Manufacturing
                        "Other-Chemicals" => {
                            tracing::trace!(
                                "Categorizing Other-Chemicals as Chemical Manufacturing."
                            );
                            if let Some(naics) = Naics::from_code("325") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 333,Machinery Manufacturing,,
                        "Other-Machinery" => {
                            tracing::trace!(
                                "Categorizing Other-Machinery as Machinery Manufacturing."
                            );
                            if let Some(naics) = Naics::from_code("333") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 334,Computer and Electronic Product Manufacturing,,
                        "Other-Computers and electronic products" => {
                            tracing::trace!(
                                "Categorizing computer and electronic product manufacturing."
                            );
                            if let Some(naics) = Naics::from_code("334") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 336,Transportation Equipment Manufacturing,,
                        "Other-Transportation equipment" => {
                            tracing::trace!("Categorizing Transportation equipment manufacturing.");
                            if let Some(naics) = Naics::from_code("336") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 339,Miscellaneous Manufacturing,,
                        "Other-Manufacturing" => {
                            tracing::trace!("Categorizing miscellaneous manufacturing.");
                            if let Some(naics) = Naics::from_code("339") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 42,Wholesale Trade,,
                        "Other-Wholesale trade" => {
                            tracing::trace!("Categorizing other wholesale trade.");
                            if let Some(naics) = Naics::from_code("42") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 51,Information,,
                        "Other-Information" => {
                            tracing::trace!("Categorizing other information.");
                            if let Some(naics) = Naics::from_code("51") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 339,Miscellaneous Manufacturing,,
                        "Other-Other industries" => {
                            tracing::trace!(
                                "Categorizing other industries as miscellaneous manufacturing."
                            );
                            if let Some(naics) = Naics::from_code("339") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 21,"Mining, Quarrying, and Oil and Gas Extraction",,
                        "Other-Mining" => {
                            tracing::trace!("Categorizing other mining.");
                            if let Some(naics) = Naics::from_code("21") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 92615,"Regulation, Licensing, and Inspection of Miscellaneous Commercial Sectors ",,
                        "Fees, taxes, permits, licenses" => {
                            tracing::trace!("Categorizing addendum.");
                            if let Some(naics) = Naics::from_code("92615") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // NAICS Code 533110 - Lessors of Nonfinancial Intangible Assets (except Copyrighted Works)
                        "Intellectual property rights" => {
                            tracing::trace!("Categorizing Intellectual property rights.");
                            if let Some(naics) = Naics::from_code("533110") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 531,Real Estate,,
                        "Land" => {
                            tracing::trace!("Categorizing Land.");
                            if let Some(naics) = Naics::from_code("531") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 23621,Industrial Building Construction,,
                        // could also be sewage treatment
                        "Plant and equipment" => {
                            tracing::trace!("Categorizing Plant and equipment.");
                            if let Some(naics) = Naics::from_code("23621") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 3399,Other Miscellaneous Manufacturing,,
                        "Other---  All  --" => {
                            tracing::trace!("Categorizing Other---  All  --.");
                            if let Some(naics) = Naics::from_code("3399") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 45999,All Other Miscellaneous Retailers ,,
                        "Miscellaneous retailers" => {
                            tracing::trace!("Categorizing Miscellaneous retailers.");
                            if let Some(naics) = Naics::from_code("45999") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // 3322,Cutlery and Handtool Manufacturing,,
                        "Cutlery and handtools" => {
                            tracing::trace!("Categorizing Cutlery and handtools.");
                            if let Some(naics) = Naics::from_code("3322") {
                                Ok(Self::Parent(naics))
                            } else {
                                let error = RowCodeMissing::new(
                                    title.to_owned(),
                                    line!(),
                                    file!().to_string(),
                                );
                                Err(error)
                            }
                        }
                        // Not a valid NAICS category.
                        // Regional designations
                        "All Countries Total" => {
                            tracing::trace!("Categoring All Countries Total.");
                            Ok(Self::Region(AreaOrCountry::AllCountries))
                        }
                        "Far East:" => {
                            tracing::trace!("Categorizing Far East.");
                            Ok(Self::Region(AreaOrCountry::FarEast))
                        }
                        "Far West:" => {
                            tracing::trace!("Categorizing Far West.");
                            Ok(Self::Region(AreaOrCountry::FarWest))
                        }
                        "Rocky Mountains:" => {
                            tracing::trace!("Categorizing Rocky Mountains.");
                            Ok(Self::Region(AreaOrCountry::RockyMountains))
                        }
                        "Southwest:" => {
                            tracing::trace!("Categorizing Southwest.");
                            Ok(Self::Region(AreaOrCountry::Southwest))
                        }
                        "Southeast:" => {
                            tracing::trace!("Categorizing Southeast.");
                            Ok(Self::Region(AreaOrCountry::Southeast))
                        }
                        "Plains:" => {
                            tracing::trace!("Categorizing Plains.");
                            Ok(Self::Region(AreaOrCountry::Plains))
                        }
                        "Great Lakes:" => {
                            tracing::trace!("Categorizing Great Lakes.");
                            Ok(Self::Region(AreaOrCountry::GreatLakes))
                        }
                        "Mideast:" => {
                            tracing::trace!("Categorizing Mideast.");
                            Ok(Self::Region(AreaOrCountry::MiddleEast))
                        }
                        "New England:" => {
                            tracing::trace!("Categorizing New England.");
                            Ok(Self::Region(AreaOrCountry::NewEngland))
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
                    let code_str = code.to_string();
                    tracing::trace!("Setting NAICS code {code} for {title}.");
                    if let Some(naics) = Naics::from_code(&code_str) {
                        Ok(Self::Naics(naics))
                    } else {
                        let error = RowCodeMissing::new(code_str, line!(), file!().to_string());
                        Err(error)
                    }
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
