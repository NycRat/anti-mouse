# Anti-Mouse

## About

Application for using the mouse cursor, without the mouse.

Currently working on vim motions, but other keybinds will be supported.

Note: This project is VERY early in development, more improvements will be added
to make the application more practical to use.

## Usage

### Motions

There are currently the basic vim motions for moving the cursor.

`H`: move left  
`J`: move down  
`K`: move up  
`L`: move right

### Speed Multiplier

Pressing `1`, `2`, `3`, or `4` will adjust the speed of the cursor.

`1` is the slowest, `4` is the fastest.

### Entering and Exiting

The `Escape` key will focus the Anti-Mouse window, preventing input to other windows.

The `I` key will unfocus the Anti-Mouse window, and stop listening for motions.

## Configuration

### Key bindings

Key bindings can be configured in the `config.json` file. The key is the action
(ex. `scroll_down, count_1`), and the value is the key code to press (ex: `E, Key1`).

The names of all the key codes can be found [_here_](https://docs.rs/device_query/latest/device_query/keymap/enum.Keycode.html).

The default config with all the keys is in the `config.json` in this repository.

**Note**: Not all the keys need to be in the config, there are defaults if a
certain key does not exist in the `config.json`
