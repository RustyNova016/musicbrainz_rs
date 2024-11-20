use chrono::NaiveDate;
use lucene_query_builder::QueryBuilder;
use serde::{Deserialize, Serialize};

use super::{Include, Relationship, Subquery};
use crate::date_format;
use crate::entity::alias::Alias;
use crate::entity::artist_credit::ArtistCredit;
use crate::entity::discid::Disc;
use crate::entity::genre::Genre;
use crate::entity::label::LabelInfo;
use crate::entity::recording::Recording;
use crate::entity::relations::Relation;
use crate::entity::release_group::ReleaseGroup;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use crate::query::browse::impl_browse_includes;
use crate::query::relations::impl_relations_includes;

/// A MusicBrainz release represents the unique release (i.e. issuing) of a product on a specific
/// date with specific release information such as the country, label, barcode and packaging.
/// If you walk into a store and purchase an album or single, they are each represented in
/// MusicBrainz as one release.
///
/// Each release belongs to a release group and contains at least one medium (commonly referred to
/// as a disc when talking about a CD release). Each medium has a tracklist.
/// A medium is the actual physical medium that stores the audio content. This means that each CD
/// in a multi-disc release will be entered as separate mediums within the release, and that both
/// sides of a vinyl record or cassette will exist on one medium. Mediums have a format (e.g. CD,
/// DVD, vinyl, and cassette) and can optionally also have a title. Sometimes a medium can be a
/// side of a disc. For example, the two sides of a hybrid SACD (the CD side and the SACD side)
/// should be entered as two mediums.
/// Tracklists represent the set and ordering of tracks as listed on a liner, and the same tracklist
/// can appear on more than one release. For example, a boxset compilation that contains previously
/// released CDs would share the same tracklists as the separate releases.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Release {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,

    /// The title of the release.
    pub title: String,

    #[serde(rename = "status-id")]
    pub status_id: Option<String>,

    /// The status describes how "official" a release is.
    pub status: Option<ReleaseStatus>,

    /// The date the release was issued.
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default)]
    pub date: Option<NaiveDate>,

    /// The country the release was issued in.
    pub country: Option<String>,

    ///  Data quality indicates how good the data for a release is. It is not a mark of how good or
    /// bad the music itself is - for that, use ratings.
    pub quality: Option<ReleaseQuality>,

    /// The barcode, if the release has one. The most common types found on releases are 12-digit
    /// UPCs and 13-digit EANs.
    pub barcode: Option<String>,

    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,

    #[serde(rename = "packaging-id")]
    pub packaging_id: Option<String>,

    /// The physical packaging that accompanies the release. See the
    /// [list of packaging](https://musicbrainz.org/doc/Release/Packaging) for more information.
    pub packaging: Option<ReleasePackaging>,

    pub relations: Option<Vec<Relation>>,
    /// The release group associated with this release.
    pub release_group: Option<ReleaseGroup>,
    /// Artist credits indicate who is the main credited artist (or artists) for releases, release
    /// groups, tracks and recordings, and how they are credited.
    pub artist_credit: Option<Vec<ArtistCredit>>,
    pub media: Option<Vec<Media>>,
    /// The label which issued the release. There may be more than one.
    pub label_info: Option<Vec<LabelInfo>>,
    pub tags: Option<Vec<Tag>>,
    /// Aliases are alternate names for a release.
    pub aliases: Option<Vec<Alias>>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,

    /// The [Amazon Standard Identification Number (ASIN)](https://musicbrainz.org/doc/ASIN) of the
    /// release.
    pub asin: Option<String>,

    /// The text representation on the release.
    pub text_representation: Option<ReleaseTextRepresentation>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ReleaseTextRepresentation {
    /// The language a release's track list is written in. The possible values are taken from the ISO
    /// 639-3 standard.
    pub language: Option<Language>,
    /// The script used to write the release's track list. The possible values are taken from the
    /// ISO 15924 standard.
    pub script: Option<ReleaseScript>,
}

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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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
    /// Gothic
    Goth,
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
    /// Latin (also known as Roman or, incorrectly, "English")
    ///
    /// Latin is the most common script, and usually the correct choice. It is used
    /// for all Western European languages, and many others. It is also the most common script used for transliterations.
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
    /// Oriya
    Orya,
    /// Phags-pa
    Phag,
    /// Runic
    Runr,
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
}

impl ReleaseScript {
    /// Get the human-readable name used by MusicBrainz.
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
            Self::Goth => "Gothic",
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
            Self::Orya => "Oriya",
            Self::Phag => "Phags-pa",
            Self::Runr => "Runic",
            Self::Sinh => "Sinhala",
            Self::Zsym => "Symbols",
            Self::Syrc => "Syriac",
            Self::Taml => "Tamil",
            Self::Telu => "Telugu",
            Self::Thai => "Thai",
            Self::Tibt => "Tibetan",
            Self::Vaii => "Vai",
        }
    }

    /// Get the [ISO 15924](https://en.wikipedia.org/wiki/ISO_15924) code as [`str`].
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
            Self::Goth => "Goth",
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
            Self::Orya => "Orya",
            Self::Phag => "Phag",
            Self::Runr => "Runr",
            Self::Sinh => "Sinh",
            Self::Zsym => "Zsym",
            Self::Syrc => "Syrc",
            Self::Taml => "Taml",
            Self::Telu => "Telu",
            Self::Thai => "Thai",
            Self::Tibt => "Tibt",
            Self::Vaii => "Vaii",
        }
    }
}

