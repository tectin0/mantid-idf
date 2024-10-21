//! This module contains the structs used to represent the data in the instrument definition file.
//! TODO: The name `structs` is not very descriptive.

use std::collections::BTreeMap;

use nalgebra::{Rotation3, Translation3};

use crate::{types::SpecialTypes, utils::spherical_to_cartesian, Point};

/// Represents the type of a component.
/// Can contain other components.
/// TODO: Move the `Component` out of this struct and replace it with only the name? -> Redundant with component tree.
#[derive(Debug, Default, Clone)]
pub struct Type {
    /// The special type of this type. Has a fixed set of possible values.
    pub special_type: SpecialTypes,
    /// The name of the type.
    pub name: String,
    /// The components that are part of this type.
    pub components: Vec<Component>,
    /// The possible hexahedron for this type. \
    /// TODO: Other possible geometric shapes are missing here. `Cuboid` for example. -> Maybe a `Shape` enum?
    pub hexahedron: Option<Hexahedron>,
    /// Other attributes that the type can have. \
    /// "Catch-all" for attributes to avoid many `Option` fields. TODO: Better solution?
    pub other_attributes: BTreeMap<String, String>,
}

/// Represents a component in the instrument definition file.
/// Has a type and can contain locations.
#[derive(Debug, Default, Clone)]
pub struct Component {
    /// The type of the component. This has to be present in the `Types` struct.
    pub type_name: String,
    /// Represents the `location` tag in the IDF file. \
    /// Contains the translation and rotation of the component.
    pub location: Vec<Location>,
    /// Represents the `locations` tag in the IDF file. \
    /// Contains the translation and rotation of the component for multiple elements.
    pub locations: Vec<Locations>,
}

impl Component {
    /// Check if the component is the root component. \
    /// Checks if the type name is empty. \
    /// TODO: Find a better way to represent the root component.
    pub fn is_root(&self) -> bool {
        self.type_name == ""
    }

    /// Push a new translation to the last location.
    pub fn push_rotation(&mut self, rotation: Rotation) {
        self.location
            .last_mut()
            .get_or_insert(&mut Location::default())
            .rotation
            .push(rotation);
    }
}

/// Represents the location of a component.
/// Contains the translations and rotations of the component.
/// Can have a name.
#[derive(Debug, Default, Clone)]
pub struct Location {
    /// The name of the location.
    pub name: String,
    /// The translations of the component.
    pub translation: Vec<Translation>,
    /// The rotations of the component.
    pub rotation: Vec<Rotation>,
}

impl Location {
    pub(crate) fn to_new_translations_and_rotations(
        &self,
    ) -> (Vec<Translation3<f32>>, Vec<Rotation3<f32>>) {
        let mut new_translations = Vec::new();

        for translation in self.translation.iter() {
            let translation = nalgebra::Translation3::from(translation.clone().into_cartesian());

            new_translations.push(translation);
        }

        let mut new_rotations = Vec::new();

        for rotation in self.rotation.iter() {
            let rotation = nalgebra::Rotation3::from_axis_angle(
                &nalgebra::Unit::new_normalize(nalgebra::Vector3::new(
                    rotation.axis.x,
                    rotation.axis.y,
                    rotation.axis.z,
                )),
                rotation.rot.to_radians(),
            );

            new_rotations.push(rotation);
        }
        (new_translations, new_rotations)
    }
}

/// Represents a translation in the IDF file.
/// Can be in cartesian or spherical coordinates.
#[derive(Debug, Clone)]
pub enum Translation {
    /// Represents a translation in cartesian coordinates.
    Cartesian(Point),
    /// Represents a translation in spherical coordinates.
    Spherical(Point),
}

impl Translation {
    pub(crate) fn inner_mut(&mut self) -> &mut Point {
        match self {
            Self::Cartesian(v) => v,
            Self::Spherical(v) => v,
        }
    }

    /// Convert the generic translation to cartesian coordinates.
    pub fn into_cartesian(self) -> Point {
        match self {
            Self::Cartesian(v) => v,
            Self::Spherical(v) => spherical_to_cartesian(v),
        }
    }
}

