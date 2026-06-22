# Event Bus 🚌

In-process typed event bus with priority and async handlers.

## Features

- **Type Safety**: Compile-time event types
- **Priority Queue**: Ordered processing
- **Async Handlers**: Tokio integration
- **Wildcard Subscriptions**: Pattern matching

## Performance

| Metric | Value |
|--------|-------|
| Publish | 2M events/s |
| Deliver | 1.8M events/s |
| Latency | <100ns |

## Quick Start

```rust
let bus = EventBus::new();
bus.subscribe(|e: UserCreated| println!("New user: {}", e.name));
bus.publish(UserCreated { name: "Alice".into() });
```

## License

MIT