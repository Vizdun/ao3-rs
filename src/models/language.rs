use std::fmt::Display;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
pub enum Language {
    Somali,
    Afrikaans,
    Arabic,
    Egyptian,
    Aramaic,
    Armenian,
    Asturian,
    Indonesian,
    Malaysian,
    Bulgarian,
    Bengali,
    Javanese,
    Belarusian,
    Bosnian,
    Breton,
    Catalan,
    Cebuano,
    Czech,
    ChinookJargon,
    Welsh,
    Danish,
    German,
    Estonian,
    Greek,
    English,
    Spanish,
    Esperanto,
    Basque,
    Persian,
    Filipino,
    French,
    Friulian,
    Irish,
    ScottishGaelic,
    Galician,
    Gothic,
    HakkaChinese,
    Korean,
    Hausa,
    Hindi,
    Croatian,
    Interlingua,
    Zulu,
    Icelandic,
    Italian,
    Hebrew,
    Kannada,
    Georgian,
    Khmer,
    Khuzdul,
    Swahili,
    HaitianCreole,
    Kyrgyz,
    QuebecSign,
    Latvian,
    Luxembourgish,
    Lithuanian,
    Latin,
    Hungarian,
    Macedonian,
    Malayalam,
    Manchu,
    Marathi,
    Mikasuki,
    Mongolian,
    Burmese,
    Nahuatl,
    TaiwaneseHokkien,
    Dutch,
    Japanese,
    Norwegian,
    Azerbaijani,
    OttomanTurkish,
    Pashto,
    LowGerman,
    Polish,
    BrazilianPortuguese,
    EuropeanPortuguese,
    Punjabi,
    Kazakh,
    Quenya,
    Romanian,
    Russian,
    Scots,
    Albanian,
    Sindarin,
    Sinhala,
    Slovakian,
    Slovenian,
    Sketchy,
    Serbian,
    Finnish,
    Swedish,
    Tamil,
    Telugu,
    Thai,
    Thermian,
    StandardTibetian,
    Vietnamese,
    Coptic,
    Klingon,
    TokiPona,
    Tsakonian,
    Turkish,
    Ukrainian,
    Urdu,
    Uyghur,
    Volapuk,
    WuChinese,
    Yiddish,
    YucatecMaya,
    YueChinese,
    StandardChinese,
}

