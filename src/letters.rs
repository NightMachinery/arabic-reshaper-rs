// Each letter is of the format:
//
//   ('<letter>', <replacement>)
//
// And replacement is of the format:
//
//   ('<isolated>', '<initial>', '<medial>', '<final>')
//
// Where <letter> is the string to replace, and <isolated> is the replacement in
// case <letter> should be in isolated form, <initial> is the replacement in
// case <letter> should be in initial form, <medial> is the replacement in case
// <letter> should be in medial form, and <final> is the replacement in case
// <letter> should be in final form. If no replacement is specified for a form,
// then no that means the letter doesn't support this form.
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const UNSHAPED: i16 = 255;
pub const ISOLATED: i16 = 0;
pub const INITIAL: i16 = 1;
pub const MEDIAL: i16 = 2;
pub const FINAL: i16 = 3;

pub const TATWEEL: char = '\u{0640}';
pub const ZWJ: char = '\u{200D}';

pub static LETTERS: Lazy<HashMap<char, [&'static str; 4]>> = Lazy::new(|| {
    [
        // ARABIC LETTER HAMZA
        ('\u{0621}', ["\u{FE80}", "", "", ""]),
        // ARABIC LETTER ALEF WITH MADDA ABOVE
        ('\u{0622}', ["\u{FE81}", "", "", "\u{FE82}"]),
        // ARABIC LETTER ALEF WITH HAMZA ABOVE
        ('\u{0623}', ["\u{FE83}", "", "", "\u{FE84}"]),
        // ARABIC LETTER WAW WITH HAMZA ABOVE
        ('\u{0624}', ["\u{FE85}", "", "", "\u{FE86}"]),
        // ARABIC LETTER ALEF WITH HAMZA BELOW
        ('\u{0625}', ["\u{FE87}", "", "", "\u{FE88}"]),
        // ARABIC LETTER YEH WITH HAMZA ABOVE
        ('\u{0626}', ["\u{FE89}", "\u{FE8B}", "\u{FE8C}", "\u{FE8A}"]),
        // ARABIC LETTER ALEF
        ('\u{0627}', ["\u{FE8D}", "", "", "\u{FE8E}"]),
        // ARABIC LETTER BEH
        ('\u{0628}', ["\u{FE8F}", "\u{FE91}", "\u{FE92}", "\u{FE90}"]),
        // ARABIC LETTER TEH MARBUTA
        ('\u{0629}', ["\u{FE93}", "", "", "\u{FE94}"]),
        // ARABIC LETTER TEH
        ('\u{062A}', ["\u{FE95}", "\u{FE97}", "\u{FE98}", "\u{FE96}"]),
        // ARABIC LETTER THEH
        ('\u{062B}', ["\u{FE99}", "\u{FE9B}", "\u{FE9C}", "\u{FE9A}"]),
        // ARABIC LETTER JEEM
        ('\u{062C}', ["\u{FE9D}", "\u{FE9F}", "\u{FEA0}", "\u{FE9E}"]),
        // ARABIC LETTER HAH
        ('\u{062D}', ["\u{FEA1}", "\u{FEA3}", "\u{FEA4}", "\u{FEA2}"]),
        // ARABIC LETTER KHAH
        ('\u{062E}', ["\u{FEA5}", "\u{FEA7}", "\u{FEA8}", "\u{FEA6}"]),
        // ARABIC LETTER DAL
        ('\u{062F}', ["\u{FEA9}", "", "", "\u{FEAA}"]),
        // ARABIC LETTER THAL
        ('\u{0630}', ["\u{FEAB}", "", "", "\u{FEAC}"]),
        // ARABIC LETTER REH
        ('\u{0631}', ["\u{FEAD}", "", "", "\u{FEAE}"]),
        // ARABIC LETTER ZAIN
        ('\u{0632}', ["\u{FEAF}", "", "", "\u{FEB0}"]),
        // ARABIC LETTER SEEN
        ('\u{0633}', ["\u{FEB1}", "\u{FEB3}", "\u{FEB4}", "\u{FEB2}"]),
        // ARABIC LETTER SHEEN
        ('\u{0634}', ["\u{FEB5}", "\u{FEB7}", "\u{FEB8}", "\u{FEB6}"]),
        // ARABIC LETTER SAD
        ('\u{0635}', ["\u{FEB9}", "\u{FEBB}", "\u{FEBC}", "\u{FEBA}"]),
        // ARABIC LETTER DAD
        ('\u{0636}', ["\u{FEBD}", "\u{FEBF}", "\u{FEC0}", "\u{FEBE}"]),
        // ARABIC LETTER TAH
        ('\u{0637}', ["\u{FEC1}", "\u{FEC3}", "\u{FEC4}", "\u{FEC2}"]),
        // ARABIC LETTER ZAH
        ('\u{0638}', ["\u{FEC5}", "\u{FEC7}", "\u{FEC8}", "\u{FEC6}"]),
        // ARABIC LETTER AIN
        ('\u{0639}', ["\u{FEC9}", "\u{FECB}", "\u{FECC}", "\u{FECA}"]),
        // ARABIC LETTER GHAIN
        ('\u{063A}', ["\u{FECD}", "\u{FECF}", "\u{FED0}", "\u{FECE}"]),
        // ARABIC TATWEEL
        (TATWEEL, ["\u{0640}", "\u{0640}", "\u{0640}", "\u{0640}"]),
        // ARABIC LETTER FEH
        ('\u{0641}', ["\u{FED1}", "\u{FED3}", "\u{FED4}", "\u{FED2}"]),
        // ARABIC LETTER QAF
        ('\u{0642}', ["\u{FED5}", "\u{FED7}", "\u{FED8}", "\u{FED6}"]),
        // ARABIC LETTER KAF
        ('\u{0643}', ["\u{FED9}", "\u{FEDB}", "\u{FEDC}", "\u{FEDA}"]),
        // ARABIC LETTER LAM
        ('\u{0644}', ["\u{FEDD}", "\u{FEDF}", "\u{FEE0}", "\u{FEDE}"]),
        // ARABIC LETTER MEEM
        ('\u{0645}', ["\u{FEE1}", "\u{FEE3}", "\u{FEE4}", "\u{FEE2}"]),
        // ARABIC LETTER NOON
        ('\u{0646}', ["\u{FEE5}", "\u{FEE7}", "\u{FEE8}", "\u{FEE6}"]),
        // ARABIC LETTER HEH
        ('\u{0647}', ["\u{FEE9}", "\u{FEEB}", "\u{FEEC}", "\u{FEEA}"]),
        // ARABIC LETTER WAW
        ('\u{0648}', ["\u{FEED}", "", "", "\u{FEEE}"]),
        // ARABIC LETTER (UIGHUR KAZAKH KIRGHIZ)? ALEF MAKSURA
        ('\u{0649}', ["\u{FEEF}", "\u{FBE8}", "\u{FBE9}", "\u{FEF0}"]),
        // ARABIC LETTER YEH
        ('\u{064A}', ["\u{FEF1}", "\u{FEF3}", "\u{FEF4}", "\u{FEF2}"]),
        // ARABIC LETTER ALEF WASLA
        ('\u{0671}', ["\u{FB50}", "", "", "\u{FB51}"]),
        // ARABIC LETTER U WITH HAMZA ABOVE
        ('\u{0677}', ["\u{FBDD}", "", "", ""]),
        // ARABIC LETTER TTEH
        ('\u{0679}', ["\u{FB66}", "\u{FB68}", "\u{FB69}", "\u{FB67}"]),
        // ARABIC LETTER TTEHEH
        ('\u{067A}', ["\u{FB5E}", "\u{FB60}", "\u{FB61}", "\u{FB5F}"]),
        // ARABIC LETTER BEEH
        ('\u{067B}', ["\u{FB52}", "\u{FB54}", "\u{FB55}", "\u{FB53}"]),
        // ARABIC LETTER PEH
        ('\u{067E}', ["\u{FB56}", "\u{FB58}", "\u{FB59}", "\u{FB57}"]),
        // ARABIC LETTER TEHEH
        ('\u{067F}', ["\u{FB62}", "\u{FB64}", "\u{FB65}", "\u{FB63}"]),
        // ARABIC LETTER BEHEH
        ('\u{0680}', ["\u{FB5A}", "\u{FB5C}", "\u{FB5D}", "\u{FB5B}"]),
        // ARABIC LETTER NYEH
        ('\u{0683}', ["\u{FB76}", "\u{FB78}", "\u{FB79}", "\u{FB77}"]),
        // ARABIC LETTER DYEH
        ('\u{0684}', ["\u{FB72}", "\u{FB74}", "\u{FB75}", "\u{FB73}"]),
        // ARABIC LETTER TCHEH
        ('\u{0686}', ["\u{FB7A}", "\u{FB7C}", "\u{FB7D}", "\u{FB7B}"]),
        // ARABIC LETTER TCHEHEH
        ('\u{0687}', ["\u{FB7E}", "\u{FB80}", "\u{FB81}", "\u{FB7F}"]),
        // ARABIC LETTER DDAL
        ('\u{0688}', ["\u{FB88}", "", "", "\u{FB89}"]),
        // ARABIC LETTER DAHAL
        ('\u{068C}', ["\u{FB84}", "", "", "\u{FB85}"]),
        // ARABIC LETTER DDAHAL
        ('\u{068D}', ["\u{FB82}", "", "", "\u{FB83}"]),
        // ARABIC LETTER DUL
        ('\u{068E}', ["\u{FB86}", "", "", "\u{FB87}"]),
        // ARABIC LETTER RREH
        ('\u{0691}', ["\u{FB8C}", "", "", "\u{FB8D}"]),
        // ARABIC LETTER JEH
        ('\u{0698}', ["\u{FB8A}", "", "", "\u{FB8B}"]),
        // ARABIC LETTER VEH
        ('\u{06A4}', ["\u{FB6A}", "\u{FB6C}", "\u{FB6D}", "\u{FB6B}"]),
        // ARABIC LETTER PEHEH
        ('\u{06A6}', ["\u{FB6E}", "\u{FB70}", "\u{FB71}", "\u{FB6F}"]),
        // ARABIC LETTER KEHEH
        ('\u{06A9}', ["\u{FB8E}", "\u{FB90}", "\u{FB91}", "\u{FB8F}"]),
        // ARABIC LETTER NG
        ('\u{06AD}', ["\u{FBD3}", "\u{FBD5}", "\u{FBD6}", "\u{FBD4}"]),
        // ARABIC LETTER GAF
        ('\u{06AF}', ["\u{FB92}", "\u{FB94}", "\u{FB95}", "\u{FB93}"]),
        // ARABIC LETTER NGOEH
        ('\u{06B1}', ["\u{FB9A}", "\u{FB9C}", "\u{FB9D}", "\u{FB9B}"]),
        // ARABIC LETTER GUEH
        ('\u{06B3}', ["\u{FB96}", "\u{FB98}", "\u{FB99}", "\u{FB97}"]),
        // ARABIC LETTER NOON GHUNNA
        ('\u{06BA}', ["\u{FB9E}", "", "", "\u{FB9F}"]),
        // ARABIC LETTER RNOON
        ('\u{06BB}', ["\u{FBA0}", "\u{FBA2}", "\u{FBA3}", "\u{FBA1}"]),
        // ARABIC LETTER HEH DOACHASHMEE
        ('\u{06BE}', ["\u{FBAA}", "\u{FBAC}", "\u{FBAD}", "\u{FBAB}"]),
        // ARABIC LETTER HEH WITH YEH ABOVE
        ('\u{06C0}', ["\u{FBA4}", "", "", "\u{FBA5}"]),
        // ARABIC LETTER HEH GOAL
        ('\u{06C1}', ["\u{FBA6}", "\u{FBA8}", "\u{FBA9}", "\u{FBA7}"]),
        // ARABIC LETTER KIRGHIZ OE
        ('\u{06C5}', ["\u{FBE0}", "", "", "\u{FBE1}"]),
        // ARABIC LETTER OE
        ('\u{06C6}', ["\u{FBD9}", "", "", "\u{FBDA}"]),
        // ARABIC LETTER U
        ('\u{06C7}', ["\u{FBD7}", "", "", "\u{FBD8}"]),
        // ARABIC LETTER YU
        ('\u{06C8}', ["\u{FBDB}", "", "", "\u{FBDC}"]),
        // ARABIC LETTER KIRGHIZ YU
        ('\u{06C9}', ["\u{FBE2}", "", "", "\u{FBE3}"]),
        // ARABIC LETTER VE
        ('\u{06CB}', ["\u{FBDE}", "", "", "\u{FBDF}"]),
        // ARABIC LETTER FARSI YEH
        ('\u{06CC}', ["\u{FBFC}", "\u{FBFE}", "\u{FBFF}", "\u{FBFD}"]),
        // ARABIC LETTER E
        ('\u{06D0}', ["\u{FBE4}", "\u{FBE6}", "\u{FBE7}", "\u{FBE5}"]),
        // ARABIC LETTER YEH BARREE
        ('\u{06D2}', ["\u{FBAE}", "", "", "\u{FBAF}"]),
        // ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
        ('\u{06D3}', ["\u{FBB0}", "", "", "\u{FBB1}"]),
        // ZWJ
        (ZWJ, ["\u{200D}", "\u{200D}", "\u{200D}", "\u{200D}"]),
    ]
    .iter()
    .cloned()
    .collect()
});

pub fn connects_with_letter_before(letter: char) -> bool {
    if !LETTERS.contains_key(&letter) {
        return false;
    }
    let forms = LETTERS[&letter];

    if !forms[FINAL as usize].is_empty() || !forms[MEDIAL as usize].is_empty() {
        return true;
    }

    false
}

pub fn connects_with_letter_after(letter: char) -> bool {
    if !LETTERS.contains_key(&letter) {
        return false;
    }
    let forms = LETTERS[&letter];

    if !forms[INITIAL as usize].is_empty() || !forms[MEDIAL as usize].is_empty() {
        return true;
    }

    false
}

pub fn connects_with_letters_before_and_after(letter: char) -> bool {
    if !LETTERS.contains_key(&letter) {
        return false;
    }
    let forms = LETTERS[&letter];

    if !forms[MEDIAL as usize].is_empty() {
        return true;
    }

    false
}
