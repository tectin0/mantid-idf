#![deny(missing_docs)]

//! Mantid Instrument Definition File (IDF) parser. \
//! Library to parse Mantid IDF files and extract information from them. \
//! https://docs.mantidproject.org/nightly/concepts/InstrumentDefinitionFile.html

pub mod component_tree;
pub mod detector_definition;
pub mod structs;
pub mod types;
pub mod utils;
pub mod xml_parser;

/// Type alias for a point in 3D space.
pub type Point = nalgebra::Point3<f32>;

pub use detector_definition::DetectorDefinition;
