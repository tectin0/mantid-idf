//! Types module defining the `Types` struct and the `SpecialTypes` enum.

use std::collections::BTreeMap;

use crate::structs::Type;

/// Struct to hold the types defined in the IDF file.
#[derive(Debug, Clone)]
pub struct Types(pub BTreeMap<String, Type>);

impl Default for Types {
    fn default() -> Self {
        let mut types = BTreeMap::new();

        // TODO: default type for `root` node - could maybe lead to issues?
        types.insert("".to_string(), Type::default());

        Self(types)
    }
}

impl std::ops::Deref for Types {
    type Target = BTreeMap<String, Type>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Types {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Enum to represent special types. \
/// TODO: Research more about the special types and their usage.
#[derive(Debug, Default, Clone, PartialEq)]
pub enum SpecialTypes {
    /// The default value. The `None` special type does not actually exist in the IDF file. \
    /// It is used to represent the absence of a special type.
    #[default]
    None,
    /// Represents a generic detector part (voxel/pixel) ?
    Detector,
    /// Represents the monitor.
    Monitor,
    /// Represents a rectangular detector. Used to quickly generate a detector with a rectangular shape. \
    /// https://docs.mantidproject.org/nightly/concepts/InstrumentDefinitionFile.html#creating-rectangular-area-detectors
    RectangularDetector,
    /// Represents a structured detector. Used to quickly generated a detector with a structured (irregular geometry) shape. \
    /// https://docs.mantidproject.org/nightly/concepts/InstrumentDefinitionFile.html#creating-structured-detectors
    StructuredDetector,
    /// Represents the source.
    Source,
    /// Represents the sample position.
    SamplePos,
    /// Represents the chopper position.
    ChopperPos,
}

impl std::str::FromStr for SpecialTypes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "Detector" => Ok(Self::Detector),
            "detector" => Ok(Self::Detector),
            "Monitor" => Ok(Self::Monitor),
            "monitor" => Ok(Self::Monitor),
            "RectangularDetector" => Ok(Self::RectangularDetector),
            "rectangularDetector" => Ok(Self::RectangularDetector),
            "rectangulardetector" => Ok(Self::RectangularDetector),
            "rectangular_detector" => Ok(Self::RectangularDetector),
            "StructuredDetector" => Ok(Self::StructuredDetector),
            "structuredDetector" => Ok(Self::StructuredDetector),
            "structureddetector" => Ok(Self::StructuredDetector),
            "structured_detector" => Ok(Self::StructuredDetector),
            "Source" => Ok(Self::Source),
            "source" => Ok(Self::Source),
            "SamplePos" => Ok(Self::SamplePos),
            "samplePos" => Ok(Self::SamplePos),
            "ChopperPos" => Ok(Self::ChopperPos),
            "chopperPos" => Ok(Self::ChopperPos),
            _ => Ok(Self::None),
        }
    }
}
