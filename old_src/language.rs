use std::fmt;

/// The languages used in TMDB.
#[derive(Debug)]
pub enum Language {
    /// Aragonese
    An,
    /// Akan
    Ak,
    /// Cree
    Cr,
    /// Azerbaijani
    Az,
    /// Czech
    Cs,
    /// Afar
    Aa,
    /// Breton
    Br,
    /// Afrikaans
    Af,
    /// Tibetan
    Bo,
    /// Chechen
    Ce,
    /// Cornish
    Kw,
    /// Faroese
    Fo,
    /// Latin
    La,
    /// Ndonga
    Ng,
    /// Sardinian
    Sc,
    /// Tigrinya
    Ti,
    /// Tswana
    Tn,
    /// Turkish
    Tr,
    /// Punjabi
    Pa,
    /// Estonian
    Et,
    /// French
    Fr,
    /// Hausa
    Ha,
    /// Icelandic
    Is,
    /// Limburgish
    Li,
    /// Lingala
    Ln,
    /// Swati
    Ss,
    /// Abkhazian
    Ab,
    /// Serbo-Croatian
    Sh,
    /// Basque
    Eu,
    /// Frisian
    Fy,
    /// Japanese
    Ja,
    /// Ojibwa
    Oj,
    /// Oriya
    Or,
    /// Pali
    Pi,
    /// Sundanese
    Su,
    /// Thai
    Th,
    /// Igbo
    Ig,
    /// Indonesian
    Id,
    /// Kazakh
    Kk,
    /// Kikuyu
    Ki,
    /// Uighur
    Ug,
    /// Venda
    Ve,
    /// Kinyarwanda
    Rw,
    /// Maori
    Mi,
    /// Navajo
    Nv,
    /// Hindi
    Hi,
    /// Portuguese
    Pt,
    /// Sango
    Sg,
    /// Slovak
    Sk,
    /// Serbian
    Sr,
    /// Tahitian
    Ty,
    /// Xhosa
    Xh,
    /// Arabic
    Ar,
    /// Corsican
    Co,
    /// Bislama
    Bi,
    /// Esperanto
    Eo,
    /// Herero
    Hz,
    /// Finnish
    Fi,
    /// Inuktitut
    Iu,
    /// Latvian
    Lv,
    /// Italian
    It,
    /// Dutch
    Nl,
    /// Kannada
    Kn,
    /// Sanskrit
    Sa,
    /// Albanian
    Sq,
    /// Tagalog
    Tl,
    /// Letzeburgesch
    Lb,
    /// Tsonga
    Ts,
    /// Malayalam
    Ml,
    /// Volapük
    Vo,
    /// Zulu
    Zu,
    /// Ossetian; Ossetic
    Os,
    /// Samoan
    Sm,
    /// Zhuang
    Za,
    /// Bengali
    Bn,
    /// Slavic
    Cu,
    /// Irish
    Ga,
    /// Manx
    Gv,
    /// Hungarian
    Hu,
    /// Javanese
    Jv,
    /// Kanuri
    Kr,
    /// Khmer
    Km,
    /// Kirghiz
    Ky,
    /// Nauru
    Na,
    /// Ndebele
    Nr,
    /// Occitan
    Oc,
    /// Romanian
    Ro,
    /// Russian
    Ru,
    /// Armenian
    Hy,
    /// Chamorro
    Ch,
    /// No Language
    Xx,
    /// Bashkir
    Ba,
    /// Galician
    Gl,
    /// Ido
    Io,
    /// Luba-Katanga
    Lu,
    /// Marshall
    Mh,
    /// Malagasy
    Mg,
    /// Moldavian
    Mo,
    /// Mongolian
    Mn,
    /// Ndebele
    Nd,
    /// Norwegian
    No,
    /// Polish
    Pl,
    /// Swahili
    Sw,
    /// Tajik
    Tg,
    /// Tonga
    To,
    /// Walloon
    Wa,
    /// Yiddish
    Yi,
    /// English
    En,
    /// Assamese
    As,
    /// Gaelic
    Gd,
    /// Kalaallisut
    Kl,
    /// Burmese
    My,
    /// Quechua
    Qu,
    /// Shona
    Sn,
    /// Ukrainian
    Uk,
    /// Persian
    Fa,
    /// Georgian
    Ka,
    /// Gujarati
    Gu,
    /// Avaric
    Av,
    /// Avestan
    Ae,
    /// Guarani
    Gn,
    /// Maltese
    Mt,
    /// Nepali
    Ne,
    /// Swedish
    Sv,
    /// Tatar
    Tt,
    /// Wolof
    Wo,
    /// Cantonese
    Cn,
    /// Chuvash
    Cv,
    /// Danish
    Da,
    /// Dzongkha
    Dz,
    /// Chichewa; Nyanja
    Ny,
    /// Rundi
    Rn,
    /// Sotho
    St,
    /// Turkmen
    Tk,
    /// Uzbek
    Uz,
    /// Vietnamese
    Vi,
    /// Greek
    El,
    /// Catalan
    Ca,
    /// Welsh
    Cy,
    /// German
    De,
    /// Kashmiri
    Ks,
    /// Malay
    Ms,
    /// Norwegian Bokmål
    Nb,
    /// Raeto-Romance
    Rm,
    /// Sinhalese
    Si,
    /// Spanish
    Es,
    /// Telugu
    Te,
    /// Twi
    Tw,
    /// Pushto
    Ps,
    /// Bulgarian
    Bg,
    /// Macedonian
    Mk,
    /// Inupiaq
    Ik,
    /// Korean
    Ko,
    /// Lithuanian
    Lt,
    /// Oromo
    Om,
    /// Northern Sami
    Se,
    /// Somali
    So,
    /// Tamil
    Ta,
    /// Urdu
    Ur,
    /// Amharic
    Am,
    /// Bosnian
    Bs,
    /// Divehi
    Dv,
    /// Aymara
    Ay,
    /// Bambara
    Bm,
    /// Yi
    Ii,
    /// Interlingue
    Ie,
    /// Komi
    Kv,
    /// Kurdish
    Ku,
    /// Norwegian Nynorsk
    Nn,
    /// Mandarin
    Zh,
    /// Hebrew
    He,
    /// Ewe
    Ee,
    /// Fijian
    Fj,
    /// Fulah
    Ff,
    /// Haitian; Haitian Creole
    Ht,
    /// Croatian
    Hr,
    /// Interlingua
    Ia,
    /// Kuanyama
    Kj,
    /// Lao
    Lo,
    /// Ganda
    Lg,
    /// Marathi
    Mr,
    /// Sindhi
    Sd,
    /// Belarusian
    Be,
    /// Hiri Motu
    Ho,
    /// Kongo
    Kg,
    /// Slovenian
    Sl,
    /// Yoruba
    Yo,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::An => write!(f, "an"),
            Language::Ak => write!(f, "ak"),
            Language::Cr => write!(f, "cr"),
            Language::Az => write!(f, "az"),
            Language::Cs => write!(f, "cs"),
            Language::Aa => write!(f, "aa"),
            Language::Br => write!(f, "br"),
            Language::Af => write!(f, "af"),
            Language::Bo => write!(f, "bo"),
            Language::Ce => write!(f, "ce"),
            Language::Kw => write!(f, "kw"),
            Language::Fo => write!(f, "fo"),
            Language::La => write!(f, "la"),
            Language::Ng => write!(f, "ng"),
            Language::Sc => write!(f, "sc"),
            Language::Ti => write!(f, "ti"),
            Language::Tn => write!(f, "tn"),
            Language::Tr => write!(f, "tr"),
            Language::Pa => write!(f, "pa"),
            Language::Et => write!(f, "et"),
            Language::Fr => write!(f, "fr"),
            Language::Ha => write!(f, "ha"),
            Language::Is => write!(f, "is"),
            Language::Li => write!(f, "li"),
            Language::Ln => write!(f, "ln"),
            Language::Ss => write!(f, "ss"),
            Language::Ab => write!(f, "ab"),
            Language::Sh => write!(f, "sh"),
            Language::Eu => write!(f, "eu"),
            Language::Fy => write!(f, "fy"),
            Language::Ja => write!(f, "ja"),
            Language::Oj => write!(f, "oj"),
            Language::Or => write!(f, "or"),
            Language::Pi => write!(f, "pi"),
            Language::Su => write!(f, "su"),
            Language::Th => write!(f, "th"),
            Language::Ig => write!(f, "ig"),
            Language::Id => write!(f, "id"),
            Language::Kk => write!(f, "kk"),
            Language::Ki => write!(f, "ki"),
            Language::Ug => write!(f, "ug"),
            Language::Ve => write!(f, "ve"),
            Language::Rw => write!(f, "rw"),
            Language::Mi => write!(f, "mi"),
            Language::Nv => write!(f, "nv"),
            Language::Hi => write!(f, "hi"),
            Language::Pt => write!(f, "pt"),
            Language::Sg => write!(f, "sg"),
            Language::Sk => write!(f, "sk"),
            Language::Sr => write!(f, "sr"),
            Language::Ty => write!(f, "ty"),
            Language::Xh => write!(f, "xh"),
            Language::Ar => write!(f, "ar"),
            Language::Co => write!(f, "co"),
            Language::Bi => write!(f, "bi"),
            Language::Eo => write!(f, "eo"),
            Language::Hz => write!(f, "hz"),
            Language::Fi => write!(f, "fi"),
            Language::Iu => write!(f, "iu"),
            Language::Lv => write!(f, "lv"),
            Language::It => write!(f, "it"),
            Language::Nl => write!(f, "nl"),
            Language::Kn => write!(f, "kn"),
            Language::Sa => write!(f, "sa"),
            Language::Sq => write!(f, "sq"),
            Language::Tl => write!(f, "tl"),
            Language::Lb => write!(f, "lb"),
            Language::Ts => write!(f, "ts"),
            Language::Ml => write!(f, "ml"),
            Language::Vo => write!(f, "vo"),
            Language::Zu => write!(f, "zu"),
            Language::Os => write!(f, "os"),
            Language::Sm => write!(f, "sm"),
            Language::Za => write!(f, "za"),
            Language::Bn => write!(f, "bn"),
            Language::Cu => write!(f, "cu"),
            Language::Ga => write!(f, "ga"),
            Language::Gv => write!(f, "gv"),
            Language::Hu => write!(f, "hu"),
            Language::Jv => write!(f, "jv"),
            Language::Kr => write!(f, "kr"),
            Language::Km => write!(f, "km"),
            Language::Ky => write!(f, "ky"),
            Language::Na => write!(f, "na"),
            Language::Nr => write!(f, "nr"),
            Language::Oc => write!(f, "oc"),
            Language::Ro => write!(f, "ro"),
            Language::Ru => write!(f, "ru"),
            Language::Hy => write!(f, "hy"),
            Language::Ch => write!(f, "ch"),
            Language::Xx => write!(f, "xx"),
            Language::Ba => write!(f, "ba"),
            Language::Gl => write!(f, "gl"),
            Language::Io => write!(f, "io"),
            Language::Lu => write!(f, "lu"),
            Language::Mh => write!(f, "mh"),
            Language::Mg => write!(f, "mg"),
            Language::Mo => write!(f, "mo"),
            Language::Mn => write!(f, "mn"),
            Language::Nd => write!(f, "nd"),
            Language::No => write!(f, "no"),
            Language::Pl => write!(f, "pl"),
            Language::Sw => write!(f, "sw"),
            Language::Tg => write!(f, "tg"),
            Language::To => write!(f, "to"),
            Language::Wa => write!(f, "wa"),
            Language::Yi => write!(f, "yi"),
            Language::En => write!(f, "en"),
            Language::As => write!(f, "as"),
            Language::Gd => write!(f, "gd"),
            Language::Kl => write!(f, "kl"),
            Language::My => write!(f, "my"),
            Language::Qu => write!(f, "qu"),
            Language::Sn => write!(f, "sn"),
            Language::Uk => write!(f, "uk"),
            Language::Fa => write!(f, "fa"),
            Language::Ka => write!(f, "ka"),
            Language::Gu => write!(f, "gu"),
            Language::Av => write!(f, "av"),
            Language::Ae => write!(f, "ae"),
            Language::Gn => write!(f, "gn"),
            Language::Mt => write!(f, "mt"),
            Language::Ne => write!(f, "ne"),
            Language::Sv => write!(f, "sv"),
            Language::Tt => write!(f, "tt"),
            Language::Wo => write!(f, "wo"),
            Language::Cn => write!(f, "cn"),
            Language::Cv => write!(f, "cv"),
            Language::Da => write!(f, "da"),
            Language::Dz => write!(f, "dz"),
            Language::Ny => write!(f, "ny"),
            Language::Rn => write!(f, "rn"),
            Language::St => write!(f, "st"),
            Language::Tk => write!(f, "tk"),
            Language::Uz => write!(f, "uz"),
            Language::Vi => write!(f, "vi"),
            Language::El => write!(f, "el"),
            Language::Ca => write!(f, "ca"),
            Language::Cy => write!(f, "cy"),
            Language::De => write!(f, "de"),
            Language::Ks => write!(f, "ks"),
            Language::Ms => write!(f, "ms"),
            Language::Nb => write!(f, "nb"),
            Language::Rm => write!(f, "rm"),
            Language::Si => write!(f, "si"),
            Language::Es => write!(f, "es"),
            Language::Te => write!(f, "te"),
            Language::Tw => write!(f, "tw"),
            Language::Ps => write!(f, "ps"),
            Language::Bg => write!(f, "bg"),
            Language::Mk => write!(f, "mk"),
            Language::Ik => write!(f, "ik"),
            Language::Ko => write!(f, "ko"),
            Language::Lt => write!(f, "lt"),
            Language::Om => write!(f, "om"),
            Language::Se => write!(f, "se"),
            Language::So => write!(f, "so"),
            Language::Ta => write!(f, "ta"),
            Language::Ur => write!(f, "ur"),
            Language::Am => write!(f, "am"),
            Language::Bs => write!(f, "bs"),
            Language::Dv => write!(f, "dv"),
            Language::Ay => write!(f, "ay"),
            Language::Bm => write!(f, "bm"),
            Language::Ii => write!(f, "ii"),
            Language::Ie => write!(f, "ie"),
            Language::Kv => write!(f, "kv"),
            Language::Ku => write!(f, "ku"),
            Language::Nn => write!(f, "nn"),
            Language::Zh => write!(f, "zh"),
            Language::He => write!(f, "he"),
            Language::Ee => write!(f, "ee"),
            Language::Fj => write!(f, "fj"),
            Language::Ff => write!(f, "ff"),
            Language::Ht => write!(f, "ht"),
            Language::Hr => write!(f, "hr"),
            Language::Ia => write!(f, "ia"),
            Language::Kj => write!(f, "kj"),
            Language::Lo => write!(f, "lo"),
            Language::Lg => write!(f, "lg"),
            Language::Mr => write!(f, "mr"),
            Language::Sd => write!(f, "sd"),
            Language::Be => write!(f, "be"),
            Language::Ho => write!(f, "ho"),
            Language::Kg => write!(f, "kg"),
            Language::Sl => write!(f, "sl"),
            Language::Yo => write!(f, "yo"),
        }
    }
}
