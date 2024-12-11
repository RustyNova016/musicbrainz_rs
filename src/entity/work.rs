use super::{Include, Relationship, Subquery};
use crate::entity::alias::Alias;
use crate::entity::genre::Genre;
use crate::entity::rating::Rating;
use crate::entity::relations::Relation;
use crate::entity::tag::Tag;
use crate::entity::BrowseBy;
use crate::query::browse::impl_browse_includes;
use crate::query::relations::impl_relations_includes;
use serde::{Deserialize, Serialize};

use lucene_query_builder::QueryBuilder;

/// The "type" of a MusicBrainz work entity.
///
/// Each work can have one work type (the vast majority of works in MusicBrainz are `Song`s).
/// Note that this enum is `non_exhaustive`; The list of work types is [subject to change](https://tickets.metabrainz.org/browse/STYLE-1884?jql=project%20%3D%20STYLE%20AND%20component%20%3D%20%22Work%20types%22).
/// Variants are derived from the `work_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "String")]
pub enum WorkType {
    /// Corresponds to the "Song" work type.
    /// Description from MusicBrainz:
    /// > A song is in its origin (and still in most cases) a composition for voice, with or without instruments, performed by singing. This is the most common form by far in folk and popular music, but also fairly common in a classical context ("art songs").
    Song,
    /// Corresponds to the "Aria" work type.
    /// Description from MusicBrainz:
    /// > An aria is a self-contained piece for one voice usually with orchestral accompaniment. They are most common inside operas, but also appear in cantatas, oratorios and even on their own (concert arias).
    Aria,
    /// Corresponds to the "Audio drama" work type.
    /// Description from MusicBrainz:
    /// > An audio drama is a dramatized, purely acoustic performance, broadcast on radio or published on an audio medium (tape, CD, etc.).
    AudioDrama,
    /// Corresponds to the "Ballet" work type.
    /// Description from MusicBrainz:
    /// > A ballet is music composed to be used, together with a choreography, for a ballet dance production.
    Ballet,
    /// Corresponds to the "Beijing opera" work type.
    /// Description from MusicBrainz:
    /// > Beijing opera is a form of traditional Chinese theatre which combines music, vocal performance, mime, dance, and acrobatics.
    BeijingOpera,
    /// Corresponds to the "Cantata" work type.
    /// Description from MusicBrainz:
    /// > A cantata is a vocal (often choral) composition with an instrumental (usually orchestral) accompaniment, typically in several movements.
    Cantata,
    /// Corresponds to the "Concerto" work type.
    /// Description from MusicBrainz:
    /// > A concerto is a musical work for soloist(s) accompanied by an orchestra.
    Concerto,
    /// Corresponds to the "Étude" work type.
    /// Description from MusicBrainz:
    /// > An étude is an instrumental musical composition, most commonly of considerable difficulty, usually designed to provide practice material for perfecting a particular technical skill.
    Etude,
    /// Corresponds to the "Incidental music" work type.
    /// Description from MusicBrainz:
    /// > Incidental music is music written as background for (usually) a theatre play.
    IncidentalMusic,
    /// Corresponds to the "Madrigal" work type.
    /// Description from MusicBrainz:
    /// > The madrigal is a type of secular vocal music composition. In its original form, it had no instrumental accompaniment, although accompaniment is much more common in later madrigals.
    Madrigal,
    /// Corresponds to the "Mass" work type.
    /// Description from MusicBrainz:
    /// > A mass is a choral composition that sets the invariable portions of the Christian Eucharistic liturgy (Kyrie - Gloria - Credo - Sanctus - Benedictus - Agnus Dei, with other portions sometimes added) to music.
    Mass,
    /// Corresponds to the "Motet" work type.
    /// Description from MusicBrainz:
    /// > Motet" is a term that applies to different types of (usually unaccompanied) choral works. What exactly is a motet depends quite a bit on the period.
    Motet,
    /// Corresponds to the "Musical" work type.
    /// Description from MusicBrainz:
    /// > Musical theatre is a form of theatrical performance that combines songs, spoken dialogue, acting, and dance.
    Musical,
    /// Corresponds to the "Opera" work type.
    /// Description from MusicBrainz:
    /// > An opera is a dramatised work (text + musical score) for singers and orchestra/ensemble. In true operas all dialog is sung, through arias and recitatives, but some styles of opera include spoken dialogue.
    Opera,
    /// Corresponds to the "Operetta" work type.
    /// Description from MusicBrainz:
    /// > The operetta is a genre of light opera, in terms both of music and subject matter. Operettas are generally short and include spoken parts.
    Operetta,
    /// Corresponds to the "Oratorio" work type.
    /// Description from MusicBrainz:
    /// > An oratorio is a large (usually sacred) musical composition including an orchestra, a choir, and soloists. While it has characters and a plot, it is usually not performed theatrically (it lacks costumes, props and strong character interaction).
    Oratorio,
    /// Corresponds to the "Overture" work type.
    /// Description from MusicBrainz:
    /// > An overture is, generally, the instrumental introduction to an opera. Independent ("concert") overtures also exist, which are generally programmatic works shorter than a symphonic poem.
    Overture,
    /// Corresponds to the "Partita" work type.
    /// Description from MusicBrainz:
    /// > A partita is an instrumental piece composed of a series of variations, and it's by its current definition very similar to a suite.
    Partita,
    /// Corresponds to the "Play" work type.
    /// Description from MusicBrainz:
    /// > A play is a form of literature usually consisting of scripted dialogue between characters, and intended for theatrical performance rather than just reading.
    Play,
    /// Corresponds to the "Poem" work type.
    /// Description from MusicBrainz:
    /// > A poem is a literary piece, generally short and in verse, where words are usually chosen for their sound and for the images and ideas they suggest.
    Poem,
    /// Corresponds to the "Prose" work type.
    /// Description from MusicBrainz:
    /// > This represents literary works written in prose, that is, written in relatively ordinary language without metrical structure (e.g. novels, short stories, essays...).
    Prose,
    /// Corresponds to the "Quartet" work type.
    /// Description from MusicBrainz:
    /// > A quartet is a musical composition scored for four voices or instruments.
    Quartet,
    /// Corresponds to the "Sonata" work type.
    /// Description from MusicBrainz:
    /// > Sonata" is a general term used to describe small scale (very often solo or solo + keyboard) instrumental works, initially in baroque music.
    Sonata,
    /// Corresponds to the "Song-cycle" work type.
    /// Description from MusicBrainz:
    /// > A song cycle is a group of songs designed to be performed in a sequence as a single entity. In most cases, all of the songs are by the same composer, and often use words from the same poet or lyricist.
    SongCycle,
    /// Corresponds to the "Soundtrack" work type.
    /// Description from MusicBrainz:
    /// > A soundtrack is the music that accompanies a film, TV program, videogame, or even book.
    Soundtrack,
    /// Corresponds to the "Suite" work type.
    /// Description from MusicBrainz:
    /// > A suite is an ordered set of instrumental or orchestral pieces normally performed in a concert setting. They may be extracts from a ballet or opera, or entirely original movements.
    Suite,
    /// Corresponds to the "Symphonic poem" work type.
    /// Description from MusicBrainz:
    /// > A symphonic poem is a piece of programmatic orchestral music, usually in a single movement, that evokes a painting, a landscape, the content of a poem, a story or novel, or other non-musical source.
    SymphonicPoem,
    /// Corresponds to the "Symphony" work type.
    /// Description from MusicBrainz:
    /// > A symphony is an extended composition, almost always scored for orchestra without soloists.
    Symphony,
    /// Corresponds to the "Zarzuela" work type.
    /// Description from MusicBrainz:
    /// > A zarzuela is a Spanish lyric-dramatic work that alternates between spoken and sung scenes, the latter incorporating operatic and popular song, as well as dance.
    Zarzuela,
    /// Any music type that does not yet have a corresponding variant in this enum. If you ever see a `WorkType::UnrecognizedWorkType` in the wild, let us know and submit an issue/pull request!
    UnrecognizedWorkType(String),
}

