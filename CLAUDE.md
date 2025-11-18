# CLAUDE.md - AI Assistant Guide for ssh-tui-portfolio

## Project Overview

**ssh-tui-portfolio** is an interactive Terminal User Interface (TUI) portfolio application accessible via SSH. Built with Rust and the Ratatui framework, it provides an immersive, terminal-based portfolio experience showcasing developer skills, projects, and experience.

**Live Access**: `ssh tuiuser@ssh.123343.xyz -p 2222`

### Key Technologies
- **Language**: Rust (Edition 2024)
- **TUI Framework**: Ratatui 0.29.0
- **Terminal Backend**: Crossterm 0.28.1
- **Error Handling**: color-eyre 0.6.3
- **Special Libraries**: tui-big-text 0.7.1 (for large ASCII text), rand 0.9.2 (for sparkline data)

## Architecture

### Project Structure

```
ssh-tui-portfolio/
├── src/
│   ├── main.rs                    # Application entry point and main loop
│   └── screens/                   # Screen modules
│       ├── mod.rs                 # Screen module exports
│       ├── theme.rs               # Color theme constants
│       ├── intro_screen.rs        # Welcome/intro screen with ASCII art
│       ├── first_screen.rs        # Skills and expertise screen
│       ├── second_screen.rs       # Experience and projects screen
│       └── third_screen.rs        # Live telemetry/sparkline demo
├── scripts/
│   ├── setup-vps.sh              # VPS initial setup script
│   └── install-ssh-tui.sh        # Installation helper
├── .github/workflows/
│   ├── ci.yml                    # CI pipeline (fmt, clippy, doc, test)
│   └── deploy.yml                # Auto-deploy to Digital Ocean VPS
├── Cargo.toml                    # Rust dependencies
├── Cargo.lock                    # Locked dependency versions
├── README.md                     # User-facing documentation
└── LICENSE                       # MIT License

```

### Application Architecture

#### Main App Structure (`src/main.rs`)
- **State Machine**: Four states (Intro, First, Second, Third) representing different screens
- **Event Loop**: 200ms tick rate for animations and sparkline updates
- **App Struct**: Holds running state, screen state, call sign, sparkline data, and animation frame index

#### Screen Widget Pattern
All screens follow a consistent pattern:
1. Define a widget struct (e.g., `IntroScreenWidget`)
2. Implement `ratatui::widgets::Widget` trait
3. Use consistent layout with margins and sections
4. Return screen-specific content through factory functions

#### Navigation
- `n` - Next screen (Intro → First → Second → Third → First)
- `p` - Previous screen (reverse order)
- `q` or `Esc` or `Ctrl+C` - Quit application
- `[` - Set call sign to "Karneeshkar V"
- `]` - Set call sign to "Veera"

### Screen Details

#### Intro Screen (`intro_screen.rs`)
- **Purpose**: Welcome screen with animated ASCII art
- **Features**:
  - Big text hero section with name
  - Animated ASCII frames (Neovim, Linux)
  - Focus areas and toolbox cards
  - Contact information
- **Animation**: Cycles through ASCII_FRAMES array every 4 ticks (800ms per frame)

#### First Screen (`first_screen.rs`)
- **Purpose**: Technical skills and expertise showcase
- **Layout**: Two-column design
  - Left: Software development expertise list, About Me section
  - Right: Technical skills with proficiency levels, Reach Out CTA
- **Content**: Backend, AI, Cloud, Frontend expertise with tech stacks

#### Second Screen (`second_screen.rs`)
- **Purpose**: Professional experience and highlighted projects
- **Layout**: Asymmetric two-column (60/40)
  - Left: 7 professional experiences (reverse chronological)
  - Right: 5 highlighted projects with tech stacks
- **Content**: Detailed work history from 2024-2025, including 2Cents Capital, Visteon, UpWork, P&G, etc.

#### Third Screen (`third_screen.rs`)
- **Purpose**: Live telemetry demonstration with sparklines
- **Features**:
  - Three animated sparkline charts (Signal Alpha, Beta, Gamma)
  - Real-time statistics (min, max, avg, trend)
  - Composite signal metrics
  - Updates every 200ms with random data
- **Data**: Rolling 100-sample window per series

### Theme System (`theme.rs`)

Consistent color palette throughout the application:

**Backgrounds**:
- `BG_CANVAS`: RGB(9, 13, 22) - Main canvas
- `BG_HERO`: RGB(17, 24, 36) - Hero sections
- `BG_SECTION`: RGB(12, 19, 30) - Content sections
- `BG_PANEL`: RGB(18, 26, 38) - Panel backgrounds
- `BG_FOOTER`: RGB(8, 12, 20) - Footer areas

**Accents**:
- `ACCENT_TEAL`: RGB(56, 217, 169) - Primary actions
- `ACCENT_BLUE`: RGB(92, 184, 246) - Secondary elements
- `ACCENT_VIOLET`: RGB(186, 154, 255) - Tertiary highlights
- `ACCENT_GOLD`: RGB(230, 190, 92) - Key elements

