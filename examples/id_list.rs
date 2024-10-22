const TEST_DETECTOR_DEFINITION_PATH: &str = "assets/test_detector_definition.xml";

fn main() {
    let content =
        std::fs::read_to_string(TEST_DETECTOR_DEFINITION_PATH).expect("could not read file");

    let detector_definition = mantid_idf::DetectorDefinition::from_str(&content)
        .expect("could not parse detector definition");

    let root = detector_definition.component_tree;

    let points = root.get_special_type_points();

    let ids = detector_definition.id_lists.get("ids").unwrap().get_ids();

    dbg!(&ids.len());

    assert_eq!(points.len(), ids.len());
}
