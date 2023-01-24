use crate::database::{
    Column,
    Database,
};
use fuel_core_storage::{
    tables::FuelBlockRoots,
    Error as StorageError,
    Mappable,
    Result as StorageResult,
    StorageInspect,
    StorageMutate,
};
use fuel_core_types::blockchain::primitives::BlockHeight;
use serde::{
    de::DeserializeOwned,
    Serialize,
};
use std::borrow::Cow;

impl DatabaseColumn for FuelBlockRoots {
    fn column() -> Column {
        Column::BlockHeaderMerkle
    }
}

/// The table has a corresponding column in the database.
trait DatabaseColumn {
    /// The column of the table.
    fn column() -> Column;
}

impl<T> StorageInspect<T> for Database
where
    T: Mappable + DatabaseColumn,
    T::Key: ToDatabaseKey,
    T::OwnedValue: DeserializeOwned,
{
    type Error = StorageError;

    fn get(&self, key: &T::Key) -> StorageResult<Option<Cow<T::OwnedValue>>> {
        self.get(key.database_key().as_ref(), T::column())
            .map_err(Into::into)
    }

    fn contains_key(&self, key: &T::Key) -> StorageResult<bool> {
        self.exists(key.database_key().as_ref(), T::column())
            .map_err(Into::into)
    }
}

impl<T> StorageMutate<T> for Database
where
    T: Mappable + DatabaseColumn,
    T::Key: ToDatabaseKey,
    T::Value: Serialize,
    T::OwnedValue: DeserializeOwned,
{
    fn insert(
        &mut self,
        key: &T::Key,
        value: &T::Value,
    ) -> StorageResult<Option<T::OwnedValue>> {
        Database::insert(self, key.database_key().as_ref(), T::column(), value)
            .map_err(Into::into)
    }

    fn remove(&mut self, key: &T::Key) -> StorageResult<Option<T::OwnedValue>> {
        Database::remove(self, key.database_key().as_ref(), T::column())
            .map_err(Into::into)
    }
}

// TODO: Implement this trait for all keys.
//  -> After replace all common implementation with blanket, if possible.
/// Some keys requires pre-processing that could change their type.
pub trait ToDatabaseKey {
    /// A new type of prepared database key that can be converted into bytes.
    type Type: AsRef<[u8]>;

    /// Coverts the key into database key that supports byte presentation.
    fn database_key(&self) -> Self::Type;
}

impl ToDatabaseKey for BlockHeight {
    type Type = [u8; 4];

    fn database_key(&self) -> Self::Type {
        self.to_bytes()
    }
}
