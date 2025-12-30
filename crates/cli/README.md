# Khronos CLI 🚀

A beautiful, modern Terminal User Interface (TUI) for interacting with the Khronos Lua runtime. Built with [Ratatui](https://ratatui.rs/) for a premium terminal experience.

## ✨ Features

### 🎨 Beautiful Interface
- **Modern TUI Design**: Sleek, responsive interface with smooth animations
- **Syntax Highlighting**: Color-coded Lua syntax for better readability
- **Rich Widgets**: Progress bars, status indicators, and interactive panels
- **Responsive Layout**: Adapts gracefully to any terminal size

### 🔥 Powerful REPL
- **Interactive Lua REPL**: Execute Lua code with instant feedback
- **Multi-line Editing**: Write complex scripts with ease
- **Smart Auto-completion**: Context-aware suggestions as you type
- **Command History**: Navigate through previous commands with arrow keys
- **Persistent History**: Your command history is saved between sessions

### 📊 Real-time Monitoring
- **Live Output Display**: See script output in real-time with scrolling support
- **Task Status**: Monitor running tasks and their progress
- **Performance Metrics**: View execution time and resource usage
- **Error Highlighting**: Clear, color-coded error messages

### 🛠️ Developer Tools
- **Script Execution**: Run Lua scripts from files or inline
- **Debug Mode**: Verbose logging and detailed error traces
- **File Browser**: Navigate and execute scripts from the TUI
- **Help System**: Built-in documentation and keyboard shortcuts

## 🚀 Quick Start

### Running the CLI

```bash
# Start the interactive REPL
cargo run -p khronos_cli

# Run a specific script
cargo run -p khronos_cli -- run script.luau

# Execute inline Lua code
cargo run -p khronos_cli -- exec "print('Hello, Khronos!')"

# Enable verbose mode
cargo run -p khronos_cli -- --verbose
```


## ⌨️ Keyboard Shortcuts

### Global
- `Ctrl+C` / `Ctrl+Q` - Quit the application
- `Ctrl+L` - Clear the output panel
- `F1` - Toggle help panel
- `Tab` - Cycle through panels

### REPL Input
- `Enter` - Execute current line/block
- `Shift+Enter` - New line (multi-line mode)
- `Up/Down` - Navigate command history
- `Ctrl+R` - Search command history
- `Tab` - Auto-complete
- `Ctrl+U` - Clear current line
- `Ctrl+W` - Delete previous word

### Output Panel
- `PgUp/PgDn` - Scroll output
- `Home/End` - Jump to top/bottom
- `Ctrl+F` - Search in output

## 🎯 Usage Examples

### Basic REPL Usage

```lua
-- Simple expressions
> 2 + 2
4

-- Variables and functions
> local x = 10
> function double(n) return n * 2 end
> double(x)
20

-- Multi-line code (use Shift+Enter)
> for i = 1, 5 do
>>   print(i)
>> end
1
2
3
4
5
```

### Running Scripts

```bash
# Run a single script
cargo run -p khronos_cli -- run examples/hello.luau

# Run multiple scripts in sequence
cargo run -p khronos_cli -- run script1.luau script2.luau

# Run with custom event data
cargo run -p khronos_cli -- run script.luau --event '{"name":"TestEvent","data":{}}'
```

### Advanced Features

```bash
# Enable experiments
cargo run -p khronos_cli -- --experiments image_classification

# Set memory limit (in bytes)
cargo run -p khronos_cli -- --memory-limit 104857600

# Set max threads
cargo run -p khronos_cli -- --max-threads 4

# Disable task library
cargo run -p khronos_cli -- --disable-task-lib
```

## 🏗️ Architecture

The CLI is built with a clean, modular architecture:

```
cli/
├── src/
│   ├── main.rs              # Entry point and CLI argument parsing
│   ├── cli.rs               # Core CLI logic and Lua integration
│   ├── tui/                 # Ratatui TUI components
│   │   ├── mod.rs           # TUI module exports
│   │   ├── app.rs           # Application state management
│   │   ├── ui.rs            # UI rendering logic
│   │   ├── events.rs        # Event handling system
│   │   ├── theme.rs         # Color schemes and styling
│   │   └── widgets/         # Custom widgets
│   │       ├── repl_input.rs
│   │       ├── output_panel.rs
│   │       ├── status_bar.rs
│   │       └── help_panel.rs
│   ├── cli_extensions/      # CLI-specific Lua extensions
│   ├── provider/            # Khronos context providers
│   ├── filestorage/         # File storage backends
│   └── experiments/         # Experimental features
└── README.md
```

### Design Principles

1. **Immediate Mode Rendering**: The UI is re-rendered on every frame for smooth updates
2. **Event-Driven Architecture**: User input and system events drive state changes
3. **Component-Based**: Reusable widgets and panels for maintainability
4. **Responsive Design**: Layouts adapt to terminal size using constraints
5. **Separation of Concerns**: TUI logic is separate from Lua runtime logic

## 🎨 Customization

### Widgets

Create custom widgets by implementing the `Widget` trait:

```rust
use ratatui::widgets::Widget;

struct MyWidget {
    // widget state
}

impl Widget for MyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // rendering logic
    }
}
```

## 🐛 Troubleshooting

### Terminal Issues

If the terminal appears corrupted after a crash:
```bash
reset
```

### Performance Issues

- Reduce output buffer size in config
- Disable animations in settings
- Limit history size

### Common Errors

**"Failed to create runtime"**
- Check memory limits
- Verify Lua scripts are valid
- Ensure sufficient system resources

**"Terminal too small"**
- Minimum terminal size: 80x24
- Resize your terminal window

## 📚 Additional Resources

- [Ratatui Documentation](https://ratatui.rs/)
- [Khronos Runtime Documentation](../runtime/README.md)
- [Lua 5.1 Reference Manual](https://www.lua.org/manual/5.1/)
- [Discord API Documentation](https://discord.com/developers/docs)

## 🤝 Contributing

Contributions are welcome! Please ensure:
- Code follows Rust style guidelines
- UI changes maintain the aesthetic vision
- All features are documented
- Tests pass before submitting
