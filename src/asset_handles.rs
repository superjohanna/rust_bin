use bevy::prelude::*;

#[derive(Debug, Clone, Default, Resource)]
pub struct AssetHandles {
    pub inner: Vec<UntypedHandle>,
}

impl std::ops::Deref for AssetHandles {
    type Target = Vec<UntypedHandle>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for AssetHandles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