/// The language the release title and track titles are written in. The possible values are taken
/// from the [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
///
/// The values for this enum have been generated with the following command:
///
/// ```bash
/// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=lang%3A%22[^"]*%22' | \
///     sort | \
///     sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=lang%3A%22\([^"]*\)%22,\/\/\/ \1\n\u\2\,,'
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// Abkhazian
    Abk,
    /// Achinese
    Ace,
    /// Acoli
    Ach,
    /// Adangme
    Ada,
    /// Adyghe
    Ady,
    /// Afar
    Aar,
    /// Afrikaans
    Afr,
    /// Ainu
    Ain,
    /// Akan
    Aka,
    /// Akkadian
    Akk,
    /// Albanian
    Sqi,
    /// Algonquin
    Alq,
    /// Amharic
    Amh,
    /// Angika
    Anp,
    /// Arabic
    Ara,
    /// Aragonese
    Arg,
    /// Arapaho
    Arp,
    /// Ardhamāgadhī Prākrit
    Pka,
    /// Armenian
    Hye,
    /// Aromanian
    Rup,
    /// [Artificial (Other)]
    Qaa,
    /// Assamese
    Asm,
    /// Asturian
    Ast,
    /// Atikamekw
    Atj,
    /// Avaric
    Ava,
    /// Awadhi
    Awa,
    /// Aymara
    Aym,
    /// Azerbaijani
    Aze,
    /// Baeggu
    Bvd,
    /// Balinese
    Ban,
    /// Baluchi
    Bal,
    /// Bambara
    Bam,
    /// Basa
    Bas,
    /// Basque
    Eus,
    /// Bavarian
    Bar,
    /// Beja
    Bej,
    /// Belarusian
    Bel,
    /// Bemba
    Bem,
    /// Bengali
    Ben,
    /// Bhojpuri
    Bho,
    /// Bikol
    Bik,
    /// Bini
    Bin,
    /// Bislama
    Bis,
    /// Bosnian
    Bos,
    /// Braj
    Bra,
    /// Breton
    Bre,
    /// Buamu
    Box,
    /// Buginese
    Bug,
    /// Bulgarian
    Bul,
    /// Buriat
    Bua,
    /// Burmese
    Mya,
    /// Burushaski
    Bsk,
    /// Caddo
    Cad,
    /// Cajun French
    Frc,
    /// Catalan
    Cat,
    /// Cebuano
    Ceb,
    /// Celtiberian
    Xce,
    /// Central Okinawan
    Ryu,
    /// Central Yupik
    Esu,
    /// Chamorro
    Cha,
    /// Chechen
    Che,
    /// Cherokee
    Chr,
    /// Chichewa
    Nya,
    /// Chinese
    Zho,
    /// Church Slavic
    Chu,
    /// Chuvash
    Chv,
    /// Coptic
    Cop,
    /// Cornish
    Cor,
    /// Corsican
    Cos,
    /// Creek
    Mus,
    /// Cree
    Cre,
    /// Crimean Tatar
    Crh,
    /// Croatian
    Hrv,
    /// Czech
    Ces,
    /// Danish
    Dan,
    /// Delaware
    Del,
    /// Divehi
    Div,
    /// Duala
    Dua,
    /// Dutch, Middle (ca.1050-1350)
    Dum,
    /// Dutch
    Nld,
    /// Dzongkha
    Dzo,
    /// Eastern Arrernte
    Aer,
    /// Egyptian (Ancient)
    Egy,
    /// Elamite
    Elx,
    /// English, Middle (1100-1500)
    Enm,
    /// English, Old (ca.450-1100)
    Ang,
    /// English
    Eng,
    /// Erzya
    Myv,
    /// Esperanto
    Epo,
    /// Estonian
    Est,
    /// Ewe
    Ewe,
    /// Fang
    Fan,
    /// Fanti
    Fat,
    /// Faroese
    Fao,
    /// Fijian
    Fij,
    /// Filipino
    Fil,
    /// Finnish
    Fin,
    /// Fon
    Fon,
    /// French, Old (842-ca.1400)
    Fro,
    /// French
    Fra,
    /// Frisian, Eastern
    Frs,
    /// Frisian, Northern
    Frr,
    /// Frisian, Western
    Fry,
    /// Friulian
    Fur,
    /// Fulah
    Ful,
    /// Galician
    Glg,
    /// Ganda
    Lug,
    /// Garifuna
    Cab,
    /// Ga
    Gaa,
    /// Geez
    Gez,
    /// Georgian
    Kat,
    /// German, Low
    Nds,
    /// German, Middle High (ca.1050-1500)
    Gmh,
    /// German, Old High (ca.750-1050)
    Goh,
    /// German, Swiss
    Gsw,
    /// German
    Deu,
    /// Gondi
    Gon,
    /// Gothic
    Got,
    /// Greek, Ancient
    Grc,
    /// Greek
    Ell,
    /// Greenlandic
    Kal,
    /// Gronings
    Gos,
    /// Guadeloupean Creole French
    Gcf,
    /// Guarani
    Grn,
    /// Gujarati
    Guj,
    /// Gupapuyngu
    Guf,
    /// Guyanese Creole English
    Gyn,
    /// Haitian Creole
    Hat,
    /// Hausa
    Hau,
    /// Hawaiian
    Haw,
    /// Hebrew
    Heb,
    /// Herero
    Her,
    /// Hindi
    Hin,
    /// Hiri Motu
    Hmo,
    /// Hmong
    Hmn,
    /// Hungarian
    Hun,
    /// Icelandic
    Isl,
    /// Igbo
    Ibo,
    /// Iloko
    Ilo,
    /// Indonesian
    Ind,
    /// Ingrian
    Izh,
    /// Innu
    Moe,
    /// Inuktitut
    Iku,
    /// Irish
    Gle,
    /// Italian
    Ita,
    /// Jamaican Creole English
    Jam,
    /// Japanese
    Jpn,
    /// Javanese
    Jav,
    /// Jewish Babylonian Aramaic (ca. 200-1200 CE)
    Tmr,
    /// Kabardian
    Kbd,
    /// Kabuverdianu
    Kea,
    /// Kabyle
    Kab,
    /// Kalmyk
    Xal,
    /// Kannada
    Kan,
    /// Karachay-Balkar
    Krc,
    /// Karelian
    Krl,
    /// Kashmiri
    Kas,
    /// Kazakh
    Kaz,
    /// Khanty
    Kca,
    /// Khasi
    Kha,
    /// Khmer, Central
    Khm,
    /// Kikuyu
    Kik,
    /// Kimbundu
    Kmb,
    /// Kinyarwanda
    Kin,
    /// Kirghiz
    Kir,
    /// Klingon
    Tlh,
    /// Kölsch
    Ksh,
    /// Komi
    Kom,
    /// Kongo
    Kon,
    /// Konkani
    Kok,
    /// Korean
    Kor,
    /// Kunigami
    Xug,
    /// Kurdish
    Kur,
    /// Ladino
    Lad,
    /// Ladin
    Lld,
    /// Lakota
    Lkt,
    /// Lao
    Lao,
    /// Latin
    Lat,
    /// Latvian
    Lav,
    /// Laz
    Lzz,
    /// Limburgish
    Lim,
    /// Lingala
    Lin,
    /// Lithuanian
    Lit,
    /// Liv
    Liv,
    /// Lojban
    Jbo,
    /// Louisiana Creole French
    Lou,
    /// Luba-Katanga
    Lub,
    /// Luba-Lulua
    Lua,
    /// Luo
    Luo,
    /// Luxembourgish
    Ltz,
    /// Luyia
    Luy,
    /// Macedonian
    Mkd,
    /// Madurese
    Mad,
    /// Malagasy
    Mlg,
    /// Malayalam
    Mal,
    /// Malay
    Msa,
    /// Maltese
    Mlt,
    /// Manchu
    Mnc,
    /// Mandarin Chinese
    Cmn,
    /// Mandar
    Mdr,
    /// Mandingo
    Man,
    /// Mansi
    Mns,
    /// Manx
    Glv,
    /// Maori
    Mri,
    /// Mapudungun
    Arn,
    /// Marathi
    Mar,
    /// Mari
    Chm,
    /// Marwari
    Mwr,
    /// Mende
    Men,
    /// Mina (Cameroon)
    Hna,
    /// Min Nan Chinese
    Nan,
    /// Miyako
    Mvi,
    /// Mohawk
    Moh,
    /// Moksha
    Mdf,
    /// Mongolian
    Mon,
    /// Mongo
    Lol,
    /// Mossi
    Mos,
    /// [Multiple languages]
    Mul,
    /// Nauru
    Nau,
    /// Navajo
    Nav,
    /// Ndebele, North
    Nde,
    /// Ndebele, South
    Nbl,
    /// Ndonga
    Ndo,
    /// Neapolitan
    Nap,
    /// Nepal Bhasa
    New,
    /// Nepali
    Nep,
    /// Nhengatu
    Yrl,
    /// Nogai
    Nog,
    /// [No linguistic content]
    Zxx,
    /// Norn
    Nrn,
    /// Norse, Old
    Non,
    /// Norwegian Bokmål
    Nob,
    /// Norwegian Nynorsk
    Nno,
    /// Norwegian
    Nor,
    /// Nzima
    Nzi,
    /// Occitan
    Oci,
    /// Oriya
    Ori,
    /// Oromo
    Orm,
    /// Osage
    Osa,
    /// Pahlavi
    Pal,
    /// Papiamento
    Pap,
    /// Persian
    Fas,
    /// Pitjantjatjara
    Pjt,
    /// Pohnpeian
    Pon,
    /// Polish
    Pol,
    /// Portuguese
    Por,
    /// Provençal, Old (to 1500)
    Pro,
    /// Prussian
    Prg,
    /// Pulaar
    Fuc,
    /// Punjabi
    Pan,
    /// Pushto
    Pus,
    /// Puyuma
    Pyu,
    /// Quechua
    Que,
    /// Quenya
    Qya,
    /// Rajasthani
    Raj,
    /// Rapanui
    Rap,
    /// Rarotongan
    Rar,
    /// Réunion Creole French
    Rcf,
    /// Romanian
    Ron,
    /// Romansh
    Roh,
    /// Romany
    Rom,
    /// Rundi
    Run,
    /// Russian
    Rus,
    /// Rusyn
    Rue,
    /// Sami, Inari
    Smn,
    /// Sami, Lule
    Smj,
    /// Sami, Northern
    Sme,
    /// Sami, Skolt
    Sms,
    /// Sami, Southern
    Sma,
    /// Samoan
    Smo,
    /// Sango
    Sag,
    /// Sanskrit
    San,
    /// Santali
    Sat,
    /// Sardinian
    Srd,
    /// Scots
    Sco,
    /// Scottish Gaelic
    Gla,
    /// Sea Island Creole English
    Gul,
    /// Serbian
    Srp,
    /// Serer
    Srr,
    /// Shan
    Shn,
    /// Shona
    Sna,
    /// Sicilian
    Scn,
    /// Sindarin
    Sjn,
    /// Sindhi
    Snd,
    /// Sinhala
    Sin,
    /// Slovak
    Slk,
    /// Slovenian
    Slv,
    /// Somali
    Som,
    /// Soninke
    Snk,
    /// Sorbian, Upper
    Hsb,
    /// Sotho, Northern
    Nso,
    /// Sotho, Southern
    Sot,
    /// Southern Altai
    Alt,
    /// Spanish
    Spa,
    /// Sranan Tongo
    Srn,
    /// Sundanese
    Sun,
    /// Susu
    Sus,
    /// Svan
    Sva,
    /// Swahili
    Swa,
    /// Swati
    Ssw,
    /// Swedish
    Swe,
    /// Syriac
    Syr,
    /// Tagalog
    Tgl,
    /// Tahitian
    Tah,
    /// Tajik
    Tgk,
    /// Tamashek
    Tmh,
    /// Tamil
    Tam,
    /// Tatar
    Tat,
    /// Telugu
    Tel,
    /// Tetum
    Tet,
    /// Thai
    Tha,
    /// Tibetan
    Bod,
    /// Tigrinya
    Tir,
    /// Tokelau
    Tkl,
    /// Toki Pona
    Tok,
    /// Tok Pisin
    Tpi,
    /// Tonga (Tonga Islands)
    Ton,
    /// Tsonga
    Tso,
    /// Tswana
    Tsn,
    /// Turkish, Ottoman
    Ota,
    /// Turkish
    Tur,
    /// Turkmen
    Tuk,
    /// Tuvalu
    Tvl,
    /// Tuvinian
    Tyv,
    /// Twi
    Twi,
    /// Udmurt
    Udm,
    /// Uighur
    Uig,
    /// Ukrainian
    Ukr,
    /// Umbundu
    Umb,
    /// Ume Sami
    Sju,
    /// Urdu
    Urd,
    /// Uzbek
    Uzb,
    /// Vai
    Vai,
    /// Venda
    Ven,
    /// Veps
    Vep,
    /// Vietnamese
    Vie,
    /// Võro
    Vro,
    /// Votic
    Vot,
    /// Walloon
    Wln,
    /// Walser
    Wae,
    /// Warlpiri
    Wbp,
    /// Washo
    Was,
    /// Welsh
    Cym,
    /// Western Arrarnta
    Are,
    /// Wolaitta
    Wal,
    /// Wolof
    Wol,
    /// Wyandot
    Wya,
    /// Xhosa
    Xho,
    /// Yaeyama
    Rys,
    /// Yakut
    Sah,
    /// Yiddish
    Yid,
    /// Yoron
    Yox,
    /// Yoruba
    Yor,
    /// Yucateco
    Yua,
    /// Yue Chinese
    Yue,
    /// Zapotec
    Zap,
    /// Zarma
    Dje,
    /// Zaza
    Zza,
    /// Zulu
    Zul,
    /// Zuni
    Zun,
}

