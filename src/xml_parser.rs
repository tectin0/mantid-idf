//! This module contains the xml_parser function which is used to parse the detector definition from a string.
//! This crate uses the `quick-xml` crate to parse the XML file.

mod try_match_attribute;
mod try_match_bytes_start;

use std::collections::BTreeMap;
use std::sync::Arc;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

use anyhow::Context;
use try_match_bytes_start::TryMatchBytesStart;

use crate::component_tree::ComponentTree;
use crate::detector_definition::DetectorDefinition;

use crate::idlists::IDList;
use crate::structs::*;
use crate::types::Types;

pub(crate) fn detector_definition_from_str(str: &str) -> anyhow::Result<DetectorDefinition> {
    let mut reader = Reader::from_str(str);

    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    let mut components = BTreeMap::<String, Component>::new();

    let mut types = Types::default();

    let mut id_lists = BTreeMap::<String, IDList>::new();

    let mut current_component = None;

    let mut current_type: Option<Type> = None;

    let mut current_hexahedron: Option<Hexahedron> = None;

    let mut current_id_list = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(error) => {
                return Err(anyhow::anyhow!(
                    "Error at position{}: {:?}",
                    reader.buffer_position(),
                    error
                ))
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(bytes_start)) => {
                if Component::try_match_bytes_start(&mut current_component, &bytes_start, None)
                    .context("could not parse component from start event")?
                    .match_found()
                {
                    continue;
                }

                if Type::try_match_bytes_start(&mut current_type, &bytes_start, None)
                    .context("could not parse type from start event")?
                    .match_found()
                {
                    continue;
                }

                if Hexahedron::try_match_bytes_start(&mut current_hexahedron, &bytes_start, None)
                    .context("could not parse hexahedron from start event")?
                    .match_found()
                {
                    continue;
                }

                if IDList::try_match_bytes_start(&mut current_id_list, &bytes_start, None)
                    .context("could not parse id list from start event")?
                    .match_found()
                {
                    continue;
                }

                let mut location = None;

                Location::try_match_bytes_start(&mut location, &bytes_start, None)
                    .context("could not parse location from start event")?;

                if let Some(mut location) = location {
                    let component = current_component.get_or_insert_default();

                    if location.name.is_empty() {
                        location.name = component.type_name.clone();
                    }

                    component.location.push(location);

                    continue;
                }

                let mut rotation = None;

                Rotation::try_match_bytes_start(&mut rotation, &bytes_start, None)
                    .context("could not parse rotation from start event")?;

                if let Some(rotation) = rotation {
                    current_component
                        .get_or_insert_default()
                        .push_rotation(rotation);
                }
            }
            Ok(Event::End(bytes_end)) => match bytes_end.name().as_ref() {
                // TODO: error if end was reached without a component/type/hexahedron?
                b"component" => {
                    if let Some(component) = current_component.take() {
                        match current_type.as_mut() {
                            Some(type_) => {
                                type_.components.push(component);
                            }
                            None => {
                                components.insert(component.type_name.clone(), component);
                            }
                        }
                    }
                }
                b"type" => {
                    if let Some(type_) = current_type.take() {
                        types.insert(type_.name.clone(), type_);
                    }
                }
                b"hexahedron" => {
                    if let Some(hexahedron) = current_hexahedron.take() {
                        current_type.get_or_insert_default().hexahedron = Some(hexahedron);
                    }
                }
                b"idlist" => {
                    if let Some(id_list) = current_id_list.take() {
                        id_lists.insert(id_list.name.clone(), id_list);
                    }
                }
                _ => (),
            },
            Ok(Event::Empty(bytes_start)) => {
                if Type::try_match_bytes_start(&mut current_type, &bytes_start, None)
                    .context("could not parse type from empty event")?
                    .match_found()
                {
                    let type_ = current_type
                        .take()
                        .context("Type is None even though it should not be")?;

                    types.insert(type_.name.clone(), type_);
                }

                if IDList::try_match_bytes_start(&mut current_id_list, &bytes_start, None)
                    .context("could not parse id list from start event")?
                    .match_found()
                {
                    continue;
                }

                let mut location = None;

                Location::try_match_bytes_start(&mut location, &bytes_start, None)
                    .context("could not parse location from empty event")?;

                if let Some(mut location) = location {
                    let component = current_component.get_or_insert_default();

                    if location.name.is_empty() {
                        location.name = component.type_name.clone();
                    }

                    component.location.push(location);

                    continue;
                }

                let mut rotation = None;

                Rotation::try_match_bytes_start(&mut rotation, &bytes_start, None)
                    .context("could not parse rotation from empty event")?;

                if let Some(rotation) = rotation {
                    current_component
                        .get_or_insert_default()
                        .push_rotation(rotation);

                    continue;
                }

                let mut locations = None;

                Locations::try_match_bytes_start(&mut locations, &bytes_start, None)
                    .context("could not parse locations from empty event")?;

                if let Some(locations) = locations {
                    current_component
                        .get_or_insert_default()
                        .locations
                        .push(locations);

                    continue;
                }

                Hexahedron::try_match_bytes_start(&mut current_hexahedron, &bytes_start, None)
                    .context("could not parse hexahedron from empty event")?;
            }
            Ok(Event::Text(_)) => (),
            _ => (),
        }
    }

    buf.clear();

    let types_pointer = Arc::new(types);

    let component_trees =
        ComponentTree::from_types_and_components(types_pointer.clone(), components);

    Ok(DetectorDefinition {
        types: types_pointer,
        component_tree: component_trees,
        id_lists,
    })
}
