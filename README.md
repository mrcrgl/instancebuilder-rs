# instancebuilder

Handy solution for dependency injection in Rust. 

## Installation

```bash
cargo add instancebuilder
```

## Examples

### Simple

```rust
use ::std::convert::Infallible;
use ::instancebuilder::{Error, InstanceBuilder, FromInstanceBuilder};

// Example set of data of any type. Must implement Send + Sync to be threadsafe.
struct TestConfig {
    key: String,
}

// Your implementation that needs to get build.
struct TestImplementation {
    message: String,
}

impl FromInstanceBuilder for TestImplementation {
    fn try_from_builder(builder: &InstanceBuilder) -> Result<Self, Error> {
        // Put here the code necessary to build the instance
        let config: &TestConfig = builder.data()?;

        Ok(Self {
            message: config.key.clone(),
        })
    }
}

fn main() {
    // Test object to inject. This can be a database pool or a shared instance of something
    let config = TestConfig {
        key: String::from("help me!"),
    };

    let mut  builder = InstanceBuilder::new();
    // Add dependency object to the builder
    builder.insert(config);

    // Build instance of the implementation
    // dependencies are injected by the `FromInstanceBuilder` trait implementation
    let instance = builder.build::<TestImplementation>().unwrap();
}
```

### Nested dependencies

```rust
use ::std::convert::Infallible;
use ::instancebuilder::{Error, InstanceBuilder, FromInstanceBuilder};

// Example set of data of any type. Must implement Send + Sync to be threadsafe.
struct TestConfig {
    key: String,
}

// Nested dependent struct
struct InnerTestImplementation {
    message: String,
}

impl FromInstanceBuilder for InnerTestImplementation {
    fn try_from_builder(builder: &InstanceBuilder) -> Result<Self, Error> {
        // Put here the code necessary to build the instance
        // the builder instance contains the initialized data
        let config: &TestConfig = builder.data()?;
        
        Ok(Self {
            message: config.key.clone(),
        })
    }
}

// Outer struct that depends on the nested one
struct OuterTestImplementation {
    inner: InnerTestImplementation,
}

impl FromInstanceBuilder for OuterTestImplementation {
    fn try_from_builder(builder: &InstanceBuilder) -> Result<Self, Error> {
        // Put here the code necessary to build the instance
        Ok(Self {
            // Builds dependency `InnerTestImplementation`
            inner: builder.build()?,
        })
    }
}

fn main() {
    // Test object to inject. This can be a database pool or a shared instance of something
    let config = TestConfig {
        key: String::from("help me!"),
    };

    let mut  builder = InstanceBuilder::new();
    // Add dependency object to the builder
    builder.insert(config);

    // Build instance of the implementation
    // dependencies are injected by the `FromInstanceBuilder` trait implementation
    let instance = builder.build::<OuterTestImplementation>().unwrap();
}

```
