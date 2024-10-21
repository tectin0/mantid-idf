#[cfg(test)]
mod test_detector_definition {

    const TEST_DETECTOR_DEFINITION_PATH: &str = "assets/test_detector_definition.xml";

    #[test]
    fn test_to_file() {
        let content =
            std::fs::read_to_string(TEST_DETECTOR_DEFINITION_PATH).expect("could not read file");

        let detector_definition = mantid_idf::DetectorDefinition::from_str(&content)
            .expect("could not parse detector definition");

        assert!(detector_definition.component_tree.children.len() > 0);
        assert!(detector_definition.component_tree.component.is_root());

        assert_eq!(
            detector_definition.component_tree.children[0]
                .component
                .type_name,
            "VoxelsRoot"
        );
    }
}