impl Language {
    /// Get the human-readable name used by MusicBrainz.
    pub fn name(&self) -> &'static str {
        match &self {
            Self::Abk => "Abkhazian",
            Self::Ace => "Achinese",
            Self::Ach => "Acoli",
            Self::Ada => "Adangme",
            Self::Ady => "Adyghe",
            Self::Aar => "Afar",
            Self::Afr => "Afrikaans",
            Self::Ain => "Ainu",
            Self::Aka => "Akan",
            Self::Akk => "Akkadian",
            Self::Sqi => "Albanian",
            Self::Alq => "Algonquin",
            Self::Amh => "Amharic",
            Self::Anp => "Angika",
            Self::Ara => "Arabic",
            Self::Arg => "Aragonese",
            Self::Arp => "Arapaho",
            Self::Pka => "Ardhamāgadhī Prākrit",
            Self::Hye => "Armenian",
            Self::Rup => "Aromanian",
            Self::Qaa => "[Artificial (Other)]",
            Self::Asm => "Assamese",
            Self::Ast => "Asturian",
            Self::Atj => "Atikamekw",
            Self::Ava => "Avaric",
            Self::Awa => "Awadhi",
            Self::Aym => "Aymara",
            Self::Aze => "Azerbaijani",
            Self::Bvd => "Baeggu",
            Self::Ban => "Balinese",
            Self::Bal => "Baluchi",
            Self::Bam => "Bambara",
            Self::Bas => "Basa",
            Self::Eus => "Basque",
            Self::Bar => "Bavarian",
            Self::Bej => "Beja",
            Self::Bel => "Belarusian",
            Self::Bem => "Bemba",
            Self::Ben => "Bengali",
            Self::Bho => "Bhojpuri",
            Self::Bik => "Bikol",
            Self::Bin => "Bini",
            Self::Bis => "Bislama",
            Self::Bos => "Bosnian",
            Self::Bra => "Braj",
            Self::Bre => "Breton",
            Self::Box => "Buamu",
            Self::Bug => "Buginese",
            Self::Bul => "Bulgarian",
            Self::Bua => "Buriat",
            Self::Mya => "Burmese",
            Self::Bsk => "Burushaski",
            Self::Cad => "Caddo",
            Self::Frc => "Cajun French",
            Self::Cat => "Catalan",
            Self::Ceb => "Cebuano",
            Self::Xce => "Celtiberian",
            Self::Ryu => "Central Okinawan",
            Self::Esu => "Central Yupik",
            Self::Cha => "Chamorro",
            Self::Che => "Chechen",
            Self::Chr => "Cherokee",
            Self::Nya => "Chichewa",
            Self::Zho => "Chinese",
            Self::Chu => "Church Slavic",
            Self::Chv => "Chuvash",
            Self::Cop => "Coptic",
            Self::Cor => "Cornish",
            Self::Cos => "Corsican",
            Self::Mus => "Creek",
            Self::Cre => "Cree",
            Self::Crh => "Crimean Tatar",
            Self::Hrv => "Croatian",
            Self::Ces => "Czech",
            Self::Dan => "Danish",
            Self::Del => "Delaware",
            Self::Div => "Divehi",
            Self::Dua => "Duala",
            Self::Dum => "Dutch, Middle (ca.1050-1350)",
            Self::Nld => "Dutch",
            Self::Dzo => "Dzongkha",
            Self::Aer => "Eastern Arrernte",
            Self::Egy => "Egyptian (Ancient)",
            Self::Elx => "Elamite",
            Self::Enm => "English, Middle (1100-1500)",
            Self::Ang => "English, Old (ca.450-1100)",
            Self::Eng => "English",
            Self::Myv => "Erzya",
            Self::Epo => "Esperanto",
            Self::Est => "Estonian",
            Self::Ewe => "Ewe",
            Self::Fan => "Fang",
            Self::Fat => "Fanti",
            Self::Fao => "Faroese",
            Self::Fij => "Fijian",
            Self::Fil => "Filipino",
            Self::Fin => "Finnish",
            Self::Fon => "Fon",
            Self::Fro => "French, Old (842-ca.1400)",
            Self::Fra => "French",
            Self::Frs => "Frisian, Eastern",
            Self::Frr => "Frisian, Northern",
            Self::Fry => "Frisian, Western",
            Self::Fur => "Friulian",
            Self::Ful => "Fulah",
            Self::Glg => "Galician",
            Self::Lug => "Ganda",
            Self::Cab => "Garifuna",
            Self::Gaa => "Ga",
            Self::Gez => "Geez",
            Self::Kat => "Georgian",
            Self::Nds => "German, Low",
            Self::Gmh => "German, Middle High (ca.1050-1500)",
            Self::Goh => "German, Old High (ca.750-1050)",
            Self::Gsw => "German, Swiss",
            Self::Deu => "German",
            Self::Gon => "Gondi",
            Self::Got => "Gothic",
            Self::Grc => "Greek, Ancient",
            Self::Ell => "Greek",
            Self::Kal => "Greenlandic",
            Self::Gos => "Gronings",
            Self::Gcf => "Guadeloupean Creole French",
            Self::Grn => "Guarani",
            Self::Guj => "Gujarati",
            Self::Guf => "Gupapuyngu",
            Self::Gyn => "Guyanese Creole English",
            Self::Hat => "Haitian Creole",
            Self::Hau => "Hausa",
            Self::Haw => "Hawaiian",
            Self::Heb => "Hebrew",
            Self::Her => "Herero",
            Self::Hin => "Hindi",
            Self::Hmo => "Hiri Motu",
            Self::Hmn => "Hmong",
            Self::Hun => "Hungarian",
            Self::Isl => "Icelandic",
            Self::Ibo => "Igbo",
            Self::Ilo => "Iloko",
            Self::Ind => "Indonesian",
            Self::Izh => "Ingrian",
            Self::Moe => "Innu",
            Self::Iku => "Inuktitut",
            Self::Gle => "Irish",
            Self::Ita => "Italian",
            Self::Jam => "Jamaican Creole English",
            Self::Jpn => "Japanese",
            Self::Jav => "Javanese",
            Self::Tmr => "Jewish Babylonian Aramaic (ca. 200-1200 CE)",
            Self::Kbd => "Kabardian",
            Self::Kea => "Kabuverdianu",
            Self::Kab => "Kabyle",
            Self::Xal => "Kalmyk",
            Self::Kan => "Kannada",
            Self::Krc => "Karachay-Balkar",
            Self::Krl => "Karelian",
            Self::Kas => "Kashmiri",
            Self::Kaz => "Kazakh",
            Self::Kca => "Khanty",
            Self::Kha => "Khasi",
            Self::Khm => "Khmer, Central",
            Self::Kik => "Kikuyu",
            Self::Kmb => "Kimbundu",
            Self::Kin => "Kinyarwanda",
            Self::Kir => "Kirghiz",
            Self::Tlh => "Klingon",
            Self::Ksh => "Kölsch",
            Self::Kom => "Komi",
            Self::Kon => "Kongo",
            Self::Kok => "Konkani",
            Self::Kor => "Korean",
            Self::Xug => "Kunigami",
            Self::Kur => "Kurdish",
            Self::Lad => "Ladino",
            Self::Lld => "Ladin",
            Self::Lkt => "Lakota",
            Self::Lao => "Lao",
            Self::Lat => "Latin",
            Self::Lav => "Latvian",
            Self::Lzz => "Laz",
            Self::Lim => "Limburgish",
            Self::Lin => "Lingala",
            Self::Lit => "Lithuanian",
            Self::Liv => "Liv",
            Self::Jbo => "Lojban",
            Self::Lou => "Louisiana Creole French",
            Self::Lub => "Luba-Katanga",
            Self::Lua => "Luba-Lulua",
            Self::Luo => "Luo",
            Self::Ltz => "Luxembourgish",
            Self::Luy => "Luyia",
            Self::Mkd => "Macedonian",
            Self::Mad => "Madurese",
            Self::Mlg => "Malagasy",
            Self::Mal => "Malayalam",
            Self::Msa => "Malay",
            Self::Mlt => "Maltese",
            Self::Mnc => "Manchu",
            Self::Cmn => "Mandarin Chinese",
            Self::Mdr => "Mandar",
            Self::Man => "Mandingo",
            Self::Mns => "Mansi",
            Self::Glv => "Manx",
            Self::Mri => "Maori",
            Self::Arn => "Mapudungun",
            Self::Mar => "Marathi",
            Self::Chm => "Mari",
            Self::Mwr => "Marwari",
            Self::Men => "Mende",
            Self::Hna => "Mina (Cameroon)",
            Self::Nan => "Min Nan Chinese",
            Self::Mvi => "Miyako",
            Self::Moh => "Mohawk",
            Self::Mdf => "Moksha",
            Self::Mon => "Mongolian",
            Self::Lol => "Mongo",
            Self::Mos => "Mossi",
            Self::Mul => "[Multiple languages]",
            Self::Nau => "Nauru",
            Self::Nav => "Navajo",
            Self::Nde => "Ndebele, North",
            Self::Nbl => "Ndebele, South",
            Self::Ndo => "Ndonga",
            Self::Nap => "Neapolitan",
            Self::New => "Nepal Bhasa",
            Self::Nep => "Nepali",
            Self::Yrl => "Nhengatu",
            Self::Nog => "Nogai",
            Self::Zxx => "[No linguistic content]",
            Self::Nrn => "Norn",
            Self::Non => "Norse, Old",
            Self::Nob => "Norwegian Bokmål",
            Self::Nno => "Norwegian Nynorsk",
            Self::Nor => "Norwegian",
            Self::Nzi => "Nzima",
            Self::Oci => "Occitan",
            Self::Ori => "Oriya",
            Self::Orm => "Oromo",
            Self::Osa => "Osage",
            Self::Pal => "Pahlavi",
            Self::Pap => "Papiamento",
            Self::Fas => "Persian",
            Self::Pjt => "Pitjantjatjara",
            Self::Pon => "Pohnpeian",
            Self::Pol => "Polish",
            Self::Por => "Portuguese",
            Self::Pro => "Provençal, Old (to 1500)",
            Self::Prg => "Prussian",
            Self::Fuc => "Pulaar",
            Self::Pan => "Punjabi",
            Self::Pus => "Pushto",
            Self::Pyu => "Puyuma",
            Self::Que => "Quechua",
            Self::Qya => "Quenya",
            Self::Raj => "Rajasthani",
            Self::Rap => "Rapanui",
            Self::Rar => "Rarotongan",
            Self::Rcf => "Réunion Creole French",
            Self::Ron => "Romanian",
            Self::Roh => "Romansh",
            Self::Rom => "Romany",
            Self::Run => "Rundi",
            Self::Rus => "Russian",
            Self::Rue => "Rusyn",
            Self::Smn => "Sami, Inari",
            Self::Smj => "Sami, Lule",
            Self::Sme => "Sami, Northern",
            Self::Sms => "Sami, Skolt",
            Self::Sma => "Sami, Southern",
            Self::Smo => "Samoan",
            Self::Sag => "Sango",
            Self::San => "Sanskrit",
            Self::Sat => "Santali",
            Self::Srd => "Sardinian",
            Self::Sco => "Scots",
            Self::Gla => "Scottish Gaelic",
            Self::Gul => "Sea Island Creole English",
            Self::Srp => "Serbian",
            Self::Srr => "Serer",
            Self::Shn => "Shan",
            Self::Sna => "Shona",
            Self::Scn => "Sicilian",
            Self::Sjn => "Sindarin",
            Self::Snd => "Sindhi",
            Self::Sin => "Sinhala",
            Self::Slk => "Slovak",
            Self::Slv => "Slovenian",
            Self::Som => "Somali",
            Self::Snk => "Soninke",
            Self::Hsb => "Sorbian, Upper",
            Self::Nso => "Sotho, Northern",
            Self::Sot => "Sotho, Southern",
            Self::Alt => "Southern Altai",
            Self::Spa => "Spanish",
            Self::Srn => "Sranan Tongo",
            Self::Sun => "Sundanese",
            Self::Sus => "Susu",
            Self::Sva => "Svan",
            Self::Swa => "Swahili",
            Self::Ssw => "Swati",
            Self::Swe => "Swedish",
            Self::Syr => "Syriac",
            Self::Tgl => "Tagalog",
            Self::Tah => "Tahitian",
            Self::Tgk => "Tajik",
            Self::Tmh => "Tamashek",
            Self::Tam => "Tamil",
            Self::Tat => "Tatar",
            Self::Tel => "Telugu",
            Self::Tet => "Tetum",
            Self::Tha => "Thai",
            Self::Bod => "Tibetan",
            Self::Tir => "Tigrinya",
            Self::Tkl => "Tokelau",
            Self::Tok => "Toki Pona",
            Self::Tpi => "Tok Pisin",
            Self::Ton => "Tonga (Tonga Islands)",
            Self::Tso => "Tsonga",
            Self::Tsn => "Tswana",
            Self::Ota => "Turkish, Ottoman",
            Self::Tur => "Turkish",
            Self::Tuk => "Turkmen",
            Self::Tvl => "Tuvalu",
            Self::Tyv => "Tuvinian",
            Self::Twi => "Twi",
            Self::Udm => "Udmurt",
            Self::Uig => "Uighur",
            Self::Ukr => "Ukrainian",
            Self::Umb => "Umbundu",
            Self::Sju => "Ume Sami",
            Self::Urd => "Urdu",
            Self::Uzb => "Uzbek",
            Self::Vai => "Vai",
            Self::Ven => "Venda",
            Self::Vep => "Veps",
            Self::Vie => "Vietnamese",
            Self::Vro => "Võro",
            Self::Vot => "Votic",
            Self::Wln => "Walloon",
            Self::Wae => "Walser",
            Self::Wbp => "Warlpiri",
            Self::Was => "Washo",
            Self::Cym => "Welsh",
            Self::Are => "Western Arrarnta",
            Self::Wal => "Wolaitta",
            Self::Wol => "Wolof",
            Self::Wya => "Wyandot",
            Self::Xho => "Xhosa",
            Self::Rys => "Yaeyama",
            Self::Sah => "Yakut",
            Self::Yid => "Yiddish",
            Self::Yox => "Yoron",
            Self::Yor => "Yoruba",
            Self::Yua => "Yucateco",
            Self::Yue => "Yue Chinese",
            Self::Zap => "Zapotec",
            Self::Dje => "Zarma",
            Self::Zza => "Zaza",
            Self::Zul => "Zulu",
            Self::Zun => "Zuni",
        }
    }

    /// Get the [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) code as [`str`].
    pub fn code(&self) -> &'static str {
        match &self {
            Self::Abk => "abk",
            Self::Ace => "ace",
            Self::Ach => "ach",
            Self::Ada => "ada",
            Self::Ady => "ady",
            Self::Aar => "aar",
            Self::Afr => "afr",
            Self::Ain => "ain",
            Self::Aka => "aka",
            Self::Akk => "akk",
            Self::Sqi => "sqi",
            Self::Alq => "alq",
            Self::Amh => "amh",
            Self::Anp => "anp",
            Self::Ara => "ara",
            Self::Arg => "arg",
            Self::Arp => "arp",
            Self::Pka => "pka",
            Self::Hye => "hye",
            Self::Rup => "rup",
            Self::Qaa => "qaa",
            Self::Asm => "asm",
            Self::Ast => "ast",
            Self::Atj => "atj",
            Self::Ava => "ava",
            Self::Awa => "awa",
            Self::Aym => "aym",
            Self::Aze => "aze",
            Self::Bvd => "bvd",
            Self::Ban => "ban",
            Self::Bal => "bal",
            Self::Bam => "bam",
            Self::Bas => "bas",
            Self::Eus => "eus",
            Self::Bar => "bar",
            Self::Bej => "bej",
            Self::Bel => "bel",
            Self::Bem => "bem",
            Self::Ben => "ben",
            Self::Bho => "bho",
            Self::Bik => "bik",
            Self::Bin => "bin",
            Self::Bis => "bis",
            Self::Bos => "bos",
            Self::Bra => "bra",
            Self::Bre => "bre",
            Self::Box => "box",
            Self::Bug => "bug",
            Self::Bul => "bul",
            Self::Bua => "bua",
            Self::Mya => "mya",
            Self::Bsk => "bsk",
            Self::Cad => "cad",
            Self::Frc => "frc",
            Self::Cat => "cat",
            Self::Ceb => "ceb",
            Self::Xce => "xce",
            Self::Ryu => "ryu",
            Self::Esu => "esu",
            Self::Cha => "cha",
            Self::Che => "che",
            Self::Chr => "chr",
            Self::Nya => "nya",
            Self::Zho => "zho",
            Self::Chu => "chu",
            Self::Chv => "chv",
            Self::Cop => "cop",
            Self::Cor => "cor",
            Self::Cos => "cos",
            Self::Mus => "mus",
            Self::Cre => "cre",
            Self::Crh => "crh",
            Self::Hrv => "hrv",
            Self::Ces => "ces",
            Self::Dan => "dan",
            Self::Del => "del",
            Self::Div => "div",
            Self::Dua => "dua",
            Self::Dum => "dum",
            Self::Nld => "nld",
            Self::Dzo => "dzo",
            Self::Aer => "aer",
            Self::Egy => "egy",
            Self::Elx => "elx",
            Self::Enm => "enm",
            Self::Ang => "ang",
            Self::Eng => "eng",
            Self::Myv => "myv",
            Self::Epo => "epo",
            Self::Est => "est",
            Self::Ewe => "ewe",
            Self::Fan => "fan",
            Self::Fat => "fat",
            Self::Fao => "fao",
            Self::Fij => "fij",
            Self::Fil => "fil",
            Self::Fin => "fin",
            Self::Fon => "fon",
            Self::Fro => "fro",
            Self::Fra => "fra",
            Self::Frs => "frs",
            Self::Frr => "frr",
            Self::Fry => "fry",
            Self::Fur => "fur",
            Self::Ful => "ful",
            Self::Glg => "glg",
            Self::Lug => "lug",
            Self::Cab => "cab",
            Self::Gaa => "gaa",
            Self::Gez => "gez",
            Self::Kat => "kat",
            Self::Nds => "nds",
            Self::Gmh => "gmh",
            Self::Goh => "goh",
            Self::Gsw => "gsw",
            Self::Deu => "deu",
            Self::Gon => "gon",
            Self::Got => "got",
            Self::Grc => "grc",
            Self::Ell => "ell",
            Self::Kal => "kal",
            Self::Gos => "gos",
            Self::Gcf => "gcf",
            Self::Grn => "grn",
            Self::Guj => "guj",
            Self::Guf => "guf",
            Self::Gyn => "gyn",
            Self::Hat => "hat",
            Self::Hau => "hau",
            Self::Haw => "haw",
            Self::Heb => "heb",
            Self::Her => "her",
            Self::Hin => "hin",
            Self::Hmo => "hmo",
            Self::Hmn => "hmn",
            Self::Hun => "hun",
            Self::Isl => "isl",
            Self::Ibo => "ibo",
            Self::Ilo => "ilo",
            Self::Ind => "ind",
            Self::Izh => "izh",
            Self::Moe => "moe",
            Self::Iku => "iku",
            Self::Gle => "gle",
            Self::Ita => "ita",
            Self::Jam => "jam",
            Self::Jpn => "jpn",
            Self::Jav => "jav",
            Self::Tmr => "tmr",
            Self::Kbd => "kbd",
            Self::Kea => "kea",
            Self::Kab => "kab",
            Self::Xal => "xal",
            Self::Kan => "kan",
            Self::Krc => "krc",
            Self::Krl => "krl",
            Self::Kas => "kas",
            Self::Kaz => "kaz",
            Self::Kca => "kca",
            Self::Kha => "kha",
            Self::Khm => "khm",
            Self::Kik => "kik",
            Self::Kmb => "kmb",
            Self::Kin => "kin",
            Self::Kir => "kir",
            Self::Tlh => "tlh",
            Self::Ksh => "ksh",
            Self::Kom => "kom",
            Self::Kon => "kon",
            Self::Kok => "kok",
            Self::Kor => "kor",
            Self::Xug => "xug",
            Self::Kur => "kur",
            Self::Lad => "lad",
            Self::Lld => "lld",
            Self::Lkt => "lkt",
            Self::Lao => "lao",
            Self::Lat => "lat",
            Self::Lav => "lav",
            Self::Lzz => "lzz",
            Self::Lim => "lim",
            Self::Lin => "lin",
            Self::Lit => "lit",
            Self::Liv => "liv",
            Self::Jbo => "jbo",
            Self::Lou => "lou",
            Self::Lub => "lub",
            Self::Lua => "lua",
            Self::Luo => "luo",
            Self::Ltz => "ltz",
            Self::Luy => "luy",
            Self::Mkd => "mkd",
            Self::Mad => "mad",
            Self::Mlg => "mlg",
            Self::Mal => "mal",
            Self::Msa => "msa",
            Self::Mlt => "mlt",
            Self::Mnc => "mnc",
            Self::Cmn => "cmn",
            Self::Mdr => "mdr",
            Self::Man => "man",
            Self::Mns => "mns",
            Self::Glv => "glv",
            Self::Mri => "mri",
            Self::Arn => "arn",
            Self::Mar => "mar",
            Self::Chm => "chm",
            Self::Mwr => "mwr",
            Self::Men => "men",
            Self::Hna => "hna",
            Self::Nan => "nan",
            Self::Mvi => "mvi",
            Self::Moh => "moh",
            Self::Mdf => "mdf",
            Self::Mon => "mon",
            Self::Lol => "lol",
            Self::Mos => "mos",
            Self::Mul => "mul",
            Self::Nau => "nau",
            Self::Nav => "nav",
            Self::Nde => "nde",
            Self::Nbl => "nbl",
            Self::Ndo => "ndo",
            Self::Nap => "nap",
            Self::New => "new",
            Self::Nep => "nep",
            Self::Yrl => "yrl",
            Self::Nog => "nog",
            Self::Zxx => "zxx",
            Self::Nrn => "nrn",
            Self::Non => "non",
            Self::Nob => "nob",
            Self::Nno => "nno",
            Self::Nor => "nor",
            Self::Nzi => "nzi",
            Self::Oci => "oci",
            Self::Ori => "ori",
            Self::Orm => "orm",
            Self::Osa => "osa",
            Self::Pal => "pal",
            Self::Pap => "pap",
            Self::Fas => "fas",
            Self::Pjt => "pjt",
            Self::Pon => "pon",
            Self::Pol => "pol",
            Self::Por => "por",
            Self::Pro => "pro",
            Self::Prg => "prg",
            Self::Fuc => "fuc",
            Self::Pan => "pan",
            Self::Pus => "pus",
            Self::Pyu => "pyu",
            Self::Que => "que",
            Self::Qya => "qya",
            Self::Raj => "raj",
            Self::Rap => "rap",
            Self::Rar => "rar",
            Self::Rcf => "rcf",
            Self::Ron => "ron",
            Self::Roh => "roh",
            Self::Rom => "rom",
            Self::Run => "run",
            Self::Rus => "rus",
            Self::Rue => "rue",
            Self::Smn => "smn",
            Self::Smj => "smj",
            Self::Sme => "sme",
            Self::Sms => "sms",
            Self::Sma => "sma",
            Self::Smo => "smo",
            Self::Sag => "sag",
            Self::San => "san",
            Self::Sat => "sat",
            Self::Srd => "srd",
            Self::Sco => "sco",
            Self::Gla => "gla",
            Self::Gul => "gul",
            Self::Srp => "srp",
            Self::Srr => "srr",
            Self::Shn => "shn",
            Self::Sna => "sna",
            Self::Scn => "scn",
            Self::Sjn => "sjn",
            Self::Snd => "snd",
            Self::Sin => "sin",
            Self::Slk => "slk",
            Self::Slv => "slv",
            Self::Som => "som",
            Self::Snk => "snk",
            Self::Hsb => "hsb",
            Self::Nso => "nso",
            Self::Sot => "sot",
            Self::Alt => "alt",
            Self::Spa => "spa",
            Self::Srn => "srn",
            Self::Sun => "sun",
            Self::Sus => "sus",
            Self::Sva => "sva",
            Self::Swa => "swa",
            Self::Ssw => "ssw",
            Self::Swe => "swe",
            Self::Syr => "syr",
            Self::Tgl => "tgl",
            Self::Tah => "tah",
            Self::Tgk => "tgk",
            Self::Tmh => "tmh",
            Self::Tam => "tam",
            Self::Tat => "tat",
            Self::Tel => "tel",
            Self::Tet => "tet",
            Self::Tha => "tha",
            Self::Bod => "bod",
            Self::Tir => "tir",
            Self::Tkl => "tkl",
            Self::Tok => "tok",
            Self::Tpi => "tpi",
            Self::Ton => "ton",
            Self::Tso => "tso",
            Self::Tsn => "tsn",
            Self::Ota => "ota",
            Self::Tur => "tur",
            Self::Tuk => "tuk",
            Self::Tvl => "tvl",
            Self::Tyv => "tyv",
            Self::Twi => "twi",
            Self::Udm => "udm",
            Self::Uig => "uig",
            Self::Ukr => "ukr",
            Self::Umb => "umb",
            Self::Sju => "sju",
            Self::Urd => "urd",
            Self::Uzb => "uzb",
            Self::Vai => "vai",
            Self::Ven => "ven",
            Self::Vep => "vep",
            Self::Vie => "vie",
            Self::Vro => "vro",
            Self::Vot => "vot",
            Self::Wln => "wln",
            Self::Wae => "wae",
            Self::Wbp => "wbp",
            Self::Was => "was",
            Self::Cym => "cym",
            Self::Are => "are",
            Self::Wal => "wal",
            Self::Wol => "wol",
            Self::Wya => "wya",
            Self::Xho => "xho",
            Self::Rys => "rys",
            Self::Sah => "sah",
            Self::Yid => "yid",
            Self::Yox => "yox",
            Self::Yor => "yor",
            Self::Yua => "yua",
            Self::Yue => "yue",
            Self::Zap => "zap",
            Self::Dje => "dje",
            Self::Zza => "zza",
            Self::Zul => "zul",
            Self::Zun => "zun",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "lowercase"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "lowercase"))]
