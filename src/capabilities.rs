/// Known bool capabilities
pub enum BoolCapability {
    /// cub1 wraps from column 0 to last column
    AutoLeftMargin = 0,
    /// Terminal has automatic margins
    AutoRightMargin = 1,
    /// Beehive (f1=escape, f2=ctrl C)
    NoEscCtlc,
    /// Standout not erased by overwriting (hp)
    CeolStandoutGlitch,
    /// Newline ignored after 80 columns (Concept)
    EatNewlineGlitch,
    /// Can erase overstrikes with a blank
    EraseOverstrike,
    /// Generic line type (e.g., dialup, switch)
    GenericType,
    /// Hardcopy terminal
    HardCopy,
    /// Has a meta key (shift, sets parity bit)
    HasMetaKey,
    /// Has extra 'status line'
    HasStatusLine,
    /// Insert mode distinguishes nulls
    InsertNullGlitch,
    /// Display may be retained above the screen
    MemoryAbove,
    /// Display may be retained below the screen
    MemoryBelow,
    /// Safe to move while in insert mode
    MoveInsertMode,
    /// Safe to move in standout modes
    MoveStandoutMode,
    /// Terminal overstrikes on hard-copy terminal
    OverStrike,
    /// Escape can be used on the status line
    StatusLineEscOk,
    /// Destructive tabs, magic smso char (t1061)
    DestTabsMagicSmso,
    /// Hazeltine; can't print tilde (~)
    TildeGlitch,
    /// Underline character overstrikes
    TransparentUnderline,
    /// Terminal uses xon/xoff handshaking
    XonXoff,
    /// Padding won't work, xon/xoff required
    NeedsXonXoff,
    /// Printer won't echo on screen
    PrtrSilent,
    /// Cursor is hard to see
    HardCursor,
    /// smcup does not reverse rmcup
    NonRevRmcup,
    /// Pad character doesn't exist
    NoPadChar,
    /// Scrolling region is nondestructive
    NonDestScrollRegion,
    /// Terminal can re-define existing colour
    CanChange,
    /// Screen erased with background colour
    BackColorErase,
    /// Terminal uses only HLS colour notation (Tektronix)
    HueLightnessSaturation,
    /// Only positive motion for hpa/mhpa caps
    ColAddrGlitch,
    /// Using cr turns off micro mode
    CrCancelsMicroMode,
    /// Printer needs operator to change character set
    HasPrintWheel,
    /// Only positive motion for vpa/mvpa caps
    RowAddrGlitch,
    /// Printing in last column causes cr
    SemiAutoRightMargin,
    /// Changing character pitch changes resolution
    CpiChangesRes,
    /// Changing line pitch changes resolution
    LpiChangesRes,
}

/// Known number capabilities
pub enum NumberCapability {
    /// Number of columns in a line
    Columns = 0,
    /// Tabs initially every # spaces
    InitTabs = 1,
    /// Number of lines on a screen or a page
    Lines,
    /// Lines of memory if > lines; 0 means varies
    LinesOfMemory,
    /// Number of blank characters left by smso or rmso
    MagicCookieGlitch,
    /// Lowest baud rate where padding needed
    PaddingBaudRate,
    /// Virtual terminal number
    VirtualTerminal,
    /// Number of columns in status line
    WidthStatusLine,
    /// Number of labels on screen (start at 1)
    NumLabels,
    /// Number of rows in each label
    LabelHeight,
    /// Number of columns in each label
    LabelWidth,
    /// Maximum combined video attributes terminal can display
    MaxAttributes,
    /// Maximum number of definable windows
    MaximumWindows,
    /// Maximum number of colours on the screen
    MaxColors,
    /// Maximum number of colour-pairs on the screen
    MaxPairs,
    /// Video attributes that can't be used with colours
    NoColorVideo,
    /// Number of bytes buffered before printing
    BufferCapacity,
    /// Spacing of pins vertically in pins per inch
    DotVertSpacing,
    /// Spacing of dots horizontally in dots per inch
    DotHorzSpacing,
    /// Maximum value in micro address
    MaxMicroAddress,
    /// Maximum value in parm micro
    MaxMicroJump,
    /// Character step size when in micro mode
    MicroColSize,
    /// Line step size when in micro mode
    MicroLineSize,
    /// Number of pins in print-head
    NumberOfPins,
    /// Horizontal resolution in units per character
    OutputResChar,
    /// Vertical resolution in units per line
    OutputResLine,
    /// Horizontal resolution in units per inch
    OutputResHorzInch,
    /// Vertical resolution in units per inch
    OutputResVertInch,
    /// Print rate in characters per second
    PrintRate,
    /// Character step size when in double-wide mode
    WideCharSize,
    /// Number of buttons on the mouse
    Buttons,
    /// Number of passes for each bit-map row
    BitImageEntwining,
    /// Type of bit image device
    BitImageType,
}

