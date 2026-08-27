/// The language the release title and track titles are written in. The possible values are taken
/// from the [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
///
/// The values for this enum have been generated with the following command:
///
/// ```bash
/// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=lang%3A%22[^"]*%22' | \
///     sort | \
///     sed -e 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=lang%3A%22\([^"]*\)%22,\/\/\/ \1\n\u\2\,,' -e "s/&#x27;/'/"
/// ```
#[non_exhaustive]
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// Abkhazian
    Abk,
    /// Achinese
    Ace,
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
    /// Bashkir
    Bak,
    /// Basque
    Eus,
    /// Bavarian
    Bar,
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
    /// Bodo (India)
    Brx,
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
    /// Chichewa
    Nya,
    /// Chinese
    Zho,
    /// Church Slavic
    Chu,
    /// Chuvash
    Chv,
    /// Classical Nahuatl
    Nci,
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
    /// Dakota
    Dak,
    /// Danish
    Dan,
    /// Dinka
    Din,
    /// Divehi
    Div,
    /// Dogri
    Doi,
    /// Duala
    Dua,
    /// Dutch, Middle (ca.1050-1350)
    Dum,
    /// Dutch
    Nld,
    /// Dyula
    Dyu,
    /// Dzongkha
    Dzo,
    /// Eastern Arrernte
    Aer,
    /// Egyptian (Ancient)
    Egy,
    /// Emilian
    Egl,
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
    /// Hiligaynon
    Hil,
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
    /// Interslavic
    Isv,
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
    /// Judeo-Arabic
    Jrb,
    /// Judeo-Persian
    Jpr,
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
    /// Kashubian
    Csb,
    /// Kazakh
    Kaz,
    /// Kedah Malay
    Meo,
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
    /// Kituba (Congo)
    Mkw,
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
    /// Magahi
    Mag,
    /// Maithili
    Mai,
    /// Malagasy
    Mlg,
    /// Malayalam
    Mal,
    /// Malay
    Msa,
    /// Malecite-Passamaquoddy
    Pqm,
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
    /// Mirandese
    Mwl,
    /// Mi'kmaq
    Mic,
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
    /// Morisyen
    Mfe,
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
    /// Negeri Sembilan Malay
    Zmi,
    /// Nepal Bhasa
    New,
    /// Nepali
    Nep,
    /// Ngad'a
    Nxg,
    /// Nhengatu
    Yrl,
    /// Niuean
    Niu,
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
    /// Nyankole
    Nyn,
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
    /// Pampanga
    Pam,
    /// Papiamento
    Pap,
    /// Pattani Malay
    Mfa,
    /// Persian
    Fas,
    /// Pitjantjatjara
    Pjt,
    /// Plains Cree
    Crk,
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
    /// Semai
    Sea,
    /// Serbian
    Srp,
    /// Serbo-Croatian
    Hbs,
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
    /// Southern East Cree
    Crj,
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
    /// Tachelhit
    Shi,
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
    /// Tonga (Nyasa)
    Tog,
    /// Tonga (Tonga Islands)
    Ton,
    /// Tsimshian
    Tsi,
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
    /// Wendat
    Wdt,
    /// Western Arrarnta
    Are,
    /// Wolaitta
    Wal,
    /// Wolof
    Wol,
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
    ///
    /// Generated using:
    /// ```console
    /// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
    ///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=lang%3A%22[^"]*%22' | \
    ///     sort | \
    ///     sed -e 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=lang%3A%22\([^"]*\)%22,Self::\2 => "\1"\,,' -e "s/&#x27;/'/"
    /// ```
    ///
    /// and using editor features to fix capitalization of the variants.
    pub fn name(&self) -> &'static str {
        match &self {
            Self::Abk => "Abkhazian",
            Self::Ace => "Achinese",
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
            Self::Bak => "Bashkir",
            Self::Eus => "Basque",
            Self::Bar => "Bavarian",
            Self::Bel => "Belarusian",
            Self::Bem => "Bemba",
            Self::Ben => "Bengali",
            Self::Bho => "Bhojpuri",
            Self::Bik => "Bikol",
            Self::Bin => "Bini",
            Self::Bis => "Bislama",
            Self::Brx => "Bodo (India)",
            Self::Bos => "Bosnian",
            Self::Bra => "Braj",
            Self::Bre => "Breton",
            Self::Box => "Buamu",
            Self::Bug => "Buginese",
            Self::Bul => "Bulgarian",
            Self::Bua => "Buriat",
            Self::Mya => "Burmese",
            Self::Bsk => "Burushaski",
            Self::Frc => "Cajun French",
            Self::Cat => "Catalan",
            Self::Ceb => "Cebuano",
            Self::Xce => "Celtiberian",
            Self::Ryu => "Central Okinawan",
            Self::Esu => "Central Yupik",
            Self::Cha => "Chamorro",
            Self::Che => "Chechen",
            Self::Nya => "Chichewa",
            Self::Zho => "Chinese",
            Self::Chu => "Church Slavic",
            Self::Chv => "Chuvash",
            Self::Nci => "Classical Nahuatl",
            Self::Cop => "Coptic",
            Self::Cor => "Cornish",
            Self::Cos => "Corsican",
            Self::Mus => "Creek",
            Self::Cre => "Cree",
            Self::Crh => "Crimean Tatar",
            Self::Hrv => "Croatian",
            Self::Ces => "Czech",
            Self::Dak => "Dakota",
            Self::Dan => "Danish",
            Self::Din => "Dinka",
            Self::Div => "Divehi",
            Self::Doi => "Dogri",
            Self::Dua => "Duala",
            Self::Dum => "Dutch, Middle (ca.1050-1350)",
            Self::Nld => "Dutch",
            Self::Dyu => "Dyula",
            Self::Dzo => "Dzongkha",
            Self::Aer => "Eastern Arrernte",
            Self::Egy => "Egyptian (Ancient)",
            Self::Egl => "Emilian",
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
            Self::Hil => "Hiligaynon",
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
            Self::Isv => "Interslavic",
            Self::Iku => "Inuktitut",
            Self::Gle => "Irish",
            Self::Ita => "Italian",
            Self::Jam => "Jamaican Creole English",
            Self::Jpn => "Japanese",
            Self::Jav => "Javanese",
            Self::Tmr => "Jewish Babylonian Aramaic (ca. 200-1200 CE)",
            Self::Jrb => "Judeo-Arabic",
            Self::Jpr => "Judeo-Persian",
            Self::Kbd => "Kabardian",
            Self::Kea => "Kabuverdianu",
            Self::Kab => "Kabyle",
            Self::Xal => "Kalmyk",
            Self::Kan => "Kannada",
            Self::Krc => "Karachay-Balkar",
            Self::Krl => "Karelian",
            Self::Kas => "Kashmiri",
            Self::Csb => "Kashubian",
            Self::Kaz => "Kazakh",
            Self::Meo => "Kedah Malay",
            Self::Kca => "Khanty",
            Self::Kha => "Khasi",
            Self::Khm => "Khmer, Central",
            Self::Kik => "Kikuyu",
            Self::Kmb => "Kimbundu",
            Self::Kin => "Kinyarwanda",
            Self::Kir => "Kirghiz",
            Self::Mkw => "Kituba (Congo)",
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
            Self::Mag => "Magahi",
            Self::Mai => "Maithili",
            Self::Mlg => "Malagasy",
            Self::Mal => "Malayalam",
            Self::Msa => "Malay",
            Self::Pqm => "Malecite-Passamaquoddy",
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
            Self::Mwl => "Mirandese",
            Self::Mic => "Mi&#x27;kmaq",
            Self::Mvi => "Miyako",
            Self::Moh => "Mohawk",
            Self::Mdf => "Moksha",
            Self::Mon => "Mongolian",
            Self::Lol => "Mongo",
            Self::Mfe => "Morisyen",
            Self::Mos => "Mossi",
            Self::Mul => "[Multiple languages]",
            Self::Nau => "Nauru",
            Self::Nav => "Navajo",
            Self::Nde => "Ndebele, North",
            Self::Nbl => "Ndebele, South",
            Self::Ndo => "Ndonga",
            Self::Nap => "Neapolitan",
            Self::Zmi => "Negeri Sembilan Malay",
            Self::New => "Nepal Bhasa",
            Self::Nep => "Nepali",
            Self::Nxg => "Ngad&#x27;a",
            Self::Yrl => "Nhengatu",
            Self::Niu => "Niuean",
            Self::Nog => "Nogai",
            Self::Zxx => "[No linguistic content]",
            Self::Nrn => "Norn",
            Self::Non => "Norse, Old",
            Self::Nob => "Norwegian Bokmål",
            Self::Nno => "Norwegian Nynorsk",
            Self::Nor => "Norwegian",
            Self::Nyn => "Nyankole",
            Self::Nzi => "Nzima",
            Self::Oci => "Occitan",
            Self::Ori => "Oriya",
            Self::Orm => "Oromo",
            Self::Osa => "Osage",
            Self::Pal => "Pahlavi",
            Self::Pam => "Pampanga",
            Self::Pap => "Papiamento",
            Self::Mfa => "Pattani Malay",
            Self::Fas => "Persian",
            Self::Pjt => "Pitjantjatjara",
            Self::Crk => "Plains Cree",
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
            Self::Sea => "Semai",
            Self::Srp => "Serbian",
            Self::Hbs => "Serbo-Croatian",
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
            Self::Crj => "Southern East Cree",
            Self::Spa => "Spanish",
            Self::Srn => "Sranan Tongo",
            Self::Sun => "Sundanese",
            Self::Sus => "Susu",
            Self::Sva => "Svan",
            Self::Swa => "Swahili",
            Self::Ssw => "Swati",
            Self::Swe => "Swedish",
            Self::Syr => "Syriac",
            Self::Shi => "Tachelhit",
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
            Self::Tog => "Tonga (Nyasa)",
            Self::Ton => "Tonga (Tonga Islands)",
            Self::Tsi => "Tsimshian",
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
            Self::Wdt => "Wendat",
            Self::Are => "Western Arrarnta",
            Self::Wal => "Wolaitta",
            Self::Wol => "Wolof",
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
    ///
    /// Generated using:
    /// ```console
    /// $ curl -s https://musicbrainz.org/statistics/languages-scripts | \
    ///     grep -Eo '<td>[^<]*</td><td class="t"><a href="https://musicbrainz.org/search\?query=lang%3A%22[^"]*%22' | \
    ///     sort | \
    ///     sed 's,<td>\([^<]*\)</td><td class="t"><a href="https://musicbrainz.org/search?query=lang%3A%22\([^"]*\)%22,Self::\2 => "\2"\,,'
    /// ```
    ///
    /// and using editor features to fix capitalization of the variants.
    pub fn code(&self) -> &'static str {
        match &self {
            Self::Abk => "abk",
            Self::Ace => "ace",
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
            Self::Bak => "bak",
            Self::Eus => "eus",
            Self::Bar => "bar",
            Self::Bel => "bel",
            Self::Bem => "bem",
            Self::Ben => "ben",
            Self::Bho => "bho",
            Self::Bik => "bik",
            Self::Bin => "bin",
            Self::Bis => "bis",
            Self::Brx => "brx",
            Self::Bos => "bos",
            Self::Bra => "bra",
            Self::Bre => "bre",
            Self::Box => "box",
            Self::Bug => "bug",
            Self::Bul => "bul",
            Self::Bua => "bua",
            Self::Mya => "mya",
            Self::Bsk => "bsk",
            Self::Frc => "frc",
            Self::Cat => "cat",
            Self::Ceb => "ceb",
            Self::Xce => "xce",
            Self::Ryu => "ryu",
            Self::Esu => "esu",
            Self::Cha => "cha",
            Self::Che => "che",
            Self::Nya => "nya",
            Self::Zho => "zho",
            Self::Chu => "chu",
            Self::Chv => "chv",
            Self::Nci => "nci",
            Self::Cop => "cop",
            Self::Cor => "cor",
            Self::Cos => "cos",
            Self::Mus => "mus",
            Self::Cre => "cre",
            Self::Crh => "crh",
            Self::Hrv => "hrv",
            Self::Ces => "ces",
            Self::Dak => "dak",
            Self::Dan => "dan",
            Self::Din => "din",
            Self::Div => "div",
            Self::Doi => "doi",
            Self::Dua => "dua",
            Self::Dum => "dum",
            Self::Nld => "nld",
            Self::Dyu => "dyu",
            Self::Dzo => "dzo",
            Self::Aer => "aer",
            Self::Egy => "egy",
            Self::Egl => "egl",
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
            Self::Hil => "hil",
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
            Self::Isv => "isv",
            Self::Iku => "iku",
            Self::Gle => "gle",
            Self::Ita => "ita",
            Self::Jam => "jam",
            Self::Jpn => "jpn",
            Self::Jav => "jav",
            Self::Tmr => "tmr",
            Self::Jrb => "jrb",
            Self::Jpr => "jpr",
            Self::Kbd => "kbd",
            Self::Kea => "kea",
            Self::Kab => "kab",
            Self::Xal => "xal",
            Self::Kan => "kan",
            Self::Krc => "krc",
            Self::Krl => "krl",
            Self::Kas => "kas",
            Self::Csb => "csb",
            Self::Kaz => "kaz",
            Self::Meo => "meo",
            Self::Kca => "kca",
            Self::Kha => "kha",
            Self::Khm => "khm",
            Self::Kik => "kik",
            Self::Kmb => "kmb",
            Self::Kin => "kin",
            Self::Kir => "kir",
            Self::Mkw => "mkw",
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
            Self::Mag => "mag",
            Self::Mai => "mai",
            Self::Mlg => "mlg",
            Self::Mal => "mal",
            Self::Msa => "msa",
            Self::Pqm => "pqm",
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
            Self::Mwl => "mwl",
            Self::Mic => "mic",
            Self::Mvi => "mvi",
            Self::Moh => "moh",
            Self::Mdf => "mdf",
            Self::Mon => "mon",
            Self::Lol => "lol",
            Self::Mfe => "mfe",
            Self::Mos => "mos",
            Self::Mul => "mul",
            Self::Nau => "nau",
            Self::Nav => "nav",
            Self::Nde => "nde",
            Self::Nbl => "nbl",
            Self::Ndo => "ndo",
            Self::Nap => "nap",
            Self::Zmi => "zmi",
            Self::New => "new",
            Self::Nep => "nep",
            Self::Nxg => "nxg",
            Self::Yrl => "yrl",
            Self::Niu => "niu",
            Self::Nog => "nog",
            Self::Zxx => "zxx",
            Self::Nrn => "nrn",
            Self::Non => "non",
            Self::Nob => "nob",
            Self::Nno => "nno",
            Self::Nor => "nor",
            Self::Nyn => "nyn",
            Self::Nzi => "nzi",
            Self::Oci => "oci",
            Self::Ori => "ori",
            Self::Orm => "orm",
            Self::Osa => "osa",
            Self::Pal => "pal",
            Self::Pam => "pam",
            Self::Pap => "pap",
            Self::Mfa => "mfa",
            Self::Fas => "fas",
            Self::Pjt => "pjt",
            Self::Crk => "crk",
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
            Self::Sea => "sea",
            Self::Srp => "srp",
            Self::Hbs => "hbs",
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
            Self::Crj => "crj",
            Self::Spa => "spa",
            Self::Srn => "srn",
            Self::Sun => "sun",
            Self::Sus => "sus",
            Self::Sva => "sva",
            Self::Swa => "swa",
            Self::Ssw => "ssw",
            Self::Swe => "swe",
            Self::Syr => "syr",
            Self::Shi => "shi",
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
            Self::Tog => "tog",
            Self::Ton => "ton",
            Self::Tsi => "tsi",
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
            Self::Wdt => "wdt",
            Self::Are => "are",
            Self::Wal => "wal",
            Self::Wol => "wol",
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
