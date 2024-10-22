//! Module for the main instrument definition struct.

use std::{collections::BTreeMap, sync::Arc};

use anyhow::Context;

use crate::{
    component_tree::ComponentTree, idlists::IDList, types::Types,
    xml_parser::detector_definition_from_str,
};

/// Main instrument definition struct. \
/// Contains the types and the component tree. \
///
/// https://docs.mantidproject.org/nightly/concepts/InstrumentDefinitionFile.html \
/// The instrument definition file is structured as a combination of components and types. \
/// A component has to have a type, and a type can have multiple components. \
/// TODO: Having the components both listed in the `Types` and in the `ComponentTree` seems redundant. \
///
/// # Example
/// ```
/// use mantid_idf::detector_definition::DetectorDefinition;
///
/// let path = "assets/test_detector_definition.xml";
/// let content = std::fs::read_to_string(path).expect("could not read file");
///
/// let detector_definition = DetectorDefinition::from_str(&content).expect("could not parse detector definition");
///
/// assert!(detector_definition.component_tree.children.len() > 0);
/// assert!(detector_definition.component_tree.component.is_root());
/// ```
#[derive(Debug, Default)]
pub struct DetectorDefinition {
    /// Pointer to the types for lookup.
    pub types: Arc<Types>,
    /// The component tree.
    pub component_tree: ComponentTree,
    /// Potential ID lists.
    pub id_lists: BTreeMap<String, IDList>,
}

impl DetectorDefinition {
    /// Parse the detector definition from a string.
    pub fn from_str(str: &str) -> anyhow::Result<Self> {
        detector_definition_from_str(str).context("could not parse detector definition")
    }
}
