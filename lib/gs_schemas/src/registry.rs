use std::fmt::{Display, Formatter};
use std::num::NonZeroU32;

use anyhow::{bail, Context, Result};
use hashbrown::HashMap;
use kstring::KString;
use serde::{Deserialize, Serialize};

pub static GEOSIA_REGISTRY_DOMAIN: &'static str = "gs";
pub static GEOSIA_REGISTRY_DOMAIN_KS: KString = KString::from_static(GEOSIA_REGISTRY_DOMAIN);

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct RegistryName {
    pub ns: KString,
    pub key: KString,
}

impl RegistryName {
    pub fn geosia(key: impl Into<KString>) -> Self {
        Self {
            ns: GEOSIA_REGISTRY_DOMAIN_KS.clone(),
            key: key.into(),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct RegistryId(pub NonZeroU32);

impl Display for RegistryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ns, self.key)
    }
}

pub trait RegistryObject {
    fn registry_name(&self) -> &RegistryName;
}

#[derive(Serialize, Deserialize)]
pub struct Registry<Object: RegistryObject> {
    next_free_id: u32,
    id_to_obj: Vec<Option<Object>>,
    name_to_id: HashMap<RegistryName, RegistryId>,
}

impl<Object: RegistryObject> Default for Registry<Object> {
    fn default() -> Self {
        Self {
            next_free_id: 1,
            id_to_obj: vec![None],
            name_to_id: HashMap::with_capacity(64),
        }
    }
}

impl<Object: RegistryObject> Registry<Object> {
    pub fn allocate_id(&mut self) -> Result<RegistryId> {
        let id = self.next_free_id;
        self.next_free_id = self
            .next_free_id
            .checked_add(1)
            .context("Maximum registry entries count exceeded")?;
        Ok(RegistryId(id.try_into()?))
    }

    pub fn push_object(&mut self, object: Object) -> Result<RegistryId> {
        let id = self.allocate_id()?;
        let raw_id = id.0.get() as usize;
        if self.id_to_obj.len() <= raw_id {
            self.id_to_obj.resize_with(raw_id + 32, || None);
        } else if self.id_to_obj[raw_id].is_some() {
            panic!(
                "Freshly allocated ID {:?} already used when trying to allocate for {}",
                id,
                object.registry_name()
            );
        }
        let name = object.registry_name().clone();
        if self.name_to_id.contains_key(&name) {
            bail!("Object {} already exists", name);
        }
        self.id_to_obj[raw_id] = Some(object);
        self.name_to_id.insert(name, id);
        Ok(id)
    }
}
