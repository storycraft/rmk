#![no_std]

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(untagged, rename_all = "SCREAMING_SNAKE_CASE")
)]
#[allow(unused)]
pub enum Key {
    #[cfg_attr(feature = "serde", serde(alias = "KC_TRNS", alias = "_______"))]
    KcTransparent,

    Quantum(QmkKey),
    Layer(LayerKey),
    Hid(HidKey),
}

impl From<QmkKey> for Key {
    fn from(value: QmkKey) -> Self {
        Self::Quantum(value)
    }
}

impl From<LayerKey> for Key {
    fn from(value: LayerKey) -> Self {
        Self::Layer(value)
    }
}

impl From<HidKey> for Key {
    fn from(value: HidKey) -> Self {
        Self::Hid(value)
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[allow(unused)]
pub enum QmkKey {
    #[cfg_attr(feature = "serde", serde(alias = "QK_BOOT"))]
    QkBootloader,
    #[cfg_attr(feature = "serde", serde(alias = "DB_TOGG"))]
    QkDebugToggle,
    #[cfg_attr(feature = "serde", serde(alias = "EE_CLR"))]
    QkClearEeprom,
    QkMake,
    #[cfg_attr(feature = "serde", serde(alias = "QK_REBOOT"))]
    QkReboot,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[allow(unused)]
pub enum LayerKey {
    #[cfg_attr(feature = "serde", serde(alias = "MO"))]
    Modifier(u8),
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[allow(unused)]
pub enum HidKey {
    #[cfg_attr(feature = "serde", serde(alias = "XXXXXXX"))]
    KcNo = 0x00,

    KcA = 0x04,
    KcB = 0x05,
    KcC = 0x06,
    KcD = 0x07,
    KcE = 0x08,
    KcF = 0x09,
    KcG = 0x0a,
    KcH = 0x0b,
    KcI = 0x0c,
    KcJ = 0x0d,
    KcK = 0x0e,
    KcL = 0x0f,
    KcM = 0x10,
    KcN = 0x11,
    KcO = 0x12,
    KcP = 0x13,
    KcQ = 0x14,
    KcR = 0x15,
    KcS = 0x16,
    KcT = 0x17,
    KcU = 0x18,
    KcV = 0x19,
    KcW = 0x1a,
    KcX = 0x1b,
    KcY = 0x1c,
    KcZ = 0x1d,

    Kc1 = 0x1e,
    Kc2 = 0x1f,
    Kc3 = 0x20,
    Kc4 = 0x21,
    Kc5 = 0x22,
    Kc6 = 0x23,
    Kc7 = 0x24,
    Kc8 = 0x25,
    Kc9 = 0x26,
    Kc0 = 0x27,

    #[cfg_attr(feature = "serde", serde(alias = "KC_ENT"))]
    KcEnter = 0x28,
    #[cfg_attr(feature = "serde", serde(alias = "KC_ESC"))]
    KcEscape = 0x29,
    #[cfg_attr(feature = "serde", serde(alias = "KC_BSPC"))]
    KcBackspace = 0x2a,
    KcTab = 0x2b,
    #[cfg_attr(feature = "serde", serde(alias = "KC_SPC"))]
    KcSpace = 0x2c,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MINS"))]
    KcMins = 0x2d,
    #[cfg_attr(feature = "serde", serde(alias = "KC_EQL"))]
    KcEql = 0x2e,
    #[cfg_attr(feature = "serde", serde(alias = "KC_LBRC"))]
    KcLbrc = 0x2f,
    #[cfg_attr(feature = "serde", serde(alias = "KC_RBRC"))]
    KcRbrc = 0x30,
    #[cfg_attr(feature = "serde", serde(alias = "KC_BSLS"))]
    KcBackslash = 0x31,
    #[cfg_attr(feature = "serde", serde(alias = "KC_NUHS"))]
    KcNonusHash = 0x32,
    #[cfg_attr(feature = "serde", serde(alias = "KC_SCLN"))]
    KcSemicolon = 0x33,
    #[cfg_attr(feature = "serde", serde(alias = "KC_QUOT"))]
    KcQuote = 0x34,
    #[cfg_attr(feature = "serde", serde(alias = "KC_GRV"))]
    KcGrave = 0x35,
    #[cfg_attr(feature = "serde", serde(alias = "KC_COMM"))]
    KcComma = 0x36,
    KcDot = 0x37,
    #[cfg_attr(feature = "serde", serde(alias = "KC_SLSH"))]
    KcSlash = 0x38,
    #[cfg_attr(feature = "serde", serde(alias = "KC_CAPS"))]
    KcCapsLock = 0x39,

    KcF1 = 0x3a,
    KcF2 = 0x3b,
    KcF3 = 0x3c,
    KcF4 = 0x3d,
    KcF5 = 0x3e,
    KcF6 = 0x3f,
    KcF7 = 0x40,
    KcF8 = 0x41,
    KcF9 = 0x42,
    KcF10 = 0x43,
    KcF11 = 0x44,
    KcF12 = 0x45,

    #[cfg_attr(feature = "serde", serde(alias = "KC_PSCR"))]
    KcPrintScreen = 0x46,
    #[cfg_attr(feature = "serde", serde(alias = "KC_SCRL", alias = "KC_BRMD"))]
    KcScrollLock = 0x47,
    #[cfg_attr(
        feature = "serde",
        serde(alias = "KC_PAUS", alias = "KC_BRK", alias = "KC_BRMU")
    )]
    KcPause = 0x48,
    #[cfg_attr(feature = "serde", serde(alias = "KC_INS"))]
    KcInsert = 0x49,
    KcHome = 0x4a,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PGUP"))]
    KcPageUp = 0x4b,
    #[cfg_attr(feature = "serde", serde(alias = "KC_DEL"))]
    KcDelete = 0x4c,
    KcEnd = 0x4d,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PGDN"))]
    KcPageDown = 0x4e,
    #[cfg_attr(feature = "serde", serde(alias = "KC_RGHT"))]
    KcRight = 0x4f,
    KcLeft = 0x50,
    KcDown = 0x51,
    KcUp = 0x52,

    #[cfg_attr(feature = "serde", serde(alias = "KC_NUM"))]
    KcNumLock = 0x53,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PSLS"))]
    KcKpSlash = 0x54,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PAST"))]
    KcKpAsterisk = 0x55,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PMNS"))]
    KcKpMinus = 0x56,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PPLS"))]
    KcKpPlus = 0x57,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PENT"))]
    KcKpEnter = 0x58,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_1", alias = "KC_P1"))]
    KcKp1 = 0x59,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_2", alias = "KC_P2"))]
    KcKp2 = 0x5a,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_3", alias = "KC_P3"))]
    KcKp3 = 0x5b,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_4", alias = "KC_P4"))]
    KcKp4 = 0x5c,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_5", alias = "KC_P5"))]
    KcKp5 = 0x5d,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_6", alias = "KC_P6"))]
    KcKp6 = 0x5e,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_7", alias = "KC_P7"))]
    KcKp7 = 0x5f,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_8", alias = "KC_P8"))]
    KcKp8 = 0x60,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_9", alias = "KC_P9"))]
    KcKp9 = 0x61,
    #[cfg_attr(feature = "serde", serde(rename = "KC_KP_0", alias = "KC_P0"))]
    KcKp0 = 0x62,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PDOT"))]
    KcKpDot = 0x63,

    #[cfg_attr(feature = "serde", serde(alias = "KC_NUBS"))]
    KcNonusBackslash = 0x64,
    #[cfg_attr(feature = "serde", serde(alias = "KC_APP"))]
    KcApplication = 0x65,
    KcKbPower = 0x66,

    #[cfg_attr(feature = "serde", serde(alias = "KC_PEQL"))]
    KcKpEqual = 0x67,

    KcF13 = 0x68,
    KcF14 = 0x69,
    KcF15 = 0x6a,
    KcF16 = 0x6b,
    KcF17 = 0x6c,
    KcF18 = 0x6d,
    KcF19 = 0x6e,
    KcF20 = 0x6f,
    KcF21 = 0x70,
    KcF22 = 0x71,
    KcF23 = 0x72,
    KcF24 = 0x73,

    #[cfg_attr(feature = "serde", serde(alias = "KC_EXEC"))]
    KcExecute = 0x74,
    KcHelp = 0x75,
    KcMenu = 0x76,
    #[cfg_attr(feature = "serde", serde(alias = "KC_SLCT"))]
    KcSelect = 0x77,
    KcStop = 0x78,

    #[cfg_attr(feature = "serde", serde(alias = "KC_AGIN"))]
    KcAgain = 0x79,
    KcUndo = 0x7a,
    KcCut = 0x7b,
    KcCopy = 0x7c,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PSTE"))]
    KcPaste = 0x7d,
    KcFind = 0x7e,

    KcKbMute = 0x7f,
    KcKbVolumeUp = 0x80,
    KcKbVolumeDown = 0x81,

    #[cfg_attr(feature = "serde", serde(alias = "KC_LCAP"))]
    KcLockingCapsLock = 0x82,
    #[cfg_attr(feature = "serde", serde(alias = "KC_LNUM"))]
    KcLockingNumLock = 0x83,
    #[cfg_attr(feature = "serde", serde(alias = "KC_LSCR"))]
    KcLockingScrollLock = 0x84,

    #[cfg_attr(feature = "serde", serde(alias = "KC_PCMM"))]
    KcKpComma = 0x85,
    KcKpEqualAs400 = 0x86,

    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_1", alias = "KC_INT1")
    )]
    KcInternational1 = 0x87,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_2", alias = "KC_INT2")
    )]
    KcInternational2 = 0x88,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_3", alias = "KC_INT3")
    )]
    KcInternational3 = 0x89,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_4", alias = "KC_INT4")
    )]
    KcInternational4 = 0x8A,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_5", alias = "KC_INT5")
    )]
    KcInternational5 = 0x8B,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_6", alias = "KC_INT6")
    )]
    KcInternational6 = 0x8C,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_7", alias = "KC_INT7")
    )]
    KcInternational7 = 0x8D,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_8", alias = "KC_INT8")
    )]
    KcInternational8 = 0x8E,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "KC_INTERNATIONAL_9", alias = "KC_INT9")
    )]
    KcInternational9 = 0x8F,

    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_1", alias = "KC_LNG1"))]
    KcLanguage1 = 0x90,
    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_2", alias = "KC_LNG2"))]
    KcLanguage2 = 0x91,
    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_3", alias = "KC_LNG3"))]
    KcLanguage3 = 0x92,
    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_4", alias = "KC_LNG4"))]
    KcLanguage4 = 0x93,
    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_5", alias = "KC_LNG5"))]
    KcLanguage5 = 0x94,
    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_6", alias = "KC_LNG6"))]
    KcLanguage6 = 0x95,
    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_7", alias = "KC_LNG7"))]
    KcLanguage7 = 0x96,
    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_8", alias = "KC_LNG8"))]
    KcLanguage8 = 0x97,
    #[cfg_attr(feature = "serde", serde(rename = "KC_LANGUAGE_9", alias = "KC_LNG9"))]
    KcLanguage9 = 0x98,

    #[cfg_attr(feature = "serde", serde(alias = "KC_ERAS"))]
    KcAlternateErase = 0x99,
    #[cfg_attr(feature = "serde", serde(alias = "KC_SYRQ"))]
    KcSystemRequest = 0x9A,
    #[cfg_attr(feature = "serde", serde(alias = "KC_CNCL"))]
    KcCancel = 0x9B,
    #[cfg_attr(feature = "serde", serde(alias = "KC_CLR"))]
    KcClear = 0x9C,
    #[cfg_attr(feature = "serde", serde(alias = "KC_PRIR"))]
    KcPrior = 0x9D,
    #[cfg_attr(feature = "serde", serde(alias = "KC_RETN"))]
    KcReturn = 0x9E,
    #[cfg_attr(feature = "serde", serde(alias = "KC_SEPR"))]
    KcSeparator = 0x9F,
    KcOut = 0xA0,
    KcOper = 0xA1,
    #[cfg_attr(feature = "serde", serde(alias = "KC_CLAG"))]
    KcClearAgain = 0xA2,
    #[cfg_attr(feature = "serde", serde(alias = "KC_CRSL"))]
    KcCrsel = 0xA3,
    #[cfg_attr(feature = "serde", serde(alias = "KC_EXSL"))]
    KcExsel = 0xA4,

    #[cfg_attr(feature = "serde", serde(alias = "KC_PWR"))]
    KcSystemPower = 0xA5,
    #[cfg_attr(feature = "serde", serde(alias = "KC_SLEP"))]
    KcSystemSleep = 0xA6,
    #[cfg_attr(feature = "serde", serde(alias = "KC_WAKE"))]
    KcSystemWake = 0xA7,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MUTE"))]
    KcAudioMute = 0xA8,
    #[cfg_attr(feature = "serde", serde(alias = "KC_VOLU"))]
    KcAudioVolUp = 0xA9,
    #[cfg_attr(feature = "serde", serde(alias = "KC_VOLD"))]
    KcAudioVolDown = 0xAA,

    #[cfg_attr(feature = "serde", serde(alias = "KC_MNXT"))]
    KcMediaNextTrack = 0xAB,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MPRV"))]
    KcMediaPrevTrack = 0xAC,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MSTP"))]
    KcMediaStop = 0xAD,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MPLY"))]
    KcMediaPlayPause = 0xAE,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MSEL"))]
    KcMediaSelect = 0xAF,
    #[cfg_attr(feature = "serde", serde(alias = "KC_EJCT"))]
    KcMediaEject = 0xB0,

    KcMail = 0xB1,
    #[cfg_attr(feature = "serde", serde(alias = "KC_CALC"))]
    KcCalculator = 0xB2,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MYCM"))]
    KcMyComputer = 0xB3,

    #[cfg_attr(feature = "serde", serde(alias = "KC_WSCH"))]
    KcWwwSearch = 0xB4,
    #[cfg_attr(feature = "serde", serde(alias = "KC_WHOM"))]
    KcWwwHome = 0xB5,
    #[cfg_attr(feature = "serde", serde(alias = "KC_WBAK"))]
    KcWwwBack = 0xB6,
    #[cfg_attr(feature = "serde", serde(alias = "KC_WFWD"))]
    KcWwwForward = 0xB7,
    #[cfg_attr(feature = "serde", serde(alias = "KC_WSTP"))]
    KcWwwStop = 0xB8,
    #[cfg_attr(feature = "serde", serde(alias = "KC_WREF"))]
    KcWwwRefresh = 0xB9,
    #[cfg_attr(feature = "serde", serde(alias = "KC_WFAV"))]
    KcWwwFavorites = 0xBA,

    #[cfg_attr(feature = "serde", serde(alias = "KC_MFFD"))]
    KcMediaFastFoward = 0xBB,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MRWD"))]
    KcMediaRewind = 0xBC,

    #[cfg_attr(feature = "serde", serde(alias = "KC_BRIU"))]
    KcBrightnessUp = 0xBD,
    #[cfg_attr(feature = "serde", serde(alias = "KC_BRID"))]
    KcBrightnessDown = 0xBE,

    #[cfg_attr(feature = "serde", serde(alias = "KC_CPNL"))]
    KcControlPanel = 0xBF,
    #[cfg_attr(feature = "serde", serde(alias = "KC_ASST"))]
    KcAssistant = 0xC0,
    #[cfg_attr(feature = "serde", serde(alias = "KC_MCTL"))]
    KcMissionControl = 0xC1,
    #[cfg_attr(feature = "serde", serde(alias = "KC_LAPD"))]
    KcLaunchpad = 0xC2,
}

impl HidKey {
    pub const fn keycode(self) -> u8 {
        self as _
    }
}
