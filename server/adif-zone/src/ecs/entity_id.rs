use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct EntityIdAllocator {
    next_id: u32,
    entity_to_id: HashMap<Entity, u32>,
    id_to_entity: HashMap<u32, Entity>,
}

impl EntityIdAllocator {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            entity_to_id: HashMap::new(),
            id_to_entity: HashMap::new(),
        }
    }

    pub fn allocate(&mut self, entity: Entity) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        if self.next_id == 0 {
            self.next_id = 1;
        }
        self.entity_to_id.insert(entity, id);
        self.id_to_entity.insert(id, entity);
        id
    }

    pub fn deallocate(&mut self, entity: Entity) {
        if let Some(id) = self.entity_to_id.remove(&entity) {
            self.id_to_entity.remove(&id);
        }
    }

    pub fn get_id(&self, entity: Entity) -> Option<u32> {
        self.entity_to_id.get(&entity).copied()
    }

    pub fn get_entity(&self, id: u32) -> Option<Entity> {
        self.id_to_entity.get(&id).copied()
    }

    pub fn entity_count(&self) -> usize {
        self.entity_to_id.len()
    }
}

impl Default for EntityIdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_returns_sequential_ids() {
        let mut world = World::new();
        let mut alloc = EntityIdAllocator::new();

        let e1 = world.spawn_empty().id();
        let e2 = world.spawn_empty().id();

        let id1 = alloc.allocate(e1);
        let id2 = alloc.allocate(e2);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn lookup_both_directions() {
        let mut world = World::new();
        let mut alloc = EntityIdAllocator::new();

        let entity = world.spawn_empty().id();
        let id = alloc.allocate(entity);

        assert_eq!(alloc.get_id(entity), Some(id));
        assert_eq!(alloc.get_entity(id), Some(entity));
    }

    #[test]
    fn deallocate_removes_mapping() {
        let mut world = World::new();
        let mut alloc = EntityIdAllocator::new();

        let entity = world.spawn_empty().id();
        let id = alloc.allocate(entity);

        alloc.deallocate(entity);

        assert_eq!(alloc.get_id(entity), None);
        assert_eq!(alloc.get_entity(id), None);
        assert_eq!(alloc.entity_count(), 0);
    }

    #[test]
    fn skips_zero_on_wrap() {
        let mut world = World::new();
        let mut alloc = EntityIdAllocator::new();
        alloc.next_id = u32::MAX;

        let e1 = world.spawn_empty().id();
        let e2 = world.spawn_empty().id();

        let id1 = alloc.allocate(e1);
        let id2 = alloc.allocate(e2);

        assert_eq!(id1, u32::MAX);
        assert_eq!(id2, 1);
    }
}
