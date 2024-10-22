//! This module contains the `ComponentTree` and `ComponentTreeNode` structs.
//! Most of the `useful` functionality is defined here

use std::{collections::BTreeMap, fmt::Debug, sync::Arc};

use anyhow::Context;
use nalgebra::{Rotation3, Translation3};

use crate::{
    structs::{Component, Type},
    types::{SpecialTypes, Types},
    utils::Axes,
    Point,
};

/// New type for a `ComponentTreeNode` that is the root of a `ComponentTree`
#[derive(Debug, Clone, Default)]
pub struct ComponentTree(ComponentTreeNode);

impl ComponentTree {
    pub(crate) fn from_types_and_components(
        types: Arc<Types>,
        components: BTreeMap<String, Component>,
    ) -> Self {
        let mut root = ComponentTreeNode::root(types.clone());

        for (_name, component) in components.iter() {
            let tree = Self::get_node_from_component(types.clone(), component);

            root.children.push(Box::new(tree));
        }

        ComponentTree(root)
    }

    fn get_node_from_component(types: Arc<Types>, component: &Component) -> ComponentTreeNode {
        let mut node = ComponentTreeNode::new(component, types.clone());

        let type_ = types
            .get(&component.type_name)
            .context(format!("could not find type {}", component.type_name))
            .unwrap();

        let mut children = Vec::new();

        for component in type_.components.iter() {
            let child = Self::get_node_from_component(types.clone(), component);

            children.push(Box::new(child));
        }

        node.children = children;

        node
    }
}

impl std::ops::Deref for ComponentTree {
    type Target = ComponentTreeNode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A node in a tree of `Component`s \
/// The nodes contains the `Component` and its children
#[derive(Clone, Default)]
pub struct ComponentTreeNode {
    /// The children of the node
    pub children: Vec<Box<ComponentTreeNode>>,
    /// The `Component` of the node
    pub component: Component,
    types: Arc<Types>,
}

impl Debug for ComponentTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentTreeNode")
            .field("component", &self.component)
            .field("children", &self.children)
            .finish()
    }
}

impl ComponentTreeNode {
    fn new(component: &Component, types: Arc<Types>) -> Self {
        Self {
            component: component.clone(),
            children: Vec::new(),
            types,
        }
    }

    fn root(types: Arc<Types>) -> Self {
        Self {
            component: Component::default(),
            children: Vec::new(),
            types,
        }
    }

    /// Returns a filtered version of the `ComponentTreeNode` \
    /// The filter function is applied to the node and all its children \
    /// If the filter function returns `true` the node is included in the result \
    /// If the filter function returns `false` the node is not included in the result \
    /// # Example
    /// ```
    /// use mantid_idf::component_tree::ComponentTreeNode;
    ///
    /// let node = ComponentTreeNode::default();
    ///
    /// let filtered = node.filtered_component_tree(|node| {
    ///    node.component.type_name == "VoxelsRoot"
    /// });
    ///
    /// assert!(filtered.is_none());
    /// ```
    pub fn filtered_component_tree<F>(&self, filter_function: F) -> Option<Box<ComponentTreeNode>>
    where
        F: Fn(&ComponentTreeNode) -> bool + Clone,
    {
        if filter_function(self) {
            let mut node = self.clone();

            node.children = self
                .children
                .iter()
                .filter_map(|child| child.filtered_component_tree(filter_function.clone()))
                .collect();

            Some(Box::new(node))
        } else {
            None
        }
    }

    /// Iterates over the children of the node and returns the first child that matches the name
    /// # Example
    /// ```
    /// use mantid_idf::component_tree::ComponentTreeNode;
    ///
    /// let node = ComponentTreeNode::default();
    ///
    /// let child = node.get_child_by_name("VoxelsRoot");
    ///
    /// assert!(child.is_none());
    /// ```
    pub fn get_child_by_name(&self, name: &str) -> Option<&ComponentTreeNode> {
        for child in self.children.iter() {
            if child.component.type_name == name {
                return Some(child);
            }
        }

        None
    }

