use std::bitv::BitvSet;

trait System {
    fn aspect(&self) -> &BitvSet;
    fn process(&mut self, entity: &mut Entity, &mut World);
}
