//! Utility functions for the crate.

use anyhow::Context;

use crate::Point;

pub(crate) fn add_suffix(key: &[u8], suffix: &str) -> Vec<u8> {
    key.into_iter()
        .chain(suffix.as_bytes())
        .copied()
        .collect::<Vec<_>>()
}

pub(crate) fn parse_attribute<T>(attribute: &[u8]) -> anyhow::Result<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let string = std::str::from_utf8(attribute).context(format!(
        "could not convert attribute to string: {:?}",
        attribute
    ))?;

    string.parse::<T>().context(format!(
        "could not parse string: {:?} to {:?}",
        string,
        std::any::type_name::<T>()
    ))
}

pub(crate) fn spherical_to_cartesian(point_in_spherical_coordinates: Point) -> Point {
    let r = point_in_spherical_coordinates.x;
    let theta = point_in_spherical_coordinates.y;
    let phi = point_in_spherical_coordinates.z;

    let x = r * theta.sin() * phi.cos();
    let y = r * theta.sin() * phi.sin();
    let z = r * theta.cos();

    Point::new(x, y, z)
}
