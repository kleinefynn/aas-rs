use strum::{Display, EnumString};

pub type KeyReference = String;

/// Specifies the reference context for this key in the Asset Administration Shell (AAS) metamodel.
///
/// When the key type is `Key::GlobalReference`, it targets an external resource identified by a
/// globally unique identifier,
/// such as an IRI, IRDI, or URI, typically pointing to referable entities like
/// ConceptDescriptions outside the current AAS context.
/// This facilitates interoperability by enabling unambiguous referencing across distributed systems.
///
/// For `Key::FragmentReference`, the key identifies a fragment or local anchor within its parent element,
/// scoped by the reference chain defined by preceding keys. This allows fine-grained references to subcomponents,
/// such as elements within documents or data blobs.
///
/// Other key types identify specific model elements (like Submodel, SubmodelElement, or Asset)
/// within the same or another AAS,
/// explicitly naming the referenced element to enable precise navigation within the AAS environment.
#[derive(EnumString, Display, Clone, PartialEq, Debug)]
pub enum Key {
    AnnotatedRelationshipElement(KeyReference),
    AssetAdministrationShell(KeyReference),
    BasicEventElement(KeyReference),
    Blob(KeyReference),
    Capability(KeyReference),
    ConceptDescription(KeyReference),
    DataElement(KeyReference),
    Entity(KeyReference),
    EventElement(KeyReference),
    File(KeyReference),
    FragmentReference(KeyReference),
    GlobalReference(KeyReference),
    Identifiable(KeyReference),
    MultiLanguageProperty(KeyReference),
    Operation(KeyReference),
    Property(KeyReference),
    Range(KeyReference),
    Referable(KeyReference),
    ReferenceElement(KeyReference),
    RelationshipElement(KeyReference),
    Submodel(KeyReference),
    SubmodelElement(KeyReference),
    SubmodelElementCollection(KeyReference),
    SubmodelElementList(KeyReference),
}
