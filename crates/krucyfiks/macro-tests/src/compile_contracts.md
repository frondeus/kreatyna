
These examples exercise compiler errors from real macro invocations, alongside
the parser's diagnostic tests in `krucyfiks-macros`.

Implicit unit returns need an explicit panic policy:

```compile_fail,E0277
#[framework::capability(crate = "::framework")]
trait Render {
    async fn render(&self);
}
```

Explicit unit returns have the same transport requirement:

```compile_fail,E0277
#[framework::capability(crate = "::framework")]
trait Render {
    async fn render(&self) -> ();
}
```

A non-unit, non-fallible return also requires the panic policy:

```compile_fail,E0277
#[framework::capability(crate = "::framework")]
trait Clock {
    async fn now(&self) -> u64;
}
```

Rust checks the error conversion; the macro does not recognize `Result`:

```compile_fail,E0277
struct AppError;
type Reply = Result<u64, AppError>;
#[framework::capability(crate = "::framework")]
trait Clock {
    async fn now(&self) -> Reply;
}
```

Mutable receivers belong on generated backend traits, not capability inputs:

```compile_fail
#[framework::capability(crate = "::framework")]
trait Clock {
    #[krucyfiks(panic_on_transport)]
    async fn sleep(&mut self);
}
```

Borrowed payloads, including nested references, are unsupported:

```compile_fail
#[framework::capability(crate = "::framework")]
trait Logger {
    #[krucyfiks(panic_on_transport)]
    async fn log(&self, lines: Vec<&str>);
}
```

Generic methods are outside the first version's scope:

```compile_fail
#[framework::capability(crate = "::framework")]
trait Logger {
    #[krucyfiks(panic_on_transport)]
    async fn log<T: Send>(&self, value: T);
}
```

Associated types are unsupported:

```compile_fail
#[framework::capability(crate = "::framework")]
trait Logger {
    type Error;
    async fn log(&self) -> Result<(), Self::Error>;
}
```

Unsupported policy attributes must not be silently ignored:

```compile_fail
#[framework::capability(crate = "::framework")]
trait Logger {
    #[krucyfiks(ignore_transport)]
    async fn log(&self);
}
```

Every composition variant needs one unnamed child:

```compile_fail
#[derive(framework::Effect)]
#[krucyfiks(crate = "::framework")]
enum Tree { Empty }
```

Composition generics are intentionally unsupported in the first version:

```compile_fail
#[derive(framework::Effect)]
#[krucyfiks(crate = "::framework")]
enum Tree<T> { Child(T) }
```

Deriving the tree does not grant raw enums recursive resolvers:

```compile_fail,E0599
use framework::effects::Envelope;
#[framework::capability(crate = "::framework")]
trait Render {
    #[krucyfiks(panic_on_transport)]
    async fn render(&self);
}
#[derive(framework::Effect)]
#[krucyfiks(crate = "::framework")]
enum Tree { Render(Envelope<RenderRequest, RenderResponse>) }
async fn invalid(tree: Tree) {
    tree.resolve_render(|| async {}).await.unwrap();
}
```
