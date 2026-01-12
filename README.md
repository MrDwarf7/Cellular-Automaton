# Cellular Ecosystem Simulation

A complex cellular automaton with 37+ cell types written entirely in Rust using Yew for the frontend and Actix-web for the backend.

## Prerequisites

- Rust 1.70+ ([Install](https://rustup.rs/))
- Trunk for building WASM ([Install](https://trunkrs.io/))

```bash
cargo install trunk
```

## Development

### Build Web Frontend

```bash
cargo install trunk
trunk serve  # Runs dev server at http://localhost:8080
```

### Build and Run Server

```bash
cargo run --bin server
```

### Full Development Setup

Terminal 1 - Web frontend:
```bash
trunk serve
```

Terminal 2 - Backend server:
```bash
cargo run --bin server
```

The server will be available at `http://localhost:3000`

## Building for Production

```bash
# Build web assets
trunk build --release

# Build server binary
cargo build --release

# Run server
cargo run --release --bin server
```

Production server will be at `http://localhost:3000`

## Project Structure

```
src/
├── main.rs              # Entry point (Yew app + server selection)
├── web/
│   ├── app.rs          # Main Yew component
│   ├── components/     # UI components
│   └── ws_handler.rs   # WebSocket communication
├── bin/
│   └── server.rs       # Actix-web server
├── lib.rs              # NAPI bindings (if using native modules)
├── cell.rs             # Cell type definitions
├── grid.rs             # Grid simulation
├── rules.rs            # Cellular automaton rules
├── stats.rs            # Ecosystem statistics
└── ...                 # Other simulation modules
```

## Features

- Real-time cellular automaton simulation
- 37+ cell types with complex interactions
- Multiple visualization modes (normal, heatmap, density)
- WebSocket-based client-server communication
- Adjustable simulation speed
- Preset ecosystem configurations
- Population statistics tracking

## API Endpoints

- `GET /api/health` - Health check
- `WebSocket /` - Main simulation communication

## Notes

- No longer using Node.js, Express, TypeScript, or webpack
- Pure Rust + Yew frontend
- Actix-web backend
- WASM target for browser compatibility
