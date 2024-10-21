use anyhow::Context;
use quick_xml::events::BytesStart;

use crate::{
    structs::{Component, Hexahedron, Location, Locations, Response, Rotation, Translation, Type},
    utils::parse_attribute,
    Point,
};

use super::try_match_attribute::TryMatchAttribute;

pub trait TryMatchBytesStart: Sized {
    fn try_match_bytes_start(
        self_option: &mut Option<Self>,
        bytes_start: &BytesStart<'_>,
        suffix: Option<&str>,
    ) -> anyhow::Result<Response>;
}

impl TryMatchBytesStart for Hexahedron {
    fn try_match_bytes_start(
        self_option: &mut Option<Self>,
        bytes_start: &BytesStart<'_>,
        _suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response::default();

        let hexahedron = self_option.get_or_insert(Hexahedron::default());

        if bytes_start.name().as_ref() == b"hexahedron" {
            response.match_found = true;

            match bytes_start.try_get_attribute(b"id")? {
                Some(attribute) => {
                    hexahedron.id = parse_attribute(&attribute.value)?;
                }
                None => (), // TODO: error if no id attribute?
            }

            return Ok(response);
        }

        match hexahedron.get_mut(bytes_start.name().as_ref()) {
            Some(current_point) => {
                response.match_found = true;

                let mut point = None;

                for attribute in bytes_start.attributes() {
                    if let Ok(attribute) = attribute {
                        Point::try_match_attribute(&mut point, &attribute, None)?;
                    }
                }

                *current_point = point.unwrap_or_default(); // TODO: error if point is None?

                return Ok(response);
            }
            None => return Ok(response),
        }
    }
}

impl TryMatchBytesStart for Locations {
    fn try_match_bytes_start(
        self_option: &mut Option<Self>,
        bytes_start: &BytesStart<'_>,
        _suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response::default();

        match bytes_start.name().as_ref() {
            b"locations" => {
                response.match_found = true;

                let locations = self_option.get_or_insert(Locations::default());

                let mut start_translation: Option<Translation> = None;
                let mut end_translation: Option<Translation> = None;

                let mut start_rotation: Option<Rotation> = None;
                let mut end_rotation: Option<Rotation> = None;

                for attribute in bytes_start.attributes() {
                    if let Ok(attribute) = attribute {
                        let key = attribute.key.as_ref();

                        match key {
                            b"n-elements" => {
                                locations.n_elements = parse_attribute(&attribute.value)?;
                                continue;
                            }
                            b"name" => {
                                locations.name = std::str::from_utf8(&attribute.value)?.to_string();
                                continue;
                            }
                            b"name-count-start" => {
                                locations.name_count_start = parse_attribute(&attribute.value)?;
                                continue;
                            }
                            _ => (),
                        }

                        if Translation::try_match_attribute(
                            &mut start_translation,
                            &attribute,
                            None,
                        )?
                        .match_found()
                        {
                            continue;
                        }

                        if Translation::try_match_attribute(
                            &mut end_translation,
                            &attribute,
                            Some("-end"),
                        )?
                        .match_found()
                        {
                            continue;
                        }

                        if Rotation::try_match_attribute(&mut start_rotation, &attribute, None)?
                            .match_found()
                        {
                            continue;
                        }

                        if Rotation::try_match_attribute(
                            &mut end_rotation,
                            &attribute,
                            Some("-end"),
                        )?
                        .match_found()
                        {
                            continue;
                        }
                    }
                }

                locations.start_translation = start_translation;
                locations.end_translation = end_translation;
                locations.start_rotation = start_rotation;
                locations.end_rotation = end_rotation;
            }
            _ => (),
        }

        Ok(response)
    }
}

impl TryMatchBytesStart for Rotation {
    fn try_match_bytes_start(
        self_option: &mut Option<Self>,
        bytes_start: &BytesStart<'_>,
        _suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response::default();

        match bytes_start.name().as_ref() {
            b"rot" => {
                response.match_found = true;

                for attribute in bytes_start.attributes() {
                    if let Ok(attribute) = attribute {
                        Rotation::try_match_attribute(self_option, &attribute, None)?;
                    }
                }
            }
            _ => (),
        }

        Ok(response)
    }
}

impl TryMatchBytesStart for Location {
    fn try_match_bytes_start(
        self_option: &mut Option<Self>,
        bytes_start: &BytesStart<'_>,
        _suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response::default();

        match bytes_start.name().as_ref() {
            b"location" => {
                response.match_found = true;

                let location = self_option.get_or_insert(Location::default());

                let mut translation: Option<Translation> = None;

                let mut rotation: Option<Rotation> = None;

                for attribute in bytes_start.attributes() {
                    if let Ok(attribute) = attribute {
                        let key = attribute.key.as_ref();

                        if key == b"name" {
                            location.name = std::str::from_utf8(&attribute.value)?.to_string();
                        }

                        if Translation::try_match_attribute(&mut translation, &attribute, None)?
                            .match_found()
                        {
                            continue;
                        }

                        if Rotation::try_match_attribute(&mut rotation, &attribute, None)?
                            .match_found()
                        {
                            continue;
                        }
                    }
                }

                if let Some(translation) = translation {
                    location.translation.push(translation);
                }

                if let Some(rotation) = rotation {
                    location.rotation.push(rotation);
                }
            }
            _ => (),
        }

        Ok(response)
    }
}

impl TryMatchBytesStart for Type {
    fn try_match_bytes_start(
        self_option: &mut Option<Self>,
        bytes_start: &BytesStart<'_>,
        _suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response::default();

        if bytes_start.name().as_ref() == b"type" {
            response.match_found = true;

            for attribute in bytes_start.attributes() {
                if let Ok(attribute) = attribute {
                    Type::try_match_attribute(self_option, &attribute, None)?;
                }
            }
        }

        Ok(response)
    }
}

impl TryMatchBytesStart for Component {
    fn try_match_bytes_start(
        self_option: &mut Option<Self>,
        bytes_start: &BytesStart<'_>,
        _suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response::default();

        match bytes_start.name().as_ref() {
            b"component" => {
                response.match_found = true;

                let component = self_option.get_or_insert(Component::default());

                let type_ = bytes_start
                    .try_get_attribute("type")?
                    .context("could not get attribute type")?
                    .value;

                component.type_name = std::str::from_utf8(&type_)?.to_string();
            }
            _ => (),
        }

        Ok(response)
    }
}