/// Represents a rotation in the IDF file.
/// Contains the rotation angle and the axis of rotation.
/// The angle is in degrees.
/// The default axis is the z-axis.
#[derive(Debug, Clone)]
pub struct Rotation {
    /// The rotation angle in degrees.
    pub rot: f32,
    /// The axis of rotation.
    pub axis: Point,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            rot: 0.0,
            // https://docs.mantidproject.org/nightly/concepts/InstrumentDefinitionFile.html
            // -> Note that the z-axis for the second rotation is implicit since no other axis information provided for the second rotation. This is hard-coded.
            axis: Point::new(0.0, 0.0, 1.0),
        }
    }
}

/// Represents the locations of a component for multiple elements.
/// Contains a start and end translation and rotation.
/// The translations and rotations are interpolated between the start and end values.
/// Generates multiple new elements.
#[derive(Debug, Default, Clone)]
pub struct Locations {
    /// The number of elements to generate.
    pub n_elements: u32,
    /// The name of the locations.
    pub name: String,
    /// Where the name count starts. TODO: not properly implemented.
    pub name_count_start: u32,
    /// The start translation.
    pub start_translation: Option<Translation>,
    /// The end translation.
    pub end_translation: Option<Translation>,
    /// The start rotation.
    pub start_rotation: Option<Rotation>,
    /// The end rotation. \
    /// TODO: The axis of the end rotation seems to assume the axis of the start rotation. Check if this is correct.
    pub end_rotation: Option<Rotation>,
}

impl Locations {
    pub(crate) fn to_new_translations_and_rotations(
        &self,
        element: u32,
    ) -> (Vec<Translation3<f32>>, Vec<Rotation3<f32>>) {
        {
            let mut new_translations = Vec::new();

            if let Some(start_translation) = self.start_translation.as_ref() {
                match self.end_translation.as_ref() {
                    Some(end_translation) => {
                        let translation = start_translation.clone().into_cartesian().lerp(
                            &end_translation.clone().into_cartesian(),
                            element as f32 / self.n_elements as f32,
                        );

                        new_translations.push(Translation3::from(translation));
                    }
                    None => {
                        let translation = start_translation.clone().into_cartesian();

                        new_translations.push(Translation3::from(translation));
                    }
                }
            }

            let mut new_rotations = Vec::new();

            if let Some(start_rotation) = self.start_rotation.as_ref() {
                match self.end_rotation.as_ref() {
                    Some(end_rotation) => {
                        let rotation = nalgebra::Rotation3::from_axis_angle(
                            &nalgebra::Unit::new_normalize(nalgebra::Vector3::new(
                                start_rotation.axis.x,
                                start_rotation.axis.y,
                                start_rotation.axis.z,
                            )),
                            start_rotation.rot
                                + (end_rotation.rot - start_rotation.rot)
                                    * (element as f32 / self.n_elements as f32),
                        );

                        new_rotations.push(rotation);
                    }
                    None => {
                        let rotation = nalgebra::Rotation3::from_axis_angle(
                            &nalgebra::Unit::new_normalize(nalgebra::Vector3::new(
                                start_rotation.axis.x,
                                start_rotation.axis.y,
                                start_rotation.axis.z,
                            )),
                            start_rotation.rot.to_radians(),
                        );

                        new_rotations.push(rotation);
                    }
                }
            }
            (new_translations, new_rotations)
        }
    }
}

/// Represents a hexahedron in the IDF file.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone)]
pub struct Hexahedron {
    pub id: u32,
    pub left_back_bottom_point: Point,
    pub left_front_bottom_point: Point,
    pub right_front_bottom_point: Point,
    pub right_back_bottom_point: Point,
    pub left_back_top_point: Point,
    pub left_front_top_point: Point,
    pub right_front_top_point: Point,
    pub right_back_top_point: Point,
}

impl Hexahedron {
    pub(crate) fn get_mut(&mut self, key: &[u8]) -> Option<&mut Point> {
        match key {
            b"left-back-bottom-point" => Some(&mut self.left_back_bottom_point),
            b"left-front-bottom-point" => Some(&mut self.left_front_bottom_point),
            b"right-front-bottom-point" => Some(&mut self.right_front_bottom_point),
            b"right-back-bottom-point" => Some(&mut self.right_back_bottom_point),
            b"left-back-top-point" => Some(&mut self.left_back_top_point),
            b"left-front-top-point" => Some(&mut self.left_front_top_point),
            b"right-front-top-point" => Some(&mut self.right_front_top_point),
            b"right-back-top-point" => Some(&mut self.right_back_top_point),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Response {
    pub match_found: bool,
}

impl Response {
    pub fn match_found(&self) -> bool {
        self.match_found
    }
}
