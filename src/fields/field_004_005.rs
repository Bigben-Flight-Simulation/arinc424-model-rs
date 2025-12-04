use crate::error::ReaderError;
use crate::error::ReaderError::Unsupported;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum::{Display, EnumString};

/// # 5.4 Section Code (SEC CODE)
/// Definition/Description: The “Section Code” field defines
/// the major section of the navigation system database in
/// which the record resides.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Section {
    MORA,
    Navaid(NavaidSubsection),
    Enroute(EnrouteSubsection),
    Heliport(HeliportSubsection),
    Airport(AirportSubsection),
    CompanyRoutes(CompanyRoutesSubsection),
    Tables(TablesSubsection),
    Airspace(AirspaceSubsection),
}

impl Display for Section {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Section::MORA => write!(f, "AS"),
            Section::Navaid(s) => write!(f, "D{}", s),
            Section::Enroute(s) => write!(f, "E{}", s),
            Section::Heliport(s) => write!(f, "H{}", s),
            Section::Airport(s) => write!(f, "P{}", s),
            Section::CompanyRoutes(s) => write!(f, "R{}", s),
            Section::Tables(s) => write!(f, "T{}", s),
            Section::Airspace(s) => write!(f, "U{}", s),
        }
    }
}

impl FromStr for Section {
    type Err = ReaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ReaderError::InvalidLength(2, s.len()));
        }

        let (first, sub_char) = s.split_at(1);

        match first {
            "A" => {
                if s == "AS" {
                    Ok(Section::MORA)
                } else {
                    Err(Unsupported(format!("unsupported MORA code: {}", s)))
                }
            }
            "D" => Ok(Section::Navaid(sub_char.parse()?)),
            "E" => Ok(Section::Enroute(sub_char.parse()?)),
            "H" => Ok(Section::Heliport(sub_char.parse()?)),
            "P" => Ok(Section::Airport(sub_char.parse()?)),
            "R" => Ok(Section::CompanyRoutes(sub_char.parse()?)),
            "T" => Ok(Section::Tables(sub_char.parse()?)),
            "U" => Ok(Section::Airspace(sub_char.parse()?)),
            _ => Err(Unsupported(format!(
                "unknown section code prefix: '{}'",
                first
            ))),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, EnumString, Display)]
pub enum NavaidSubsection {
    #[strum(serialize = " ")]
    VHF,
    #[strum(serialize = "B")]
    NDB,
}

#[derive(Debug, Eq, PartialEq, Clone, EnumString, Display)]
pub enum EnrouteSubsection {
    #[strum(serialize = "A")]
    Waypoints,
    #[strum(serialize = "M")]
    AirwayMarkers,
    #[strum(serialize = "P")]
    HoldingPatterns,
    #[strum(serialize = "R")]
    AirwaysAndRoutes,
    #[strum(serialize = "T")]
    PreferredRoutes,
    #[strum(serialize = "U")]
    AirwayRestrictions,
    #[strum(serialize = "V")]
    Communications,
}

#[derive(Debug, Eq, PartialEq, Clone, EnumString, Display)]
pub enum HeliportSubsection {
    #[strum(serialize = "A")]
    Pads,
    #[strum(serialize = "C")]
    TerminalWaypoints,
    #[strum(serialize = "D")]
    SIDs,
    #[strum(serialize = "E")]
    STARs,
    #[strum(serialize = "F")]
    ApproachProcedures,
    #[strum(serialize = "K")]
    TAA,
    #[strum(serialize = "S")]
    MSA,
    #[strum(serialize = "V")]
    Communications,
}

#[derive(Debug, Eq, PartialEq, Clone, EnumString, Display)]
pub enum AirportSubsection {
    #[strum(serialize = "A")]
    ReferencePoints,
    #[strum(serialize = "B")]
    Gates,
    #[strum(serialize = "C")]
    TerminalWaypoints,
    #[strum(serialize = "D")]
    SIDs,
    #[strum(serialize = "E")]
    STARs,
    #[strum(serialize = "F")]
    ApproachProcedures,
    #[strum(serialize = "G")]
    Runways,
    #[strum(serialize = "I")]
    LocalizerGlideSlope,
    #[strum(serialize = "K")]
    TAA,
    #[strum(serialize = "L")]
    MLS,
    #[strum(serialize = "M")]
    LocalizerMarker,
    #[strum(serialize = "N")]
    TerminalNDB,
    #[strum(serialize = "P")]
    PathPoint,
    #[strum(serialize = "R")]
    FltPlanningARRorDEP,
    #[strum(serialize = "S")]
    MSA,
    #[strum(serialize = "T")]
    GLSStation,
    #[strum(serialize = "V")]
    Communications,
}

#[derive(Debug, Eq, PartialEq, Clone, EnumString, Display)]
pub enum CompanyRoutesSubsection {
    #[strum(serialize = " ")]
    CompanyRoutes,
    #[strum(serialize = "A")]
    AlternateRecords,
}

#[derive(Debug, Eq, PartialEq, Clone, EnumString, Display)]
pub enum TablesSubsection {
    #[strum(serialize = "C")]
    CruisingTables,
    #[strum(serialize = "G")]
    GeographicalReference,
    #[strum(serialize = "N")]
    RNAVNameTable,
}

#[derive(Debug, Eq, PartialEq, Clone, EnumString, Display)]
pub enum AirspaceSubsection {
    #[strum(serialize = "C")]
    ControlledAirspace,
    #[strum(serialize = "F")]
    FIRUIR,
    #[strum(serialize = "R")]
    RestrictiveAirspace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            Section::Navaid(NavaidSubsection::VHF),
            "D ".parse().unwrap()
        );
        assert_eq!(
            Section::Airport(AirportSubsection::ApproachProcedures),
            "PF".parse().unwrap()
        );

        assert_eq!(
            "S".parse::<Section>(),
            Err(ReaderError::InvalidLength(2, 1))
        )
    }

    #[test]
    fn test_display() {
        assert_eq!(
            Section::Enroute(EnrouteSubsection::AirwaysAndRoutes).to_string(),
            "ER"
        );
    }
}