/// Known string capabilities
pub enum StringCapability {
    /// Back tab
    BackTab = 0,
    /// Audible signal (bell)
    Bell = 1,
    /// Carriage return
    CarriageReturn,
    /// Change to lines #1 through #2 (VT100)
    ChangeScrollRegion,
    /// Clear all tab stops
    ClearAllTabs,
    /// Clear screen and home cursor
    ClearScreen,
    /// Clear to end of line
    ClearEOL,
    /// Clear to end of display
    ClearEOS,
    /// Set horizontal position to absolute #1
    ColumnAddress,
    /// Terminal settable cmd characterin prototype
    CommandCharacter,
    /// Move to row #1 col #2
    CursorAddress,
    /// Down one line
    CursorDown,
    /// Home cursor (if no cup)
    CursorHome,
    /// Make cursor invisible
    CursorInvisible,
    /// Move left one space.
    CursorLeft,
    /// Memory relative cursor addressing
    CursorMemAddress,
    /// Make cursor appear normal (undo vs/vi)
    CursorNormal,
    /// Non-destructive space (cursor or carriage right)
    CursorRight,
    /// Last line, first column (if no cup)
    CursorToLastLine,
    /// Upline (cursor up)
    CursorUp,
    /// Make cursor very visible
    CursorVisible,
    /// Delete character
    DeleteCharacter,
    /// Delete line
    DeleteLine,
    /// Disable status line
    DisStatusLine,
    /// Half-line down (forward 1/2 linefeed)
    DownHalfLine,
    /// Start alternate character set
    EnterAltCharsetMode,
    /// Turn on blinking
    EnterBlinkMode,
    /// Turn on bold (extra bright) mode
    EnterBoldMode,
    /// String to begin programs that use cup
    EnterAlternativeMode,
    /// Delete mode (enter)
    EnterDeleteMode,
    /// Turn on half-bright mode
    EnterDimMode,
    /// Insert mode (enter)
    EnterInsertMode,
    /// Turn on blank mode (characters invisible)
    EnterSecureMode,
    /// Turn on protected mode
    EnterProtectedMode,
    /// Turn on reverse video mode
    EnterReverseMode,
    /// Begin standout mode
    EnterStandoutMode,
    /// Start underscore mode
    EnterUnderlineMode,
    /// Erase #1 characters
    EraseChars,
    /// End alternate character set
    ExitAltCharsetMode,
    /// Turn off all attributes
    ExitAttributeMode,
    /// String to end programs that use cup
    ExitAlternativeMode,
    /// End delete mode
    ExitDeleteMode,
    /// End insert mode
    ExitInsertMode,
    /// End standout mode
    ExitStandoutMode,
    /// End underscore mode
    ExitUnderlineMode,
    /// Visible bell (may move cursor)
    FlashScreen,
    /// Hardcopy terminal page eject
    FormFeed,
    /// Return from status line
    FromStatusLine,
    /// Terminal or printer initialisation string
    Init1String,
    /// Terminal or printer initialisation string
    Init2String,
    /// Terminal or printer initialisation string
    Init3String,
    /// Name of initialisation file
    InitFile,
    /// Insert character
    InsertCharacter,
    /// Add new blank line
    InsertLine,
    /// Insert pad after character inserted
    InsertPadding,
    /// sent by backspace key
    KeyBackspace,
    /// sent by clear-all-tabs key
    KeyClearAllTabs,
    /// sent by clear-screen or erase key
    KeyClear,
    /// sent by clear-tab key
    KeyClearTab,
    /// sent by delete-character key
    KeyDeleteCharacter,
    /// sent by delete-line key
    KeyDeleteLine,
    /// sent by terminal down-arrow key
    KeyDown,
    /// sent by rmir or smir in insert mode
    KeyEic,
    /// sent by clear-to-end-of-line key
    KeyClearEOL,
    /// sent by clear-to-end-of-screen key
    KeyClearEOS,
    /// sent by function key f0
    KeyF0,
    /// sent by function key f1
    KeyF1,
    /// sent by function key f10
    KeyF10,
    /// sent by function key f2
    KeyF2,
    /// sent by function key f3
    KeyF3,
    /// sent by function key f4
    KeyF4,
    /// sent by function key f5
    KeyF5,
    /// sent by function key f6
    KeyF6,
    /// sent by function key f7
    KeyF7,
    /// sent by function key f8
    KeyF8,
    /// sent by function key f9
    KeyF9,
    /// sent by home key
    KeyHome,
    /// sent by ins-char/enter ins-mode key
    KeyInsertCharacter,
    /// sent by insert-line key
    KeyInsertLine,
    /// sent by terminal left-arrow key
    KeyLeft,
    /// sent by home-down key
    KeyLastLine,
    /// sent by next-page key
    KeyNextPage,
    /// sent by previous-page key
    KeyPreviousPage,
    /// sent by terminal right-arrow key
    KeyRight,
    /// sent by scroll-forward/down key
    KeyScrollForward,
    /// sent by scroll-backward/up key
    KeyScrollBackward,
    /// sent by set-tab key
    KeySetTab,
    /// sent by terminal up-arrow key
    KeyUp,
    /// Out of 'keypad-transmit' mode
    KeypadLocal,
    /// Put terminal in 'keypad-transmit' mode
    KeypadXmit,
    /// Labels on function key f0 if not f0
    LabF0,
    /// Labels on function key f1 if not f1
    LabF1,
    /// Labels on function key f10 if not f10
    LabF10,
    /// Labels on function key f2 if not f2
    LabF2,
    /// Labels on function key f3 if not f3
    LabF3,
    /// Labels on function key f4 if not f4
    LabF4,
    /// Labels on function key f5 if not f5
    LabF5,
    /// Labels on function key f6 if not f6
    LabF6,
    /// Labels on function key f7 if not f7
    LabF7,
    /// Labels on function key f8 if not f8
    LabF8,
    /// Labels on function key f9 if not f9
    LabF9,
    /// Turn off 'meta mode'
    MetaOff,
    /// Turn on 'meta mode' (8th bit)
    MetaOn,
    /// Newline (behaves like cr followed by lf)
    Newline,
    /// Pad character (rather than null)
    PadChar,
    /// Delete #1 chars
    ParmDeleteCharacters,
    /// Delete #1 lines
    ParmDeleteLine,
    /// Move down #1 lines.
    ParmDownCursor,
    /// Insert #1 blank chars
    ParmInsertCharacters,
    /// Scroll forward #1 lines.
    ParmIndex,
    /// Add #1 new blank lines
    ParmInsertLine,
    /// Move cursor left #1 spaces
    ParmLeftCursor,
    /// Move right #1 spaces.
    ParmRightCursor,
    /// Scroll backward #1 lines.
    ParmReverseIndex,
    /// Move cursor up #1 lines.
    ParmUpCursor,
    /// Prog funct key #1 to type string #2
    PKeyKey,
    /// Prog funct key #1 to execute string #2
    PKeyLocal,
    /// Prog funct key #1 to xmit string #2
    PKeyXmit,
    /// Print contents of the screen
    PrintScreen,
    /// Turn off the printer
    PrinterOff,
    /// Turn on the printer
    PrinterOn,
    /// Repeat char #1 #2 times
    RepeatChar,
    /// Reset terminal completely to sane modes
    Reset1String,
    /// Reset terminal completely to sane modes
    Reset2String,
    /// Reset terminal completely to sane modes
    Reset3String,
    /// Name of file containing reset string
    ResetFile,
    /// Restore cursor to position of last sc
    RestoreCursor,
    /// Set vertical position to absolute #1
    RowAddress,
    /// Save cursor position
    SaveCursor,
    /// Scroll text up
    ScrollForward,
    /// Scroll text down
    ScrollReverse,
    /// Define first set of video attributes #1-#9
    SetAttributes,
    /// Set a tab in all rows, current column
    SetTab,
    /// Current window is lines #1-#2 cols #3-#4
    SetWindow,
    /// Tab to next 8-space hardware tab stop
    Tab,
    /// Go to status line, col #1
    ToStatusLine,
    /// Underscore one char and move past it
    UnderlineChar,
    /// Half-line up (reverse 1/2 linefeed)
    UpHalfLine,
    /// Path name of program for initialisation
    InitProg,
    /// upper left of keypad
    KeyA1,
    /// upper right of keypad
    KeyA3,
    /// center of keypad
    KeyB2,
    /// lower left of keypad
    KeyC1,
    /// lower right of keypad
    KeyC3,
    /// Turn on the printer for #1 bytes
    PrinterOnForNBytes,
    /// Like ip but when in replace mode
    CharPadding,
    /// Graphic charset pairs aAbBcC
    AcsChars,
    /// Prog label #1 to show string #2
    PlabNorm,
    /// sent by back-tab key
    KeyBackTab,
    /// Turn on xon/xoff handshaking
    EnterXonMode,
    /// Turn off xon/xoff handshaking
    ExitXonMode,
    /// Turn on automatic margins
    EnterAutomaticMarginsMode,
    /// Turn off automatic margins
    ExitAutomaticMarginsMode,
    /// X-on character
    XOnCharacter,
    /// X-off character
    XOffCharacter,
    /// Enable alternate character set
    EnableAlternateCharSet,
    /// Turn on soft labels
    LabelOn,
    /// Turn off soft labels
    LabelOff,
    /// 1
    KeyBegin,
    /// 2
    KeyCancel,
    /// 3
    KeyClose,
    /// 4
    KeyCommand,
    /// 5
    KeyCopy,
    /// 6
    KeyCreate,
    /// 7
    KeyEnd,
    /// 8
    KeyEnter,
    /// 9
    KeyExit,
    /// 0
    KeyFind,
    /// sent by help key
    KeyHelp,
    /// sent by mark key
    KeyMark,
    /// sent by message key
    KeyMessage,
    /// sent by move key
    KeyMove,
    /// sent by next-object key
    KeyNext,
    /// sent by open key
    KeyOpen,
    /// sent by options key
    KeyOptions,
    /// sent by previous-object key
    KeyPrevious,
    /// sent by print or copy key
    KeyPrint,
    /// sent by redo key
    KeyRedo,
    /// sent by ref(erence) key
    KeyReference,
    /// sent by refresh key
    KeyRefresh,
    /// sent by replace key
    KeyReplace,
    /// sent by restart key
    KeyRestart,
    /// sent by resume key
    KeyResume,
    /// sent by save key
    KeySave,
    /// sent by suspend key
    KeySuspend,
    /// sent by undo key
    KeyUndo,
    /// sent by shifted beginning key
    KeyShiftBegin,
    /// sent by shifted cancel key
    KeyShiftCancel,
    /// sent by shifted command key
    KeyShiftCommand,
    /// sent by shifted copy key
    KeyShiftCopy,
    /// sent by shifted create key
    KeyShiftCreate,
    /// sent by shifted delete-char key
    KeyShiftDeleteChar,
    /// sent by shifted delete-line key
    KeyShiftDeleteLine,
    /// sent by select key
    KeySelect,
    /// sent by shifted end key
    KeyShiftEnd,
    /// sent by shifted clear-line key
    KeyShiftEOL,
    /// sent by shifted exit key
    KeyShiftExit,
    /// sent by shifted find key
    KeyShiftFind,
    /// #1  sent by shifted help key
    KeyShiftHelp,
    /// #2  sent by shifted home key
    KeyShiftHome,
    /// #3  sent by shifted input key
    KeyShiftInputKey,
    /// #4  sent by shifted left-arrow key
    KeyShiftLeft,
    /// sent by shifted message key
    KeyShiftMessage,
    /// sent by shifted move key
    KeyShiftMove,
    /// sent by shifted next key
    KeyShiftNext,
    /// sent by shifted options key
    KeyShiftOptions,
    /// sent by shifted prev key
    KeyShiftPrevious,
    /// sent by shifted print key
    KeyShiftPrint,
    /// sent by shifted redo key
    KeyShiftRedo,
    /// sent by shifted replace key
    KeyShiftReplace,
    /// sent by shifted right-arrow key
    KeyShiftRight,
    /// sent by shifted resume key
    KeyShiftResume,
    /// !1  sent by shifted save key
    KeyShiftSave,
    /// !2  sent by shifted suspend key
    KeyShiftSuspend,
    /// !3  sent by shifted undo key
    KeyShiftUndo,
    /// Send next input char (for ptys)
    ReqForInput,
    /// sent by function key f11
    KeyF11,
    /// sent by function key f12
    KeyF12,
    /// sent by function key f13
    KeyF13,
    /// sent by function key f14
    KeyF14,
    /// sent by function key f15
    KeyF15,
    /// sent by function key f16
    KeyF16,
    /// sent by function key f17
    KeyF17,
    /// sent by function key f18
    KeyF18,
    /// sent by function key f19
    KeyF19,
    /// sent by function key f20
    KeyF20,
    /// sent by function key f21
    KeyF21,
    /// sent by function key f22
    KeyF22,
    /// sent by function key f23
    KeyF23,
    /// sent by function key f24
    KeyF24,
    /// sent by function key f25
    KeyF25,
    /// sent by function key f26
    KeyF26,
    /// sent by function key f27
    KeyF27,
    /// sent by function key f28
    KeyF28,
    /// sent by function key f29
    KeyF29,
    /// sent by function key f30
    KeyF30,
    /// sent by function key f31
    KeyF31,
    /// sent by function key f32
    KeyF32,
    /// sent by function key f33
    KeyF33,
    /// sent by function key f34
    KeyF34,
    /// sent by function key f35
    KeyF35,
    /// sent by function key f36
    KeyF36,
    /// sent by function key f37
    KeyF37,
    /// sent by function key f38
    KeyF38,
    /// sent by function key f39
    KeyF39,
    /// sent by function key f40
    KeyF40,
    /// sent by function key f41
    KeyF41,
    /// sent by function key f42
    KeyF42,
    /// sent by function key f43
    KeyF43,
    /// sent by function key f44
    KeyF44,
    /// sent by function key f45
    KeyF45,
    /// sent by function key f46
    KeyF46,
    /// sent by function key f47
    KeyF47,
    /// sent by function key f48
    KeyF48,
    /// sent by function key f49
    KeyF49,
    /// sent by function key f50
    KeyF50,
    /// sent by function key f51
    KeyF51,
    /// sent by function key f52
    KeyF52,
    /// sent by function key f53
    KeyF53,
    /// sent by function key f54
    KeyF54,
    /// sent by function key f55
    KeyF55,
    /// sent by function key f56
    KeyF56,
    /// sent by function key f57
    KeyF57,
    /// sent by function key f58
    KeyF58,
    /// sent by function key f59
    KeyF59,
    /// sent by function key f60
    KeyF60,
    /// sent by function key f61
    KeyF61,
    /// sent by function key f62
    KeyF62,
    /// sent by function key f63
    KeyF63,
    /// Clear to beginning of line, inclusive
    ClearBOL,
    /// Clear all margins (top, bottom, and sides)
    ClearMargins,
    /// Set left margin at current column
    SetLeftMargin,
    /// Set right margin at current column
    SetRightMargin,
    /// Label format
    LabelFormat,
    /// Set clock to hours (#1), minutes (#2), seconds (#3)
    SetClock,
    /// Display time-of-day clock
    DisplayClock,
    /// Remove time-of-day clock
    RemoveClock,
    /// Define win #1 to go from #2,#3 to #4,#5
    CreateWindow,
    /// Go to window #1
    GotoWindow,
    /// Hang-up phone
    Hangup,
    /// Dial phone number #1
    DialPhone,
    /// Dial phone number #1, without progress detection
    QuickDial,
    /// Select touch tone dialing
    Tone,
    /// Select pulse dialing
    Pulse,
    /// Flash the switch hook
    FlashHook,
    /// Pause for 2-3 seconds
    FixedPause,
    /// Wait for dial tone
    WaitTone,
    /// User string 0
    User0,
    /// User string 1
    User1,
    /// User string 2
    User2,
    /// User string 3
    User3,
    /// User string 4
    User4,
    /// User string 5
    User5,
    /// User string 6
    User6,
    /// User string 7
    User7,
    /// User string 8
    User8,
    /// User string 9
    User9,
    /// Set default colour-pair to the original one
    OrigColorPair,
    /// Set all colour(-pair)s to the original ones
    OrigColors,
    /// Set colour #1 to RGB #2, #3, #4
    InitializeColor,
    /// Set colour-pair #1 to fg #2, bg #3
    InitializePair,
    /// Set current colour pair to #1
    SetColorPair,
    /// Set foreground colour to #1
    SetForeground,
    /// Set background colour to #1
    SetBackground,
    /// Change number of characters per inch
    ChangeCharPitch,
    /// Change number of lines per inch
    ChangeLinePitch,
    /// Change horizontal resolution
    ChangeResHorz,
    /// Change vertical resolution
    ChangeResVert,
    /// Define a character in a character set
    DefineChar,
    /// Enable double wide printing
    EnterDoublewideMode,
    /// Set draft quality print
    EnterDraftQuality,
    /// Enable italics
    EnterItalicsMode,
    /// Enable leftward carriage motion
    EnterLeftwardMode,
    /// Enable micro motion capabilities
    EnterMicroMode,
    /// Set near-letter quality print
    EnterNearLetterQuality,
    /// Set normal quality print
    EnterNormalQuality,
    /// Enable shadow printing
    EnterShadowMode,
    /// Enable subscript printing
    EnterSubscriptMode,
    /// Enable superscript printing
    EnterSuperscriptMode,
    /// Enable upward carriage motion
    EnterUpwardMode,
    /// Disable double wide printing
    ExitDoublewideMode,
    /// Disable italics
    ExitItalicsMode,
    /// Enable rightward (normal) carriage motion
    ExitLeftwardMode,
    /// Disable micro motion capabilities
    ExitMicroMode,
    /// Disable shadow printing
    ExitShadowMode,
    /// Disable subscript printing
    ExitSubscriptMode,
    /// Disable superscript printing
    ExitSuperscriptMode,
    /// Enable downward (normal) carriage motion
    ExitUpwardMode,
    /// Like columnaddress for micro adjustment
    MicroColumnAddress,
    /// Like cursordown for micro adjustment
    MicroDown,
    /// Like cursorleft for micro adjustment
    MicroLeft,
    /// Like cursorright for micro adjustment
    MicroRight,
    /// Like rowaddress for micro adjustment
    MicroRowAddress,
    /// Like cursorup for micro adjustment
    MicroUp,
    /// Matches software bits to print-head pins
    OrderOfPins,
    /// Like parmdowncursor for micro adjust.
    ParmDownMicro,
    /// Like parmleftcursor for micro adjust.
    ParmLeftMicro,
    /// Like parmrightcursor for micro adjust.
    ParmRightMicro,
    /// Like parmupcursor for micro adjust.
    ParmUpMicro,
    /// Select character set
    SelectCharSet,
    /// Set bottom margin at current line
    SetBottomMargin,
    /// Set bottom margin at line #1 or #2 lines from bottom
    SetBottomMarginParm,
    /// Set left (right) margin at column #1 (#2)
    SetLeftMarginParm,
    /// Set right margin at column #1
    SetRightMarginParm,
    /// Set top margin at current line
    SetTopMargin,
    /// Set top (bottom) margin at line #1 (#2)
    SetTopMarginParm,
    /// Start printing bit image graphics
    StartBitImage,
    /// Start definition of a character set
    StartCharSetDef,
    /// End printing bit image graphics
    StopBitImage,
    /// End definition of a character set
    StopCharSetDef,
    /// List of 'subscript-able' characters
    SubscriptCharacters,
    /// List of 'superscript-able' characters
    SuperscriptCharacters,
    /// Printing any of these chars causes cr
    TheseCauseCr,
    /// No motion for the subsequent character
    ZeroMotion,
    /// Returns a list of character set names
    CharSetNames,
    /// 0631, Mouse event has occured
    KeyMouse,
    /// Mouse status information
    MouseInfo,
    /// Request mouse position report
    ReqMousePos,
    /// Curses should get button events
    GetMouse,
    /// Set foreground colour to #1 using ANSI escape
    SetAnsiForeground,
    /// Set background colour to #1 using ANSI escape
    SetAnsiBackground,
    /// Prog key #1 to xmit string #2 and show string #3
    PKeyPlab,
    /// Indicate language/codeset support
    DeviceType,
    /// Init sequence for multiple codesets
    CodeSetInit,
    /// Shift into codeset 0 (EUC set 0, ASCII)
    Set0DesSeq,
    /// Shift into codeset 1
    Set1DesSeq,
    /// Shift into codeset 2
    Set2DesSeq,
    /// Shift into codeset 3
    Set3DesSeq,
    /// Sets both left and right margins
    SetLrMargin,
    /// Sets both top and bottom margins
    SetTbMargin,
    /// Repeat bit-image cell #1 #2 times
    BitImageRepeat,
    /// Move to next row of the bit image
    BitImageNewline,
    /// Move to beginning of same row
    BitImageCarriageReturn,
    /// Give name for colour #1
    ColorNames,
    /// Define rectangular bit-image region
    DefineBitImageRegion,
    /// End a bit-image region
    EndBitImageRegion,
    /// Change to ribbon colour #1
    SetColorBand,
    /// Set page length to #1 lines
    SetPageLength,
    /// Display PC character
    DisplayPcChar,
    /// Enter PC character display mode
    EnterPcCharsetMode,
    /// Disable PC character display mode
    ExitPcCharsetMode,
    /// Enter PC scancode mode
    EnterScancodeMode,
    /// Disable PC scancode mode
    ExitScancodeMode,
    /// PC terminal options
    PcTermOptions,
    /// Escape for scancode emulation
    ScancodeEscape,
    /// Alternate escape for scancode emulation (default is for VT100)
    AltScancodeEsc,
    /// Turn on horizontal highlight mode
    EnterHorizontalHlMode,
    /// Turn on left highlight mode
    EnterLeftHlMode,
    /// Turn on low highlight mode
    EnterLowHlMode,
    /// Turn on right highlight mode
    EnterRightHlMode,
    /// Turn on top highlight mode
    EnterTopHlMode,
    /// Turn on vertical highlight mode
    EnterVerticalHlMode,
    /// Define second set of video attributes #1-#6
    SetAAttributes,
    /// Set page length to #1 hundredth of an inch
    SetPageLenInch,
}
