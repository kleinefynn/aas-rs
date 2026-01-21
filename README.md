# Asset Administration Types

A crate for mapping the types of Asset Administration Shells to
Rust.

## Structure

For supporting multiple versions of the AAS specifications,  
each part of the Spec (Part 1, Part 3...) is going to be seperated,  
as well as individually versions.   
I.e. you can find the types of Part 1 version 3.1 under `src/part_1/v3_1`.

## Goals:

- Provide as much type-safety as possible
    - Parsing types to ensure proper formats.
    - Constraints are to be respected.
- Provide support for JSON and XML simultaneous
- Implement all AAS specs
- As many tests as possible
- Code-Documentation based on the AAS spec.
  Close to no need to consult spec papers.

For 1.0 JSON, XML and .AASX Format should work.
As many tests as possible. Test multiple demo Shells.

## Todos:

- [ ] Part 1 🚧
    - [ ] Version 3.1.1 🚧
        - [x] Type definitions
        - [ ] Move DataSpecifications to part 3
- [ ] Part 3a
- [ ] Documentation 🚧
- [] Tests 🚧
- [ ] JSON 🚧
    - [x] "Normal" De-/Serialization for APIs (Part 2)
    - [] Metdadata only 🚧
    - [] Value only 🚧
- [] XML 🚧
- [ ] AASX Package Format
