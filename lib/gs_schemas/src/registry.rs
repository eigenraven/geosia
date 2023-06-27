use std::fmt::{Display, Formatter};
use std::num::{NonZeroU32, TryFromIntError};

use anyhow::{bail, Context, Result};
use hashbrown::{Equivalent, HashMap};
use kstring::{KString, KStringRef};
use serde::{Deserialize, Serialize};

pub static GEOSIA_REGISTRY_DOMAIN: &str = "gs";
pub static GEOSIA_REGISTRY_DOMAIN_KS: KString = KString::from_static(GEOSIA_REGISTRY_DOMAIN);

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct RegistryName {
    pub ns: KString,
    pub key: KString,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Default, Hash)]
pub struct RegistryNameRef<'n> {
    pub ns: KStringRef<'n>,
    pub key: KStringRef<'n>,
}

impl RegistryName {
    pub fn geosia(key: impl Into<KString>) -> Self {
        Self {
            ns: GEOSIA_REGISTRY_DOMAIN_KS.clone(),
            key: key.into(),
        }
    }

    pub fn as_ref(&self) -> RegistryNameRef {
        self.into()
    }
}

impl<'a> RegistryNameRef<'a> {
    pub fn geosia(key: impl Into<KStringRef<'a>>) -> Self {
        Self {
            ns: KStringRef::from(&GEOSIA_REGISTRY_DOMAIN_KS),
            key: key.into(),
        }
    }

    pub fn to_owned(&self) -> RegistryName {
        self.into()
    }
}

impl<'a> Equivalent<RegistryName> for RegistryNameRef<'a> {
    fn equivalent(&self, key: &RegistryName) -> bool {
        key.as_ref() == *self
    }
}

impl<'a> Equivalent<RegistryNameRef<'a>> for RegistryName {
    fn equivalent(&self, key: &RegistryNameRef) -> bool {
        *key == self.as_ref()
    }
}

impl<'a> From<&'a RegistryName> for RegistryNameRef<'a> {
    fn from(value: &'a RegistryName) -> Self {
        RegistryNameRef {
            ns: value.ns.as_ref(),
            key: value.key.as_ref(),
        }
    }
}

impl<'a> From<&RegistryNameRef<'a>> for RegistryName {
    fn from(value: &RegistryNameRef<'a>) -> Self {
        RegistryName {
            ns: value.ns.into(),
            key: value.key.into(),
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct RegistryId(pub NonZeroU32);

impl Display for RegistryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl TryFrom<u32> for RegistryId {
    type Error = TryFromIntError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self(NonZeroU32::try_from(value)?))
    }
}

impl Display for RegistryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ns, self.key)
    }
}

impl<'a> Display for RegistryNameRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ns, self.key)
    }
}

pub trait RegistryObject {
    /// Should be trivial
    fn registry_name(&self) -> RegistryNameRef;
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
        let name = object.registry_name().to_owned();
        if self.name_to_id.contains_key(&name) {
            bail!("Object {} already exists", name);
        }
        self.id_to_obj[raw_id] = Some(object);
        self.name_to_id.insert(name, id);
        Ok(id)
    }

    pub fn insert_object_with_id(&mut self, id: RegistryId, object: Object) -> Result<()> {
        let raw_id = id.0.get() as usize;
        if self.id_to_obj.len() <= raw_id {
            self.id_to_obj.resize_with(raw_id + 32, || None);
        } else if let Some(obj) = self.id_to_obj[raw_id].as_ref() {
            bail!("Object with ID {} already exists: {}", id, obj.registry_name());
        }
        let name = object.registry_name().to_owned();
        if self.name_to_id.contains_key(&name) {
            bail!("Object {} already exists", name);
        }
        if id.0.get() >= self.next_free_id {
            self.next_free_id = id.0.get() + 1;
        }
        self.id_to_obj[raw_id] = Some(object);
        self.name_to_id.insert(name, id);
        Ok(())
    }

    pub fn lookup_name_to_object(&self, name: RegistryNameRef) -> Option<(RegistryId, &Object)> {
        let id = *self.name_to_id.get(&name)?;
        let obj = self.id_to_obj.get(id.0.get() as usize)?.as_ref()?;
        Some((id, obj))
    }

    pub fn lookup_id_to_object(&self, id: RegistryId) -> Option<&Object> {
        self.id_to_obj.get(id.0.get() as usize)?.as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, Eq, PartialEq, Debug, Default)]
    struct DummyObject(RegistryName);

    impl RegistryObject for DummyObject {
        fn registry_name(&self) -> RegistryNameRef {
            self.0.as_ref()
        }
    }

    #[test]
    pub fn simple_registry() {
        let mut reg: Registry<DummyObject> = Registry::default();
        let a_id = reg.push_object(DummyObject(RegistryName::geosia("a"))).unwrap();
        assert_eq!(a_id.0.get(), 1);
        let b_id = RegistryId::try_from(2).unwrap();
        let c_id = RegistryId::try_from(3).unwrap(); // non-existent
        reg.insert_object_with_id(b_id, DummyObject(RegistryName::geosia("b")))
            .unwrap();
        assert!(reg.push_object(DummyObject(RegistryName::geosia("a"))).is_err());
        assert!(reg.push_object(DummyObject(RegistryName::geosia("b"))).is_err());
        assert!(reg
            .insert_object_with_id(b_id, DummyObject(RegistryName::geosia("new")))
            .is_err());
        assert!(reg
            .insert_object_with_id(c_id, DummyObject(RegistryName::geosia("b")))
            .is_err());

        assert_eq!(reg.lookup_id_to_object(a_id).map(|o| o.0.key.as_str()), Some("a"));
        assert_eq!(reg.lookup_id_to_object(b_id).map(|o| o.0.key.as_str()), Some("b"));
        assert_eq!(reg.lookup_id_to_object(c_id).map(|o| o.0.key.as_str()), None);

        let dyn_a = KString::from_string(String::from("a"));
        let dyn_b = KString::from_string(String::from("b"));
        let dyn_c = KString::from_string(String::from("c"));

        assert_eq!(
            reg.lookup_name_to_object(RegistryNameRef::geosia(&dyn_a))
                .map(|(id, o)| (id, o.0.key.as_str())),
            Some((a_id, "a"))
        );
        assert_eq!(
            reg.lookup_name_to_object(RegistryNameRef::geosia(&dyn_b))
                .map(|(id, o)| (id, o.0.key.as_str())),
            Some((b_id, "b"))
        );
        assert_eq!(
            reg.lookup_name_to_object(RegistryNameRef::geosia(&dyn_c))
                .map(|(id, o)| (id, o.0.key.as_str())),
            None
        );
    }
}
