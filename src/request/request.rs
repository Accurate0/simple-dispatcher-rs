use std::any::{Any, TypeId};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait BaseRequest {
    fn eq(&self, other: &dyn BaseRequest) -> bool;
    fn hash(&self) -> u64;
    fn as_any(&self) -> &dyn Any;
}

impl<T: PartialEq + Hash + 'static + Send + Sync> BaseRequest for T {
    fn eq(&self, other: &dyn BaseRequest) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            return self == other;
        }
        false
    }

    fn hash(&self) -> u64 {
        let mut h = DefaultHasher::new();
        Hash::hash(&(TypeId::of::<T>(), self), &mut h);
        h.finish()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for Box<dyn BaseRequest + Send + Sync> {
    fn eq(&self, other: &Self) -> bool {
        BaseRequest::eq(self.as_ref(), other.as_ref())
    }
}

impl Eq for Box<dyn BaseRequest + Send + Sync> {}

impl Hash for Box<dyn BaseRequest + Send + Sync> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let key_hash = BaseRequest::hash(self.as_ref());
        state.write_u64(key_hash);
    }
}