pub enum ReleaseQuality {
    /// The release needs serious fixes, or its existence is hard to prove (but it's not clearly fake).
    Low,

    /// All available data has been added, if possible including cover art with liner info that
    /// proves it.
    High,

    /// This is the default setting - technically "unknown" if the quality has never been modified,
    /// "normal" if it has.
    Normal,

    Unknown,

    None,
}

/// The release status describes how "official" a release is.
/// Note that this enum is `non_exhaustive`; The list of release types is subject to change and
/// these changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `release_status` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ReleaseStatus {
    /// Any release officially sanctioned by the artist and/or their record company. Most releases
    /// will fit into this category.
    Official,
    /// A give-away release or a release intended to promote an upcoming official release (e.g.
    /// pre-release versions, releases included with a magazine, versions supplied to radio DJs
    /// for air-play).
    Promotion,
    /// An unofficial/underground release that was not sanctioned by the artist and/or the record
    /// company. This includes unofficial live recordings and pirated releases.
    Bootleg,
    /// An alternate version of a release where the titles have been changed. These don't correspond
    /// to any real release and should be linked to the original release using the transl(iter)ation
    /// [transl(iter)ation relationship](https://musicbrainz.org/relationship/fc399d47-23a7-4c28-bfcf-0607a562b644).
    #[serde(rename = "Pseudo-Release")]
    PseudoRelease,
    /// Any release_status that does not yet have a corresponding variant in this enum.
    /// If you ever see a `ReleaseStatus::UnrecognizedReleaseStatus` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedReleaseStatus,
}