impl From<String> for WorkType {
    fn from(source: String) -> Self {
        match source.as_str() {
            "Song" => Self::Song,
            "Aria" => Self::Aria,
            "Audio drama" => Self::AudioDrama,
            "Ballet" => Self::Ballet,
            "Beijing opera" => Self::BeijingOpera,
            "Cantata" => Self::Cantata,
            "Concerto" => Self::Concerto,
            "Étude" => Self::Etude,
            "Incidental music" => Self::IncidentalMusic,
            "Madrigal" => Self::Madrigal,
            "Mass" => Self::Mass,
            "Motet" => Self::Motet,
            "Musical" => Self::Musical,
            "Opera" => Self::Opera,
            "Operetta" => Self::Operetta,
            "Oratorio" => Self::Oratorio,
            "Overture" => Self::Overture,
            "Partita" => Self::Partita,
            "Play" => Self::Play,
            "Poem" => Self::Poem,
            "Prose" => Self::Prose,
            "Quartet" => Self::Quartet,
            "Sonata" => Self::Sonata,
            "Song-cycle" => Self::SongCycle,
            "Soundtrack" => Self::Soundtrack,
            "Suite" => Self::Suite,
            "Symphonic poem" => Self::SymphonicPoem,
            "Symphony" => Self::Symphony,
            "Zarzuela" => Self::Zarzuela,
            t => Self::UnrecognizedWorkType(t.to_string()),
        }
    }
}

