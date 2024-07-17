use std::mem;

use crate::{
    components::{ComponentsPayload, Figure, Name, Transform},
    Entity,
};

use super::Spawner;

// TODO: this should be configurable by user.
const MAX_ENTITIES: usize = 5000;

pub struct Scene {
    next_entity: Entity,
    free_entities: Vec<Entity>,

    entities: [Option<Entity>; MAX_ENTITIES],
    name_components: [Option<Name>; MAX_ENTITIES],
    transform_components: [Option<Transform>; MAX_ENTITIES],
    figure_components: [Option<Figure>; MAX_ENTITIES],

    spawner: Spawner,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene::new()
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            next_entity: 0,
            free_entities: Vec::new(),

            entities: [(); MAX_ENTITIES].map(|_| None),
            name_components: [(); MAX_ENTITIES].map(|_| None),
            transform_components: [(); MAX_ENTITIES].map(|_| None),
            figure_components: [(); MAX_ENTITIES].map(|_| None),

            spawner: Spawner::new(),
        }
    }

    pub(crate) fn play(&mut self) {
        self.add_entities();
        self.remove_entities();
    }

    pub fn add_initial_entities(&mut self, component_payloads: Vec<ComponentsPayload>) {
        let payloads_count = component_payloads.len();

        for (entity, payload) in component_payloads.into_iter().enumerate() {
            if payloads_count >= MAX_ENTITIES {
                panic!(
                    "{} entities were provided while only {} can exist",
                    payloads_count, MAX_ENTITIES
                );
            };

            self.populate_entity(entity, payload);
        }

        self.next_entity = payloads_count;
    }

    fn add_entities(&mut self) {
        for payload in mem::take(&mut self.spawner.entities_to_add) {
            let entity = self.free_entities.pop().unwrap_or_else(|| {
                self.next_entity += 1;

                self.next_entity - 1
            });

            if entity >= MAX_ENTITIES {
                panic!("Too many entities");
            }

            self.populate_entity(entity, payload);
        }
    }

    fn remove_entities(&mut self) {
        for entity_to_remove in mem::take(&mut self.spawner.entities_to_remove) {
            let entity = self.entities.iter().position(|entity| match entity {
                None => false,
                Some(entity) => *entity == entity_to_remove,
            });

            match entity {
                None => (),
                Some(entity) => self.depopulate_entity(entity),
            }
        }
    }

    fn populate_entity(&mut self, entity: Entity, payload: ComponentsPayload) {
        self.entities[entity] = Some(entity);
        self.name_components[entity] = Some(payload.name);
        self.transform_components[entity] = Some(payload.transform);
        self.figure_components[entity] = payload.figure;
    }

    fn depopulate_entity(&mut self, entity: Entity) {
        self.entities[entity] = None;
        self.name_components[entity] = None;
        self.transform_components[entity] = None;
        self.figure_components[entity] = None;

        self.free_entities.push(entity);
    }
}
