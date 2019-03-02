use std::collections::HashMap;

use bincode;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::any::TypeId;

use embla::ecs::{Entity, World};

use components::Prefab as PrefabComponent;

pub trait Prefab {
    type Config: Serialize + for<'a> Deserialize<'a>;

    fn store(world: &mut World, e: Entity) -> Result<Self::Config, Error>;
    fn create(world: &mut World, config: Self::Config) -> Result<Entity, Error>;
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PrefabId(usize);

#[derive(Clone, Serialize, Deserialize)]
struct PrefabStore(usize, Vec<u8>);

pub struct Registry {
    prefabs: HashMap<TypeId, usize>,
    loaders: Vec<Box<Fn(&mut World, Vec<u8>) -> Result<Entity, Error>>>,
    storers: Vec<Box<Fn(&mut World, Entity) -> Result<Vec<u8>, Error>>>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            prefabs: HashMap::new(),
            loaders: Vec::new(),
            storers: Vec::new(),
        }
    }

    pub fn register_prefab<T: Prefab + 'static>(&mut self) {
        self.prefabs.insert(TypeId::of::<T>(), self.loaders.len());
        self.loaders.push(Box::new(
            |world: &mut World, store: Vec<u8>| -> Result<Entity, Error> {
                let config = bincode::deserialize(&store)?;
                T::create(world, config)
            },
        ));
        self.storers.push(Box::new(
            |world: &mut World, e: Entity| -> Result<Vec<u8>, Error> {
                Ok(bincode::serialize(&T::store(world, e)?)?)
            },
        ))
    }

    pub fn create<T: Prefab + 'static>(
        &self,
        world: &mut World,
        config: T::Config,
    ) -> Result<Entity, Error> {
        let id = *self
            .prefabs
            .get(&TypeId::of::<T>())
            .ok_or_else(|| format_err!("prefab not registered"))?;
        let e = T::create(world, config)?;
        world
            .entity(e)
            .unwrap()
            .insert(PrefabComponent(PrefabId(id)))?;
        Ok(e)
    }

    pub fn serialize<T: Prefab + 'static>(&self, config: &T::Config) -> Result<Vec<u8>, Error> {
        let id = self
            .prefabs
            .get(&TypeId::of::<T>())
            .ok_or_else(|| format_err!("prefab not registered"))?;
        Ok(bincode::serialize(&PrefabStore(
            *id,
            bincode::serialize(config)?,
        ))?)
    }

    pub fn load(&self, world: &mut World, store: &[u8]) -> Result<Entity, Error> {
        let store = bincode::deserialize::<PrefabStore>(store)?;
        let f = self
            .loaders
            .get(store.0)
            .ok_or_else(|| format_err!("prefab id out of range"))?;
        f(world, store.1)
    }

    #[allow(dead_code)]
    pub fn store(&self, world: &mut World, e: Entity) -> Result<Vec<u8>, Error> {
        let prefab_id = world
            .get_component::<PrefabComponent>(e)
            .ok_or_else(|| format_err!("entity is not a prefab"))?
            .0;
        let f = self.storers.get(prefab_id.0).unwrap();
        Ok(bincode::serialize(&PrefabStore(prefab_id.0, f(world, e)?))?)
    }
}
