const TEST_DETECTOR_DEFINITION_PATH: &str = "assets/test_detector_definition.xml";

fn main() {
    let content =
        std::fs::read_to_string(TEST_DETECTOR_DEFINITION_PATH).expect("could not read file");

    let detector_definition = mantid_idf::DetectorDefinition::from_str(&content)
        .expect("could not parse detector definition");

    let types = detector_definition.types;

    let root = types.get("VoxelsRoot").unwrap();

    dbg!(root);

    let voxels = root.components.first().unwrap();

    dbg!(voxels);

    let x00: &mantid_idf::structs::Component = types
        .get(&voxels.type_name)
        .unwrap()
        .components
        .first()
        .unwrap();

    let x00_locations = x00.locations.first().unwrap();

    let start_rotation = x00_locations.start_rotation.as_ref().unwrap();
    let end_rotation = x00_locations.end_rotation.as_ref().unwrap();

    let rotation_axis = &start_rotation.axis;

    let start_rot = start_rotation.rot;
    let end_rot = end_rotation.rot;

    let n_elements = x00_locations.n_elements;

    let x00_type = types.get(&x00.type_name).unwrap();

    let x00a = x00_type.components.first().unwrap();
    let x00b = x00_type.components.last().unwrap();

    let _x00a_type = types.get(&x00a.type_name).unwrap();
    let x00b_type = types.get(&x00b.type_name).unwrap();

    for element in 0..n_elements {
        let rotation_angle =
            start_rot + (end_rot - start_rot) * (element as f32 / n_elements as f32);

        let rotation = nalgebra::Rotation3::from_axis_angle(
            &nalgebra::Unit::new_normalize(nalgebra::Vector3::new(
                rotation_axis.x,
                rotation_axis.y,
                rotation_axis.z,
            )),
            rotation_angle,
        );

        for component in x00b_type.components.iter() {
            let center_point = component
                .location
                .first()
                .unwrap()
                .translation
                .first()
                .unwrap()
                .clone()
                .into_cartesian();

            dbg!(center_point);

            let rotated_point = rotation.transform_point(&nalgebra::Point3::new(
                center_point.x,
                center_point.y,
                center_point.z,
            ));

            dbg!(rotated_point);

            let type_ = types.get(&component.type_name).unwrap();

            let hexahedron = type_.hexahedron.as_ref().unwrap();

            dbg!(hexahedron);
        }
    }
}