    /// Returns the `SpecialTypes` of the node
    /// # Example
    /// ```
    /// use mantid_idf::component_tree::ComponentTreeNode;
    ///
    /// let node = ComponentTreeNode::default();
    ///
    /// let special_type = node.get_special_type();
    ///
    /// assert_eq!(special_type, &mantid_idf::types::SpecialTypes::None);
    /// ```
    pub fn get_special_type(&self) -> &SpecialTypes {
        let type_ = match self.types.get(&self.component.type_name) {
            Some(t) => t,
            None => {
                unreachable!(
                    "Could not find type {}. This should not be able to happen.",
                    self.component.type_name
                );
            }
        };

        &type_.special_type
    }

    /// Returns the `type_name` of the node. \
    /// The `type_name` is used to look up the `Type` in the `Types` struct
    /// This is different from the `SpecialTypes` which is a property of the `Type`
    /// # Example
    /// ```
    /// use mantid_idf::component_tree::ComponentTreeNode;
    ///
    /// let node = ComponentTreeNode::default();
    ///
    /// let type_name = node.get_type_name();
    ///
    /// assert_eq!(type_name.name, "");
    /// ```
    pub fn get_type_name(&self) -> &Type {
        match self.types.get(&self.component.type_name) {
            Some(t) => t,
            None => {
                unreachable!(
                    "Could not find type {}. This should not be able to happen.",
                    self.component.type_name
                );
            }
        }
    }

    /// Returns the points of any special type, transformed by the node and all previous parents \
    /// TODO: Add example + more explanation
    pub fn get_special_type_points(&self) -> Vec<Point> {
        let translations = vec![];
        let rotations = vec![];

        let (points, _ids) = self.recursive_transform_points(translations, rotations);

        points
    }

