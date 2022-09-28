use std::fmt;

/// The countries used in TMDB.
#[derive(Debug)]
pub enum Country {
    /// Andorra
    Ad,
    /// United Arab Emirates
    Ae,
    /// Afghanistan
    Af,
    /// Antigua and Barbuda
    Ag,
    /// Anguilla
    Ai,
    /// Albania
    Al,
    /// Armenia
    Am,
    /// Netherlands Antilles
    An,
    /// Angola
    Ao,
    /// Antarctica
    Aq,
    /// Argentina
    Ar,
    /// American Samoa
    As,
    /// Austria
    At,
    /// Australia
    Au,
    /// Aruba
    Aw,
    /// Azerbaijan
    Az,
    /// Bosnia and Herzegovina
    Ba,
    /// Barbados
    Bb,
    /// Bangladesh
    Bd,
    /// Belgium
    Be,
    /// Burkina Faso
    Bf,
    /// Bulgaria
    Bg,
    /// Bahrain
    Bh,
    /// Burundi
    Bi,
    /// Benin
    Bj,
    /// Bermuda
    Bm,
    /// Brunei Darussalam
    Bn,
    /// Bolivia
    Bo,
    /// Brazil
    Br,
    /// Bahamas
    Bs,
    /// Bhutan
    Bt,
    /// Bouvet Island
    Bv,
    /// Botswana
    Bw,
    /// Belarus
    By,
    /// Belize
    Bz,
    /// Canada
    Ca,
    /// Cocos  Islands
    Cc,
    /// Congo
    Cd,
    /// Central African Republic
    Cf,
    /// Congo
    Cg,
    /// Switzerland
    Ch,
    /// Cote D'Ivoire
    Ci,
    /// Cook Islands
    Ck,
    /// Chile
    Cl,
    /// Cameroon
    Cm,
    /// China
    Cn,
    /// Colombia
    Co,
    /// Costa Rica
    Cr,
    /// Serbia and Montenegro
    Cs,
    /// Cuba
    Cu,
    /// Cape Verde
    Cv,
    /// Christmas Island
    Cx,
    /// Cyprus
    Cy,
    /// Czech Republic
    Cz,
    /// Germany
    De,
    /// Djibouti
    Dj,
    /// Denmark
    Dk,
    /// Dominica
    Dm,
    /// Dominican Republic
    Do,
    /// Algeria
    Dz,
    /// Ecuador
    Ec,
    /// Estonia
    Ee,
    /// Egypt
    Eg,
    /// Western Sahara
    Eh,
    /// Eritrea
    Er,
    /// Spain
    Es,
    /// Ethiopia
    Et,
    /// Finland
    Fi,
    /// Fiji
    Fj,
    /// Falkland Islands
    Fk,
    /// Micronesia
    Fm,
    /// Faeroe Islands
    Fo,
    /// France
    Fr,
    /// Gabon
    Ga,
    /// United Kingdom
    Gb,
    /// Grenada
    Gd,
    /// Georgia
    Ge,
    /// French Guiana
    Gf,
    /// Ghana
    Gh,
    /// Gibraltar
    Gi,
    /// Greenland
    Gl,
    /// Gambia
    Gm,
    /// Guinea
    Gn,
    /// Guadaloupe
    Gp,
    /// Equatorial Guinea
    Gq,
    /// Greece
    Gr,
    /// South Georgia and the South Sandwich Islands
    Gs,
    /// Guatemala
    Gt,
    /// Guam
    Gu,
    /// Guinea-Bissau
    Gw,
    /// Guyana
    Gy,
    /// Hong Kong
    Hk,
    /// Heard and McDonald Islands
    Hm,
    /// Honduras
    Hn,
    /// Croatia
    Hr,
    /// Haiti
    Ht,
    /// Hungary
    Hu,
    /// Indonesia
    Id,
    /// Ireland
    Ie,
    /// Israel
    Il,
    /// India
    In,
    /// British Indian Ocean Territory
    Io,
    /// Iraq
    Iq,
    /// Iran
    Ir,
    /// Iceland
    Is,
    /// Italy
    It,
    /// Jamaica
    Jm,
    /// Jordan
    Jo,
    /// Japan
    Jp,
    /// Kenya
    Ke,
    /// Kyrgyz Republic
    Kg,
    /// Cambodia
    Kh,
    /// Kiribati
    Ki,
    /// Comoros
    Km,
    /// St. Kitts and Nevis
    Kn,
    /// North Korea
    Kp,
    /// South Korea
    Kr,
    /// Kuwait
    Kw,
    /// Cayman Islands
    Ky,
    /// Kazakhstan
    Kz,
    /// Lao People's Democratic Republic
    La,
    /// Lebanon
    Lb,
    /// St. Lucia
    Lc,
    /// Liechtenstein
    Li,
    /// Sri Lanka
    Lk,
    /// Liberia
    Lr,
    /// Lesotho
    Ls,
    /// Lithuania
    Lt,
    /// Luxembourg
    Lu,
    /// Latvia
    Lv,
    /// Libyan Arab Jamahiriya
    Ly,
    /// Morocco
    Ma,
    /// Monaco
    Mc,
    /// Moldova
    Md,
    /// Montenegro
    Me,
    /// Madagascar
    Mg,
    /// Marshall Islands
    Mh,
    /// Macedonia
    Mk,
    /// Mali
    Ml,
    /// Myanmar
    Mm,
    /// Mongolia
    Mn,
    /// Macao
    Mo,
    /// Northern Mariana Islands
    Mp,
    /// Martinique
    Mq,
    /// Mauritania
    Mr,
    /// Montserrat
    Ms,
    /// Malta
    Mt,
    /// Mauritius
    Mu,
    /// Maldives
    Mv,
    /// Malawi
    Mw,
    /// Mexico
    Mx,
    /// Malaysia
    My,
    /// Mozambique
    Mz,
    /// Namibia
    Na,
    /// New Caledonia
    Nc,
    /// Niger
    Ne,
    /// Norfolk Island
    Nf,
    /// Nigeria
    Ng,
    /// Nicaragua
    Ni,
    /// Netherlands
    Nl,
    /// Norway
    No,
    /// Nepal
    Np,
    /// Nauru
    Nr,
    /// Niue
    Nu,
    /// New Zealand
    Nz,
    /// Oman
    Om,
    /// Panama
    Pa,
    /// Peru
    Pe,
    /// French Polynesia
    Pf,
    /// Papua New Guinea
    Pg,
    /// Philippines
    Ph,
    /// Pakistan
    Pk,
    /// Poland
    Pl,
    /// St. Pierre and Miquelon
    Pm,
    /// Pitcairn Island
    Pn,
    /// Puerto Rico
    Pr,
    /// Palestinian Territory
    Ps,
    /// Portugal
    Pt,
    /// Palau
    Pw,
    /// Paraguay
    Py,
    /// Qatar
    Qa,
    /// Reunion
    Re,
    /// Romania
    Ro,
    /// Serbia
    Rs,
    /// Russia
    Ru,
    /// Rwanda
    Rw,
    /// Saudi Arabia
    Sa,
    /// Solomon Islands
    Sb,
    /// Seychelles
    Sc,
    /// Sudan
    Sd,
    /// Sweden
    Se,
    /// Singapore
    Sg,
    /// St. Helena
    Sh,
    /// Slovenia
    Si,
    /// Svalbard & Jan Mayen Islands
    Sj,
    /// Slovakia
    Sk,
    /// Sierra Leone
    Sl,
    /// San Marino
    Sm,
    /// Senegal
    Sn,
    /// Somalia
    So,
    /// Suriname
    Sr,
    /// South Sudan
    Ss,
    /// Sao Tome and Principe
    St,
    /// Soviet Union
    Su,
    /// El Salvador
    Sv,
    /// Syrian Arab Republic
    Sy,
    /// Swaziland
    Sz,
    /// Turks and Caicos Islands
    Tc,
    /// Chad
    Td,
    /// French Southern Territories
    Tf,
    /// Togo
    Tg,
    /// Thailand
    Th,
    /// Tajikistan
    Tj,
    /// Tokelau
    Tk,
    /// Timor-Leste
    Tl,
    /// Turkmenistan
    Tm,
    /// Tunisia
    Tn,
    /// Tonga
    To,
    /// Turkey
    Tr,
    /// Trinidad and Tobago
    Tt,
    /// Tuvalu
    Tv,
    /// Taiwan
    Tw,
    /// Tanzania
    Tz,
    /// Ukraine
    Ua,
    /// Uganda
    Ug,
    /// United States Minor Outlying Islands
    Um,
    /// United States of America
    Us,
    /// Uruguay
    Uy,
    /// Uzbekistan
    Uz,
    /// Holy See
    Va,
    /// St. Vincent and the Grenadines
    Vc,
    /// Venezuela
    Ve,
    /// British Virgin Islands
    Vg,
    /// US Virgin Islands
    Vi,
    /// Vietnam
    Vn,
    /// Vanuatu
    Vu,
    /// Wallis and Futuna Islands
    Wf,
    /// Samoa
    Ws,
    /// Czechoslovakia
    Xc,
    /// East Germany
    Xg,
    /// Kosovo
    Xk,
    /// Yemen
    Ye,
    /// Mayotte
    Yt,
    /// Yugoslavia
    Yu,
    /// South Africa
    Za,
    /// Zambia
    Zm,
    /// Zimbabwe
    Zw,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Country::Ad => write!(f, "AD"),
            Country::Ae => write!(f, "AE"),
            Country::Af => write!(f, "AF"),
            Country::Ag => write!(f, "AG"),
            Country::Ai => write!(f, "AI"),
            Country::Al => write!(f, "AL"),
            Country::Am => write!(f, "AM"),
            Country::An => write!(f, "AN"),
            Country::Ao => write!(f, "AO"),
            Country::Aq => write!(f, "AQ"),
            Country::Ar => write!(f, "AR"),
            Country::As => write!(f, "AS"),
            Country::At => write!(f, "AT"),
            Country::Au => write!(f, "AU"),
            Country::Aw => write!(f, "AW"),
            Country::Az => write!(f, "AZ"),
            Country::Ba => write!(f, "BA"),
            Country::Bb => write!(f, "BB"),
            Country::Bd => write!(f, "BD"),
            Country::Be => write!(f, "BE"),
            Country::Bf => write!(f, "BF"),
            Country::Bg => write!(f, "BG"),
            Country::Bh => write!(f, "BH"),
            Country::Bi => write!(f, "BI"),
            Country::Bj => write!(f, "BJ"),
            Country::Bm => write!(f, "BM"),
            Country::Bn => write!(f, "BN"),
            Country::Bo => write!(f, "BO"),
            Country::Br => write!(f, "BR"),
            Country::Bs => write!(f, "BS"),
            Country::Bt => write!(f, "BT"),
            Country::Bv => write!(f, "BV"),
            Country::Bw => write!(f, "BW"),
            Country::By => write!(f, "BY"),
            Country::Bz => write!(f, "BZ"),
            Country::Ca => write!(f, "CA"),
            Country::Cc => write!(f, "CC"),
            Country::Cd => write!(f, "CD"),
            Country::Cf => write!(f, "CF"),
            Country::Cg => write!(f, "CG"),
            Country::Ch => write!(f, "CH"),
            Country::Ci => write!(f, "CI"),
            Country::Ck => write!(f, "CK"),
            Country::Cl => write!(f, "CL"),
            Country::Cm => write!(f, "CM"),
            Country::Cn => write!(f, "CN"),
            Country::Co => write!(f, "CO"),
            Country::Cr => write!(f, "CR"),
            Country::Cs => write!(f, "CS"),
            Country::Cu => write!(f, "CU"),
            Country::Cv => write!(f, "CV"),
            Country::Cx => write!(f, "CX"),
            Country::Cy => write!(f, "CY"),
            Country::Cz => write!(f, "CZ"),
            Country::De => write!(f, "DE"),
            Country::Dj => write!(f, "DJ"),
            Country::Dk => write!(f, "DK"),
            Country::Dm => write!(f, "DM"),
            Country::Do => write!(f, "DO"),
            Country::Dz => write!(f, "DZ"),
            Country::Ec => write!(f, "EC"),
            Country::Ee => write!(f, "EE"),
            Country::Eg => write!(f, "EG"),
            Country::Eh => write!(f, "EH"),
            Country::Er => write!(f, "ER"),
            Country::Es => write!(f, "ES"),
            Country::Et => write!(f, "ET"),
            Country::Fi => write!(f, "FI"),
            Country::Fj => write!(f, "FJ"),
            Country::Fk => write!(f, "FK"),
            Country::Fm => write!(f, "FM"),
            Country::Fo => write!(f, "FO"),
            Country::Fr => write!(f, "FR"),
            Country::Ga => write!(f, "GA"),
            Country::Gb => write!(f, "GB"),
            Country::Gd => write!(f, "GD"),
            Country::Ge => write!(f, "GE"),
            Country::Gf => write!(f, "GF"),
            Country::Gh => write!(f, "GH"),
            Country::Gi => write!(f, "GI"),
            Country::Gl => write!(f, "GL"),
            Country::Gm => write!(f, "GM"),
            Country::Gn => write!(f, "GN"),
            Country::Gp => write!(f, "GP"),
            Country::Gq => write!(f, "GQ"),
            Country::Gr => write!(f, "GR"),
            Country::Gs => write!(f, "GS"),
            Country::Gt => write!(f, "GT"),
            Country::Gu => write!(f, "GU"),
            Country::Gw => write!(f, "GW"),
            Country::Gy => write!(f, "GY"),
            Country::Hk => write!(f, "HK"),
            Country::Hm => write!(f, "HM"),
            Country::Hn => write!(f, "HN"),
            Country::Hr => write!(f, "HR"),
            Country::Ht => write!(f, "HT"),
            Country::Hu => write!(f, "HU"),
            Country::Id => write!(f, "ID"),
            Country::Ie => write!(f, "IE"),
            Country::Il => write!(f, "IL"),
            Country::In => write!(f, "IN"),
            Country::Io => write!(f, "IO"),
            Country::Iq => write!(f, "IQ"),
            Country::Ir => write!(f, "IR"),
            Country::Is => write!(f, "IS"),
            Country::It => write!(f, "IT"),
            Country::Jm => write!(f, "JM"),
            Country::Jo => write!(f, "JO"),
            Country::Jp => write!(f, "JP"),
            Country::Ke => write!(f, "KE"),
            Country::Kg => write!(f, "KG"),
            Country::Kh => write!(f, "KH"),
            Country::Ki => write!(f, "KI"),
            Country::Km => write!(f, "KM"),
            Country::Kn => write!(f, "KN"),
            Country::Kp => write!(f, "KP"),
            Country::Kr => write!(f, "KR"),
            Country::Kw => write!(f, "KW"),
            Country::Ky => write!(f, "KY"),
            Country::Kz => write!(f, "KZ"),
            Country::La => write!(f, "LA"),
            Country::Lb => write!(f, "LB"),
            Country::Lc => write!(f, "LC"),
            Country::Li => write!(f, "LI"),
            Country::Lk => write!(f, "LK"),
            Country::Lr => write!(f, "LR"),
            Country::Ls => write!(f, "LS"),
            Country::Lt => write!(f, "LT"),
            Country::Lu => write!(f, "LU"),
            Country::Lv => write!(f, "LV"),
            Country::Ly => write!(f, "LY"),
            Country::Ma => write!(f, "MA"),
            Country::Mc => write!(f, "MC"),
            Country::Md => write!(f, "MD"),
            Country::Me => write!(f, "ME"),
            Country::Mg => write!(f, "MG"),
            Country::Mh => write!(f, "MH"),
            Country::Mk => write!(f, "MK"),
            Country::Ml => write!(f, "ML"),
            Country::Mm => write!(f, "MM"),
            Country::Mn => write!(f, "MN"),
            Country::Mo => write!(f, "MO"),
            Country::Mp => write!(f, "MP"),
            Country::Mq => write!(f, "MQ"),
            Country::Mr => write!(f, "MR"),
            Country::Ms => write!(f, "MS"),
            Country::Mt => write!(f, "MT"),
            Country::Mu => write!(f, "MU"),
            Country::Mv => write!(f, "MV"),
            Country::Mw => write!(f, "MW"),
            Country::Mx => write!(f, "MX"),
            Country::My => write!(f, "MY"),
            Country::Mz => write!(f, "MZ"),
            Country::Na => write!(f, "NA"),
            Country::Nc => write!(f, "NC"),
            Country::Ne => write!(f, "NE"),
            Country::Nf => write!(f, "NF"),
            Country::Ng => write!(f, "NG"),
            Country::Ni => write!(f, "NI"),
            Country::Nl => write!(f, "NL"),
            Country::No => write!(f, "NO"),
            Country::Np => write!(f, "NP"),
            Country::Nr => write!(f, "NR"),
            Country::Nu => write!(f, "NU"),
            Country::Nz => write!(f, "NZ"),
            Country::Om => write!(f, "OM"),
            Country::Pa => write!(f, "PA"),
            Country::Pe => write!(f, "PE"),
            Country::Pf => write!(f, "PF"),
            Country::Pg => write!(f, "PG"),
            Country::Ph => write!(f, "PH"),
            Country::Pk => write!(f, "PK"),
            Country::Pl => write!(f, "PL"),
            Country::Pm => write!(f, "PM"),
            Country::Pn => write!(f, "PN"),
            Country::Pr => write!(f, "PR"),
            Country::Ps => write!(f, "PS"),
            Country::Pt => write!(f, "PT"),
            Country::Pw => write!(f, "PW"),
            Country::Py => write!(f, "PY"),
            Country::Qa => write!(f, "QA"),
            Country::Re => write!(f, "RE"),
            Country::Ro => write!(f, "RO"),
            Country::Rs => write!(f, "RS"),
            Country::Ru => write!(f, "RU"),
            Country::Rw => write!(f, "RW"),
            Country::Sa => write!(f, "SA"),
            Country::Sb => write!(f, "SB"),
            Country::Sc => write!(f, "SC"),
            Country::Sd => write!(f, "SD"),
            Country::Se => write!(f, "SE"),
            Country::Sg => write!(f, "SG"),
            Country::Sh => write!(f, "SH"),
            Country::Si => write!(f, "SI"),
            Country::Sj => write!(f, "SJ"),
            Country::Sk => write!(f, "SK"),
            Country::Sl => write!(f, "SL"),
            Country::Sm => write!(f, "SM"),
            Country::Sn => write!(f, "SN"),
            Country::So => write!(f, "SO"),
            Country::Sr => write!(f, "SR"),
            Country::Ss => write!(f, "SS"),
            Country::St => write!(f, "ST"),
            Country::Su => write!(f, "SU"),
            Country::Sv => write!(f, "SV"),
            Country::Sy => write!(f, "SY"),
            Country::Sz => write!(f, "SZ"),
            Country::Tc => write!(f, "TC"),
            Country::Td => write!(f, "TD"),
            Country::Tf => write!(f, "TF"),
            Country::Tg => write!(f, "TG"),
            Country::Th => write!(f, "TH"),
            Country::Tj => write!(f, "TJ"),
            Country::Tk => write!(f, "TK"),
            Country::Tl => write!(f, "TL"),
            Country::Tm => write!(f, "TM"),
            Country::Tn => write!(f, "TN"),
            Country::To => write!(f, "TO"),
            Country::Tr => write!(f, "TR"),
            Country::Tt => write!(f, "TT"),
            Country::Tv => write!(f, "TV"),
            Country::Tw => write!(f, "TW"),
            Country::Tz => write!(f, "TZ"),
            Country::Ua => write!(f, "UA"),
            Country::Ug => write!(f, "UG"),
            Country::Um => write!(f, "UM"),
            Country::Us => write!(f, "US"),
            Country::Uy => write!(f, "UY"),
            Country::Uz => write!(f, "UZ"),
            Country::Va => write!(f, "VA"),
            Country::Vc => write!(f, "VC"),
            Country::Ve => write!(f, "VE"),
            Country::Vg => write!(f, "VG"),
            Country::Vi => write!(f, "VI"),
            Country::Vn => write!(f, "VN"),
            Country::Vu => write!(f, "VU"),
            Country::Wf => write!(f, "WF"),
            Country::Ws => write!(f, "WS"),
            Country::Xc => write!(f, "XC"),
            Country::Xg => write!(f, "XG"),
            Country::Xk => write!(f, "XK"),
            Country::Ye => write!(f, "YE"),
            Country::Yt => write!(f, "YT"),
            Country::Yu => write!(f, "YU"),
            Country::Za => write!(f, "ZA"),
            Country::Zm => write!(f, "ZM"),
            Country::Zw => write!(f, "ZW"),
        }
    }
}
