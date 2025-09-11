|     |     |     |
| --- | --- | --- |
|  | ANSI Colors |  |

|     |
| --- |

| [![MUDhalla is a website dedicated to multiplayer text adventure games, also known as MUDs.](https://mudhalla.net/mudhalla_logo.png)](https://mudhalla.net/) |

|     |
| --- |
|  |

|     |     |     |     |     |     |     |     |     |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |
|  |
| ANSI escape codes are a standard for in-band signaling to control the cursor location, color, and other options on text terminals. The ANSI x3.64 standard was released in 1979 and later updated by the 5th edition ECMA-48 standard in 1991, ISO/IEC 6429 in 1992, and ISO-8613-3 in 1995. |
|  |
| ANSI escape codes are started using \\e\[ and contain numeric codes separated by semicolons. The escape code is terminated with an 'm'. |\
|  |\
|  |\
|  |\
|  |\
|  |\
| **ANSI Color Codes** |\
|  |\
| * * * |\
|  |\
|  |\
|  |\
| ANSI Color Escape Sequences |\
|  |\
|  |\
|  |\
| |     |     |     |\
| --- | --- | --- |\
| **Code** | **Effect** | **Note** |\
| \\e \[ | CSI | Control Sequence Indicator |\
| CSI _n_ m | ANSICOLOR | ANSI color code (Select Graphic Rendition) |\
| CSI 38 ; 5 ; _n_ m | 256COLOR | Foreground 8 bit 256 color code |\
| CSI 48 ; 5 ; _n_ m | 256COLOR | Background 8 bit 256 color code |\
| CSI 38 ; 2 ; _r_ ; _g_ ; _b_ m | TRUECOLOR | Foreground 24 bit rgb color code |\
| CSI 48 ; 2 ; _r_ ; _g_ ; _b_ m | TRUECOLOR | Background 24 bit rgb color code | |\
|  |\
|  |\
|  |\
| ANSI Color Options |\
|  |\
|  |\
|  |\
| |     |     |     |\
| --- | --- | --- |\
| **Code** | **Effect** | **Note** |\
| 0 | Reset | Set color and display state to default |\
| 1 | Bold | Set color intensity to bold |\
| 2 | Faint | Set color intensity to faint |\
| 3 | Italic | Display text as italic |\
| 4 | Underline | Display text as underline |\
| 5 | Slow blink | Blink less than 150 times a minute |\
| 6 | Fast blink | Blink 150 times a minute or more |\
| 7 | Reverse video | Reverse foreground and background colors |\
| 8 | Erase | Display text as blank spaces |\
| 9 | Strikethrough | Display text as strikethrough |\
| 21 | Double underline |  |\
| 22 | Bold: off | Set color intensity to default |\
| 23 | Italic: off |  |\
| 24 | Underline: off | Neither single or double |\
| 25 | Slow blink: off |  |\
| 26 | Fast blink: off |  |\
| 27 | Reverse video: off |  |\
| 28 | Erase: off |  |\
| 29 | Strikethrough: Off |  |\
| 30 | Foreground: black |  |\
| 31 | Foreground: red |  |\
| 32 | Foreground: green |  |\
| 33 | Foreground: yellow | Faint yellow often looks brown |\
| 34 | Foreground: blue |  |\
| 35 | Foreground: magenta |  |\
| 36 | Foreground: cyan |  |\
| 37 | Foreground: white |  |\
| 39 | Foreground: reset | Reset foreground color to default |\
| 40 | Background: black |  |\
| 41 | Background: red |  |\
| 42 | Background: green |  |\
| 43 | Background: yellow |  |\
| 44 | Background: blue |  |\
| 45 | Background: magenta |  |\
| 46 | Background: cyan |  |\
| 47 | Background: white |  |\
| 49 | Background: reset | Reset backround color to default | |\
|  |\
|  |\
|  |\
| ANSI Color RGB Values |\
|  |\
|  |\
|  |\
| The 24 bit RGB values used for the 16 ANSI colors differ greatly for each terminal implementation. Below is a system that is both consistent and 12 bit compatible. |\
|  |\
|  |\
|  |\
| |     |     |     |     |     |     |     |     |     |\
| --- | --- | --- | --- | --- | --- | --- | --- | --- |\
| **Code** | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |\
| **Name** | Black | Red | Green | Yellow | Blue | Magenta | Cyan | White |\
| **Normal** | 000000 | AA0000 | 00AA00 | AAAA00 | 0000AA | AA00AA | 00AAAA | AAAAAA |\
| **Bold** | 555555 | FF5555 | 55FF55 | FFFF55 | 5555FF | FF55FF | 55FFFF | FFFFFF | |\
|  |\
|  |\
|  |\
|  |\
|  |\
| OSC Color Palette |\
|  |\
|  |\
|  |\
| The OSC color palette is an extension to the ANSI color standard using a modification of the Operating System Command sequence, documented in the Linux console manual as early as 1996. Support has been added by a handful of terminal emulators, most notably xterm. The extension allows changing the default 16 ANSI colors using a 24 bit RGB value in hexadecimal notation. |\
|  |\
| To send an OSC color palette code one must print \\e\]P followed by 1 hexadecimal number indicating one of the 16 ANSI colors, followed by 6 hexadecimal numbers to set a corresponding RGB value. Colors can be reset to their default by printing \\e\]R. The terminal's default foreground and background color cannot be changed. |
|  |
| The default blue ANSI color is hard to read on a black background. To set it to a lighter shade of blue one would print "\\e\]P43333BB", and to display the updated color one would print "\\e\[22;34m". |\
|  |\
|  |\
|  |\
|  |\
|  |\
| **Support** |\
|  |\
| ANSI color support in MUD clients is generally weak. Universally supported codes are 0 and 1, as well as code 30-37. |\
|  |\
| Terminal emulators generally support background colors as well as code 4, 5, 7, 9, and 22. |\
|  |\
|  |\
|  |\
|  |\
|  |\
| **Detection** |\
|  |\
| There is no universal standard for the detection or negotiation of ANSI color support. The following methods exist: |\
|  |\
| |     |\
| --- |\
| By using the TTYPE TELNET option and checking for a terminal emulator or mud client known to support ANSI colors, such as xterm. |\
| By using the TTYPE TELNET option and checking for the -color, -256color, or -truecolor suffix. For example: xterm-color. |\
| By using the TTYPE TELNET option and using the [MUD Terminal Type Standard](https://tintin.mudhalla.net/protocols/mtts) to detect ANSI color support. | |\
|  |\
|  |\
|  |\
|  |\
|  |\
| **Links** |\
|  |\
|  |\
|  |\
| If you want a link added, you can email me at mudclient@gmail.com. |\
|  |\
|  |\
|  |\
| **Clients supporting ANSI color** |\
|  |\
| * * * |\
|  |\
| Virtually all MUD clients and terminal emulators support ANSI color. |\
|  |\
|  |\
|  |\
| **Codebases** |\
|  |\
| * * * |\
|  |\
| [Comparison of MUD codebases](http://www.mudpedia.org/mediawiki/index.php/Comparison_of_MUD_codebases) \- Mudpedia contains a list of MUD servers with ANSI color support. |\
|  |\
|  |\
|  |\
| **Snippets** |\
|  |\
| [256 color parser](http://www.mudbytes.net/files/2596) \- Handles ANSI 16 colors and xterm 256 colors. |\
|  |\
| * * * |\
|  |\
|  |\
|  |\
|  |\
|  |