/// The type of packaging of a MusicBrainz release entity.
/// Note that this enum is `non_exhaustive`; The list of release types is subject to change and
/// these changes are only reflected in the DB, not in actual MB code.
/// Variants are derived from the `release_packaging` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ReleasePackaging {
    Book,
    Box,
    #[serde(rename = "Cardboard/Paper Sleeve")]
    CardboardPaperSleeve,
    #[serde(rename = "Cassette Case")]
    CassetteCase,
    /// A perfect bound book with a sleeve at the end to hold a CD
    Digibook,
    Digipak,
    #[serde(rename = "Discbox Slider")]
    DiscboxSlider,
    Fatbox,
    #[serde(rename = "Gatefold Cover")]
    GatefoldCover,
    /// The traditional CD case, made of hard, brittle plastic.
    #[serde(rename = "Jewel Case")]
    JewelCase,
    #[serde(rename = "Keep Case")]
    KeepCase,
    #[serde(rename = "Plastic Sleeve")]
    PlasticSleeve,
    /// Plastic CD tray inside a cardboard slipcover
    Slidepack,
    /// A thinner jewel case, commonly used for CD singles.
    #[serde(rename = "Slim Jewel Case")]
    SlimJewelCase,
    #[serde(rename = "Snap Case")]
    SnapCase,
    /// Japanese case that holds an 8cm CD. It is rectangular but can be snapped to make it more
    /// compact (hence the name).
    #[serde(rename = "SnapPack")]
    Snappack,
    #[serde(rename = "Super Jewel Box")]
    SuperJewelBox,
    Other,
    None,
    /// Any release_packaging that does not yet have a corresponding variant in this enum.
    /// If you ever see a `ReleasePackaging::UnrecognizedReleasePackaging` in the wild, let us know and file an issue/pull request!
    #[serde(other)]
    UnrecognizedReleasePackaging,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Media {
    pub discs: Option<Vec<Disc>>,
    pub title: Option<String>,
    pub position: Option<u32>,
    pub track_count: u32,
    pub disc_count: Option<u32>,
    pub format_id: Option<String>,
    pub format: Option<String>,
    pub tracks: Option<Vec<Track>>,
    pub track_offset: Option<u32>,
}

