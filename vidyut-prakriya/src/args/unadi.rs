use crate::core::errors::*;
use crate::enum_boilerplate;

use wasm_bindgen::prelude::wasm_bindgen;

/// The complete list of unadi-pratyayas.
///
/// Rust's naming convention is to start enum values with capital letters. However, we allow mixed
/// case explicitly here so that we can name pratyayas more concisely with SLP1. Doing so helps us
/// distinguish between pratyayas like `naN` and `nan`.
///
/// NOTE: we generated this list programmatically. Many of these pratyayas have typos.
#[allow(dead_code, non_camel_case_types)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[wasm_bindgen]
pub enum Unadi {
    /// -aknu
    aknuc,
    /// -aNga
    aNgac,
    /// -adAnu
    radAnuk,
    /// -a
    ac,
    /// -aji
    aji,
    /// -awa
    awan,
    /// -awi
    awi,
    /// -aWa
    aWa,
    /// -aRqa
    aRqan,
    /// -ata
    atac,
    /// -at
    ati,
    /// -ati
    ati_,
    /// -atra
    atran,
    /// -atri
    atrin,
    /// -aTa
    aTa,
    /// -adi
    adi,
    /// -a
    an,
    /// -ani
    ani,
    /// -anu
    anuN,
    /// -anya
    anya,
    /// -anyu
    anyuc,
    /// -apa
    apa,
    /// -abaka
    abaka,
    /// -aba
    abac,
    /// -aBa
    aBac,
    /// -ama
    ama,
    /// -ama (praTama)
    amac,
    /// -amba
    ambaj,
    /// -ayu
    ayu,
    /// -ara
    ara,
    /// -ara
    aran,
    /// -aru
    aru,
    /// -a
    al,
    /// -ala (maNgala)
    alac,
    /// -ali
    alic,
    /// -avi
    avi,
    /// -a
    asa,
    /// -asa
    asac,
    /// -asAna
    asAnac,
    /// -asi
    asi,
    /// -as (cetas)
    asun,
    /// -A
    A,
    /// -Aka
    Aka,
    /// -AgU
    AgUc,
    /// -Awa
    Awac,
    /// -ARaka
    ARaka,
    /// -Atu
    Atu,
    /// -Atfka
    Atfkan,
    /// -Anaka
    Anaka,
    /// -Ana
    Anac,
    /// -Anu
    Anuk,
    /// -Anya
    Anya,
    /// -Ayya
    Ayya,
    /// -Ara
    Aran,
    /// -Ala
    Ala,
    /// -Ala
    Alac,
    /// -Ala
    AlaY,
    /// -AlIya
    AlIyac,
    /// -A
    Asa,
    /// -Asi
    Asi,
    /// -i
    i,
    /// -ika
    ikan,
    /// -iji
    iji,
    /// -i
    iY,
    /// -i
    iR,
    /// -ita
    ita,
    /// -ita
    itac,
    /// -ita
    itan,
    /// -iti
    iti,
    /// -itnu
    itnuc,
    /// -itra
    itra,
    /// -itva
    itvan,
    /// -iTi
    iTin,
    /// -i
    in_,
    /// -ina
    inac,
    /// -ina
    inaR,
    /// -ina
    inan,
    /// -in
    ini,
    /// -imani
    imanic,
    /// -imani
    imanin,
    /// -ila
    ilac,
    /// -izWa
    izWac,
    /// -izWu
    izWuc,
    /// -izRu
    izRuc,
    /// -isa
    isan,
    /// -is
    isi,
    /// -is
    isin,
    /// -I
    I,
    /// -Ika
    Ikan,
    /// -Ici
    Ici,
    /// -Ida
    Ida,
    /// -Ira
    Irac,
    /// -Ira
    Iran,
    /// -Iza
    Izan,
    /// -u
    u,
    /// -uka
    ukan,
    /// -uqa
    uqac,
    /// -u
    uR,
    /// -uti
    uti,
    /// -utra
    utra,
    /// -una
    una,
    /// -una
    unan,
    /// -unasi
    unasi,
    /// -uni
    uni,
    /// -unta
    unta,
    /// -unti
    unti,
    /// -uma
    uma,
    /// -umBa
    umBa,
    /// -ura
    urac,
    /// -ura
    uran,
    /// -uri
    urin,
    /// -ula
    ulac,
    /// -uli
    uli,
    /// -uza
    uzac,
    /// -us (Danus)
    usi,
    /// -U
    U,
    /// -Uka
    Uka,
    /// -Uka
    UkaR,
    /// -UKa
    UKa,
    /// -UTa
    UTan,
    /// -Uma
    Uma,
    /// -U
    Ur,
    /// -Ura
    Uran,
    /// -Uza
    Uzan,
    /// -f
    f,
    /// -fti
    ftin,
    /// -f
    fn_,
    /// -eRu
    eRu,
    /// -eRya
    eRya,
    /// -era
    erak,
    /// -elima
    elimac,
    /// -ota
    otac,
    /// -ora
    oran,
    /// -ola
    olac,
    /// -ka
    ka,
    /// -ka
    kak,
    /// -kaNkaRa
    kaNkaRa,
    /// -kaRa
    kaRa,
    /// -katu
    katu,
    /// -katni
    katnic,
    /// -katra
    katra,
    /// -kTa
    kTan,
    /// -ka
    kan,
    /// -kanasi
    kanasi,
    /// -kani
    kanin,
    /// -kanu
    kanum,
    /// -kanya
    kanyan,
    /// -kanyu
    kanyuc,
    /// -kapa
    kapa,
    /// -kapa
    kapan,
    /// -kami
    kamin,
    /// -kaya
    kayan,
    /// -kara
    karan,
    /// -kala
    kala,
    /// -kAku
    kAku,
    /// -kAla
    kAlan,
    /// -kika
    kikan,
    /// -kita
    kitac,
    /// -kinda
    kindac,
    /// -kira
    kirac,
    /// -kizya
    kizyan,
    /// -kIka
    kIkac,
    /// -kIka
    kIkan,
    /// -kIwa
    kIwan,
    /// -ku
    ku,
    /// -ku
    kuk,
    /// -kuka
    kukan,
    /// -kuza
    kuzan,
    /// -kU
    kU,
    /// -kta
    kta,
    /// -ktnu
    ktnu,
    /// -ktra
    ktra,
    /// -kTi
    kTin,
    /// -kna
    kna,
    /// -kni
    knin,
    /// -kmala
    kmalan,
    /// -kyu
    kyu,
    /// -kyu
    kyun,
    /// -kra
    kran,
    /// -krara
    kraran,
    /// -kri
    kri,
    /// -kri
    krin,
    /// -kruka
    krukan,
    /// -kru
    krun,
    /// -kla
    kla,
    /// -kva
    kvan,
    /// -kvani
    kvanip,
    /// -kvi
    kvin,
    /// -kvi
    kvip,
    /// -kvu
    kvun,
    /// -ksara
    ksaran,
    /// -ksi
    ksi,
    /// -ksu
    ksu,
    /// -kseyya
    kseyya,
    /// -ksna
    ksna,
    /// -Ka
    Ka,
    /// -ga
    ga,
    /// -ga
    gak,
    /// -ga
    gaR,
    /// -ga
    gan,
    /// -GaTi
    GaTin,
    /// -ca
    caw,
    /// -catu
    catu,
    /// -ci
    cik,
    /// -Ja
    Jac,
    /// -Ji
    Jic,
    /// -Yu
    YuR,
    /// -wa
    wa,
    /// -wa
    wan,
    /// -wiza
    wizac,
    /// -Wa
    Wa,
    /// -qa
    qa,
    /// -qau
    qau,
    /// -qa
    qaw,
    /// -qati
    qati,
    /// -avat
    qavatu,
    /// -qimi
    qimi,
    /// -quta
    qutac,
    /// -qu
    qun,
    /// -qumsu
    qumsun,
    /// -qU
    qU,
    /// -qE
    qE,
    /// -Es
    qEsi,
    /// -qo
    qo,
    /// -qosi
    qosi,
    /// -qO
    qO,
    /// -qri
    qri,
    /// -Qa
    Qa,
    /// -Ritra
    Ritran,
    /// -Ru
    Ru,
    /// -Ruka
    Rukan,
    /// -ta
    ta,
    /// -taka
    takan,
    /// -ta
    tan,
    /// -tana
    tanan,
    /// -taSa
    taSan,
    /// -taSasu
    taSasun,
    /// -ti
    ti,
    /// -tika
    tikan,
    /// -tu
    tu,
    /// -tu
    tun,
    /// -tf
    tfc,
    /// -tf
    tfn,
    /// -tna
    tnaR,
    /// -tyu
    tyuk,
    /// -tra
    tra,
    /// -tra
    tran,
    /// -tri
    trin,
    /// -tri
    trip,
    /// -tva
    tvan,
    /// -Ta
    Tak,
    /// -da
    da,
    /// -da
    dan,
    /// -Du
    Duk,
    /// -na
    na,
    /// -na
    nak,
    /// -ni
    ni,
    /// -nu
    nu,
    /// -pa
    pa,
    /// -pAsa
    pAsa,
    /// -Pa
    Pak,
    /// -ba
    ban,
    /// -Ba
    Ba,
    /// -Ba
    Ban,
    /// -ma
    mak,
    /// -madi
    madik,
    /// -ma
    man,
    /// -man
    mani,
    /// -man
    maniR,
    /// -man
    manin,
    /// -mi
    mi,
    /// -mi
    min,
    /// -mu
    muk,
    /// -ya
    ya,
    /// -ya
    yak,
    /// -ya
    yat,
    /// -yatu
    yatuc,
    /// -yu
    yuk,
    /// -yu
    yuc,
    /// -yu
    yun,
    /// -ra
    ra,
    /// -ra
    rak,
    /// -ra
    ran,
    /// -ru
    ru,
    /// -la
    lak,
    /// -va
    va,
    /// -va
    vaR,
    /// -va
    van,
    /// -vani
    vanip,
    /// -vara
    varaw,
    /// -vala
    valaY,
    /// -vAla
    vAlac,
    /// -vAla
    vAlan,
    /// -vi
    vin,
    /// -vu
    vun,
    /// -Sa
    Sak,
    /// -Su
    Sun,
    /// -Sva
    SvaR,
    /// -ziva
    zivan,
    /// -zwra
    zwran,
    /// -zvara
    zvarac,
    /// -sa
    sa,
    /// -sa
    san,
    /// -sara
    sara,
    /// -sika
    sikan,
    /// -sTa
    sTan,
    /// -sma
    sman,
    /// -sya
    sya,
    /// -sya
    syan,
}

