use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use components::Networked;

use specs::{Component, Join, World};

pub type ClientId = u8;

pub type EntityId = u16;

#[derive(Clone, Serialize, Deserialize)]
pub struct ComponentStore(NetComponentIndex, Vec<u8>);

#[derive(Clone, Serialize, Deserialize)]
pub struct ComponentDelta(NetComponentIndex, Vec<u8>);

pub trait NetComponent {
    fn net_store(&self) -> Vec<u8>;
    fn net_load(&mut self, data: &[u8]);

    fn read_delta(&self) -> Vec<u8>;
    fn write_delta(&mut self, data: &[u8]);
}

type NetComponentIndex = u8;
static NET_COMPONENT_MAX: usize = std::u8::MAX as usize;

pub struct NetComponentAdapter {
    index: HashMap<TypeId, u8>,
    next_index: u8,
}

impl NetComponentAdapter {
    pub fn new() -> Self {
        NetComponentAdapter {
            index: HashMap::new(),
            next_index: 0,
        }
    }

    pub fn register_component<C>(&mut self)
    where
        C: NetComponent + 'static,
    {
        if (self.next_index as usize) + 1 > NET_COMPONENT_MAX {
            panic!("max number of net components is {}", NET_COMPONENT_MAX);
        }

        let type_id = TypeId::of::<C>();
        if self.index.contains_key(&type_id) {
            panic!("component already registered");
        }
        self.index.insert(type_id, self.next_index);
        self.next_index += 1;
    }

    pub fn net_store<'a, C>(
        &self,
        world: &World,
        out: &mut HashMap<EntityId, Vec<ComponentStore>>,
        entity_index: Option<&HashSet<EntityId>>,
    ) where
        C: Component + NetComponent + 'static,
    {
        let id = self.component_id::<C>();
        let networked = world.read_storage::<Networked>();
        let cs = world.read_storage::<C>();
        for (Networked { entity_id, .. }, c) in (&networked, &cs).join() {
            if entity_index
                .map(|set| set.contains(&entity_id))
                .unwrap_or(true)
            {
                out.entry(*entity_id)
                    .or_insert_with(|| Vec::new())
                    .push(ComponentStore(id, c.net_store()))
            }
        }
    }

    pub fn net_load<'a, C>(&self, world: &World, source: &HashMap<EntityId, Vec<ComponentStore>>)
    where
        C: Component + NetComponent + 'static,
    {
        let id = self.component_id::<C>();
        let networked = world.read_storage::<Networked>();
        let mut cs = world.write_storage::<C>();
        for (Networked { entity_id, .. }, ref mut c) in (&networked, &mut cs).join() {
            if !source.contains_key(entity_id) {
                continue;
            }
            if let Some(store) = source.get(entity_id).unwrap().iter().find(|s| s.0 == id) {
                c.net_load(&store.1);
            }
        }
    }

    pub fn read_delta<'a, C>(&self, world: &World, out: &mut HashMap<EntityId, Vec<ComponentDelta>>)
    where
        C: Component + NetComponent + 'static,
    {
        let id = self.component_id::<C>();
        let networked = world.read_storage::<Networked>();
        let cs = world.read_storage::<C>();
        for (Networked { entity_id, .. }, c) in (&networked, &cs).join() {
            out.entry(*entity_id)
                .or_insert_with(|| Vec::new())
                .push(ComponentDelta(id, c.read_delta()));
        }
    }

    pub fn write_delta<'a, C>(&self, world: &World, source: &HashMap<EntityId, Vec<ComponentDelta>>)
    where
        C: Component + NetComponent + 'static,
    {
        let id = self.component_id::<C>();
        let networked = world.read_storage::<Networked>();
        let mut cs = world.write_storage::<C>();
        for (Networked { entity_id, .. }, ref mut c) in (&networked, &mut cs).join() {
            if !source.contains_key(entity_id) {
                continue;
            }
            if let Some(store) = source.get(entity_id).unwrap().iter().find(|s| s.0 == id) {
                c.write_delta(&store.1);
            }
        }
    }

    fn component_id<COMPONENT: NetComponent + 'static>(&self) -> NetComponentIndex {
        *self
            .index
            .get(&TypeId::of::<COMPONENT>())
            .expect("component not registered")
    }
}