/// A track is the way a recording is represented on a particular release (or, more exactly, on a
/// particular medium).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Track {
    pub recording: Option<Recording>,
    pub title: String,
    pub number: String,
    pub length: Option<u32>,
    pub position: u32,
    pub id: String,
    pub artist_credit: Option<Vec<ArtistCredit>>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct ReleaseSearchQuery {
    /// (part of) any alias attached to the release group (diacritics are ignored)
    alias: String,
    /// the MBID of any of the release group artists
    arid: String,
    /// (part of) the combined credited artist name for the release group, including join phrases (e.g. "Artist X feat.")
    artist: String,
    /// (part of) the name of any of the release group artists
    #[query_builder_field = "artistname"]
    artist_name: String,
    /// an Amazon ASIN for the release
    asin: String,
    /// the barcode for the release
    barcode: String,
    /// any catalog number for this release (insensitive to case, spaces and separators)
    #[query_builder_field = "catno"]
    catalog_number: String,
    /// (part of) the release group's disambiguation comment
    comment: String,
    /// the 2-letter code (ISO 3166-1 alpha-2) for any country the release was released in
    country: String,
    /// (part of) the credited name of any of the release group artists on this particular release group
    #[query_builder_field = "creditname"]
    credit_name: String,
    /// a release date for the release (e.g. "1980-01-22")
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    #[serde(default)]
    date: Option<NaiveDate>,
    /// the total number of disc IDs attached to all mediums on the release
    discids: u32,
    /// the number of disc IDs attached to any one medium on the release
    #[query_builder_field = "discidsmedium"]
    discids_medium: u32,
    /// the format of any medium in the release (insensitive to case, spaces, and separators)
    format: String,
    /// the MBID of any of the release labels
    laid: String,
    /// (part of) the name of any of the release labels
    label: String,
    /// the ISO 639-3 code for the release language
    lang: String,
    /// the number of mediums on the release
    mediums: u32,
    /// the format of the release (insensitive to case, spaces, and separators)
    packaging: String,
    /// the primary type of the release group
    #[query_builder_field = "primarytype"]
    primary_type: String,
    /// the listed quality of the data for the release (one of "low", "normal", "high")
    quality: String,
    /// the MBID of any of the releases in the release group
    reid: String,
    /// (part of) the title of any of the releases in the release group
    release: String,
    /// (part of) the release's title (with the specified diacritics)
    #[query_builder_field = "releaseaccent"]
    release_accent: String,
    /// the release group's MBID
    rgid: String,
    /// the ISO 15924 code for the release script
    script: String,
    /// any of the secondary types of the release group
    #[query_builder_field = "secondarytype"]
    secondary_type: String,
    /// the status of any of the releases in the release group
    status: String,
    /// the status of any of the releases in the release group
    tag: String,
    /// the total number of tracks on the release
    tracks: u32,
    /// the number of tracks on any one medium on the release
    #[query_builder_field = "tracksmedium"]
    tracks_medium: u32,
    /// legacy release group type field that predates the ability to set multiple types (see calculation code)
    #[query_builder_field = "type"]
    release_type: String,
}

