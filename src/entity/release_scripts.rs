/// The script used to write the release's track list. The possible values are taken from the
/// [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) standard.
///
/// The values for this enum have been generated with the following command:
///
/// ```bash
/// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=script%3A%22[^"]*%22' | \
///     sort | \
///     sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=script%3A%22\([^"]*\)%22,\/\/\/ \1\n\2\,,'
/// ```
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub enum ReleaseScript {
    /// Arabic
    Arab,
    /// Armenian
    Armn,
    /// Bengali
    Beng,
    /// Braille
    Brai,
    /// Buginese
    Bugi,
    /// Canadian Syllabics
    Cans,
    /// Cherokee
    Cher,
    /// Coptic
    Copt,
    /// Cuneiform, Sumero-Akkadian
    Xsux,
    /// Cyrillic
    Cyrl,
    /// Devanagari
    Deva,
    /// Egyptian hieroglyphs
    Egyp,
    /// Ethiopic
    Ethi,
    /// Georgian
    Geor,
    /// Glagolitic
    Glag,
    /// Greek
    Grek,
    /// Gujarati
    Gujr,
    /// Gurmukhi
    Guru,
    /// Hangul
    Hang,
    /// Han (Hanzi, Kanji, Hanja)
    Hani,
    /// Han (Simplified variant)
    Hans,
    /// Han (Traditional variant)
    Hant,
    /// Hebrew
    Hebr,
    /// Hiragana
    Hira,
    /// Japanese syllabaries
    Hrkt,
    /// Japanese
    Jpan,
    /// Kannada
    Knda,
    /// Katakana
    Kana,
    /// Khmer
    Khmr,
    /// Korean
    Kore,
    /// Lao
    Laoo,
    /// Latin
    Latn,
    /// Malayalam
    Mlym,
    /// Mathematical notation
    Zmth,
    /// [Multiple scripts]
    Qaaa,
    /// Myanmar
    Mymr,
    /// Old Turkic
    Orkh,
    /// Phags-pa
    Phag,
    /// Phoenician
    Phnx,
    /// Runic
    Runr,
    /// Shavian
    Shaw,
    /// Sinhala
    Sinh,
    /// Symbols
    Zsym,
    /// Syriac
    Syrc,
    /// Tamil
    Taml,
    /// Telugu
    Telu,
    /// Thai
    Thai,
    /// Tibetan
    Tibt,
    /// Vai
    Vaii,
    /// Yi
    Yiii,
}

impl ReleaseScript {
    /// Get the human-readable name used by MusicBrainz.
    ///
    /// The values for this enum have been generated with the following command:
    ///
    /// ```bash
    /// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
    ///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=script%3A%22[^"]*%22' | \
    ///     sort | \
    ///     sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=script%3A%22\([^"]*\)%22,            Self::\2 => "\1"\,,'
    /// ```
    pub fn name(&self) -> &'static str {
        match &self {
            Self::Arab => "Arabic",
            Self::Armn => "Armenian",
            Self::Beng => "Bengali",
            Self::Brai => "Braille",
            Self::Bugi => "Buginese",
            Self::Cans => "Canadian Syllabics",
            Self::Cher => "Cherokee",
            Self::Copt => "Coptic",
            Self::Xsux => "Cuneiform, Sumero-Akkadian",
            Self::Cyrl => "Cyrillic",
            Self::Deva => "Devanagari",
            Self::Egyp => "Egyptian hieroglyphs",
            Self::Ethi => "Ethiopic",
            Self::Geor => "Georgian",
            Self::Glag => "Glagolitic",
            Self::Grek => "Greek",
            Self::Gujr => "Gujarati",
            Self::Guru => "Gurmukhi",
            Self::Hang => "Hangul",
            Self::Hani => "Han (Hanzi, Kanji, Hanja)",
            Self::Hans => "Han (Simplified variant)",
            Self::Hant => "Han (Traditional variant)",
            Self::Hebr => "Hebrew",
            Self::Hira => "Hiragana",
            Self::Hrkt => "Japanese syllabaries",
            Self::Jpan => "Japanese",
            Self::Knda => "Kannada",
            Self::Kana => "Katakana",
            Self::Khmr => "Khmer",
            Self::Kore => "Korean",
            Self::Laoo => "Lao",
            Self::Latn => "Latin",
            Self::Mlym => "Malayalam",
            Self::Zmth => "Mathematical notation",
            Self::Qaaa => "[Multiple scripts]",
            Self::Mymr => "Myanmar",
            Self::Orkh => "Old Turkic",
            Self::Phag => "Phags-pa",
            Self::Phnx => "Phoenician",
            Self::Runr => "Runic",
            Self::Shaw => "Shavian",
            Self::Sinh => "Sinhala",
            Self::Zsym => "Symbols",
            Self::Syrc => "Syriac",
            Self::Taml => "Tamil",
            Self::Telu => "Telugu",
            Self::Thai => "Thai",
            Self::Tibt => "Tibetan",
            Self::Vaii => "Vai",
            Self::Yiii => "Yi",
        }
    }

    /// Get the [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) code as [`str`].
    ///
    /// The values for this enum have been generated with the following command:
    ///
    /// ```bash
    /// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
    ///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=script%3A%22[^"]*%22' | \
    ///     sort | \
    ///     sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=script%3A%22\([^"]*\)%22,            Self::\2 => "\2"\,,'
    /// ```
    pub fn code(&self) -> &'static str {
        match &self {
            Self::Arab => "Arab",
            Self::Armn => "Armn",
            Self::Beng => "Beng",
            Self::Brai => "Brai",
            Self::Bugi => "Bugi",
            Self::Cans => "Cans",
            Self::Cher => "Cher",
            Self::Copt => "Copt",
            Self::Xsux => "Xsux",
            Self::Cyrl => "Cyrl",
            Self::Deva => "Deva",
            Self::Egyp => "Egyp",
            Self::Ethi => "Ethi",
            Self::Geor => "Geor",
            Self::Glag => "Glag",
            Self::Grek => "Grek",
            Self::Gujr => "Gujr",
            Self::Guru => "Guru",
            Self::Hang => "Hang",
            Self::Hani => "Hani",
            Self::Hans => "Hans",
            Self::Hant => "Hant",
            Self::Hebr => "Hebr",
            Self::Hira => "Hira",
            Self::Hrkt => "Hrkt",
            Self::Jpan => "Jpan",
            Self::Knda => "Knda",
            Self::Kana => "Kana",
            Self::Khmr => "Khmr",
            Self::Kore => "Kore",
            Self::Laoo => "Laoo",
            Self::Latn => "Latn",
            Self::Mlym => "Mlym",
            Self::Zmth => "Zmth",
            Self::Qaaa => "Qaaa",
            Self::Mymr => "Mymr",
            Self::Orkh => "Orkh",
            Self::Phag => "Phag",
            Self::Phnx => "Phnx",
            Self::Runr => "Runr",
            Self::Shaw => "Shaw",
            Self::Sinh => "Sinh",
            Self::Zsym => "Zsym",
            Self::Syrc => "Syrc",
            Self::Taml => "Taml",
            Self::Telu => "Telu",
            Self::Thai => "Thai",
            Self::Tibt => "Tibt",
            Self::Vaii => "Vaii",
            Self::Yiii => "Yiii",
        }
    }
}
