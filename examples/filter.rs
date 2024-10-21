use mantid_idf::types::SpecialTypes;

const TEST_DETECTOR_DEFINITION_PATH: &str = "assets/test_detector_definition.xml";

fn main() {
    let content =
        std::fs::read_to_string(TEST_DETECTOR_DEFINITION_PATH).expect("could not read file");

    let detector_definition = mantid_idf::DetectorDefinition::from_str(&content)
        .expect("could not parse detector definition");

    let root = detector_definition.component_tree;

    let filtered = root
        .filtered_component_tree(|node| {
            let type_ = node.get_type_name();

            type_.special_type == SpecialTypes::Detector || type_.special_type == SpecialTypes::None
        })
        .unwrap();

    dbg!(filtered);
}
