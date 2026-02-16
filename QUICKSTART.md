# ZK Rust API - Quick Start Guide

Complete implementation of Merkle Tree visualization with API backend and web frontend.

## Architecture

```
zk-rust-api/
├── src/                          # Rust API backend
│   ├── controllers/
│   │   └── merkle_tree_controller.rs   # POST /tree, GET /tree/visualize
│   ├── services/
│   │   ├── merkle_tree.rs              # Core tree implementation
│   │   └── merkle_tree_service.rs      # Tree service + visualization
│   └── main.rs
├── static/                       # Generated tree images
└── web-client/                   # React frontend
    ├── src/
    │   ├── components/
    │   │   └── TreeVisualization.tsx
    │   ├── lib/
    │   │   └── api.ts
    │   └── types/
    │       └── tree.ts
    └── package.json
```

## Features Implemented

### Backend (Rust)
- ✅ **POST /tree** - Add value to Merkle tree
- ✅ **GET /tree/visualize** - Generate tree visualization image
- ✅ Dynamic PNG generation using `plotters` library
- ✅ Static file serving at `/static/`
- ✅ Poseidon hash-based Merkle tree with automatic depth adjustment

### Frontend (React + TypeScript + Vite)
- ✅ Interactive UI for adding values to tree
- ✅ Real-time tree visualization display
- ✅ TanStack Query for state management
- ✅ Tailwind CSS styling
- ✅ API proxy configuration (dev mode)

## Running the Application

### 1. Start the Rust API Backend

```bash
# In the project root
cargo run
```

The API will be available at `http://localhost:3000`

**Endpoints:**
- `POST /tree` - Add value: `{"value": 99}`
- `GET /tree/visualize` - Get visualization: `{"image_url": "/static/tree_xxx.png"}`
- `GET /static/*` - Serve generated images

### 2. Start the Web Frontend

```bash
# In a new terminal
cd web-client
npm install    # First time only
npm run dev
```

The UI will be available at `http://localhost:5173`

## Usage

1. Open your browser to `http://localhost:5173`
2. Enter a number in the input field
3. Click "Add Value" to add it to the tree
4. The tree visualization updates automatically
5. Click "Refresh" to regenerate the visualization

## API Examples

### Add a value to the tree
```bash
curl -X POST http://localhost:3000/tree \
  -H "Content-Type: application/json" \
  -d '{"value": 99}'
```

Response:
```json
{
  "data": "0x05469c7b9655f637c45e3a83039577615706673d900954ab789141df783796dc"
}
```

### Generate tree visualization
```bash
curl http://localhost:3000/tree/visualize
```

Response:
```json
{
  "image_url": "/static/tree_1771085735735.png"
}
```

### View the generated image
```bash
open http://localhost:3000/static/tree_1771085735735.png
```

## Technology Stack

### Backend
- **Rust** with Axum web framework
- **Halo2** for ZK primitives (Poseidon hash)
- **Plotters** for image generation
- **Tower HTTP** for static file serving

### Frontend
- **React 19** with TypeScript
- **Vite** build tool
- **Tailwind CSS** for styling
- **TanStack Query** for data fetching
- **React Router** (ready for expansion)

## Development Notes

### Tree Visualization
- Nodes shown as blue circles
- Hash values truncated to 12 characters for readability
- Lines connect parent-child relationships
- Image dimensions scale with tree size
- Default tree starts with 8 leaves: [10, 20, 30, 40, 50, 60, 70, 80]

### Image Storage
- Images saved in `static/` directory
- Unique timestamp-based filenames
- No automatic cleanup (consider adding cron job)

### API Proxy (Development)
The Vite dev server proxies `/api/*` to `http://localhost:3000/*` to avoid CORS issues.

## Building for Production

### Backend
```bash
cargo build --release
./target/release/zk-rust-api
```

### Frontend
```bash
cd web-client
npm run build
# Serve the dist/ folder with any static server
```

## Next Steps

Potential enhancements:
- [ ] Add authentication
- [ ] Implement proof generation UI
- [ ] Add proof verification endpoint
- [ ] Display tree statistics (depth, node count)
- [ ] Export tree as JSON
- [ ] SVG export option
- [ ] Dark mode toggle
- [ ] Image cleanup service
- [ ] WebSocket updates for real-time collaboration