**Text**:
- `FG_PRIMARY`: RGB(189, 198, 216) - Primary text
- `FG_SECONDARY`: RGB(160, 175, 197) - Secondary text
- `FG_MUTED`: RGB(111, 123, 143) - Muted text

## Development Workflows

### Building and Running

```bash
# Build debug version
cargo build

# Build release version (optimized)
cargo build --release

# Run locally
cargo run

# Run with release optimizations
cargo run --release
```

### Code Quality

The project enforces strict Rust best practices:

```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt -- --check

# Run Clippy linter
cargo clippy

# Generate documentation
cargo doc --no-deps --all-features

# Run tests
cargo test --locked --all-features --all-targets
```

### CI/CD Pipeline

#### Continuous Integration (`.github/workflows/ci.yml`)
Triggers on: PRs, pushes to main/master/develop

**Jobs**:
1. **fmt**: Checks code formatting with `rustfmt`
2. **clippy**: Lints code with Clippy and generates check annotations
3. **doc**: Generates documentation on nightly Rust
4. **test**: Runs tests on macOS and Windows

**Concurrency**: Uses GitHub concurrency groups to cancel outdated runs

#### Deployment (`.github/workflows/deploy.yml`)
Triggers on: Pushes to main, manual workflow dispatch

**Process**:
1. Checkout code
2. Install Rust stable toolchain
3. Cache Cargo dependencies
4. Build release binary (`target/release/ssh-tui`)
5. Deploy to Digital Ocean VPS via SCP
6. Restart systemd service (`ssh-tui.service`)

**Required Secrets**:
- `DO_HOST`: Digital Ocean server hostname/IP
- `DO_USERNAME`: SSH username
- `DO_SSH_KEY`: Private SSH key for deployment
- `DO_SSH_PASSPHRASE`: SSH key passphrase (if applicable)
- `DO_PORT`: SSH port

### VPS Setup

Use `scripts/setup-vps.sh` for initial server configuration:
1. Creates `sshtui` system user
2. Sets up `/opt/ssh-tui` directory
3. Creates systemd service file
4. Configures SSH with ForceCommand for portfolio user
5. Hardens security settings

**Manual Steps After Setup**:
1. Add public SSH key to `/home/portfolio/.ssh/authorized_keys`
2. Deploy binary to `/opt/ssh-tui/ssh-tui`
3. Start service: `sudo systemctl start ssh-tui`
4. Enable on boot: `sudo systemctl enable ssh-tui`

## Code Conventions

### Rust Style
- **Edition**: 2024
- **Formatting**: Follow standard `rustfmt` rules
- **Naming**: snake_case for functions/variables, PascalCase for types
- **Error Handling**: Use `color_eyre::Result` for fallible operations

### Widget Pattern
Every screen widget should:
1. Define a struct to hold widget-specific data
2. Implement `ratatui::widgets::Widget` trait
3. Provide a factory function (e.g., `intro_screen()`)
4. Use consistent layout with `Margin { horizontal: 2, vertical: 1 }`
5. Apply theme colors from `theme.rs`
6. Include graceful degradation for small terminal sizes

### Layout Best Practices
- Use `Layout::default()` with constraints for responsive design
- Apply backgrounds with `buf.set_style()` before rendering widgets
- Use `Block` with `BorderType::Rounded` for consistent borders
- Center align headers/footers, left/center align content as appropriate
- Check minimum dimensions before rendering complex layouts

### State Management
- **Screen State**: Enum-based state machine in `App`
- **Animation State**: Frame indices and tick accumulators in `App`
- **Data State**: Sparkline data maintained as rolling arrays
- **Input Handling**: Key events processed in `on_key_event()`

## Testing

### Current Test Coverage
The project currently has minimal automated tests. Testing is primarily:
- Manual testing via `cargo run`
- Visual inspection of terminal output
- CI validation on macOS and Windows

### Testing Recommendations for AI Assistants
When making changes:
1. **Build Test**: Always run `cargo build` to verify compilation
2. **Format Check**: Run `cargo fmt -- --check` before committing
3. **Lint**: Run `cargo clippy` to catch common issues
4. **Visual Test**: Run `cargo run` and navigate through all screens
5. **Responsive Test**: Test with different terminal sizes (resize window)
6. **Navigation Test**: Verify all keybindings work (n, p, q, [, ])

## Common Modification Scenarios

### Adding a New Screen
1. Create new file: `src/screens/new_screen.rs`
2. Add to `src/screens/mod.rs`: `pub mod new_screen;`
3. Add state variant in `main.rs`: `enum State { ..., NewScreen }`
4. Add to `ScreenWidget` enum and match implementations
5. Update navigation logic in `next_screen()` and `previous_screen()`
6. Create widget struct and implement `Widget` trait
7. Test navigation flow