    /// Recursively transforms the points of the node and its children \
    /// Recursion ends on nodes with no children or nodes with a special type \
    /// Returns a tuple of the transformed points and their ids \
    ///
    /// The ids don't have to be specified by any of the components &rarr; They are then typically defined in the `IDList` \
    ///
    /// TODO: Add example + more explanation
    pub fn recursive_transform_points(
        &self,
        translations: Vec<Translation3<f32>>,
        rotations: Vec<Rotation3<f32>>,
    ) -> (Vec<Point>, Vec<u32>) {
        let mut points = Vec::new();
        let mut ids = Vec::new();

        if self.component.is_root() {
            let new_translations = vec![];
            let new_rotations = vec![];

            self.extend_points_with_children_points(
                &mut points,
                &mut ids,
                new_translations,
                new_rotations,
                &translations,
                &rotations,
            );

            return (points, ids);
        }

        let no_children = self.children.is_empty();

        match self.get_special_type() {
            SpecialTypes::RectangularDetector => {
                let type_ = self.get_type_name();

                let xpixels = type_
                    .other_attributes
                    .get("xpixels")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let xstart = type_
                    .other_attributes
                    .get("xstart")
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();

                let xstep = type_
                    .other_attributes
                    .get("xstep")
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();

                let ypixels = type_
                    .other_attributes
                    .get("ypixels")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let ystart = type_
                    .other_attributes
                    .get("ystart")
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();

                let ystep = type_
                    .other_attributes
                    .get("ystep")
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();

                let idfillbyfirst = self
                    .component
                    .other_attributes
                    .get("idfillbyfirst")
                    .unwrap()
                    .parse::<Axes>()
                    .unwrap();

                let idstart = self
                    .component
                    .other_attributes
                    .get("idstart")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let idstepbyrow = self
                    .component
                    .other_attributes
                    .get("idstepbyrow")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let mut untransformed_points = Vec::new();
                let mut ids_ = Vec::new();

                let mut id = idstart;

                for x in 0..xpixels {
                    for y in 0..ypixels {
                        let x = xstart + x as f32 * xstep;
                        let y = ystart + y as f32 * ystep;

                        let point = Point::new(x, y, 0.0);

                        untransformed_points.push(point);
                        ids_.push(id);

                        id = match idfillbyfirst {
                            Axes::X => id + idstepbyrow,
                            Axes::Y => id + 1,
                            _ => unreachable!("idfillbyfirst should be either X or Y"),
                        };
                    }

                    id = match idfillbyfirst {
                        Axes::X => id + idstepbyrow,
                        Axes::Y => id + 1,
                        _ => unreachable!("idfillbyfirst should be either X or Y"),
                    };
                }

                for location in self.component.location.iter() {
                    let (new_translations, new_rotations) =
                        location.to_new_translations_and_rotations();

                    untransformed_points.iter_mut().for_each(|point| {
                        for translation in new_translations.iter() {
                            *point = translation.transform_point(point);
                        }

                        for rotation in new_rotations.iter() {
                            *point = rotation.transform_point(point);
                        }
                    });

                    points.extend(untransformed_points.clone());
                    ids.extend(ids_.clone());
                }
            }
            SpecialTypes::Detector => {
                for location in self.component.location.iter() {
                    let (new_translations, new_rotations) =
                        location.to_new_translations_and_rotations();

                    let mut point = Point::default();

                    for translation in new_translations.iter() {
                        point = translation.transform_point(&point);
                    }

                    for rotation in new_rotations.iter() {
                        point = rotation.transform_point(&point);
                    }

                    points.push(point);
                }

                for locations in self.component.locations.iter() {
                    for element in 0..locations.n_elements {
                        let (new_translations, new_rotations) =
                            locations.to_new_translations_and_rotations(element);

                        let mut point = Point::default();

                        for translation in new_translations.iter() {
                            point = translation.transform_point(&point);
                        }

                        for rotation in new_rotations.iter() {
                            point = rotation.transform_point(&point);
                        }

                        points.push(point);
                    }
                }
            }
            SpecialTypes::None => {
                if self.component.location.is_empty() && self.component.locations.is_empty() {
                    match no_children {
                        true => {
                            unreachable!(
                                "Node has no locations, no children and no special type: {:?}",
                                self
                            );
                        }
                        false => {
                            let new_translations = vec![nalgebra::Translation3::identity()];
                            let new_rotations = vec![nalgebra::Rotation3::identity()];

                            self.extend_points_with_children_points(
                                &mut points,
                                &mut ids,
                                new_translations,
                                new_rotations,
                                &translations,
                                &rotations,
                            );

                            return (points, ids);
                        }
                    }
                }

                for location in self.component.location.iter() {
                    let (new_translations, new_rotations) =
                        location.to_new_translations_and_rotations();

                    self.extend_points_with_children_points(
                        &mut points,
                        &mut ids,
                        new_translations,
                        new_rotations,
                        &translations,
                        &rotations,
                    );
                }

                for locations in self.component.locations.iter() {
                    for element in 0..locations.n_elements {
                        let (new_translations, new_rotations) =
                            locations.to_new_translations_and_rotations(element);

                        self.extend_points_with_children_points(
                            &mut points,
                            &mut ids,
                            new_translations,
                            new_rotations,
                            &translations,
                            &rotations,
                        );
                    }
                }
            }
            _ => (),
        }

        (points, ids)
    }

    fn extend_points_with_children_points(
        &self,
        points: &mut Vec<Point>,
        ids: &mut Vec<u32>,
        new_translations: Vec<Translation3<f32>>,
        new_rotations: Vec<Rotation3<f32>>,
        translations: &Vec<Translation3<f32>>,
        rotations: &Vec<Rotation3<f32>>,
    ) {
        for child in self.children.iter() {
            let (mut child_points, child_ids) =
                child.recursive_transform_points(new_translations.clone(), new_rotations.clone());

            child_points.iter_mut().for_each(|p| {
                for translation in translations.iter() {
                    *p = translation.transform_point(&p);
                }

                for rotation in rotations.iter() {
                    *p = rotation.transform_point(&p);
                }
            });

            points.extend(child_points);
            ids.extend(child_ids);
        }
    }
}
