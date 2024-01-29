# Rugby
Game Boy and Game Boy Color emulator written in Rust

## Gallery

# Installation and usage:
### Build youself
- Clone the repository, go to the `rugby_desktop` directory and run
```
cd rugby/rugby_desktop
cargo build --release
```
The executable will be show in the subfolder `target`.

# Usage
 You can run the executable without any arguments or pass the ROM's name as the first argument:
```
# Both ways are valid
./rugby_desktop
./rugby_desktop ROM.gb 
```

# Features
- Both Game Boy and Game Boy Color have been implemented.
- ROM Only, MBC1, MBC2, MBC3, and MBC5 cartridges are supported.
- Real-time clock (RTC) on supported cartridges.
- Full sound support, with the ability to enable/disable individual sound channels
- Pause/unpause whenever you want.
- Integer scaling from x1 to x5.
- Custom palettes.
- Enable/disable individual display layers.
- Multiple save states.
- Rewind for up to 5 seconds.
- And a simple UI to show all of the above!

# Controls
| Key | Action |
| ---| --- |
|`UP` |Up|
|`DOWN` |Down|
|`LEFT`|Left|
|`RIGHT`| Right|
|`A`| A|
|`S`| B|
|`Z`| Start|
|`X`| Select|
|`O`| Save state|
|`P`| Load state|
|`I`| Load last save state|
|`R`| Rewind
|`ESC` | Exit |

# Resources
### Documentation
- [The Ultimate Game Boy talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&feature=youtu.be)
- [Pandocs](https://gbdev.io/pandocs/About.html): Comprehensive technical reference
- [GBEDG](https://hacktix.github.io/GBEDG/): Useful explanations on how the PPU works
- [gbops](https://izik1.github.io/gbops/index.html): Opcodes table
- [Explanation on the DAA instruction](https://ehaskins.com/2018-01-30%20Z80%20DAA/)

### Other great Game Boy emulators that helped me
- [BGB](https://bgb.bircd.org/), used mainly for debugging
- [Argentum](https://github.com/NightShade256/Argentum)
- [kevboy](https://github.com/xkevio/kevboy), especially for the parts regarding audio
- [rboy](https://github.com/mvdnes/rboy),

# License
Rugby is licensed under the (MIT license)[https://choosealicense.com/licenses/mit/]
