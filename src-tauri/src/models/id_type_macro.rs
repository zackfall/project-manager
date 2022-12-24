// Macro rule taken from Octocrab
macro_rules! id_type {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($($name:ident),+) => {$(
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        pub struct $name(pub BaseIdType);
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        impl Deref for $name {
            type Target = BaseIdType;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl $name {
            pub fn into_inner(self) -> BaseIdType {
                self.0
            }
        }
        impl From<BaseIdType> for $name {
            fn from(value: BaseIdType) -> Self {
                Self(value)
            }
        }
        impl AsRef<BaseIdType> for $name {
            fn as_ref(&self) -> &BaseIdType {
                &self.0
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                struct IdVisitor;
                impl<'de> de::Visitor<'de> for IdVisitor {
                    type Value = $name;
                    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                        where E: de::Error {
                        Ok($name(value))
                    }
                    fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
                        where E: de::Error {
                        id.parse::<u64>().map($name).map_err(de::Error::custom)
                    }
                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "expected {} as number or string", stringify!($name)) // TODO: $name
                    }
                }

                deserializer.deserialize_any(IdVisitor)
            }
         }
    )+};
}
