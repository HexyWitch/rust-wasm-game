use std::collections::HashMap;

use failure::Error;
use std::any::TypeId;

use specs::{Entity, World};

use prefab::Prefab;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PrefabIndex(u8);
static MAX_PREFAB_TYPE: usize = std::u8::MAX as usize;

pub struct Registry {
    prefabs: HashMap<TypeId, u8>,
    loaders: Vec<Box<Fn(&mut World) -> Result<Entity, Error>>>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            prefabs: HashMap::new(),
            loaders: Vec::new(),
        }
    }

    pub fn register_prefab<T: Prefab + 'static>(&mut self) {
        if self.loaders.len() > MAX_PREFAB_TYPE {
            panic!("max number of prefabs is {}", MAX_PREFAB_TYPE);
        }
        self.prefabs
            .insert(TypeId::of::<T>(), self.loaders.len() as u8);
        self.loaders.push(Box::new(T::create));
    }

    pub fn create<T: Prefab + 'static>(
        &self,
        world: &mut World,
    ) -> Result<(Entity, PrefabIndex), Error> {
        let prefab = *self
            .prefabs
            .get(&TypeId::of::<T>())
            .expect("prefab not registered");
        let e = T::create(world)?;
        Ok((e, PrefabIndex(prefab)))
    }

    pub fn instantiate(&self, world: &mut World, prefab: PrefabIndex) -> Result<Entity, Error> {
        let loader = self
            .loaders
            .get(prefab.0 as usize)
            .ok_or_else(|| format_err!("attempt to load unregistered prefab"))?;
        loader(world)
    }
}
