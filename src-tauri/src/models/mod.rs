#[macro_use]
pub mod id_type_macro;
pub mod repos;
pub mod user;

use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;
use std::ops::{Deref, DerefMut};

type BaseIdType = u64;

id_type!(UserId, RepositoryId);