/// An attribute (Musical Key, ID from a rights society (like ASCAP or GEMA), Raga/Tala/Form/etc.) associated with a MusicBrainz work entity. Should have a value as well.
///
/// This enum is marked as `non_exhaustive` because it is subject to schema changes, adding in new rights societies or traditional melody/rhythm types.
/// Variants are derived from the `work_attribute_type` table in the MusicBrainz database.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum WorkAttribute {
    /// A musical key and mode
    Key(MusicalKey),
    /// ID for the Honduran rights society AACIMH
    #[serde(rename = "AACIMH ID")]
    AacimhId(String),
    /// ID for the Costa Rican rights society ACAM
    #[serde(rename = "ACAM ID")]
    AcamId(String),
    /// ID for the Cuban rights society ACDAM
    #[serde(rename = "ACDAM ID")]
    AcdamId(String),
    /// ID for the Guatemalan rights society AEI
    #[serde(rename = "AEI ID")]
    AeiId(String),
    /// ID for the Uruguayan rights society AGADU
    #[serde(rename = "AGADU ID")]
    AgaduId(String),
    /// ID for the Latvian rights society AKKA/LAA
    #[serde(rename = "AKKA/LAA ID")]
    AkkaLaaId(String),
    /// ID for the Austrian rights society AKM
    #[serde(rename = "AKM ID")]
    AkmId(String),
    /// ID for the international (formerly US) rights society AMRA
    #[serde(rename = "AMRA ID")]
    AmraId(String),
    /// ID for the Paraguayan rights society APA
    #[serde(rename = "APA ID")]
    ApaId(String),
    /// ID for the Peruvian rights society APDAYC
    #[serde(rename = "APDAYC ID")]
    ApdaycId(String),
    /// ID for the Australasian rights society APRA
    #[serde(rename = "APRA ID")]
    ApraId(String),
    /// ID for the Hungarian rights society ARTISJUS
    #[serde(rename = "ARTISJUS ID")]
    ArtisjusId(String),
    /// ID for the US rights society ASCAP
    #[serde(rename = "ASCAP ID")]
    AscapId(String),
    /// ID for the US rights society BMI
    #[serde(rename = "BMI ID")]
    BmiId(String),
    /// ID for the Dutch rights society BUMA/STEMRA
    #[serde(rename = "BUMA/STEMRA ID")]
    BumaStemraId(String),
    /// ID for the Hong Kong rights society CASH
    #[serde(rename = "CASH ID")]
    CashId(String),
    /// ID for the private licensing company CCLI
    #[serde(rename = "CCLI ID")]
    CcliId(String),
    /// ID for the Singaporean rights society COMPASS
    #[serde(rename = "COMPASS ID")]
    CompassId(String),
    /// ID for the Barbadian rights society COSCAP
    #[serde(rename = "COSCAP ID")]
    CoscapId(String),
    /// ID for the Trinidadian and Tobagonian rights society COTT
    #[serde(rename = "COTT ID")]
    CottId(String),
    /// ID for the Brazilian rights society ECAD
    #[serde(rename = "ECAD ID")]
    EcadId(String),
    /// ID for the German rights society GEMA
    #[serde(rename = "GEMA ID")]
    GemaId(String),
    /// ID for the private licensing company HFA (Harry Fox Agency)
    #[serde(rename = "HFA ID")]
    HfaId(String),
    /// ID for the International Copyright Enterprise
    #[serde(rename = "ICE ID")]
    IceId(String),
    /// ID for the Irish rights society IMRO
    #[serde(rename = "IMRO ID")]
    ImroId(String),
    /// ID for the Indian rights society IPRS
    #[serde(rename = "IPRS ID")]
    IprsId(String),
    /// ID for the Jamaican rights society JACAP
    #[serde(rename = "JACAP ID")]
    JacapId(String),
    /// ID for the Japanese rights society JASRAC
    #[serde(rename = "JASRAC ID")]
    JasracId(String),
    /// ID for the Danish rights society KODA
    #[serde(rename = "KODA ID")]
    KodaId(String),
    /// ID for the Korean rights society KOMCA
    #[serde(rename = "KOMCA ID")]
    KomcaId(String),
    /// ID for the international rights society consortium LatinNet
    #[serde(rename = "LatinNet ID")]
    LatinnetId(String),
    /// ID for the Malaysian rights society MACP
    #[serde(rename = "MACP ID")]
    MacpId(String),
    /// ID for the Chinese rights society MCSC
    #[serde(rename = "MCSC ID")]
    McscId(String),
    /// ID for the Thai rights society MCT
    #[serde(rename = "MCT ID")]
    MctId(String),
    /// ID for the Taiwanese rights society MÜST
    #[serde(rename = "MÜST ID")]
    MustId(String),
    /// ID for the Nicaraguan rights society NICAUTOR
    #[serde(rename = "NexTone ID")]
    NextoneId(String),
    /// ID for the Japanese rights society NexTone
    #[serde(rename = "NICAUTOR ID")]
    NicautorId(String),
    /// ID for the Czech rights society OSA
    #[serde(rename = "OSA ID")]
    OsaId(String),
    /// ID for the British rights society PRS for Music
    #[serde(rename = "PRS tune code")]
    PrsTuneCode(String),
    /// ID for the Belgian rights society SABAM
    #[serde(rename = "SABAM ID")]
    SabamId(String),
    /// ID for the French rights society Sacem
    #[serde(rename = "SACEM ID")]
    SacemId(String),
    /// ID for the Salvadoran rights society SACIM
    #[serde(rename = "SACIM ID")]
    SacimId(String),
    /// ID for the Mexican rights society SACM
    #[serde(rename = "SACM ID")]
    SacmId(String),
    /// ID for the Venezuelan rights society SACVEN
    #[serde(rename = "SACVEN ID")]
    SacvenId(String),
    /// ID for the Argentinean rights society SADAIC
    #[serde(rename = "SADAIC ID")]
    SadaicId(String),
    /// ID for the South African rights society SAMRO
    #[serde(rename = "SAMRO ID")]
    SamroId(String),
    /// ID for the Ecuadorian rights society SAYCE
    #[serde(rename = "SAYCE ID")]
    SayceId(String),
    /// ID for the Colombian rights society SAYCO
    #[serde(rename = "SAYCO ID")]
    SaycoId(String),
    /// ID for the US rights society SESAC
    #[serde(rename = "SESAC ID")]
    SesacId(String),
    /// ID for the Dominican rights society SGACEDOM
    #[serde(rename = "SGACEDOM ID")]
    SgacedomId(String),
    /// ID for the Spanish rights society SGAE
    #[serde(rename = "SGAE ID")]
    SgaeId(String),
    /// ID for the Italian rights society SIAE
    #[serde(rename = "SIAE ID")]
    SiaeId(String),
    /// ID for the Bolivian rights society SOBODAYCOM
    #[serde(rename = "SOBODAYCOM ID")]
    SobodaycomId(String),
    /// ID for the Canadian rights society SOCAN
    #[serde(rename = "SOCAN ID")]
    SocanId(String),
    /// ID for the Canadian rights society SODRAC
    #[serde(rename = "SODRAC ID")]
    SodracId(String),
    /// ID for the Portuguese rights society SPA
    #[serde(rename = "SPA ID")]
    SpaId(String),
    /// ID for the Panamanian rights society SPAC
    #[serde(rename = "SPAC ID")]
    SpacId(String),
    /// ID for the Icelandic rights society STEF
    #[serde(rename = "STEF ID")]
    StefId(String),
    /// ID for the Swedish rights society STIM
    #[serde(rename = "STIM ID")]
    StimId(String),
    /// ID for the Swiss rights society SUISA
    #[serde(rename = "SUISA ID")]
    SuisaId(String),
    /// ID for the Finnish rights society TEOSTO
    #[serde(rename = "TEOSTO ID")]
    TeostoId(String),
    /// ID for the Norwegian rights society TONO
    #[serde(rename = "TONO ID")]
    TonoId(String),
    /// ID for the Polish rights society ZAiKS
    #[serde(rename = "ZAiKS ID")]
    ZaiksId(String),
    /// A [Carnatic Rāga](https://en.wikipedia.org/wiki/Carnatic_raga), a "melodic framework for improvization" in the Carnatic/South Indian Classical tradition.
    #[serde(rename = "Rāga (Carnatic)")]
    RagaCarnatic(String),
    /// A Carnatic [Tāla](https://en.wikipedia.org/wiki/Tala_(music)), a musical meter for Carnatic/South Indian Classical music.
    #[serde(rename = "Tāla (Carnatic)")]
    TalaCarnatic(String),
    #[serde(rename = "Makam (Ottoman, Turkish)")]
    FormOttomanTurkish(String),
    #[serde(rename = "Form (Ottoman, Turkish)")]
    MakamOttomanTurkish(String),
    #[serde(rename = "Usul (Ottoman, Turkish)")]
    UsulOttomanTurkish(String),
    /// A Hindustani [Rāga](https://en.wikipedia.org/wiki/Raga), a "melodic framework for improvization" in the Hindustani/North Indian Classical music.
    #[serde(rename = "Rāga (Hindustani)")]
    RagaHindustani(String),
    /// A Hindustani [Tāla](https://en.wikipedia.org/wiki/Tala_(music)), a musical meter for Hindustani/North Indian Classical music.
    #[serde(rename = "Tāla (Hindustani)")]
    TalaHindustani(String),
    /// Any music type that does not yet have a corresponding variant in this enum. If you ever see an `UnrecognizedAttribute` in the wild, let us know and submit an issue/pull request!
    #[serde(other)]
    UnrecognizedAttribute,
}

