use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use components::Networked;

use specs::{Component, Join, World};

pub type ClientId = u8;

pub type EntityId = u16;

#[derive(Clone, Serialize, Deserialize)]
pub struct ComponentStore(HashMap<NetComponentIndex, HashMap<EntityId, Vec<u8>>>);

#[derive(Clone, Serialize, Deserialize)]
pub struct ComponentDelta(HashMap<NetComponentIndex, HashMap<EntityId, Vec<u8>>>);

pub trait NetComponent {
    fn net_store(&self) -> Vec<u8>;
    fn net_load(&mut self, data: &[u8]);

    fn read_delta(&self) -> Vec<u8>;
    fn write_delta(&mut self, data: &[u8]);
}

type NetComponentIndex = u8;
static NET_COMPONENT_MAX: usize = std::u8::MAX as usize;

type PackerFunction = Box<
    Fn(
        &World,
        Option<&HashSet<EntityId>>,
        &Fn(&NetComponent) -> Vec<u8>,
    ) -> HashMap<EntityId, Vec<u8>>,
>;
type LoaderFunction = Box<Fn(&World, HashMap<EntityId, Vec<u8>>, &Fn(&mut NetComponent, &[u8]))>;

pub struct NetComponentAdapter {
    index: HashMap<TypeId, u8>,
    next_index: u8,

    packers: HashMap<NetComponentIndex, PackerFunction>,
    loaders: HashMap<NetComponentIndex, LoaderFunction>,
}

impl NetComponentAdapter {
    pub fn new() -> Self {
        NetComponentAdapter {
            index: HashMap::new(),
            next_index: 0,

            packers: HashMap::new(),
            loaders: HashMap::new(),
        }
    }

    pub fn register_component<C>(&mut self)
    where
        C: Component + NetComponent + 'static,
    {
        if (self.next_index as usize) + 1 > NET_COMPONENT_MAX {
            panic!("max number of net components is {}", NET_COMPONENT_MAX);
        }

        let index = self.next_index;
        self.next_index += 1;

        let type_id = TypeId::of::<C>();
        if self.index.contains_key(&type_id) {
            panic!("component already registered");
        }
        self.index.insert(type_id, self.next_index);

        self.packers.insert(
            index,
            Box::new(|world, entity_set, store_fn| {
                let networked = world.read_storage::<Networked>();
                let cs = world.read_storage::<C>();
                (&networked, &cs)
                    .join()
                    .filter_map(|(Networked { entity_id, .. }, c)| {
                        // include entities in the set, default to true if there's no set
                        let include = entity_set.map(|s| s.contains(entity_id)).unwrap_or(true);
                        if include {
                            Some((*entity_id, store_fn(c)))
                        } else {
                            None
                        }
                    })
                    .collect()
            }),
        );

        self.loaders.insert(
            index,
            Box::new(|world, pack, load_fn| {
                let networked = world.read_storage::<Networked>();
                let mut cs = world.write_storage::<C>();
                for (networked, c) in (&networked, &mut cs).join() {
                    if let Some(store) = pack.get(&networked.entity_id) {
                        load_fn(c, store);
                    }
                }
            }),
        );
    }

    pub fn net_store(
        &self,
        world: &World,
        entity_set: Option<&HashSet<EntityId>>,
    ) -> ComponentStore {
        let mut pack = HashMap::new();
        let store_fn = |c: &NetComponent| c.net_store();
        for (component_index, packer) in self.packers.iter() {
            pack.insert(*component_index, packer(world, entity_set, &store_fn));
        }
        ComponentStore(pack)
    }

    pub fn net_load(&self, world: &World, pack: ComponentStore) {
        let load_fn = |c: &mut NetComponent, data: &[u8]| c.net_load(data);
        for (component_index, store) in pack.0 {
            let loader = self
                .loaders
                .get(&component_index)
                .expect("attempt to load unregistered net component");
            loader(world, store, &load_fn);
        }
    }

    pub fn read_delta(
        &self,
        world: &World,
        entity_set: Option<&HashSet<EntityId>>,
    ) -> ComponentDelta {
        let mut pack = HashMap::new();
        let store_fn = |c: &NetComponent| c.read_delta();
        for (component_index, packer) in self.packers.iter() {
            pack.insert(*component_index, packer(world, entity_set, &store_fn));
        }
        ComponentDelta(pack)
    }

    pub fn write_delta(&self, world: &World, pack: ComponentDelta) {
        let load_fn = |c: &mut NetComponent, data: &[u8]| c.write_delta(data);
        for (component_index, delta) in pack.0 {
            let loader = self
                .loaders
                .get(&component_index)
                .expect("attempt to load unregistered net component");
            loader(world, delta, &load_fn);
        }
    }
}
