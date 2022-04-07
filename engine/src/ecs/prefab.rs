use specs::world::LazyBuilder;


pub trait PrefabBuilder {
    type PrefabState;

    fn build<'a>(entity_builder: LazyBuilder<'a>, state: Self::PrefabState) -> LazyBuilder<'a>;

    // TODO: Make use updates to existing entity instead of making a new entty
    // fn build_from_panel<'a>(entity_builder: LazyBuilder<'a>, panel_values: &ImguiPanelValues) -> LazyBuilder<'a>;
}