/// The musical key and mode associated with a work
///
/// Marked as `non_exhaustive` because it is conceivable that MusicBrainz would add more musical modes.
/// Musical Keys are found as possible allowed values for work attribute types in `work_attribute_type_allowed_value`.  
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum MusicalKey {
    CFlatMajor,
    CFlatMinor,
    CMajor,
    CMinor,
    CSharpMajor,
    CSharpMinor,
    DFlatMajor,
    DFlatMinor,
    DMajor,
    DMinor,
    DSharpMajor,
    DSharpMinor,
    EFlatMajor,
    EFlatMinor,
    EMajor,
    EMinor,
    ESharpMajor,
    ESharpMinor,
    FFlatMajor,
    FFlatMinor,
    FMajor,
    FMinor,
    FSharpMajor,
    FSharpMinor,
    GFlatMajor,
    GFlatMinor,
    GMajor,
    GMinor,
    GSharpMajor,
    GSharpMinor,
    AFlatMajor,
    AFlatMinor,
    AMajor,
    AMinor,
    ASharpMajor,
    ASharpMinor,
    BFlatMajor,
    BFlatMinor,
    BMajor,
    BMinor,
    BSharpMajor,
    BSharpMinor,
    CDorian,
    DDorian,
    EDorian,
    FDorian,
    GDorian,
    ADorian,
    BDorian,
    CMixolydian,
    DMixolydian,
    EMixolydian,
    FMixolydian,
    GMixolydian,
    AMixolydian,
    BMixolydian,
    /// Any musical key that does not yet have a corresponding variant in this enum. If you ever see an `UnrecognizedKey` in the wild, let us know and submit an issue/pull request!
    UnrecognizedKey(String),
}

