use std::collections::HashMap;
use std::intrinsics::{type_id, type_name};
use std::convert::From;
use core::nonzero::NonZero;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ShortTypeId(NonZero<u16>);

impl ShortTypeId {
    pub fn new(id: u16) -> Option<Self> {
        NonZero::new(id).map(ShortTypeId)
    }

    pub fn as_usize(&self) -> usize {
        self.0.get() as usize
    }
}

impl From<ShortTypeId> for u16 {
    fn from(id: ShortTypeId) -> Self {
        id.0.get()
    }
}

pub struct TypeRegistry {
    next_short_id: ShortTypeId,
    long_to_short_ids: HashMap<u64, ShortTypeId>,
    short_ids_to_names: HashMap<ShortTypeId, String>,
}

impl TypeRegistry {
    pub fn new() -> TypeRegistry {
        TypeRegistry {
            next_short_id: ShortTypeId::new(1).unwrap(), // Non nullable optimization
            long_to_short_ids: HashMap::new(),
            short_ids_to_names: HashMap::new(),
        }
    }

    pub fn register_new<T: 'static>(&mut self) -> ShortTypeId {
        let short_id = self.next_short_id;
        let long_id = unsafe { type_id::<T>() };
        assert!(self.long_to_short_ids.get(&long_id).is_none());
        self.long_to_short_ids.insert(long_id, short_id);
        self.short_ids_to_names.insert(
            short_id,
            unsafe { type_name::<T>() }.into(),
        );
        self.next_short_id = ShortTypeId::new(u16::from(self.next_short_id) + 1).unwrap();
        short_id
    }

    pub fn get<T: 'static>(&self) -> ShortTypeId {
        if let Some(&short_id) = self.long_to_short_ids.get(&unsafe { type_id::<T>() }) {
            short_id
        } else {
            panic!("{:?} not known.", &unsafe { type_name::<T>() })
        }
    }

    pub fn get_or_register<T: 'static>(&mut self) -> ShortTypeId {
        self.long_to_short_ids
            .get(&unsafe { type_id::<T>() })
            .cloned()
            .unwrap_or_else(|| self.register_new::<T>())
    }

    pub fn get_name(&self, short_id: ShortTypeId) -> &String {
        &self.short_ids_to_names[&short_id]
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}
