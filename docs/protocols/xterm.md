|     |     |     |
| --- | --- | --- |
|  | xterm |  |

|     |
| --- |

| [![MUDhalla is a website dedicated to multiplayer text adventure games, also known as MUDs.](https://mudhalla.net/mudhalla_logo.png)](https://mudhalla.net/) |

|     |
| --- |
|  |

|     |     |     |
| --- | --- | --- |
|  |
|  |
| xterm was originally written in 1984 as a stand-alone terminal emulator for the VS100 by Mark Vandevoorde and became the default terminal for the X Window System. Around 1996 the main line of development shifted to XFree86, and xterm is currently maintained by Thomas Dickey. |
|  |
| xterm implemented several enhancements to the VT100 standard, most notably mouse tracking. |
|  |
|  |
|  |
|  |
|  |
| **XTERM Console Codes** |
|  |
| * * * |
|  |
| Control Characters |
|  |
| |     |     |     |
| --- | --- | --- |
| **Code** | **Effect** | **Note** |
| \\x07 | Bel | Ring the terminal bell |
| \\x08 | BS | Backspace |
| \\x09 | HT | Tab |
| \\x0A | LF | Linefeed |
| \\x0B | CR | Carriage return |
| \\x1B | ESC | Escape |
| \\x20 | SP | Space | |
|  |
|  |
|  |
| Escape Sequences |
|  |
| |     |     |     |
| --- | --- | --- |
| **Code** | **Effect** | **Note** |
| ESC c | RIS | Reset terminal to initial state |
| ESC 7 | DECSC | Save cursor position |
| ESC 8 | DECRC | Restore cursor position |
| ESC > | DECPNM | Set numeric keypad mode |
| ESC = | DECPAM | Set application keypad mode |
| ESC % @ | 8859-1 | Select ISO 8859-1 character set (ISO 2022). |
| ESC % G | UTF-8 | Select UTF-8 character set (ISO 2022). |
| ESC \] | OSC | Operating System Command |
| ESC \ | ST | String terminator used by OSC |
| ESC \[ | CSI | Control Sequence Introducer | |\
|  |\
|  |\
|  |\
| CSI Color Sequences |\
|  |\
| |     |     |     |\
| --- | --- | --- |\
| **Code** | **Effect** | **Note** |\
| CSI _n_ m | SGR | ANSI color code (Select Graphic Rendition) |\
| CSI 38 ; 5 ; _n_ m | 256COLOR | Foreground 256 color code |\
| ESC 48 ; 5 ; _n_ m | 256COLOR | Background 256 color code |\
| CSI 38 ; 2 ; _r_ ; _g_ ; _b_ m | TRUECOLOR | Foreground 24 bit rgb color code |\
| ESC 48 ; 2 ; _r_ ; _g_ ; _b_ m | TRUECOLOR | Background 24 bit rgb color code | |\
|  |\
|  |\
|  |\
| CSI System Sequences |\
|  |\
| |     |     |     |\
| --- | --- | --- |\
| **Code** | **Effect** | **Note** |\
| CSI c | DECID | Request device attributes |\
| CSI _n_ SP q | DECSCUSR | Set cursor style |\
| CSI _n_ ; _m_ r | DECSTBM | Set scrolling region |\
| CSI _n_ t | WINOP | Window operations |\
| CSI _n_ SP t | DECSWBV | Set Alarm Bell Volume |\
| CSI _n_ SP u | DECSMBV | Set Margin Bell Volume |\
| CSI ? _n_ h | DECSET | Enable xterm Mouse Tracking |\
| CSI ? _n_ l | DECSET | Disable xterm Mouse Tracking |\
| CSI > _n_ p | MOUSEP | Set Mouse Pointer Behavior |\
| CSI > _n_ t | TITLEM | Set Title Mode |\
| CSI _n_ ' \| | DECRQLP | DEC Mouse Tracking | |\
|  |\
|  |\
|  |\
| OSC System Sequences |\
|  |\
| |     |     |     |\
| --- | --- | --- |\
| **Code** | **Effect** | **Note** |\
| OSC _n_ ; _s_ ST | WINDOWT | Set Text Parameters |\
| OSC P _n_ | COLORP | Set color palette | |\
|  |\
| The ST (String Terminator) can be either \\e\ or \\a. You should only use \\a because the slash in \\e\ is a restricted/special character in most environments. |\
|  |\
|  |\
|  |\
| xterm Mouse Tracking |\
|  |\
| |     |     |     |\
| --- | --- | --- |\
| **Code** | **Effect** | **Note** |\
| CSI ? 1000 h | MTM | Enable Mouse Tracking Mode |\
| CSI ? 1001 h | HMTM | Set Highlight Mouse Tracking Mode |\
| CSI ? 1002 h | BMMTM | Set Button Motion Mouse Tracking Mode |\
| CSI ? 1004 h | WFTM | Enable Window Focus Tracking Mode |\
| CSI ? 1006 h | DMTM | Set Decimal Mouse Tracking Mode | |\
|  |\
| When MTM and DMTM are enabled the terminal will begin reporting mouse events using the **CSI < _flag_ ; _x_ ; _y_ M** format for key presses and **CSI < _flag_ ; _x_ ; _y_ m** for key releases. The exception is WFTM which reports focus in using **CSI I** and focus out using **CSI O**. |\
|  |\
| Special handling is required for HMTM which is not described in this document. In order to highlight terminals should support shift-mouseclick while mouse tracking is enabled for the user to copy and paste. |\
|  |\
| xterm Mouse Tracking Flags |\
|  |\
| |     |     |     |\
| --- | --- | --- |\
| **Code** | **Effect** | **Note** |\
| 0 | MB1 | Mouse Button 1 pressed |\
| 1 | MB2 | Mouse Button 2 pressed |\
| 2 | MB3 | Mouse Button 3 pressed |\
| 4 | MB+S | Mouse Button + Shift |\
| 8 | MB+A | Mouse Button + Alt |\
| 8 | MB4 | Mouse Button 4 pressed |\
| 10 | MB5 | Mouse Button 5 pressed |\
| 16 | MB+C | Mouse Button + Ctrl |\
| 32 | MBM | Mouse Button Motion |\
| 64 | MWU | Mouse Wheel Up |\
| 65 | MWD | Mouse Wheel Down |\
| 128 | MB8 | Mouse Button 8 pressed | |\
|  |\
|  |\
|  |\
|  |\
|  |\
| xterm Window Operations |\
|  |\
| |     |     |     |\
| --- | --- | --- |\
| **Code** | **Effect** | **Note** |\
| CSI 1 t | ICON | De-iconify window |\
| CSI 2 t | ICON | Iconify window |\
| CSI 3 ; _x_ ; _y_ t | POSITION | Move window to pixel x, y |\
| CSI 4 ; _h_ ; _w_ t | SIZE | Resize window to height, width pixels |\
| CSI 5 t | FOCUS | Focus window in |\
| CSI 6 t | FOCUS | Focus window out |\
| CSI 7 t | REFRESH | Refresh window |\
| CSI 8 ; _h_ ; _w_ t | SIZE | Resize window to height, width characters |\
| CSI 9 ; 0 t | MAXIMIZE | Restore maximized window |\
| CSI 9 ; 1 t | MAXIMIZE | Maximize window |\
| CSI 9 ; 2 t | MAXIMIZE | Maximize window vertically |\
| CSI 9 ; 3 t | MAXIMIZE | Maximize window horizontally |\
| CSI 10 ; 0 t | FULLSCREEN | Disable fullscreen mode |\
| CSI 10 ; 1 t | FULLSCREEN | Enable fullscreen mode |\
| CSI 10 ; 2 t | FULLSCREEN | Toggle fullscreen mode |\
| CSI 11 t | REPORT 1\|2 | Report ICON state |\
| CSI 13 t | REPORT 3 | Report POSITION state |\
| CSI 14 ; 2 t | REPORT 4 | Report SIZE state in pixels |\
| CSI 15 t | REPORT 5 | Report display resolution in pixels |\
| CSI 16 t | REPORT 6 | Report character size in pixels |\
| CSI 18 t | REPORT 8 | Report SIZE in characters |\
| CSI 19 t | REPORT 9 | Report display resolution in characters | |\
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
| **Clients supporting XTERM** |\
|  |\
| * * * |\
|  |\
| [Comparison of terminal emulators](https://en.wikipedia.org/wiki/Comparison_of_terminal_emulators) \- Wikipedia has a list of terminal emulators with XTERM support. |\
|  |\
|  |\
|  |\
| **Codebases** |\
|  |\
| * * * |\
|  |\
|  |\
|  |\
| **Snippets** |\
|  |\
| * * * |\
|  |\
|  |\
|  |