enum_boilerplate!(Unadi, {
    aknuc => "aknuc",
    aNgac => "aNgac",
    radAnuk => "radAnuk",
    ac => "ac",
    aji => "aji",
    awan => "awan",
    awi => "awi",
    aWa => "aWa",
    aRqan => "aRqan",
    atac => "atac",
    ati => "ati~",
    ati_ => "ati",
    atran => "atran",
    atrin => "atrin",
    aTa => "aTa",
    adi => "adi",
    an => "an",
    ani => "ani",
    anuN => "anuN",
    anya => "anya",
    anyuc => "anyuc",
    apa => "apa",
    abaka => "abaka",
    abac => "abac",
    aBac => "aBac",
    ama => "ama",
    amac => "amac",
    ambaj => "ambaj",
    ayu => "ayu",
    ara => "ara",
    aran => "aran",
    aru => "aru",
    al => "al",
    alac => "alac",
    alic => "alic",
    avi => "avi",
    asa => "asa",
    asac => "asac",
    asAnac => "asAnac",
    asi => "asi",
    asun => "asu~n",
    A => "A",
    Aka => "Aka",
    AgUc => "AgUc",
    Awac => "Awac",
    ARaka => "ARaka",
    Atu => "Atu",
    Atfkan => "Atfkan",
    Anaka => "Anaka",
    Anac => "Anac",
    Anuk => "Anuk",
    Anya => "Anya",
    Ayya => "Ayya",
    Aran => "Aran",
    Ala => "Ala",
    Alac => "Alac",
    AlaY => "AlaY",
    AlIyac => "AlIyac",
    Asa => "Asa",
    Asi => "Asi",
    i => "i",
    ikan => "ikan",
    iji => "iji",
    iY => "iY",
    iR => "iR",
    ita => "ita",
    itac => "itac",
    itan => "itan",
    iti => "iti",
    itnuc => "itnuc",
    itra => "itra",
    itvan => "itvan",
    iTin => "iTin",
    in_ => "in",
    inac => "inac",
    inaR => "inaR",
    inan => "inan",
    ini => "ini~",
    imanic => "imanic",
    imanin => "imanin",
    ilac => "ilac",
    izWac => "izWac",
    izWuc => "izWuc",
    izRuc => "izRuc",
    isan => "isan",
    isi => "isi~",
    isin => "isi~n",
    I => "I",
    Ikan => "Ikan",
    Ici => "Ici",
    Ida => "Ida",
    Irac => "Irac",
    Iran => "Iran",
    Izan => "Izan",
    u => "u",
    ukan => "ukan",
    uqac => "uqac",
    uR => "uR",
    uti => "uti",
    utra => "utra",
    una => "una",
    unan => "unan",
    unasi => "unasi",
    uni => "uni",
    unta => "unta",
    unti => "unti",
    uma => "uma",
    umBa => "umBa",
    urac => "urac",
    uran => "uran",
    urin => "urin",
    ulac => "ulac",
    uli => "uli",
    uzac => "uzac",
    usi => "usi~",
    U => "U",
    Uka => "Uka",
    UkaR => "UkaR",
    UKa => "UKa",
    UTan => "UTan",
    Uma => "Uma",
    Ur => "Ur",
    Uran => "Uran",
    Uzan => "Uzan",
    f => "f",
    ftin => "ftin",
    fn_ => "fn",
    eRu => "eRu",
    eRya => "eRya",
    erak => "erak",
    elimac => "elimac",
    otac => "otac",
    oran => "oran",
    olac => "olac",
    ka => "ka",
    kak => "kak",
    kaNkaRa => "kaNkaRa",
    kaRa => "kaRa",
    katu => "katu",
    katnic => "katnic",
    katra => "katra",
    kTan => "kTan",
    kan => "kan",
    kanasi => "kanasi",
    kanin => "kanin",
    kanum => "kanum",
    kanyan => "kanyan",
    kanyuc => "kanyuc",
    kapa => "kapa",
    kapan => "kapan",
    kamin => "kamin",
    kayan => "kayan",
    karan => "karan",
    kala => "kala",
    kAku => "kAku",
    kAlan => "kAlan",
    kikan => "kikan",
    kitac => "kitac",
    kindac => "kindac",
    kirac => "kirac",
    kizyan => "kizyan",
    kIkac => "kIkac",
    kIkan => "kIkan",
    kIwan => "kIwan",
    ku => "ku",
    kuk => "kuk",
    kukan => "kukan",
    kuzan => "kuzan",
    kU => "kU",
    kta => "kta",
    ktnu => "ktnu",
    ktra => "ktra",
    kTin => "kTin",
    kna => "kna",
    knin => "knin",
    kmalan => "kmalan",
    kyu => "kyu",
    kyun => "kyun",
    kran => "kran",
    kraran => "kraran",
    kri => "kri",
    krin => "krin",
    krukan => "krukan",
    krun => "krun",
    kla => "kla",
    kvan => "kvan",
    kvanip => "kvanip",
    kvin => "kvin",
    kvip => "kvip",
    kvun => "kvun",
    ksaran => "ksaran",
    ksi => "ksi",
    ksu => "ksu",
    kseyya => "kseyya",
    ksna => "ksna",
    Ka => "Ka",
    ga => "ga",
    gak => "gak",
    gaR => "gaR",
    gan => "gan",
    GaTin => "GaTin",
    caw => "caw",
    catu => "catu",
    cik => "cik",
    Jac => "Jac",
    Jic => "Jic",
    YuR => "YuR",
    wa => "wa",
    wan => "wan",
    wizac => "wizac",
    Wa => "Wa",
    qa => "qa",
    qau => "qau",
    qaw => "qaw",
    qati => "qati",
    qavatu => "qavatu~",
    qimi => "qimi",
    qutac => "qutac",
    qun => "qun",
    qumsun => "qumsun",
    qU => "qU",
    qE => "qE",
    qEsi => "qEsi~",
    qo => "qo",
    qosi => "qosi",
    qO => "qO",
    qri => "qri",
    Qa => "Qa",
    Ritran => "Ritran",
    Ru => "Ru",
    Rukan => "Rukan",
    ta => "ta",
    takan => "takan",
    tan => "tan",
    tanan => "tanan",
    taSan => "taSan",
    taSasun => "taSasun",
    ti => "ti",
    tikan => "tikan",
    tu => "tu",
    tun => "tun",
    tfc => "tfc",
    tfn => "tfn",
    tnaR => "tnaR",
    tyuk => "tyuk",
    tra => "tra",
    tran => "tran",
    trin => "trin",
    trip => "trip",
    tvan => "tvan",
    Tak => "Tak",
    da => "da",
    dan => "dan",
    Duk => "Duk",
    na => "na",
    nak => "nak",
    ni => "ni",
    nu => "nu",
    pa => "pa",
    pAsa => "pAsa",
    Pak => "Pak",
    ban => "ban",
    Ba => "Ba",
    Ban => "Ban",
    mak => "mak",
    madik => "madik",
    man => "man",
    mani => "mani~",
    maniR => "mani~R",
    manin => "mani~n",
    mi => "mi",
    min => "min",
    muk => "muk",
    ya => "ya",
    yak => "yak",
    yat => "yat",
    yatuc => "yatuc",
    yuk => "yuk",
    yuc => "yuc",
    yun => "yun",
    ra => "ra",
    rak => "rak",
    ran => "ran",
    ru => "ru",
    lak => "lak",
    va => "va",
    vaR => "vaR",
    van => "van",
    vanip => "vanip",
    varaw => "varaw",
    valaY => "valaY",
    vAlac => "vAlac",
    vAlan => "vAlan",
    vin => "vin",
    vun => "vun",
    Sak => "Sak",
    Sun => "Sun",
    SvaR => "SvaR",
    zivan => "zivan",
    zwran => "zwran",
    zvarac => "zvarac",
    sa => "sa",
    san => "san",
    sara => "sara",
    sikan => "sikan",
    sTan => "sTan",
    sman => "sman",
    sya => "sya",
    syan => "syan",
});