impl From<String> for MusicalKey {
    fn from(source: String) -> Self {
        match source.as_str() {
            "C major" => Self::CMajor,
            "C minor" => Self::CMinor,
            "C-flat major" => Self::CFlatMajor,
            "C-flat minor" => Self::CFlatMinor,
            "C-sharp major" => Self::CSharpMajor,
            "C-sharp minor" => Self::CSharpMinor,
            "D major" => Self::DMajor,
            "D minor" => Self::DMinor,
            "D-flat major" => Self::DFlatMajor,
            "D-flat minor" => Self::DFlatMinor,
            "D-sharp major" => Self::DSharpMajor,
            "D-sharp minor" => Self::DSharpMinor,
            "E major" => Self::EMajor,
            "E minor" => Self::EMinor,
            "E-flat major" => Self::EFlatMajor,
            "E-flat minor" => Self::EFlatMinor,
            "E-sharp major" => Self::ESharpMajor,
            "E-sharp minor" => Self::ESharpMinor,
            "F major" => Self::FMajor,
            "F minor" => Self::FMinor,
            "F-flat major" => Self::FFlatMajor,
            "F-flat minor" => Self::FFlatMinor,
            "F-sharp major" => Self::FSharpMajor,
            "F-sharp minor" => Self::FSharpMinor,
            "G major" => Self::GMajor,
            "G minor" => Self::GMinor,
            "G-flat major" => Self::GFlatMajor,
            "G-flat minor" => Self::GFlatMinor,
            "G-sharp major" => Self::GSharpMajor,
            "G-sharp minor" => Self::GSharpMinor,
            "A major" => Self::AMajor,
            "A minor" => Self::AMinor,
            "A-flat major" => Self::AFlatMajor,
            "A-flat minor" => Self::AFlatMinor,
            "A-sharp major" => Self::ASharpMajor,
            "A-sharp minor" => Self::ASharpMinor,
            "B major" => Self::BMajor,
            "B minor" => Self::BMinor,
            "B-flat major" => Self::BFlatMajor,
            "B-flat minor" => Self::BFlatMinor,
            "B-sharp major" => Self::BSharpMajor,
            "B-sharp minor" => Self::BSharpMinor,
            "C Dorian" => Self::CDorian,
            "D Dorian" => Self::DDorian,
            "E Dorian" => Self::EDorian,
            "F Dorian" => Self::FDorian,
            "G Dorian" => Self::GDorian,
            "A Dorian" => Self::ADorian,
            "B Dorian" => Self::BDorian,
            "C Mixolydian" => Self::CMixolydian,
            "D Mixolydian" => Self::DMixolydian,
            "E Mixolydian" => Self::EMixolydian,
            "F Mixolydian" => Self::FMixolydian,
            "G Mixolydian" => Self::GMixolydian,
            "A Mixolydian" => Self::AMixolydian,
            "B Mixolydian" => Self::BMixolydian,
            k => Self::UnrecognizedKey(k.to_string()),
        }
    }
}