impl_browse! {
Release,
   (by_area, BrowseBy::Area),
   (by_artist, BrowseBy::Artist),
   (by_label, BrowseBy::Label),
   (by_track, BrowseBy::Track),
   (by_track_artist, BrowseBy::TrackArtist),
   (by_recording, BrowseBy::Recording),
   (by_release_group, BrowseBy::ReleaseGroup),
   (by_collection, BrowseBy::Collection)
}

impl_browse_includes!(
    Release,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_artist_credits, Include::Other("artist-credits")),
    (with_labels, Include::Other("labels")),
    (with_recordings, Include::Other("recordings")),
    (with_release_groups, Include::Other("release-groups")),
    (with_medias, Include::Other("media")),
    (with_discids, Include::Other("discids")),
    (with_isrcs, Include::Other("isrcs"))
);

impl_includes!(
    Release,
    (with_artists, Include::Subquery(Subquery::Artists)),
    (with_labels, Include::Subquery(Subquery::Labels)),
    (
        with_work_level_relations,
        Include::Relationship(Relationship::WorkLevel)
    ),
    (
        with_release_group_level_relations,
        Include::Relationship(Relationship::ReleaseGroupLevel)
    ),
    (
        with_recording_level_relations,
        Include::Relationship(Relationship::RecordingLevel)
    ),
    (with_recordings, Include::Subquery(Subquery::Recordings)),
    (
        with_release_groups,
        Include::Subquery(Subquery::ReleaseGroups)
    ),
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations)),
    (
        with_artist_credits,
        Include::Subquery(Subquery::ArtistCredits)
    )
);

// Relationships includes
impl_relations_includes!(Release);
