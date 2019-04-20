use failure::Error;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops::{BitAnd, Sub};

use embla::math::Vec2;
use specs::{Entity, Join, RunNow, World};

use components;
use components::{Networked, Player, Transform};
use net::{ClientId, ComponentDelta, EntityId, NetComponentAdapter};
use packets::{EntityStore, Packet};
use prefab;
use prefab::{PlayerPrefab, Prefab};
use systems::{MovementSystem, PlayerControlSystem};

pub static TIMESTEP: f64 = 1.0 / 60.0;

struct ClientInput {
    left: bool,
    right: bool,
    up: bool,
}

struct ClientData {
    outgoing: Vec<Packet>,
    known_entities: HashSet<EntityId>,
    input: ClientInput,
    player_ship: Option<Entity>,
}

pub struct GameServer {
    world: World,
    prefabs: prefab::Registry,
    entity_id: u16,
    net_adapter: NetComponentAdapter,

    clients: HashMap<ClientId, ClientData>,

    movement_system: MovementSystem,
    player_control_system: PlayerControlSystem,
}

impl GameServer {
    pub fn new() -> Result<GameServer, Error> {
        let mut world = World::new();
        let mut net_adapter = NetComponentAdapter::new();
        components::register_components(&mut world, &mut net_adapter);

        let mut prefabs = prefab::Registry::new();
        prefab::register_prefabs(&mut prefabs);

        Ok(GameServer {
            world,
            prefabs,
            entity_id: 0,
            net_adapter,

            clients: HashMap::new(),

            movement_system: MovementSystem::new(),
            player_control_system: PlayerControlSystem::new(),
        })
    }

    pub fn add_client(&mut self, client_id: ClientId) {
        self.clients.insert(
            client_id,
            ClientData {
                outgoing: Vec::new(),
                known_entities: HashSet::new(),
                input: ClientInput {
                    left: false,
                    right: false,
                    up: false,
                },
                player_ship: None,
            },
        );
    }

    pub fn handle_incoming(&mut self, client_id: ClientId, packet: &Packet) -> Result<(), Error> {
        match *packet {
            Packet::Connect => {
                let e = self.create_net_entity::<PlayerPrefab>()?;
                self.world
                    .write_storage::<Transform>()
                    .get_mut(e)
                    .unwrap()
                    .position = Vec2::new(200.0, 200.0);

                let mut client_data = self.clients.get_mut(&client_id).unwrap();
                client_data.outgoing.push(Packet::Initialize);
                client_data.player_ship = Some(e);
            }
            Packet::PlayerInput { left, right, up } => {
                self.clients.get_mut(&client_id).unwrap().input = ClientInput { left, right, up };
            }
            _ => {
                return Err(format_err!("server received unexpected packet"));
            }
        }

        Ok(())
    }

    pub fn take_outgoing(&mut self, client_id: &ClientId) -> Option<Vec<Packet>> {
        self.clients
            .get_mut(&client_id)
            .map(|c| mem::replace(&mut c.outgoing, Vec::new()))
    }

    pub fn update(&mut self, _dt: f64) -> Result<(), Error> {
        // send new entities to clients
        let entity_ids = self.entity_ids();
        for (_, mut client_data) in self.clients.iter_mut() {
            client_data.known_entities = entity_ids.bitand(&client_data.known_entities);
        }
        let client_unknowns: HashMap<ClientId, HashSet<EntityId>> = self
            .clients
            .iter()
            .map(|(client_id, client_data)| {
                (*client_id, entity_ids.sub(&client_data.known_entities))
            })
            .collect();
        for (client_id, unknown_entities) in client_unknowns {
            let entity_stores = self.store_net_entities(Some(&unknown_entities));
            let client_data = self.clients.get_mut(&client_id).unwrap();
            for store in entity_stores {
                client_data.outgoing.push(Packet::CreateEntity(store));
            }
            client_data.known_entities = entity_ids.clone();
        }

        // game logic
        {
            // set client inputs to their respective ship player components
            let mut player = self.world.write_storage::<Player>();
            for (_, client_data) in self.clients.iter() {
                if client_data.player_ship.is_none() {
                    continue;
                }

                if let Some(player) = player.get_mut(client_data.player_ship.unwrap()) {
                    player.left = client_data.input.left;
                    player.right = client_data.input.right;
                    player.up = client_data.input.up;
                }
            }
        }

        self.player_control_system.run_now(&self.world.res);
        self.movement_system.run_now(&self.world.res);

        // send new net deltas to clients
        let net_deltas = self.read_net_deltas();
        for (_, client_data) in self.clients.iter_mut() {
            client_data
                .outgoing
                .push(Packet::Update(net_deltas.clone()));
        }

        Ok(())
    }

    fn entity_ids(&self) -> HashSet<EntityId> {
        let networked = self.world.read_storage::<Networked>();
        networked.join().map(|n| n.entity_id).collect()
    }

    fn create_net_entity<P: Prefab + 'static>(&mut self) -> Result<Entity, Error> {
        // TODO: Reuse old IDs, this will eventually run out of IDs and error
        let entity_id = self.entity_id;
        self.entity_id += 1;

        let (e, prefab) = self.prefabs.create::<P>(&mut self.world)?;
        self.world
            .write_storage::<Networked>()
            .insert(e, Networked { entity_id, prefab })?;

        Ok(e)
    }

    fn store_net_entities(&mut self, index: Option<&HashSet<EntityId>>) -> Vec<EntityStore> {
        let mut components = HashMap::new();

        self.net_adapter
            .net_store::<Transform>(&self.world, &mut components, index);

        let mut stores = Vec::new();
        for &Networked { entity_id, prefab } in self.world.read_storage().join() {
            if components.contains_key(&entity_id) {
                stores.push(EntityStore {
                    entity_id,
                    prefab,
                    components: components.remove(&entity_id).unwrap(),
                });
            }
        }

        stores
    }

    fn read_net_deltas(&mut self) -> HashMap<EntityId, Vec<ComponentDelta>> {
        let mut deltas = HashMap::new();

        self.net_adapter
            .read_delta::<Transform>(&self.world, &mut deltas);

        deltas
    }
}
