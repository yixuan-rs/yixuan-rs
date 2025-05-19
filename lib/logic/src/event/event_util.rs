use config::GraphReference;
use vivian_proto::EventGraphOwnerType;

pub fn graph_reference_to_owner_type(graph_ref: GraphReference) -> EventGraphOwnerType {
    use GraphReference::*;

    match graph_ref {
        MainCitySection(_) => EventGraphOwnerType::Section,
        Interact(_) => EventGraphOwnerType::SceneUnit,
        Quest(_) => EventGraphOwnerType::Scene,
        HollowEvent(_) => EventGraphOwnerType::Hollow,
    }
}