impl From<MusicalKey> for String {
    fn from(value: MusicalKey) -> Self {
        match value {
            MusicalKey::CMajor => "C major".to_string(),
            MusicalKey::CMinor => "C minor".to_string(),
            MusicalKey::CFlatMajor => "C-flat major".to_string(),
            MusicalKey::CFlatMinor => "C-flat minor".to_string(),
            MusicalKey::CSharpMajor => "C-sharp major".to_string(),
            MusicalKey::CSharpMinor => "C-sharp minor".to_string(),
            MusicalKey::DMajor => "D major".to_string(),
            MusicalKey::DMinor => "D minor".to_string(),
            MusicalKey::DFlatMajor => "D-flat major".to_string(),
            MusicalKey::DFlatMinor => "D-flat minor".to_string(),
            MusicalKey::DSharpMajor => "D-sharp major".to_string(),
            MusicalKey::DSharpMinor => "D-sharp minor".to_string(),
            MusicalKey::EMajor => "E major".to_string(),
            MusicalKey::EMinor => "E minor".to_string(),
            MusicalKey::EFlatMajor => "E-flat major".to_string(),
            MusicalKey::EFlatMinor => "E-flat minor".to_string(),
            MusicalKey::ESharpMajor => "E-sharp major".to_string(),
            MusicalKey::ESharpMinor => "E-sharp minor".to_string(),
            MusicalKey::FMajor => "F major".to_string(),
            MusicalKey::FMinor => "F minor".to_string(),
            MusicalKey::FFlatMajor => "F-flat major".to_string(),
            MusicalKey::FFlatMinor => "F-flat minor".to_string(),
            MusicalKey::FSharpMajor => "F-sharp major".to_string(),
            MusicalKey::FSharpMinor => "F-sharp minor".to_string(),
            MusicalKey::GMajor => "G major".to_string(),
            MusicalKey::GMinor => "G minor".to_string(),
            MusicalKey::GFlatMajor => "G-flat major".to_string(),
            MusicalKey::GFlatMinor => "G-flat minor".to_string(),
            MusicalKey::GSharpMajor => "G-sharp major".to_string(),
            MusicalKey::GSharpMinor => "G-sharp minor".to_string(),
            MusicalKey::AMajor => "A major".to_string(),
            MusicalKey::AMinor => "A minor".to_string(),
            MusicalKey::AFlatMajor => "A-flat major".to_string(),
            MusicalKey::AFlatMinor => "A-flat minor".to_string(),
            MusicalKey::ASharpMajor => "A-sharp major".to_string(),
            MusicalKey::ASharpMinor => "A-sharp minor".to_string(),
            MusicalKey::BMajor => "B major".to_string(),
            MusicalKey::BMinor => "B minor".to_string(),
            MusicalKey::BFlatMajor => "B-flat major".to_string(),
            MusicalKey::BFlatMinor => "B-flat minor".to_string(),
            MusicalKey::BSharpMajor => "B-sharp major".to_string(),
            MusicalKey::BSharpMinor => "B-sharp minor".to_string(),
            MusicalKey::CDorian => "C Dorian".to_string(),
            MusicalKey::DDorian => "D Dorian".to_string(),
            MusicalKey::EDorian => "E Dorian".to_string(),
            MusicalKey::FDorian => "F Dorian".to_string(),
            MusicalKey::GDorian => "G Dorian".to_string(),
            MusicalKey::ADorian => "A Dorian".to_string(),
            MusicalKey::BDorian => "B Dorian".to_string(),
            MusicalKey::CMixolydian => "C Mixolydian".to_string(),
            MusicalKey::DMixolydian => "D Mixolydian".to_string(),
            MusicalKey::EMixolydian => "E Mixolydian".to_string(),
            MusicalKey::FMixolydian => "F Mixolydian".to_string(),
            MusicalKey::GMixolydian => "G Mixolydian".to_string(),
            MusicalKey::AMixolydian => "A Mixolydian".to_string(),
            MusicalKey::BMixolydian => "B Mixolydian".to_string(),
            MusicalKey::UnrecognizedKey(k) => k,
        }
    }
}

