# Orion Interface

Easy to use, lightweight interface for GOG Galaxy 2.0 integrations installation.

A desktop application built with **Tauri + Rust + Vite (React & TypeScript)** that simplifies the process of discovering, installing, and managing GOG Galaxy 2.0 community integrations.

## Features

- 🎮 Browse available GOG Galaxy 2.0 integrations
- ✅ Verify installed integration versions
- 🔄 Check for updates to installed integrations
- 🌍 Multi-language support
- 🎨 Clean, intuitive user interface

## System Requirements

### All Platforms
- **Node.js** 18+ (for development)
- **npm** or **bun** (package manager)

### Platform-Specific Dependencies

#### Linux
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev \
  libayatana-appindicator3-dev librsvg2-dev
```

#### macOS
- Xcode Command Line Tools

#### Windows
- Visual Studio Build Tools (if building from source)

## Installation

### For Users
Download the latest release for your platform from the [Releases](https://github.com/GOG-Orion/orion-interface/releases) page.

### For Developers

1. **Clone the repository**
   ```bash
   git clone https://github.com/GOG-Orion/orion-interface.git
   cd orion-interface
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```
   This will start both the Vite dev server and launch the Tauri application window.

4. **Build for production**
   ```bash
   npm run tauri build
   ```
   The built application will be in `src-tauri/target/release/`.

## Development

### Available Scripts

- `npm run dev` - Start Vite development server only (frontend)
- `npm run build` - Build frontend for production
- `npm run preview` - Preview production build
- `npm run tauri dev` - Run the full Tauri application in development mode
- `npm run tauri build` - Build the Tauri application for production

### Project Structure

```
orion-interface/
├── src/                    # Frontend React/TypeScript code
│   ├── components/         # React components
│   │   ├── utils/          # Utility components and helpers
│   │   └── assets/         # Component-specific assets
│   ├── App.tsx             # Main application component
│   └── main.tsx            # Application entry point
├── src-tauri/              # Rust backend code
│   ├── src/                # Rust source files
│   ├── icons/              # Application icons
│   └── Cargo.toml          # Rust dependencies
├── public/                 # Static assets
├── PROJECT_ANALYSIS.md     # Comprehensive project audit and recommendations
└── VERIFICATION_SUMMARY.md # Quick reference for recent changes
```

### Technology Stack

- **Frontend**: React 18, TypeScript, Vite
- **Backend**: Rust, Tauri 1.x
- **UI**: Custom CSS
- **State Management**: React Context API

## Contributing

Contributions are welcome! This is a community project - don't feel intimidated to contribute.

1. Check the [Projects](https://github.com/GOG-Orion/orion-interface/projects) tab for planned features
2. For translations, see `src/components/utils/languages.json`
3. Read [CONTRIBUTING](./CONTRIBUTING) for development guidelines
4. See [PROJECT_ANALYSIS.md](./PROJECT_ANALYSIS.md) for improvement recommendations

### Development Guidelines

- All Tauri commands must be marked with `#[tauri::command]`
- Follow the existing code style
- Test changes with `npm run tauri dev`
- Update documentation as needed

## Current Version

**0.1.7** - Alpha

See [CHANGELOG](./CHANGELOG) for detailed version history.

## Roadmap

- ✅ Basic integration browsing and verification
- ✅ Multi-language support
- 🚧 Actual integration download and installation
- 🚧 Configuration management
- 📋 Auto-update functionality
- 📋 Enhanced error handling and user feedback

## Known Issues

- Some components are still in placeholder state (Contributors, Source Code tabs)
- No automated testing infrastructure yet
- See [PROJECT_ANALYSIS.md](./PROJECT_ANALYSIS.md) for detailed improvement recommendations

## License

See [LICENSE](./LICENSE) for details.

## Credits

- Icons from [Icons8](https://icons8.com) and various open source icon libraries
- Built with [Tauri](https://tauri.app/) framework
- Community integrations from [FriendsOfGalaxy](https://github.com/FriendsOfGalaxy)

## Links

- [GOG Galaxy](https://www.gog.com/galaxy)
- [FriendsOfGalaxy Integrations](https://github.com/FriendsOfGalaxy)
- [Tauri Documentation](https://tauri.app/)

---

**Note**: This project is in active development. See [PROJECT_ANALYSIS.md](./PROJECT_ANALYSIS.md) for current status and planned improvements.