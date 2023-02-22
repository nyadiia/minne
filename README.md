# minne
A small and simple todo list tui program in rust.

## Build
```sh
cargo build --release
```

## Current TODO
### Basic Functionality
- [X] Display (proper) boxes
- [X] List
  - [X] Arrow keys
  - [ ] Select different items
- [X] Details
  - [ ] Functionality
- [ ] Item manipulation
  - [ ] Add
  - [ ] Edit
  - [ ] Delete
- [ ] Reminders
  - [ ] Repeating reminders
- [ ] Simple priority
- [ ] Text wrapping
  
### Extended Functionality
- [ ] Output number of tasks/simple return options (for [waybar](https://github.com/Alexays/Waybar) or [polybar](https://github.com/polybar/polybar))
- [ ] Tabs of different days
- [ ] Calendar view
- [ ] Auto-priority (like in [taskwarrior](https://github.com/GothenburgBitFactory/taskwarrior))
  - Could make this a return option

### QOL/pretty :3
- [ ] Graph of total tasks completed
- [ ] Config (probably [TOML](https://github.com/toml-lang/toml))
  - [ ] Themes
  - [ ] Box themes (Rounded, double, etc.)
  - [ ] Layout (very unsure of how to do this atm)
- [ ] ~~actually understand the code i'm writing~~
