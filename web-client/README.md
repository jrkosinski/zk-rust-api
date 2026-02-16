# ZK Merkle Tree UI

A React-based web interface for visualizing and interacting with the ZK Merkle Tree API.

## Features

- **Tree Visualization**: View a dynamically generated image of the current Merkle tree structure
- **Add Values**: Add new values to the tree and see it update in real-time
- **Poseidon Hashing**: Uses Poseidon hash function for ZK-friendly operations
- **Modern Stack**: Built with React, TypeScript, Vite, and Tailwind CSS

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- The ZK Rust API server running on `http://localhost:3000`

### Installation

```bash
yarn install
```

### Development

Start the development server:

```bash
yarn run dev
```

The application will be available at `http://localhost:5173`

### Building for Production

```bash
yarn build
```

### Preview Production Build

```bash
yarn preview
```

## Project Structure

```
web-client/
├── src/
│   ├── components/       # React components
│   │   └── TreeVisualization.tsx
│   ├── lib/             # Utilities and API client
│   │   ├── api.ts
│   │   └── queryClient.ts
│   ├── types/           # TypeScript type definitions
│   │   └── tree.ts
│   ├── App.tsx          # Main application component
│   ├── main.tsx         # Application entry point
│   └── index.css        # Global styles
├── public/              # Static assets
├── index.html           # HTML entry point
├── vite.config.ts       # Vite configuration
├── tsconfig.json        # TypeScript configuration
└── package.json         # Dependencies and scripts
```

## API Proxy

The Vite dev server proxies API requests from `/api/*` to `http://localhost:3000/*` to avoid CORS issues during development.

## Technologies

- **React 19** - UI framework
- **TypeScript** - Type safety
- **Vite** - Build tool and dev server
- **Tailwind CSS** - Styling
- **TanStack Query** - Data fetching and state management
- **React Router** - Routing (prepared for future expansion)