/// In MusicBrainz terminology, a work is a distinct intellectual or artistic creation, which can be
/// expressed in the form of one or more audio recordings. While a work in MusicBrainz is usually
/// musical in nature, it is not necessarily so. For example, a work could be a novel, play,
/// poem or essay, later recorded as an oratory or audiobook.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[cfg_attr(
    feature = "legacy_serialize",
    serde(rename_all(deserialize = "kebab-case"))
)]
#[cfg_attr(not(feature = "legacy_serialize"), serde(rename_all = "kebab-case"))]
pub struct Work {
    /// See [MusicBrainz Identifier](https://musicbrainz.org/doc/MusicBrainz_Identifier).
    pub id: String,
    pub title: String,
    pub type_id: Option<String>,
    /// Works are represented predominantly at two levels: Discrete works, Aggregate works.
    #[serde(rename = "type")]
    pub work_type: Option<WorkType>,
    // Discuss: Is it reasonable to have a Language enum (which would almost certainly have up to thousands of variants)?
    pub language: Option<String>,
    pub languages: Option<Vec<String>>,
    pub iswcs: Option<Vec<String>>,
    pub attributes: Option<Vec<WorkAttribute>>,
    /// The disambiguation comments are fields in the database used to help distinguish identically
    /// named artists, labels and other entities.
    pub disambiguation: Option<String>,
    pub relations: Option<Vec<Relation>>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
    /// If a discrete work is known by name(s) or in language(s) other than its canonical name,
    /// these are specified in the work’s aliases.
    pub aliases: Option<Vec<Alias>>,
    /// Genres are currently supported in MusicBrainz as part of the tag system.
    pub genres: Option<Vec<Genre>>,
    /// Annotations are text fields, functioning like a miniature wiki, that can be added to any
    /// existing artists, labels, recordings, releases, release groups and works.
    pub annotation: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, QueryBuilder)]