### Modifying Content
- **Personal Info**: Update in `first_screen.rs` and `second_screen.rs`
- **Experience**: Modify the `experience_items` vec in `second_screen.rs:77`
- **Projects**: Modify the `project_items` vec in `second_screen.rs:225`
- **Skills**: Modify the `skill_items` vec in `first_screen.rs:167`
- **ASCII Art**: Update `ASCII_FRAMES` in `intro_screen.rs:21`

### Changing Theme
Modify color constants in `src/screens/theme.rs`. Colors are RGB tuples wrapped in `Color::Rgb()`.

### Adjusting Animation Speed
- **Intro Animation**: Change tick threshold in `main.rs:203` (currently 4 ticks = 800ms)
- **Sparkline Update**: Change tick_rate in `main.rs:76` (currently 200ms)

### Deployment Changes
- **VPS Configuration**: Edit `scripts/setup-vps.sh`
- **Systemd Service**: Modify service definition in setup script
- **CI/CD**: Update `.github/workflows/deploy.yml` for deployment steps

## Important Context for AI Assistants

### When Modifying This Project

1. **Preserve the Color Theme**: The dark theme with teal/blue/violet/gold accents is intentional. Maintain consistency with `theme.rs`.

2. **Respect Layout Constraints**: Screens are designed for full-screen terminals. Keep minimum dimension checks (usually 40-50 width, 12-15 height).

3. **Maintain Navigation Flow**: The circular navigation (Intro → First → Second → Third → First) is intentional. Don't break the state machine.

4. **Keep Performance in Mind**: 200ms tick rate means the render loop runs 5 times per second. Avoid heavy computation in the render path.

5. **Preserve ASCII Art**: The ASCII frames in intro_screen.rs are carefully crafted. If modifying, test visual alignment.

6. **SSH Security**: The VPS setup includes security hardening (NoNewPrivileges, ProtectSystem, etc.). Don't weaken these settings.

7. **Personal Information**: This is a personal portfolio. Content changes should maintain professional tone and accuracy.

8. **Dependencies**: Keep dependencies minimal. This is a simple TUI app that should compile quickly.

### Known Limitations

- **No Tests**: Project lacks automated unit/integration tests
- **No Configuration File**: All content is hardcoded (intentional for simplicity)
- **Single Binary**: Entire app is a single binary with no external assets
- **Fixed Animation**: Intro animation has only 2 frames (can be expanded)
- **Sparkline Data**: Third screen uses random data (not real metrics)

### Git Workflow

- **Main Branch**: Production branch, auto-deploys to VPS
- **Feature Branches**: Use `claude/` prefix for AI-generated branches
- **Commits**: Use conventional commits (fix:, feat:, docs:, etc.)
- **CI Required**: All PRs must pass fmt, clippy, and doc checks

### Deployment Architecture

```
GitHub Actions (on push to main)
    ↓
Build Rust binary (cargo build --release)
    ↓
SCP to Digital Ocean VPS (/opt/ssh-tui/)
    ↓
Restart systemd service (ssh-tui.service)
    ↓
Live at: ssh.123343.xyz:2222
```

### Contact and Collaboration

- **Developer**: Karneeshkar V
- **Email**: karneeshkar68@gmail.com / karneeshkar01@gmail.com
- **GitHub**: github.com/KarneeshkarV
- **LinkedIn**: linkedin.com/in/karneeshkar-velmurugan/
- **License**: MIT

## Quick Reference

### File Locations
| Purpose | File Path |
|---------|-----------|
| Main application logic | `src/main.rs` |
| Screen modules | `src/screens/*.rs` |
| Theme colors | `src/screens/theme.rs` |
| Dependencies | `Cargo.toml` |
| CI pipeline | `.github/workflows/ci.yml` |
| Deployment | `.github/workflows/deploy.yml` |
| VPS setup | `scripts/setup-vps.sh` |

### Key Constants
| Constant | Value | Location |
|----------|-------|----------|
| Tick rate | 200ms | `main.rs:76` |
| Animation speed | 4 ticks (800ms) | `main.rs:203` |
| Sparkline history | 100 samples | `main.rs:64-67` |
| Primary call sign | "Karneeshkar V" | `main.rs:56` |
| Secondary call sign | "Veera" | `main.rs:57` |

### Commands
```bash
# Development
cargo fmt              # Format code
cargo clippy           # Lint code
cargo build            # Debug build
cargo run              # Run debug
cargo build --release  # Production build

# Deployment
./scripts/setup-vps.sh           # Initial VPS setup
scp target/release/ssh-tui ...   # Manual deploy
sudo systemctl restart ssh-tui   # Restart service
ssh tuiuser@ssh.123343.xyz -p 2222  # Test connection
```

---

**Last Updated**: 2025-11-18
**For**: AI assistants (Claude, etc.)
**Status**: Active development
