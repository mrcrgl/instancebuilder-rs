use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;
use std::fmt::Formatter;

/// InstanceBuilder offers the creation of configured instances. Due to this pattern, you can for
/// example use dependency injection in your tests without exposing those.
///
/// The object to be build implements the FromInstanceBuilder trait in its module.
///
/// ```
/// use std::convert::Infallible;
/// use ::instancebuilder::{BuilderError, InstanceBuilder, FromInstanceBuilder};
///
/// struct TestImplementation {
///     inner: String,
/// }
///
/// struct TestConfig {
///     key: String,
/// }
///
/// impl FromInstanceBuilder for TestImplementation {
///     fn try_from_builder(builder: &InstanceBuilder) -> Result<Self, BuilderError> {
///         let config: &TestConfig = builder.data()?;
///         Ok(Self {
///             inner: config.key.clone(),
///         })
///     }
/// }
///
/// let config = TestConfig {
///    key: String::from("help me!"),
/// };
///
/// let mut  builder = InstanceBuilder::new();
/// builder.insert(config);
///
/// let instance = builder.build::<TestImplementation>().unwrap();
///
/// ```
pub struct InstanceBuilder {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl InstanceBuilder {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub fn insert<D: Any + Send + Sync>(&mut self, data: D) {
        self.data.insert(TypeId::of::<D>(), Box::new(data));
    }

    pub fn data<D: Any + Send + Sync>(&self) -> Result<&D, BuilderError> {
        self.data_opt()
            .ok_or_else(|| BuilderError::DataDoesNotExist {
                ty: type_name::<D>().to_string(),
            })
    }

    pub fn data_opt<D: Any + Send + Sync>(&self) -> Option<&D> {
        self.data
            .get(&TypeId::of::<D>())
            .and_then(|d| d.downcast_ref::<D>())
    }

    pub fn build<T>(&self) -> Result<T, BuilderError>
    where
        T: FromInstanceBuilder,
    {
        T::try_from_builder(self)
    }
}

impl Default for InstanceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum BuilderError {
    DataDoesNotExist { ty: String },
    Other(String),
}

impl ::std::error::Error for BuilderError {}

impl ::std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuilderError::DataDoesNotExist { ty } => write!(f, "data of type {ty} does not exist"),
            BuilderError::Other(err) => {
                write!(f, "other error: {err}")
            }
        }
    }
}

pub trait FromInstanceBuilder: Sized {
    fn try_from_builder(builder: &InstanceBuilder) -> Result<Self, BuilderError>;
}

#[cfg(test)]
mod tests {
    use super::{BuilderError, FromInstanceBuilder, InstanceBuilder};
    use std::any::{Any, TypeId};

    struct TestImplementation {
        inner: String,
    }

    struct TestConfig {
        key: String,
    }

    impl FromInstanceBuilder for TestImplementation {
        fn try_from_builder(builder: &InstanceBuilder) -> Result<Self, BuilderError> {
            let config: &TestConfig = builder.data()?;
            Ok(Self {
                inner: config.key.clone(),
            })
        }
    }

    #[test]
    fn it_creates_new_instance_of_demanded_impl() {
        let config = TestConfig {
            key: String::from("help me!"),
        };
        let config_key = config.key.clone();
        let mut builder = InstanceBuilder::new();
        builder.insert(config);

        let instance = builder.build::<TestImplementation>().unwrap();

        assert_eq!(instance.type_id(), TypeId::of::<TestImplementation>());
        assert_eq!(instance.inner, config_key);
    }
}