pub struct WorkSearchQuery {
    /// (part of) any alias attached to the work (diacritics are ignored)
    pub alias: String,
    /// the MBID of an artist related to the event (e.g. a composer or lyricist)
    pub arid: String,
    /// (part of) the name of an artist related to the work (e.g. a composer or lyricist)
    pub artist: String,
    /// (part of) the work's disambiguation comment
    pub comment: String,
    /// any ISWC associated to the work
    pub iswc: String,
    /// the ISO 639-3 code for any of the languages of the work's lyrics
    pub lang: String,
    /// (part of) the title of a recording related to the work
    pub recording: String,
    /// the number of recordings related to the work
    pub recording_count: String,
    /// the MBID of a recording related to the work
    pub rid: String,
    /// (part of) a tag attached to the work
    pub tag: String,
    /// the work's type (e.g. "opera", "song", "symphony")
    #[query_builder_field = "type"]
    pub work_type: String,
    /// the work's MBID
    pub wid: String,
    /// (part of) the work's title (diacritics are ignored)
    pub work: String,
    /// (part of) the work's title (with the specified diacritics)
    #[query_builder_field = "workaccent"]
    pub work_accent: String,
}

impl_browse! {
Work,
   (by_artist, BrowseBy::Artist),
   (by_collection, BrowseBy::Collection)
}

impl_browse_includes!(
    Work,
    // Common includes.
    (with_annotation, Include::Other("annotation")),
    (with_tags, Include::Other("tags")),
    (with_user_tags, Include::Other("user-tags")),
    (with_genres, Include::Other("genres")),
    (with_user_genres, Include::Other("user-genres")),
    (with_aliases, Include::Other("aliases"))
);

impl_includes!(
    Work,
    (with_tags, Include::Subquery(Subquery::Tags)),
    (with_ratings, Include::Subquery(Subquery::Rating)),
    (with_aliases, Include::Subquery(Subquery::Aliases)),
    (with_genres, Include::Subquery(Subquery::Genres)),
    (with_annotations, Include::Subquery(Subquery::Annotations))
);

// Relationships includes
impl_relations_includes!(Work);