impl TryFrom<&str> for Language {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "af Soomaali" => Ok(Language::Somali),
            "Afrikaans" => Ok(Language::Afrikaans),
            "العربية" => Ok(Language::Arabic),
            "𓂋𓏺𓈖 𓆎𓅓𓏏𓊖" => Ok(Language::Egyptian),
            "ܐܪܡܝܐ | ארמיא" => Ok(Language::Aramaic),
            "հայերեն" => Ok(Language::Armenian),
            "asturianu" => Ok(Language::Asturian),
            "Bahasa Indonesia" => Ok(Language::Indonesian),
            "Bahasa Malaysia" => Ok(Language::Malaysian),
            "Български" => Ok(Language::Bulgarian),
            "বাংলা" => Ok(Language::Bengali),
            "Basa Jawa" => Ok(Language::Javanese),
            "беларуская" => Ok(Language::Belarusian),
            "Bosanski" => Ok(Language::Bosnian),
            "brezhoneg" => Ok(Language::Breton),
            "Català" => Ok(Language::Catalan),
            "Cebuano" => Ok(Language::Cebuano),
            "Čeština" => Ok(Language::Czech),
            "Chinuk Wawa" => Ok(Language::ChinookJargon),
            "Cymraeg" => Ok(Language::Welsh),
            "Dansk" => Ok(Language::Danish),
            "Deutsch" => Ok(Language::German),
            "eesti keel" => Ok(Language::Estonian),
            "Ελληνικά" => Ok(Language::Greek),
            "English" => Ok(Language::English),
            "Español" => Ok(Language::Spanish),
            "Esperanto" => Ok(Language::Esperanto),
            "Euskara" => Ok(Language::Basque),
            "فارسی" => Ok(Language::Persian),
            "Filipino" => Ok(Language::Filipino),
            "Français" => Ok(Language::French),
            "Furlan" => Ok(Language::Friulian),
            "Gaeilge" => Ok(Language::Irish),
            "Gàidhlig" => Ok(Language::ScottishGaelic),
            "Galego" => Ok(Language::Galician),
            "𐌲𐌿𐍄𐌹𐍃𐌺𐌰" => Ok(Language::Gothic),
            "中文-客家话" => Ok(Language::HakkaChinese),
            "한국어" => Ok(Language::Korean),
            "Hausa | هَرْشَن هَوْسَ" => Ok(Language::Hausa),
            "हिन्दी" => Ok(Language::Hindi),
            "Hrvatski" => Ok(Language::Croatian),
            "Interlingua" => Ok(Language::Interlingua),
            "isiZulu" => Ok(Language::Zulu),
            "Íslenska" => Ok(Language::Icelandic),
            "Italiano" => Ok(Language::Italian),
            "עברית" => Ok(Language::Hebrew),
            "ಕನ್ನಡ" => Ok(Language::Kannada),
            "ქართული" => Ok(Language::Georgian),
            "ភាសាខ្មែរ" => Ok(Language::Khmer),
            "Khuzdul" => Ok(Language::Khuzdul),
            "Kiswahili" => Ok(Language::Swahili),
            "kreyòl ayisyen" => Ok(Language::HaitianCreole),
            "Кыргызча" => Ok(Language::Kyrgyz),
            "Langue des signes québécoise" => Ok(Language::QuebecSign),
            "Latviešu valoda" => Ok(Language::Latvian),
            "Lëtzebuergesch" => Ok(Language::Luxembourgish),
            "Lietuvių kalba" => Ok(Language::Lithuanian),
            "Lingua latina" => Ok(Language::Latin),
            "Magyar" => Ok(Language::Hungarian),
            "македонски" => Ok(Language::Macedonian),
            "മലയാളം" => Ok(Language::Malayalam),
            "ᠮᠠᠨᠵᡠ ᡤᡳᠰᡠᠨ" => Ok(Language::Manchu),
            "मराठी" => Ok(Language::Marathi),
            "Mikisúkî" => Ok(Language::Mikasuki),
            "ᠮᠣᠩᠭᠣᠯ ᠪᠢᠴᠢᠭ᠌ | Монгол Кирилл үсэг" => {
                Ok(Language::Mongolian)
            }
            "မြန်မာဘာသာ" => Ok(Language::Burmese),
            "Nāhuatl" => Ok(Language::Nahuatl),
            "中文-闽南话 臺語" => Ok(Language::TaiwaneseHokkien),
            "Nederlands" => Ok(Language::Dutch),
            "日本語" => Ok(Language::Japanese),
            "Norsk" => Ok(Language::Norwegian),
            "Азәрбајҹан дили | آذربایجان دیلی" => {
                Ok(Language::Azerbaijani)
            }
            "لسان عثمانى" => Ok(Language::OttomanTurkish),
            "پښتو" => Ok(Language::Pashto),
            "Plattdüütsch" => Ok(Language::LowGerman),
            "Polski" => Ok(Language::Polish),
            "Português brasileiro" => Ok(Language::BrazilianPortuguese),
            "Português europeu" => Ok(Language::EuropeanPortuguese),
            "ਪੰਜਾਬੀ" => Ok(Language::Punjabi),
            "qazaqşa | қазақша" => Ok(Language::Kazakh),
            "Quenya" => Ok(Language::Quenya),
            "Română" => Ok(Language::Romanian),
            "Русский" => Ok(Language::Russian),
            "Scots" => Ok(Language::Scots),
            "Shqip" => Ok(Language::Albanian),
            "Sindarin" => Ok(Language::Sindarin),
            "සිංහල" => Ok(Language::Sinhala),
            "Slovenčina" => Ok(Language::Slovakian),
            "Slovenščina" => Ok(Language::Slovenian),
            "Sprēkō Þiudiskō" => Ok(Language::Sketchy),
            "Српски" => Ok(Language::Serbian),
            "Suomi" => Ok(Language::Finnish),
            "Svenska" => Ok(Language::Swedish),
            "தமிழ்" => Ok(Language::Tamil),
            "తెలుగు" => Ok(Language::Telugu),
            "ไทย" => Ok(Language::Thai),
            "Thermian" => Ok(Language::Thermian),
            "བོད་སྐད་" => Ok(Language::StandardTibetian),
            "Tiếng Việt" => Ok(Language::Vietnamese),
            "ϯⲙⲉⲧⲣⲉⲙⲛ̀ⲭⲏⲙⲓ" => Ok(Language::Coptic),
            "tlhIngan-Hol" => Ok(Language::Klingon),
            "toki pona" => Ok(Language::TokiPona),
            "τσακώνικα" => Ok(Language::Tsakonian),
            "Türkçe" => Ok(Language::Turkish),
            "Українська" => Ok(Language::Ukrainian),
            "اُردُو" => Ok(Language::Urdu),
            "ئۇيغۇر تىلى" => Ok(Language::Uyghur),
            "Volapük" => Ok(Language::Volapuk),
            "中文-吴语" => Ok(Language::WuChinese),
            "יידיש" => Ok(Language::Yiddish),
            "maayaʼ tʼàan" => Ok(Language::YucatecMaya),
            "中文-广东话 粵語" => Ok(Language::YueChinese),
            "中文-普通话 國語" => Ok(Language::StandardChinese),
            _ => Err(()),
        }
    }
}

