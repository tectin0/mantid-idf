use quick_xml::events::attributes::Attribute;

use anyhow::Ok;

use crate::structs::{Response, Rotation, Translation, Type};
use crate::utils::{add_suffix, parse_attribute};
use crate::Point;

pub trait TryMatchAttribute: Sized {
    fn try_match_attribute(
        self_option: &mut Option<Self>,
        attribute: &Attribute<'_>,
        suffix: Option<&str>,
    ) -> anyhow::Result<Response>;
}

impl TryMatchAttribute for Translation {
    fn try_match_attribute(
        self_option: &mut Option<Self>,
        attribute: &Attribute<'_>,
        suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response { match_found: true };

        let key = attribute.key.as_ref();
        let value = attribute.value.as_ref();

        let suffix = suffix.unwrap_or_default();

        let x_suffix = add_suffix(b"x", suffix);
        let y_suffix = add_suffix(b"y", suffix);
        let z_suffix = add_suffix(b"z", suffix);
        let r_suffix = add_suffix(b"r", suffix);
        let t_suffix = add_suffix(b"t", suffix);
        let p_suffix = add_suffix(b"p", suffix);

        match key {
            val if val == &x_suffix => {
                self_option
                    .get_or_insert(Translation::Cartesian(Point::default()))
                    .inner_mut()
                    .x = parse_attribute(value)?;
            }
            val if val == &y_suffix => {
                self_option
                    .get_or_insert(Translation::Cartesian(Point::default()))
                    .inner_mut()
                    .y = parse_attribute(value)?;
            }
            val if val == &z_suffix => {
                self_option
                    .get_or_insert(Translation::Cartesian(Point::default()))
                    .inner_mut()
                    .z = parse_attribute(value)?;
            }
            val if val == &r_suffix => {
                self_option
                    .get_or_insert(Translation::Spherical(Point::default()))
                    .inner_mut()
                    .x = parse_attribute(value)?;
            }
            val if val == &t_suffix => {
                self_option
                    .get_or_insert(Translation::Spherical(Point::default()))
                    .inner_mut()
                    .y = parse_attribute(value)?;
            }
            val if val == &p_suffix => {
                self_option
                    .get_or_insert(Translation::Spherical(Point::default()))
                    .inner_mut()
                    .z = parse_attribute(value)?;
            }
            _ => {
                response.match_found = false;
            }
        };

        Ok(response)
    }
}

impl TryMatchAttribute for Rotation {
    fn try_match_attribute(
        self_option: &mut Option<Self>,
        attribute: &Attribute<'_>,
        suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response { match_found: true };

        let rotation = self_option.get_or_insert_default();

        let key = attribute.key.as_ref();
        let value = attribute.value.as_ref();

        let suffix = suffix.unwrap_or_default();

        let val_suffix = add_suffix(b"val", suffix);
        let rot_suffix = add_suffix(b"rot", suffix);

        let x_suffix = add_suffix(b"axis-x", suffix);
        let y_suffix = add_suffix(b"axis-y", suffix);
        let z_suffix = add_suffix(b"axis-z", suffix);

        match key {
            val if val == &val_suffix || val == &rot_suffix => {
                rotation.rot = parse_attribute(value)?
            }
            val if val == &x_suffix => rotation.axis.x = parse_attribute(value)?,
            val if val == &y_suffix => rotation.axis.y = parse_attribute(value)?,
            val if val == &z_suffix => rotation.axis.z = parse_attribute(value)?,
            _ => {
                response.match_found = false;
            }
        };

        Ok(response)
    }
}

impl TryMatchAttribute for Point {
    fn try_match_attribute(
        self_option: &mut Option<Self>,
        attribute: &Attribute<'_>,
        _suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response { match_found: true };

        let point = self_option.get_or_insert(Point::default());

        match attribute.key.as_ref() {
            b"x" => {
                point.x = parse_attribute(&attribute.value)?;
            }
            b"y" => {
                point.y = parse_attribute(&attribute.value)?;
            }
            b"z" => {
                point.z = parse_attribute(&attribute.value)?;
            }
            _ => {
                response.match_found = false;
            }
        }

        Ok(response)
    }
}

impl TryMatchAttribute for Type {
    fn try_match_attribute(
        self_option: &mut Option<Self>,
        attribute: &Attribute<'_>,
        _suffix: Option<&str>,
    ) -> anyhow::Result<Response> {
        let mut response = Response { match_found: true };

        let type_ = self_option.get_or_insert_default();

        let key = attribute.key.as_ref();

        match key {
            b"name" => {
                type_.name = std::str::from_utf8(&attribute.value)?.to_string();
            }
            b"is" => {
                type_.special_type = std::str::from_utf8(&attribute.value)?
                    .parse()
                    .unwrap_or_default();
            }
            b"" => {
                response.match_found = false;
            }
            _ => {
                type_.other_attributes.insert(
                    std::str::from_utf8(key)?.to_string(),
                    std::str::from_utf8(&attribute.value)?.to_string(),
                );
            }
        };

        Ok(response)
    }
}
