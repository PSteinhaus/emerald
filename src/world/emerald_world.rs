use crate::world::physics::*;

use hecs::*;

pub struct EmeraldWorld {
    pub(crate) physics_engine: PhysicsEngine,
    pub(crate) inner: World,
}
impl EmeraldWorld {
    pub fn new() -> Self {
        EmeraldWorld {
            physics_engine: PhysicsEngine::new(),
            inner: World::default(),
        }
    }

    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.inner.spawn(components)
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.inner.despawn(entity)
    }

    pub fn query<Q: Query>(&self) -> QueryBorrow<'_, Q> {
        self.inner.query::<Q>()
    }

    pub fn get_mut<T: Component>(&self, entity: Entity) -> Result<RefMut<'_, T>, ComponentError> {
        self.inner.get_mut::<T>(entity)
    }

    pub fn get<T: Component>(&self, entity: Entity) -> Result<Ref<'_, T>, ComponentError> {
        self.inner.get::<T>(entity)
    }
    pub fn insert(
        &mut self,
        entity: Entity,
        components: impl DynamicBundle,
    ) -> Result<(), NoSuchEntity> {
        self.inner.insert(entity, components)
    }

    pub fn physics(&mut self) -> PhysicsHandler {
        PhysicsHandler::new(&mut self.physics_engine, &mut self.inner)
    }
}