impl From<&Language> for &str {
    fn from(l: &Language) -> Self {
        match l {
            Language::Somali => "so",
            Language::Afrikaans => "afr",
            Language::Arabic => "ar",
            Language::Egyptian => "egy",
            Language::Aramaic => "arc",
            Language::Armenian => "hy",
            Language::Asturian => "ast",
            Language::Indonesian => "id",
            Language::Malaysian => "ms",
            Language::Bulgarian => "bg",
            Language::Bengali => "bn",
            Language::Javanese => "jv",
            Language::Belarusian => "be",
            Language::Bosnian => "bos",
            Language::Breton => "br",
            Language::Catalan => "ca",
            Language::Cebuano => "ceb",
            Language::Czech => "cs",
            Language::ChinookJargon => "chn",
            Language::Welsh => "cy",
            Language::Danish => "da",
            Language::German => "de",
            Language::Estonian => "et",
            Language::Greek => "el",
            Language::English => "en",
            Language::Spanish => "es",
            Language::Esperanto => "eo",
            Language::Basque => "eu",
            Language::Persian => "fa",
            Language::Filipino => "fil",
            Language::French => "fr",
            Language::Friulian => "fur",
            Language::Irish => "ga",
            Language::ScottishGaelic => "gd",
            Language::Galician => "gl",
            Language::Gothic => "got",
            Language::HakkaChinese => "hak",
            Language::Korean => "ko",
            Language::Hausa => "hau",
            Language::Hindi => "hi",
            Language::Croatian => "hr",
            Language::Interlingua => "ia",
            Language::Zulu => "zu",
            Language::Icelandic => "is",
            Language::Italian => "it",
            Language::Hebrew => "he",
            Language::Kannada => "kan",
            Language::Georgian => "kat",
            Language::Khmer => "khm",
            Language::Khuzdul => "qkz",
            Language::Swahili => "sw",
            Language::HaitianCreole => "ht",
            Language::Kyrgyz => "kir",
            Language::QuebecSign => "fcs",
            Language::Latvian => "lv",
            Language::Luxembourgish => "lb",
            Language::Lithuanian => "lt",
            Language::Latin => "la",
            Language::Hungarian => "hu",
            Language::Macedonian => "mk",
            Language::Malayalam => "ml",
            Language::Manchu => "mnc",
            Language::Marathi => "mr",
            Language::Mikasuki => "mik",
            Language::Mongolian => "mon",
            Language::Burmese => "my",
            Language::Nahuatl => "nah",
            Language::TaiwaneseHokkien => "nan",
            Language::Dutch => "nl",
            Language::Japanese => "ja",
            Language::Norwegian => "no",
            Language::Azerbaijani => "azj",
            Language::OttomanTurkish => "ota",
            Language::Pashto => "ps",
            Language::LowGerman => "nds",
            Language::Polish => "pl",
            Language::BrazilianPortuguese => "ptBR",
            Language::EuropeanPortuguese => "ptPT",
            Language::Punjabi => "pa",
            Language::Kazakh => "kaz",
            Language::Quenya => "qya",
            Language::Romanian => "ro",
            Language::Russian => "ru",
            Language::Scots => "sco",
            Language::Albanian => "sq",
            Language::Sindarin => "sjn",
            Language::Sinhala => "si",
            Language::Slovakian => "sk",
            Language::Slovenian => "slv",
            Language::Sketchy => "gem",
            Language::Serbian => "sr",
            Language::Finnish => "fi",
            Language::Swedish => "sv",
            Language::Tamil => "ta",
            Language::Telugu => "tel",
            Language::Thai => "th",
            Language::Thermian => "tqx",
            Language::StandardTibetian => "bod",
            Language::Vietnamese => "vi",
            Language::Coptic => "cop",
            Language::Klingon => "tlh",
            Language::TokiPona => "qtp",
            Language::Tsakonian => "tsd",
            Language::Turkish => "tr",
            Language::Ukrainian => "uk",
            Language::Urdu => "urd",
            Language::Uyghur => "uig",
            Language::Volapuk => "vol",
            Language::WuChinese => "wuu",
            Language::Yiddish => "yi",
            Language::YucatecMaya => "yua",
            Language::YueChinese => "yue",
            Language::StandardChinese => "zh",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Somali => f.write_str("af Soomaali"),
            Language::Afrikaans => f.write_str("Afrikaans"),
            Language::Arabic => f.write_str("العربية"),
            Language::Egyptian => f.write_str("𓂋𓏺𓈖 𓆎𓅓𓏏𓊖"),
            Language::Aramaic => f.write_str("ܐܪܡܝܐ | ארמיא"),
            Language::Armenian => f.write_str("հայերեն"),
            Language::Asturian => f.write_str("asturianu"),
            Language::Indonesian => f.write_str("Bahasa Indonesia"),
            Language::Malaysian => f.write_str("Bahasa Malaysia"),
            Language::Bulgarian => f.write_str("Български"),
            Language::Bengali => f.write_str("বাংলা"),
            Language::Javanese => f.write_str("Basa Jawa"),
            Language::Belarusian => f.write_str("беларуская"),
            Language::Bosnian => f.write_str("Bosanski"),
            Language::Breton => f.write_str("brezhoneg"),
            Language::Catalan => f.write_str("Català"),
            Language::Cebuano => f.write_str("Cebuano"),
            Language::Czech => f.write_str("Čeština"),
            Language::ChinookJargon => f.write_str("Chinuk Wawa"),
            Language::Welsh => f.write_str("Cymraeg"),
            Language::Danish => f.write_str("Dansk"),
            Language::German => f.write_str("Deutsch"),
            Language::Estonian => f.write_str("eesti keel"),
            Language::Greek => f.write_str("Ελληνικά"),
            Language::English => f.write_str("English"),
            Language::Spanish => f.write_str("Español"),
            Language::Esperanto => f.write_str("Esperanto"),
            Language::Basque => f.write_str("Euskara"),
            Language::Persian => f.write_str("فارسی"),
            Language::Filipino => f.write_str("Filipino"),
            Language::French => f.write_str("Français"),
            Language::Friulian => f.write_str("Furlan"),
            Language::Irish => f.write_str("Gaeilge"),
            Language::ScottishGaelic => f.write_str("Gàidhlig"),
            Language::Galician => f.write_str("Galego"),
            Language::Gothic => f.write_str("𐌲𐌿𐍄𐌹𐍃𐌺𐌰"),
            Language::HakkaChinese => f.write_str("中文-客家话"),
            Language::Korean => f.write_str("한국어"),
            Language::Hausa => f.write_str("Hausa | هَرْشَن هَوْسَ"),
            Language::Hindi => f.write_str("हिन्दी"),
            Language::Croatian => f.write_str("Hrvatski"),
            Language::Interlingua => f.write_str("Interlingua"),
            Language::Zulu => f.write_str("isiZulu"),
            Language::Icelandic => f.write_str("Íslenska"),
            Language::Italian => f.write_str("Italiano"),
            Language::Hebrew => f.write_str("עברית"),
            Language::Kannada => f.write_str("ಕನ್ನಡ"),
            Language::Georgian => f.write_str("ქართული"),
            Language::Khmer => f.write_str("ភាសាខ្មែរ"),
            Language::Khuzdul => f.write_str("Khuzdul"),
            Language::Swahili => f.write_str("Kiswahili"),
            Language::HaitianCreole => f.write_str("kreyòl ayisyen"),
            Language::Kyrgyz => f.write_str("Кыргызча"),
            Language::QuebecSign => f.write_str("Langue des signes québécoise"),
            Language::Latvian => f.write_str("Latviešu valoda"),
            Language::Luxembourgish => f.write_str("Lëtzebuergesch"),
            Language::Lithuanian => f.write_str("Lietuvių kalba"),
            Language::Latin => f.write_str("Lingua latina"),
            Language::Hungarian => f.write_str("Magyar"),
            Language::Macedonian => f.write_str("македонски"),
            Language::Malayalam => f.write_str("മലയാളം"),
            Language::Manchu => f.write_str("ᠮᠠᠨᠵᡠ ᡤᡳᠰᡠᠨ"),
            Language::Marathi => f.write_str("मराठी"),
            Language::Mikasuki => f.write_str("Mikisúkî"),
            Language::Mongolian => f.write_str("ᠮᠣᠩᠭᠣᠯ ᠪᠢᠴᠢᠭ᠌ | Монгол Кирилл үсэг"),
            Language::Burmese => f.write_str("မြန်မာဘာသာ"),
            Language::Nahuatl => f.write_str("Nāhuatl"),
            Language::TaiwaneseHokkien => f.write_str("中文-闽南话 臺語"),
            Language::Dutch => f.write_str("Nederlands"),
            Language::Japanese => f.write_str("日本語"),
            Language::Norwegian => f.write_str("Norsk"),
            Language::Azerbaijani => f.write_str("Азәрбајҹан дили | آذربایجان دیلی"),
            Language::OttomanTurkish => f.write_str("لسان عثمانى"),
            Language::Pashto => f.write_str("پښتو"),
            Language::LowGerman => f.write_str("Plattdüütsch"),
            Language::Polish => f.write_str("Polski"),
            Language::BrazilianPortuguese => f.write_str("Português brasileiro"),
            Language::EuropeanPortuguese => f.write_str("Português europeu"),
            Language::Punjabi => f.write_str("ਪੰਜਾਬੀ"),
            Language::Kazakh => f.write_str("qazaqşa | қазақша"),
            Language::Quenya => f.write_str("Quenya"),
            Language::Romanian => f.write_str("Română"),
            Language::Russian => f.write_str("Русский"),
            Language::Scots => f.write_str("Scots"),
            Language::Albanian => f.write_str("Shqip"),
            Language::Sindarin => f.write_str("Sindarin"),
            Language::Sinhala => f.write_str("සිංහල"),
            Language::Slovakian => f.write_str("Slovenčina"),
            Language::Slovenian => f.write_str("Slovenščina"),
            Language::Sketchy => f.write_str("Sprēkō Þiudiskō"),
            Language::Serbian => f.write_str("Српски"),
            Language::Finnish => f.write_str("Suomi"),
            Language::Swedish => f.write_str("Svenska"),
            Language::Tamil => f.write_str("தமிழ்"),
            Language::Telugu => f.write_str("తెలుగు"),
            Language::Thai => f.write_str("ไทย"),
            Language::Thermian => f.write_str("Thermian"),
            Language::StandardTibetian => f.write_str("བོད་སྐད་"),
            Language::Vietnamese => f.write_str("Tiếng Việt"),
            Language::Coptic => f.write_str("ϯⲙⲉⲧⲣⲉⲙⲛ̀ⲭⲏⲙⲓ"),
            Language::Klingon => f.write_str("tlhIngan-Hol"),
            Language::TokiPona => f.write_str("toki pona"),
            Language::Tsakonian => f.write_str("τσακώνικα"),
            Language::Turkish => f.write_str("Türkçe"),
            Language::Ukrainian => f.write_str("Українська"),
            Language::Urdu => f.write_str("اُردُو"),
            Language::Uyghur => f.write_str("ئۇيغۇر تىلى"),
            Language::Volapuk => f.write_str("Volapük"),
            Language::WuChinese => f.write_str("中文-吴语"),
            Language::Yiddish => f.write_str("יידיש"),
            Language::YucatecMaya => f.write_str("maayaʼ tʼàan"),
            Language::YueChinese => f.write_str("中文-广东话 粵語"),
            Language::StandardChinese => f.write_str("中文-普通话 國語"),
        }
    }
}
