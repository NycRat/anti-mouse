# Anti-Mouse

## About

Application for using the mouse cursor, without the mouse.

Currently working on vim motions, but other keybinds will be supported.

Note: This project is VERY early in development, more improvements will be added
to make the application more practical to use.

## Support

Currently only supports MacOS, but should support Linux and Windows in the near
future. Again, this project is very early in development.

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
