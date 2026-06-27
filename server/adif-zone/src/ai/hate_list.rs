use bevy_ecs::prelude::*;

#[derive(Debug, Clone)]
pub struct HateEntry {
    pub entity_id: u32,
    pub hate: i64,
    pub damage: i64,
}

#[derive(Component, Debug, Default)]
pub struct HateList {
    entries: Vec<HateEntry>,
}

impl HateList {
    pub fn add_hate(&mut self, entity_id: u32, amount: i64) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.entity_id == entity_id) {
            entry.hate += amount;
        } else {
            self.entries.push(HateEntry {
                entity_id,
                hate: amount,
                damage: 0,
            });
        }
    }

    pub fn add_damage(&mut self, entity_id: u32, amount: i64) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.entity_id == entity_id) {
            entry.damage += amount;
            entry.hate += amount;
        } else {
            self.entries.push(HateEntry {
                entity_id,
                hate: amount,
                damage: amount,
            });
        }
    }

    pub fn remove(&mut self, entity_id: u32) {
        self.entries.retain(|e| e.entity_id != entity_id);
    }

    pub fn top_target(&self) -> Option<u32> {
        self.entries.iter().max_by_key(|e| e.hate).map(|e| e.entity_id)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn contains(&self, entity_id: u32) -> bool {
        self.entries.iter().any(|e| e.entity_id == entity_id)
    }

    pub fn total_hate_for(&self, entity_id: u32) -> i64 {
        self.entries
            .iter()
            .find(|e| e.entity_id == entity_id)
            .map_or(0, |e| e.hate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_hate_new_entry() {
        let mut list = HateList::default();
        list.add_hate(1, 100);
        assert_eq!(list.len(), 1);
        assert_eq!(list.top_target(), Some(1));
    }

    #[test]
    fn add_hate_stacks() {
        let mut list = HateList::default();
        list.add_hate(1, 100);
        list.add_hate(1, 50);
        assert_eq!(list.total_hate_for(1), 150);
    }

    #[test]
    fn top_target_highest_hate() {
        let mut list = HateList::default();
        list.add_hate(1, 100);
        list.add_hate(2, 200);
        list.add_hate(3, 50);
        assert_eq!(list.top_target(), Some(2));
    }

    #[test]
    fn damage_adds_hate() {
        let mut list = HateList::default();
        list.add_damage(1, 500);
        assert_eq!(list.total_hate_for(1), 500);
    }

    #[test]
    fn remove_entry() {
        let mut list = HateList::default();
        list.add_hate(1, 100);
        list.add_hate(2, 200);
        list.remove(2);
        assert_eq!(list.len(), 1);
        assert_eq!(list.top_target(), Some(1));
    }

    #[test]
    fn clear_empties() {
        let mut list = HateList::default();
        list.add_hate(1, 100);
        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.top_target(), None);
    }
}
