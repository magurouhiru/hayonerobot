# Development

This workspace contains standalone bundles for web, desktop, and mobile platforms. Each platform runs independently without requiring a server. The workspace includes:

- **Platform crates** (`web`, `desktop`, `mobile`): Platform-specific entry points and UI
- **`ui` crate**: Shared components used across multiple platforms
- **`api` crate**: Placeholder for future shared logic (currently unused)

```
hayonerobot/
├─ README.md
├─ Cargo.toml
└─ packages/
   ├─ web/
   │  └─ ... # Web specific UI/logic (standalone web app)
   ├─ desktop/
   │  └─ ... # Desktop specific UI/logic (standalone desktop app)
   ├─ mobile/
   │  └─ ... # Mobile specific UI/logic (standalone mobile app)
   ├─ api/
   │  └─ ... # Placeholder for shared logic
   └─  ui/
      └─ ... # Components shared between multiple platforms
```

## Platform crates

Each platform crate contains the entry point for the platform, and any assets, components and dependencies that are specific to that platform. For example, the desktop crate in the workspace looks something like this:

```
desktop/ # The desktop crate contains all platform specific UI, logic and dependencies for the desktop app
├─ assets/ # Assets used by the desktop app - Any platform specific assets should go in this folder
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app. It also defines the routes for the desktop platform
│  ├─ views/ # The views each route will render in the desktop version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

When you start developing with the workspace setup each of the platform crates will look almost identical. The UI starts out exactly the same on all platforms. However, as you continue developing your application, this setup makes it easy to let the views for each platform change independently.

## Shared UI crate

The workspace contains a `ui` crate with components that are shared between multiple platforms. You should put any UI elements you want to use in multiple platforms in this crate. The `ui` crate contains:

```
ui/
├─ src/
│  ├─ lib.rs # The entrypoint for the ui crate
│  ├─ hero.rs # The Hero component that will be used in every platform
│  ├─ echo.rs # The shared echo component (client-side only)
│  ├─ navbar.rs # The Navbar component that will be used in the layout of every platform's router
```

## Architecture

This project uses a **standalone bundle architecture**. Each platform (web, desktop, mobile) is completely independent and does not require a server to run. All logic runs client-side within each application.

### Key Features

- ✅ **No server required**: Each platform runs as a standalone application
- ✅ **Independent builds**: Build and run each platform separately
- ✅ **Shared UI components**: Reuse components across platforms via the `ui` crate
- ✅ **Platform-specific customization**: Each platform can have its own routes and views

### Future Backend Integration

> **Note**: The current architecture is standalone for simplicity. When backend functionality is needed in the future, the `api` package can be used for server functions.

To add backend functionality:

1. **Update `api/Cargo.toml`**:
   ```toml
   [dependencies]
   dioxus = { workspace = true, features = ["fullstack"] }
   
   [features]
   server = ["dioxus/server"]
   ```

2. **Update platform `Cargo.toml` files** (web/desktop/mobile):
   ```toml
   [dependencies]
   dioxus = { workspace = true, features = ["router", "fullstack"] }
   ui = { workspace = true }
   
   [features]
   default = []
   web = ["dioxus/web"]  # or desktop/mobile
   server = ["dioxus/server", "ui/server"]
   ```

3. **Update `ui/Cargo.toml`**:
   ```toml
   [dependencies]
   dioxus = { workspace = true }
   api = { workspace = true }
   
   [features]
   server = ["api/server"]
   ```

4. **Add server functions to `api/src/lib.rs`**:
   ```rust
   use dioxus::prelude::*;
   
   #[post("/api/example")]
   pub async fn example(input: String) -> Result<String, ServerFnError> {
       // Server logic here
       Ok(input)
   }
   ```

5. **Call server functions from UI components**:
   ```rust
   let data = api::example(value).await?;
   ```

## Running Your App

Each platform can be built and run independently:

### Web

```bash
dx serve --package web --platform web
```

This will start a development server and open the web app in your browser.

### Desktop

```bash
dx serve --package desktop --platform desktop
```

This will launch the desktop application as a native window.

### Mobile

```bash
# For Android
dx serve --package mobile --platform android

# For iOS
dx serve --package mobile --platform ios
```

This will launch the mobile app in an emulator or on a connected device.

## Building for Production

To create production builds:

```bash
# Web
dx build --package web --platform web --release

# Desktop
dx build --package desktop --platform desktop --release

# Mobile
dx build --package mobile --platform android --release
```
