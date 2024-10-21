# Mantid IDF: Library for the Instrument Definition Files of the Mantid Project

https://mantidproject.org/ (not affilliated)

## Not feature complete / bug free

The library offers the ability to read Mantid instrument definition files (.xml) into a tree struct and provides a (small) range of useful functions.

Please note that the structure and design of the library are still in development and may change significantly in the future.

## Example
```rust
let content = std::fs::read_to_string(TEST_DETECTOR_DEFINITION_PATH).unwrap();

let detector_definition = mantid_idf::detector_definition_from_str(&content).unwrap();

let tree_root = detector_definition.component_tree;

let filtered_tree = tree_root
    .filtered_component_tree(|node| {
        let type_ = node.get_type();

        type_.special_type == SpecialTypes::Detector || type_.special_type == SpecialTypes::None
    })
    .unwrap();
```