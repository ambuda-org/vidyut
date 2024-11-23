use vidyut_lipi::Scheme::*;
use vidyut_lipi::{Lipika, Scheme};

/// A quick alias for transliterating.
fn t(input: &str, from: Scheme, to: Scheme) -> String {
    let mut lipika = Lipika::new();
    lipika.transliterate(input, from, to)
}

/// Asserts that `input` produces `expected` when transliterated from `from` to `to`.
fn assert_transliterate(input: &str, from: Scheme, to: Scheme, expected: &str) {
    let actual = t(input, from, to);
    if expected != actual {
        let e_codes: Vec<_> = expected.chars().map(|c| c as u32).collect();
        let a_codes: Vec<_> = actual.chars().map(|c| c as u32).collect();
        let e_slp1 = t(expected, to, Scheme::Slp1);
        let a_slp1 = t(&actual, to, Scheme::Slp1);
        println!(
            "input: {input} ({from:?} --> {to:?})

expected: {}
          {e_codes:x?}
          (slp1: {e_slp1})

actual:   {}
          {a_codes:x?}
          (slp1: {a_slp1})
",
            expected, actual
        );
    }
    assert_eq!(*expected, actual);
}

/// Transliterates all input strings against each other.
///
/// Use this function if round-trip transliteration is lossless.
fn assert_two_way_pairwise(examples: &[(Scheme, &str)]) {
    for (from, input) in examples {
        for (to, expected) in examples {
            // Also test the case where "from == to." In this case, we should return the original
            // input string unchanged.
            assert_transliterate(input, *from, *to, expected);
        }
    }
}

/// Transliterates `reference` into each item in `examples`.
///
/// Use this function if transliteration is lossy from `reference` to the given `examples`. All
/// `examples` are also round-tripped with each other through `assert_two_way_pairwise`.
fn assert_one_way_pairwise(reference: (Scheme, &str), examples: &[(Scheme, &str)]) {
    let from = reference.0;
    let input = reference.1;

    for (to, expected) in examples {
        // Also test the case where "from == to." In this case, we should return the original
        // input string unchanged.
        assert_transliterate(input, from, *to, expected);
    }

    assert_two_way_pairwise(examples);
}

/// Asserts that:
/// 1. each test case has multiple unicode variants
/// 2. for each test case, all unicode variants have the same transliteration output.
fn assert_supports_unicode_variants(scheme: Scheme, test_cases: &[&str]) {
    use unicode_normalization::UnicodeNormalization;

    let mut lipika = Lipika::new();
    for case in test_cases {
        let input_nfc: String = case.nfc().collect();
        let input_nfd: String = case.nfd().collect();
        assert_ne!(
            input_nfc, input_nfd,
            "Unicode test case '{case}' should have separate NFC and NFD forms."
        );

        // The choice of output_scheme does not really matter here. What we're testing is that the
        // transliterator treats our two inputs in the same way.
        let output_scheme = Scheme::Devanagari;
        let output_nfc = lipika.transliterate(&input_nfc, scheme, output_scheme);
        let output_nfd = lipika.transliterate(&input_nfd, scheme, output_scheme);
        assert_eq!(
            output_nfc, output_nfd,
            "Unicode test case '{case}' has two transliterations. (NFC = '{output_nfc}', NFD = '{output_nfd}').",
        );
    }
}

// Sanskrit (Basic)
// ----------------

#[test]
fn sanskrit_independent_vowels() {
    let slp1_text = "a A i I u U f F x X e E o O";
    assert_two_way_pairwise(&[
        (BarahaSouth, "a A i I u U Ru RU ~lu ~lU E ai O au"),
        (HarvardKyoto, "a A i I u U R RR lR lRR e ai o au"),
        (Iast, "a ā i ī u ū ṛ ṝ ḷ ḹ e ai o au"),
        (Iso15919, "a ā i ī u ū r̥ r̥̄ l̥ l̥̄ ē ai ō au"),
        (Itrans, "a A i I u U RRi RRI LLi LLI e ai o au"),
        (Slp1, slp1_text),
        (Velthuis, "a aa i ii u uu .r .R .l .L e ai o au"),
        (Wx, "a A i I u U q Q L LV e E o O"),
        // Indic
        (Assamese, "অ আ ই ঈ উ ঊ ঋ ৠ ঌ ৡ এ ঐ ও ঔ"),
        (Balinese, "ᬅ ᬆ ᬇ ᬈ ᬉ ᬊ ᬋ ᬌ ᬍ ᬎ ᬏ ᬐ ᬑ ᬒ"),
        (Bengali, "অ আ ই ঈ উ ঊ ঋ ৠ ঌ ৡ এ ঐ ও ঔ"),
        (Brahmi, "𑀅 𑀆 𑀇 𑀈 𑀉 𑀊 𑀋 𑀌 𑀍 𑀎 𑀏 𑀐 𑀑 𑀒"),
        (Burmese, "အ အာ ဣ ဤ ဥ ဦ ၒ ၓ ၔ ၕ ဧ အဲ ဩ ဪ"),
        (Cham, "ꨀ ꨀꨩ ꨁ ꨁꨩ ꨂ ꨂꨩ ꨣꨮ ꨣꨮꨩ ꨤꨮ ꨤꨮꨩ ꨃ ꨄ ꨅ ꨀꨯꨱ"),
        (Devanagari, "अ आ इ ई उ ऊ ऋ ॠ ऌ ॡ ए ऐ ओ औ"),
        (Dogra, "𑠀 𑠁 𑠂 𑠃 𑠄 𑠅 𑠤𑠭 𑠤𑠮 𑠥𑠭 𑠥𑠮 𑠆 𑠇 𑠈 𑠉"),
        (Grantha, "𑌅 𑌆 𑌇 𑌈 𑌉 𑌊 𑌋 𑍠 𑌌 𑍡 𑌏 𑌐 𑌓 𑌔"),
        (Gujarati, "અ આ ઇ ઈ ઉ ઊ ઋ ૠ ઌ ૡ એ ઐ ઓ ઔ"),
        (Javanese, "ꦄ ꦄꦴ ꦆ ꦇ ꦈ ꦈꦴ ꦉ ꦉꦴ ꦊ ꦋ ꦌ ꦍ ꦎ ꦎꦴ"),
        (Kannada, "ಅ ಆ ಇ ಈ ಉ ಊ ಋ ೠ ಌ ೡ ಏ ಐ ಓ ಔ"),
        (Khmer, "អ អា ឥ ឦ ឧ ឩ ឫ ឬ ឭ ឮ ឯ ឰ ឱ ឳ"),
        (Malayalam, "അ ആ ഇ ഈ ഉ ഊ ഋ ൠ ഌ ൡ ഏ ഐ ഓ ഔ"),
        (Modi, "𑘀 𑘁 𑘂 𑘃 𑘄 𑘅 𑘆 𑘇 𑘈 𑘉 𑘊 𑘋 𑘌 𑘍"),
        (Mon, "အ အာ ဣ ဣဳ ဥ ဥူ ၒ ၓ ၔ ၕ ဨ အဲ ဩ ဪ"),
        (Newa, "𑐀 𑐁 𑐂 𑐃 𑐄 𑐅 𑐆 𑐇 𑐈 𑐉 𑐊 𑐋 𑐌 𑐍"),
        (Odia, "ଅ ଆ ଇ ଈ ଉ ଊ ଋ ୠ ଌ ୡ ଏ ଐ ଓ ଔ"),
        (Saurashtra, "ꢂ ꢃ ꢄ ꢅ ꢆ ꢇ ꢈ ꢉ ꢊ ꢋ ꢍ ꢎ ꢐ ꢑ"),
        (Sharada, "𑆃 𑆄 𑆅 𑆆 𑆇 𑆈 𑆉 𑆊 𑆋 𑆌 𑆍 𑆎 𑆏 𑆐"),
        (Sinhala, "අ ආ ඉ ඊ උ ඌ ඍ ඎ ඏ ඐ ඒ ඓ ඕ ඖ"),
        (Soyombo, "𑩐 𑩐𑩛 𑩐𑩑 𑩐𑩑𑩛 𑩐𑩒 𑩐𑩒𑩛 𑩐𑩙 𑩐𑩙𑩛 𑩐𑩚 𑩐𑩚𑩛 𑩐𑩔 𑩐𑩗 𑩐𑩖 𑩐𑩘"),
        (TaiTham, "ᩋ ᩋᩣ ᩍ ᩎ ᩏ ᩐ ᩂ ᩂᩣ ᩄ ᩄᩣ ᩑ ᩋᩱ ᩒ ᩏᩫᩣ"),
        (Tamil, "அ ஆ இ ஈ உ ஊ ருʼ ரூʼ லுʼ லூʼ ஏ ஐ ஓ ஔ"),
        (Telugu, "అ ఆ ఇ ఈ ఉ ఊ ఋ ౠ ఌ ౡ ఏ ఐ ఓ ఔ"),
        (Thai, "อ อา อิ อี อุ อู ฤ ฤๅ ฦ ฦๅ เอ ไอ โอ เอา"),
        (Tirhuta, "𑒁 𑒂 𑒃 𑒄 𑒅 𑒆 𑒇 𑒈 𑒉 𑒊 𑒋 𑒌 𑒍 𑒎"),
        (Tibetan, "ཨ་ཨཱ་ཨི་ཨཱི་ཨུ་ཨཱུ་རྀ་རཱྀ་ལྀ་ལཱྀ་ཨེ་ཨཻ་ཨོ་ཨཽ"),
        (ZanabazarSquare, "𑨀 𑨀𑨊 𑨀𑨁 𑨀𑨁𑨊 𑨀𑨂 𑨀𑨂𑨊 𑨫𑨉 𑨫𑨉𑨊 𑨬𑨉 𑨬𑨉𑨊 𑨀𑨄 𑨀𑨄𑨊 𑨀𑨆 𑨀𑨆𑨊"),
    ]);

    // Scripts with no vocalic LL
    assert_one_way_pairwise(
        (Slp1, slp1_text),
        &[(Bhaiksuki, "𑰀𑱃𑰁𑱃𑰂𑱃𑰃𑱃𑰄𑱃𑰅𑱃𑰆𑱃𑰇𑱃𑰈𑱃𑰈𑱃𑰊𑱃𑰋𑱃𑰌𑱃𑰍")],
    );

    // Scripts with no vocalic L/LL
    assert_two_way_pairwise(&[
        (Slp1, slp1_text),
        (Kharoshthi, "𐨀 𐨀𐨌 𐨀𐨁 𐨀𐨁𐨌 𐨀𐨂 𐨀𐨂𐨌 𐨀𐨃 𐨀𐨃𐨌 𐨫𐨂 𐨫𐨂𐨌 𐨀𐨅 𐨀𐨅𐨌 𐨀𐨆 𐨀𐨆𐨌"),
        (Nandinagari, "𑦠 𑦡 𑦢 𑦣 𑦤 𑦥 𑦦 𑦧 𑧉𑧔 𑧉𑧕 𑦪 𑦫 𑦬 𑦭"),
        (Siddham, "𑖀 𑖁 𑖂 𑖃 𑖄 𑖅 𑖆 𑖇 𑖈 𑖉 𑖊 𑖋 𑖌 𑖍"),
    ]);

    // Scripts with no vocalic R/RR or L/LL
    assert_two_way_pairwise(&[
        (Slp1, slp1_text),
        (GunjalaGondi, "𑵠 𑵡 𑵢 𑵣 𑵤 𑵥 𑶈𑶍 𑶈𑶎 𑵵𑶍 𑵵𑶎 𑵧 𑵨 𑵪 𑵫"),
        (Gurmukhi, "ਅ ਆ ਇ ਈ ਉ ਊ ਰੁ ਰੂ ਲੁ ਲੂ ਏ ਐ ਓ ਔ"),
        (Kaithi, "𑂃 𑂄 𑂅 𑂆 𑂇 𑂈 𑂩𑂱 𑂩𑂲 𑂪𑂱 𑂪𑂲 𑂉 𑂊 𑂋 𑂌"),
        (Khudawadi, "𑊰 𑊱 𑊲 𑊳 𑊴 𑊵 𑋙𑋡 𑋙𑋢 𑋚𑋡 𑋚𑋢 𑊶 𑊷 𑊸 𑊹"),
        (Limbu, "ᤀ ᤀᤠ ᤀᤡ ᤀᤡ᤺ ᤀᤢ ᤀᤢ᤺ ᤖᤢ ᤖᤢ᤺ ᤗᤢ ᤗᤢ᤺ ᤀᤣ ᤀᤤ ᤀᤥ ᤀᤦ"),
        (MasaramGondi, "𑴀 𑴁 𑴂 𑴃 𑴄 𑴅 𑴦𑴶 𑴦𑴵 𑴧𑴴 𑴧𑴵 𑴆 𑴈 𑴉 𑴋"),
        (OlChiki, "ᱚ ᱟ ᱤ ᱤᱻ ᱩ ᱩᱻ ᱨᱩ ᱨᱩᱻ ᱞᱩ ᱞᱩᱻ ᱮ ᱚᱤ ᱳ ᱚᱩ"),
        (Takri, "𑚀 𑚁 𑚂 𑚃 𑚄 𑚅 𑚤𑚮 𑚤𑚯 𑚥𑚮 𑚥𑚯 𑚆 𑚇 𑚈 𑚉"),
    ]);

    // No vocalir R/RR/L/LL or long U
    assert_transliterate(
        slp1_text,
        Slp1,
        MeeteiMayek,
        "ꯑ ꯑꯥ ꯏ ꯏ ꯎ ꯎ ꯔꯨ ꯔꯨ ꯂꯨ ꯂꯨ ꯑꯦ ꯑꯩ ꯑꯣ ꯑꯧ",
    );
}

#[test]
fn sanskrit_dependent_vowels() {
    let slp1_text = "ka kA ki kI ku kU kf kF kx kX ke kE ko kO";

    assert_two_way_pairwise(&[
        (
            BarahaSouth,
            "ka kA ki kI ku kU kRu kRU k~lu k~lU kE kai kO kau",
        ),
        (
            HarvardKyoto,
            "ka kA ki kI ku kU kR kRR klR klRR ke kai ko kau",
        ),
        (Iast, "ka kā ki kī ku kū kṛ kṝ kḷ kḹ ke kai ko kau"),
        (Iso15919, "ka kā ki kī ku kū kr̥ kr̥̄ kl̥ kl̥̄ kē kai kō kau"),
        (
            Itrans,
            "ka kA ki kI ku kU kRRi kRRI kLLi kLLI ke kai ko kau",
        ),
        (Slp1, slp1_text),
        (
            Velthuis,
            "ka kaa ki kii ku kuu k.r k.R k.l k.L ke kai ko kau",
        ),
        (Wx, "ka kA ki kI ku kU kq kQ kL kLV ke kE ko kO"),
        // Indic
        (Assamese, "ক কা কি কী কু কূ কৃ কৄ কৢ কৣ কে কৈ কো কৌ"),
        (Balinese, "ᬓ ᬓᬵ ᬓᬶ ᬓᬷ ᬓᬸ ᬓᬹ ᬓᬺ ᬓᬻ ᬓᬼ ᬓᬽ ᬓᬾ ᬓᬿ ᬓᭀ ᬓᭁ"),
        (Bengali, "ক কা কি কী কু কূ কৃ কৄ কৢ কৣ কে কৈ কো কৌ"),
        (Brahmi, "𑀓 𑀓𑀸 𑀓𑀺 𑀓𑀻 𑀓𑀼 𑀓𑀽 𑀓𑀾 𑀓𑀿 𑀓𑁀 𑀓𑁁 𑀓𑁂 𑀓𑁃 𑀓𑁄 𑀓𑁅"),
        (Burmese, "က ကာ ကိ ကီ ကု ကူ ကၖ ကၗ ကၘ ကၙ ကေ ကဲ ကော ကော်"),
        (Cham, "ꨆ ꨆꨩ ꨆꨪ ꨆꨫ ꨆꨭ ꨆꨭꨩ ꨆꨴꨮ ꨆꨴꨮꨩ ꨆꨵꨮ ꨆꨵꨮꨩ ꨆꨯꨮ ꨆꨰ ꨆꨯ ꨆꨯꨱ"),
        (Devanagari, "क का कि की कु कू कृ कॄ कॢ कॣ के कै को कौ"),
        (Dogra, "𑠊 𑠊𑠬 𑠊𑠭 𑠊𑠮 𑠊𑠯 𑠊𑠰 𑠊𑠱 𑠊𑠲 𑠊𑠹𑠥𑠭 𑠊𑠹𑠥𑠮 𑠊𑠳 𑠊𑠴 𑠊𑠵 𑠊𑠶"),
        (Grantha, "𑌕 𑌕𑌾 𑌕𑌿 𑌕𑍀 𑌕𑍁 𑌕𑍂 𑌕𑍃 𑌕𑍄 𑌕𑍢 𑌕𑍣 𑌕𑍇 𑌕𑍈 𑌕𑍋 𑌕𑍌"),
        (Gujarati, "ક કા કિ કી કુ કૂ કૃ કૄ કૢ કૣ કે કૈ કો કૌ"),
        (Javanese, "ꦏ ꦏꦴ ꦏꦶ ꦏꦷ ꦏꦸ ꦏꦹ ꦏꦽ ꦏ꧀ꦉꦴ ꦏ꧀ꦊ ꦏ꧀ꦋ ꦏꦺ ꦏꦻ ꦏꦺꦴ ꦏꦻꦴ"),
        (Kannada, "ಕ ಕಾ ಕಿ ಕೀ ಕು ಕೂ ಕೃ ಕೄ ಕೢ ಕೣ ಕೇ ಕೈ ಕೋ ಕೌ"),
        (Khmer, "ក កា កិ កី កុ កូ ក្ឫ ក្ឬ ក្ឭ ក្ឮ កេ កៃ កោ កៅ"),
        (Malayalam, "ക കാ കി കീ കു കൂ കൃ കൄ കൢ കൣ കേ കൈ കോ കൗ"),
        (Modi, "𑘎 𑘎𑘰 𑘎𑘱 𑘎𑘲 𑘎𑘳 𑘎𑘴 𑘎𑘵 𑘎𑘶 𑘎𑘷 𑘎𑘸 𑘎𑘹 𑘎𑘺 𑘎𑘻 𑘎𑘼"),
        (Mon, "က ကာ ကိ ကဳ ကု ကူ ကၖ ကၗ ကၘ ကၙ ကေ ကဲ ကော ကော်"),
        (Newa, "𑐎 𑐎𑐵 𑐎𑐶 𑐎𑐷 𑐎𑐸 𑐎𑐹 𑐎𑐺 𑐎𑐻 𑐎𑐼 𑐎𑐽 𑐎𑐾 𑐎𑐿 𑐎𑑀 𑐎𑑁"),
        (Odia, "କ କା କି କୀ କୁ କୂ କୃ କୄ କୢ କୣ କେ କୈ କୋ କୌ"),
        (Saurashtra, "ꢒ ꢒꢵ ꢒꢶ ꢒꢷ ꢒꢸ ꢒꢹ ꢒꢺ ꢒꢻ ꢒꢼ ꢒꢽ ꢒꢿ ꢒꣀ ꢒꣂ ꢒꣃ"),
        (Sharada, "𑆑 𑆑𑆳 𑆑𑆴 𑆑𑆵 𑆑𑆶 𑆑𑆷 𑆑𑆸 𑆑𑆹 𑆑𑆺 𑆑𑆻 𑆑𑆼 𑆑𑆽 𑆑𑆾 𑆑𑆿"),
        (Sinhala, "ක කා කි කී කු කූ කෘ කෲ කෟ කෳ කේ කෛ කෝ කෞ"),
        (Soyombo, "𑩜 𑩜𑩛 𑩜𑩑 𑩜𑩑𑩛 𑩜𑩒 𑩜𑩒𑩛 𑩜𑩙 𑩜𑩙𑩛 𑩜𑩚 𑩜𑩚𑩛 𑩜𑩔 𑩜𑩗 𑩜𑩖 𑩜𑩘"),
        (TaiTham, "ᨠ ᨠᩣ ᨠᩥ ᨠᩦ ᨠᩩ ᨠᩪ ᨠ᩺ᩂ ᨠ᩺ᩂᩣ ᨠ᩺ᩄ ᨠ᩺ᩄᩣ ᨠᩮ ᨠᩱ ᨠᩮᩣ ᨠᩮᩫᩣ"),
        (Tamil, "க கா கி கீ கு கூ க்ருʼ க்ரூʼ க்லுʼ க்லூʼ கே கை கோ கௌ"),
        (Telugu, "క కా కి కీ కు కూ కృ కౄ కౢ కౣ కే కై కో కౌ"),
        (Thai, "ก กา กิ กี กุ กู กฺฤ กฺฤๅ กฺฦ กฺฦๅ เก ไก โก เกา"),
        (Tibetan, "ཀ་ཀཱ་ཀི་ཀཱི་ཀུ་ཀཱུ་ཀྲྀ་ཀྲཱྀ་ཀླྀ་ཀླཱྀ་ཀེ་ཀཻ་ཀོ་ཀཽ"),
        (Tirhuta, "𑒏 𑒏𑒰 𑒏𑒱 𑒏𑒲 𑒏𑒳 𑒏𑒴 𑒏𑒵 𑒏𑒶 𑒏𑒷 𑒏𑒸 𑒏𑒹 𑒏𑒻 𑒏𑒼 𑒏𑒾"),
    ]);

    // Scripts without vocalic L
    assert_one_way_pairwise(
        (Slp1, slp1_text),
        &[(Bhaiksuki, "𑰎𑱃𑰎𑰯𑱃𑰎𑰰𑱃𑰎𑰱𑱃𑰎𑰲𑱃𑰎𑰳𑱃𑰎𑰴𑱃𑰎𑰵𑱃𑰎𑰶𑱃𑰎𑰶𑱃𑰎𑰸𑱃𑰎𑰹𑱃𑰎𑰺𑱃𑰎𑰻")],
    );

    // Scripts without vocalic L/LL
    assert_two_way_pairwise(&[
        (Slp1, slp1_text),
        (Kharoshthi, "𐨐 𐨐𐨌 𐨐𐨁 𐨐𐨁𐨌 𐨐𐨂 𐨐𐨂𐨌 𐨐𐨃 𐨐𐨃𐨌 𐨐𐨿𐨫𐨂 𐨐𐨿𐨫𐨂𐨌 𐨐𐨅 𐨐𐨅𐨌 𐨐𐨆 𐨐𐨆𐨌"),
        (Nandinagari, "𑦮 𑦮𑧑 𑦮𑧒 𑦮𑧓 𑦮𑧔 𑦮𑧕 𑦮𑧖 𑦮𑧗 𑦮𑧠𑧉𑧔 𑦮𑧠𑧉𑧕 𑦮𑧚 𑦮𑧛 𑦮𑧜 𑦮𑧝"),
        (Siddham, "𑖎 𑖎𑖯 𑖎𑖰 𑖎𑖱 𑖎𑖲 𑖎𑖳 𑖎𑖴 𑖎𑖵 𑖎𑖿𑖩𑖰 𑖎𑖿𑖩𑖱 𑖎𑖸 𑖎𑖹 𑖎𑖺 𑖎𑖻"),
    ]);

    // Scripts without vocalic R/RR or L/LL
    assert_two_way_pairwise(&[
        (Slp1, slp1_text),
        (Gurmukhi, "ਕ ਕਾ ਕਿ ਕੀ ਕੁ ਕੂ ਕ੍ਰੁ ਕ੍ਰੂ ਕ੍ਲੁ ਕ੍ਲੂ ਕੇ ਕੈ ਕੋ ਕੌ"),
        (GunjalaGondi, "𑵱 𑵱𑶊 𑵱𑶋 𑵱𑶌 𑵱𑶍 𑵱𑶎 𑵱𑶗𑶈𑶍 𑵱𑶗𑶈𑶎 𑵱𑶗𑵵𑶍 𑵱𑶗𑵵𑶎 𑵱𑶐 𑵱𑶑 𑵱𑶓 𑵱𑶔"),
        (Kaithi, "𑂍 𑂍𑂰 𑂍𑂱 𑂍𑂲 𑂍𑂳 𑂍𑂴 𑂍𑂹𑂩𑂱 𑂍𑂹𑂩𑂲 𑂍𑂹𑂪𑂱 𑂍𑂹𑂪𑂲 𑂍𑂵 𑂍𑂶 𑂍𑂷 𑂍𑂸"),
        (Khudawadi, "𑊺 𑊺𑋠 𑊺𑋡 𑊺𑋢 𑊺𑋣 𑊺𑋤 𑊺𑋪𑋙𑋡 𑊺𑋪𑋙𑋢 𑊺𑋪𑋚𑋡 𑊺𑋪𑋚𑋢 𑊺𑋥 𑊺𑋦 𑊺𑋧 𑊺𑋨"),
        (Limbu, "ᤁ ᤁᤠ ᤁᤡ ᤁᤡ᤺ ᤁᤢ ᤁᤢ᤺ ᤁᤪᤢ ᤁᤪᤢ᤺ ᤁ᤻ᤗᤢ ᤁ᤻ᤗᤢ᤺ ᤁᤣ ᤁᤤ ᤁᤥ ᤁᤦ"),
        (MasaramGondi, "𑴌 𑴌𑴱 𑴌𑴲 𑴌𑴳 𑴌𑴴 𑴌𑴵 𑴌𑴶 𑴌𑵇𑴵 𑴌𑵅𑴧𑴴 𑴌𑵅𑴧𑴵 𑴌𑴺 𑴌𑴼 𑴌𑴽 𑴌𑴿"),
        (Takri, "𑚊 𑚊𑚭 𑚊𑚮 𑚊𑚯 𑚊𑚰 𑚊𑚱 𑚊𑚶𑚤𑚮 𑚊𑚶𑚤𑚯 𑚊𑚶𑚥𑚮 𑚊𑚶𑚥𑚯 𑚊𑚲 𑚊𑚳 𑚊𑚴 𑚊𑚵"),
        (ZanabazarSquare, "𑨋 𑨋𑨊 𑨋𑨁 𑨋𑨁𑨊 𑨋𑨂 𑨋𑨂𑨊 𑨋𑩇𑨫𑨉 𑨋𑩇𑨫𑨉𑨊 𑨋𑩇𑨬𑨉 𑨋𑩇𑨬𑨉𑨊 𑨋𑨄 𑨋𑨄𑨊 𑨋𑨆 𑨋𑨆𑨊"),
    ]);

    // Scripts without R/RR/L/LL or long O.
    assert_transliterate(
        slp1_text,
        Slp1,
        OlChiki,
        "ᱠᱚ ᱠᱟ ᱠᱤ ᱠᱤᱻ ᱠᱩ ᱠᱩᱻ ᱠᱨᱩ ᱠᱨᱩᱻ ᱠᱞᱩ ᱠᱞᱩᱻ ᱠᱮ ᱠᱚᱤ ᱠᱳ ᱠᱚᱩ",
    );

    // Scripts without R/RR/L/LL or long U.
    assert_transliterate(
        slp1_text,
        Slp1,
        MeeteiMayek,
        "ꯀ ꯀꯥ ꯀꯤ ꯀꯤ ꯀꯨ ꯀꯨ ꯛꯔꯨ ꯛꯔꯨ ꯛꯂꯨ ꯛꯂꯨ ꯀꯦ ꯀꯩ ꯀꯣ ꯀꯧ",
    );
}

#[test]
fn sanskrit_ayogavahas() {
    assert_two_way_pairwise(&[
        (BarahaSouth, "aM aH a~M"),
        (HarvardKyoto, "aM aH a~"),
        (Iast, "aṃ aḥ am̐"),
        (Iso15919, "aṁ aḥ am̐"),
        (Itrans, "aM aH a.N"),
        (Slp1, "aM aH a~"),
        (Velthuis, "a.m a.h a~m"),
        (Wx, "aM aH az"),
        // Indic
        (Assamese, "অং অঃ অঁ"),
        (Balinese, "ᬅᬂ ᬅᬄ ᬅᬁ"),
        (Bengali, "অং অঃ অঁ"),
        (Bhaiksuki, "𑰀𑰽𑱃𑰀𑰾𑱃𑰀𑰼"),
        (Brahmi, "𑀅𑀁 𑀅𑀂 𑀅𑀀"),
        (Cham, "ꨀꩌ ꨀꩍ ꨀꩃ"),
        (Devanagari, "अं अः अँ"),
        (Grantha, "𑌅𑌂 𑌅𑌃 𑌅𑌁"),
        (Gujarati, "અં અઃ અઁ"),
        (Javanese, "ꦄꦁ ꦄꦃ ꦄꦀ"),
        (Kaithi, "𑂃𑂁 𑂃𑂂 𑂃𑂀"),
        (Kannada, "ಅಂ ಅಃ ಅಁ"),
        (Malayalam, "അം അഃ അഁ"),
        (Newa, "𑐀𑑄 𑐀𑑅 𑐀𑑃"),
        (Odia, "ଅଂ ଅଃ ଅଁ"),
        (OlChiki, "ᱚᱝ ᱚᱷ ᱚᱸ"),
        (Saurashtra, "ꢂꢀ ꢂꢁ ꢂꣅ"),
        (Sharada, "𑆃𑆁 𑆃𑆂 𑆃𑆀"),
        (Siddham, "𑖀𑖽 𑖀𑖾 𑖀𑖼"),
        (Tamil, "அம்ʼ அ꞉ அம்ˮ"),
        (Telugu, "అం అః అఁ"),
        (Tibetan, "ཨཾ་ཨཿ་ཨྃ"),
        (Tirhuta, "𑒁𑓀 𑒁𑓁 𑒁𑒿"),
        (ZanabazarSquare, "𑨀𑨸 𑨀𑨹 𑨀𑨵"),
    ]);

    // Scripts without a chandrabindu
    assert_one_way_pairwise(
        (Slp1, "aM aH a~"),
        &[
            (Burmese, "အံ အး အံ"),
            (Dogra, "𑠀𑠷 𑠀𑠸 𑠀𑠷"),
            (GunjalaGondi, "𑵠𑶕 𑵠𑶖 𑵠𑶕"),
            (Kharoshthi, "𐨀𐨎 𐨀𐨏 𐨀𐨎"),
            (Khmer, "អំ អះ អំ"),
            (Khudawadi, "𑊰𑋟 𑊰𑋞𑋪 𑊰𑋟"),
            (Limbu, "ᤀᤱ ᤀᤜ᤻ ᤀᤱ"),
            (MasaramGondi, "𑴀𑵀 𑴀𑵁 𑴀𑵀"),
            (MeeteiMayek, "ꯑꯪ ꯑꯍ꯭ ꯑꯪ"),
            (Modi, "𑘀𑘽 𑘀𑘾 𑘀𑘽"),
            (Mon, "အံ အး အံ"),
            (Nandinagari, "𑦠𑧞 𑦠𑧟 𑦠𑧞"),
            (Sinhala, "අං අඃ අං"),
            (Soyombo, "𑩐𑪖 𑩐𑪗 𑩐𑪖"),
            (TaiTham, "ᩋᩴ ᩋᩡ ᩋᩴ"),
            (Takri, "𑚀𑚫 𑚀𑚬 𑚀𑚫"),
            (Thai, "อํ อห์ อํ"),
        ],
    );
}

#[test]
fn sanskrit_consonants_non_vedic() {
    let slp1 = "ka Ka ga Ga Na ca Ca ja Ja Ya wa Wa qa Qa Ra ta Ta da Da na pa Pa ba Ba ma ya ra la va Sa za sa ha";

    assert_two_way_pairwise(&[
        (BarahaSouth, "ka kha ga gha ~ga cha Cha ja jha ~ja Ta Tha Da Dha Na ta tha da dha na pa pha ba bha ma ya ra la va sha Sha sa ha"),
        (HarvardKyoto, "ka kha ga gha Ga ca cha ja jha Ja Ta Tha Da Dha Na ta tha da dha na pa pha ba bha ma ya ra la va za Sa sa ha"),
        (Iast, "ka kha ga gha ṅa ca cha ja jha ña ṭa ṭha ḍa ḍha ṇa ta tha da dha na pa pha ba bha ma ya ra la va śa ṣa sa ha"),
        (Iso15919, "ka kha ga gha ṅa ca cha ja jha ña ṭa ṭha ḍa ḍha ṇa ta tha da dha na pa pha ba bha ma ya ra la va śa ṣa sa ha"),
        (Itrans, "ka kha ga gha ~Na cha Cha ja jha ~na Ta Tha Da Dha Na ta tha da dha na pa pha ba bha ma ya ra la va sha Sha sa ha"),
        (Slp1, slp1),
        (Velthuis, "ka kha ga gha \"na ca cha ja jha ~na .ta .tha .da .dha .na ta tha da dha na pa pha ba bha ma ya ra la va \"sa .sa sa ha"),
        (Wx, "ka Ka ga Ga fa ca Ca ja Ja Fa ta Ta da Da Na wa Wa xa Xa na pa Pa ba Ba ma ya ra la va Sa Ra sa ha"),
        // Indic
        (Assamese, "ক খ গ ঘ ঙ চ ছ জ ঝ ঞ ট ঠ ড ঢ ণ ত থ দ ধ ন প ফ ব ভ ম য ৰ ল ৱ শ ষ স হ"),
        (Balinese, "ᬓ ᬔ ᬕ ᬖ ᬗ ᬘ ᬙ ᬚ ᬛ ᬜ ᬝ ᬞ ᬟ ᬠ ᬡ ᬢ ᬣ ᬤ ᬥ ᬦ ᬧ ᬨ ᬩ ᬪ ᬫ ᬬ ᬭ ᬮ ᬯ ᬰ ᬱ ᬲ ᬳ"),
        (Bhaiksuki, "𑰎𑱃𑰏𑱃𑰐𑱃𑰑𑱃𑰒𑱃𑰓𑱃𑰔𑱃𑰕𑱃𑰖𑱃𑰗𑱃𑰘𑱃𑰙𑱃𑰚𑱃𑰛𑱃𑰜𑱃𑰝𑱃𑰞𑱃𑰟𑱃𑰠𑱃𑰡𑱃𑰢𑱃𑰣𑱃𑰤𑱃𑰥𑱃𑰦𑱃𑰧𑱃𑰨𑱃𑰩𑱃𑰪𑱃𑰫𑱃𑰬𑱃𑰭𑱃𑰮"),
        (Brahmi, "𑀓 𑀔 𑀕 𑀖 𑀗 𑀘 𑀙 𑀚 𑀛 𑀜 𑀝 𑀞 𑀟 𑀠 𑀡 𑀢 𑀣 𑀤 𑀥 𑀦 𑀧 𑀨 𑀩 𑀪 𑀫 𑀬 𑀭 𑀮 𑀯 𑀰 𑀱 𑀲 𑀳"),
        (Burmese, "က ခ ဂ ဃ င စ ဆ ဇ ဈ ဉ ဋ ဌ ဍ ဎ ဏ တ ထ ဒ ဓ န ပ ဖ ဗ ဘ မ ယ ရ လ ဝ ၐ ၑ သ ဟ"),
        (Devanagari, "क ख ग घ ङ च छ ज झ ञ ट ठ ड ढ ण त थ द ध न प फ ब भ म य र ल व श ष स ह"),
        (Dogra, "𑠊 𑠋 𑠌 𑠍 𑠎 𑠏 𑠐 𑠑 𑠒 𑠓 𑠔 𑠕 𑠖 𑠗 𑠘 𑠙 𑠚 𑠛 𑠜 𑠝 𑠞 𑠟 𑠠 𑠡 𑠢 𑠣 𑠤 𑠥 𑠦 𑠧 𑠨 𑠩 𑠪"),
        (Grantha, "𑌕 𑌖 𑌗 𑌘 𑌙 𑌚 𑌛 𑌜 𑌝 𑌞 𑌟 𑌠 𑌡 𑌢 𑌣 𑌤 𑌥 𑌦 𑌧 𑌨 𑌪 𑌫 𑌬 𑌭 𑌮 𑌯 𑌰 𑌲 𑌵 𑌶 𑌷 𑌸 𑌹"),
        (Gujarati, "ક ખ ગ ઘ ઙ ચ છ જ ઝ ઞ ટ ઠ ડ ઢ ણ ત થ દ ધ ન પ ફ બ ભ મ ય ર લ વ શ ષ સ હ"),
        (Javanese, "ꦏ ꦑ ꦒ ꦓ ꦔ ꦕ ꦖ ꦗ ꦙ ꦚ ꦛ ꦜ ꦝ ꦞ ꦟ ꦠ ꦡ ꦢ ꦣ ꦤ ꦥ ꦦ ꦧ ꦨ ꦩ ꦪ ꦫ ꦭ ꦮ ꦯ ꦰ ꦱ ꦲ"),
        (Kaithi, "𑂍 𑂎 𑂏 𑂐 𑂑 𑂒 𑂓 𑂔 𑂕 𑂖 𑂗 𑂘 𑂙 𑂛 𑂝 𑂞 𑂟 𑂠 𑂡 𑂢 𑂣 𑂤 𑂥 𑂦 𑂧 𑂨 𑂩 𑂪 𑂫 𑂬 𑂭 𑂮 𑂯"),
        (Kannada, "ಕ ಖ ಗ ಘ ಙ ಚ ಛ ಜ ಝ ಞ ಟ ಠ ಡ ಢ ಣ ತ ಥ ದ ಧ ನ ಪ ಫ ಬ ಭ ಮ ಯ ರ ಲ ವ ಶ ಷ ಸ ಹ"),
        (Khmer, "ក ខ គ ឃ ង ច ឆ ជ ឈ ញ ដ ឋ ឌ ឍ ណ ត ថ ទ ធ ន ប ផ ព ភ ម យ រ ល វ ឝ ឞ ស ហ"),
        (Khudawadi, "𑊺 𑊻 𑊼 𑊾 𑊿 𑋀 𑋁 𑋂 𑋄 𑋅 𑋆 𑋇 𑋈 𑋋 𑋌 𑋍 𑋎 𑋏 𑋐 𑋑 𑋒 𑋓 𑋔 𑋖 𑋗 𑋘 𑋙 𑋚 𑋛 𑋜 𑋜𑋩 𑋝 𑋞"),
        (Malayalam, "ക ഖ ഗ ഘ ങ ച ഛ ജ ഝ ഞ ട ഠ ഡ ഢ ണ ത ഥ ദ ധ ന പ ഫ ബ ഭ മ യ ര ല വ ശ ഷ സ ഹ"),
        (MasaramGondi, "𑴌 𑴍 𑴎 𑴏 𑴐 𑴑 𑴒 𑴓 𑴔 𑴕 𑴖 𑴗 𑴘 𑴙 𑴚 𑴛 𑴜 𑴝 𑴞 𑴟 𑴠 𑴡 𑴢 𑴣 𑴤 𑴥 𑴦 𑴧 𑴨 𑴩 𑴪 𑴫 𑴬"),
        (Modi, "𑘎 𑘏 𑘐 𑘑 𑘒 𑘓 𑘔 𑘕 𑘖 𑘗 𑘘 𑘙 𑘚 𑘛 𑘜 𑘝 𑘞 𑘟 𑘠 𑘡 𑘢 𑘣 𑘤 𑘥 𑘦 𑘧 𑘨 𑘩 𑘪 𑘫 𑘬 𑘭 𑘮"),
        (Mon, "က ခ ဂ ဃ ၚ စ ဆ ဇ ၛ ည ဋ ဌ ဍ ဎ ဏ တ ထ ဒ ဓ န ပ ဖ ဗ ဘ မ ယ ရ လ ဝ ၐ ၑ သ ဟ"),
        (Nandinagari, "𑦮 𑦯 𑦰 𑦱 𑦲 𑦳 𑦴 𑦵 𑦶 𑦷 𑦸 𑦹 𑦺 𑦻 𑦼 𑦽 𑦾 𑦿 𑧀 𑧁 𑧂 𑧃 𑧄 𑧅 𑧆 𑧇 𑧈 𑧉 𑧊 𑧋 𑧌 𑧍 𑧎"),
        (Newa, "𑐎 𑐏 𑐐 𑐑 𑐒 𑐔 𑐕 𑐖 𑐗 𑐘 𑐚 𑐛 𑐜 𑐝 𑐞 𑐟 𑐠 𑐡 𑐢 𑐣 𑐥 𑐦 𑐧 𑐨 𑐩 𑐫 𑐬 𑐮 𑐰 𑐱 𑐲 𑐳 𑐴"),
        (Odia, "କ ଖ ଗ ଘ ଙ ଚ ଛ ଜ ଝ ଞ ଟ ଠ ଡ ଢ ଣ ତ ଥ ଦ ଧ ନ ପ ଫ ବ ଭ ମ ଯ ର ଲ ଵ ଶ ଷ ସ ହ"),
        (Saurashtra, "ꢒ ꢓ ꢔ ꢕ ꢖ ꢗ ꢘ ꢙ ꢚ ꢛ ꢜ ꢝ ꢞ ꢟ ꢠ ꢡ ꢢ ꢣ ꢤ ꢥ ꢦ ꢧ ꢨ ꢩ ꢪ ꢫ ꢬ ꢭ ꢮ ꢯ ꢰ ꢱ ꢲ"),
        (Sharada, "𑆑 𑆒 𑆓 𑆔 𑆕 𑆖 𑆗 𑆘 𑆙 𑆚 𑆛 𑆜 𑆝 𑆞 𑆟 𑆠 𑆡 𑆢 𑆣 𑆤 𑆥 𑆦 𑆧 𑆨 𑆩 𑆪 𑆫 𑆬 𑆮 𑆯 𑆰 𑆱 𑆲"),
        (Siddham, "𑖎 𑖏 𑖐 𑖑 𑖒 𑖓 𑖔 𑖕 𑖖 𑖗 𑖘 𑖙 𑖚 𑖛 𑖜 𑖝 𑖞 𑖟 𑖠 𑖡 𑖢 𑖣 𑖤 𑖥 𑖦 𑖧 𑖨 𑖩 𑖪 𑖫 𑖬 𑖭 𑖮"),
        (Sinhala, "ක ඛ ග ඝ ඞ ච ඡ ජ ඣ ඤ ට ඨ ඩ ඪ ණ ත ථ ද ධ න ප ඵ බ භ ම ය ර ල ව ශ ෂ ස හ"),
        (Soyombo, "𑩜 𑩝 𑩞 𑩟 𑩠 𑩵 𑩶 𑩷 𑩤 𑩥 𑩦 𑩧 𑩨 𑩩 𑩪 𑩫 𑩬 𑩭 𑩮 𑩯 𑩰 𑩱 𑩲 𑩳 𑩴 𑩻 𑩼 𑩽 𑩾 𑩿 𑪀 𑪁 𑪂"),
        (TaiTham, "ᨠ ᨡ ᨣ ᨥ ᨦ ᨧ ᨨ ᨩ ᨫ ᨬ ᨭ ᨮ ᨯ ᨰ ᨱ ᨲ ᨳ ᨴ ᨵ ᨶ ᨸ ᨹ ᨻ ᨽ ᨾ ᨿ ᩁ ᩃ ᩅ ᩆ ᩇ ᩈ ᩉ"),
        (Takri, "𑚊 𑚸 𑚌 𑚍 𑚎 𑚏 𑚐 𑚑 𑚒 𑚓 𑚔 𑚕 𑚖 𑚗 𑚘 𑚙 𑚚 𑚛 𑚜 𑚝 𑚞 𑚟 𑚠 𑚡 𑚢 𑚣 𑚤 𑚥 𑚦 𑚧 𑚋 𑚨 𑚩"),
        (Tamil, "க க² க³ க⁴ ங ச ச² ஜ ஜ² ஞ ட ட² ட³ ட⁴ ண த த² த³ த⁴ ந ப ப² ப³ ப⁴ ம ய ர ல வ ஶ ஷ ஸ ஹ"),
        (Telugu, "క ఖ గ ఘ ఙ చ ఛ జ ఝ ఞ ట ఠ డ ఢ ణ త థ ద ధ న ప ఫ బ భ మ య ర ల వ శ ష స హ"),
        (Thai, "ก ข ค ฆ ง จ ฉ ช ฌ ญ ฏ ฐ ฑ ฒ ณ ต ถ ท ธ น ป ผ พ ภ ม ย ร ล ว ศ ษ ส ห"),
        (Tirhuta, "𑒏 𑒐 𑒑 𑒒 𑒓 𑒔 𑒕 𑒖 𑒗 𑒘 𑒙 𑒚 𑒛 𑒜 𑒝 𑒞 𑒟 𑒠 𑒡 𑒢 𑒣 𑒤 𑒥 𑒦 𑒧 𑒨 𑒩 𑒪 𑒫 𑒬 𑒭 𑒮 𑒯"),
        (ZanabazarSquare, "𑨋 𑨌 𑨍 𑨎 𑨏 𑨣 𑨤 𑨥 𑨦 𑨓 𑨔 𑨕 𑨖 𑨗 𑨘 𑨙 𑨚 𑨛 𑨜 𑨝 𑨞 𑨟 𑨠 𑨡 𑨢 𑨪 𑨫 𑨬 𑨭 𑨮 𑨯 𑨰 𑨱"),
    ]);

    // No distinction between ba / va
    assert_one_way_pairwise(
        (Slp1, slp1),
        &[
            (
                Bengali,
                "ক খ গ ঘ ঙ চ ছ জ ঝ ঞ ট ঠ ড ঢ ণ ত থ দ ধ ন প ফ ব ভ ম য র ল ব শ ষ স হ",
            ),
            (
                Tibetan,
                "ཀ་ཁ་ག་གྷ་ང་ཙ་ཚ་ཛ་ཛྷ་ཉ་ཊ་ཋ་ཌ་ཌྷ་ཎ་ཏ་ཐ་ད་དྷ་ན་པ་ཕ་བ་བྷ་མ་ཡ་ར་ལ་བ་ཤ་ཥ་ས་ཧ",
            ),
        ],
    );

    // No nga or jha
    assert_transliterate(
        slp1,
        Slp1,
        Kharoshthi,
        "𐨐 𐨑 𐨒 𐨓 𐨣 𐨕 𐨖 𐨗 𐨗 𐨙 𐨚 𐨛 𐨜 𐨝 𐨞 𐨟 𐨠 𐨡 𐨢 𐨣 𐨤 𐨥 𐨦 𐨧 𐨨 𐨩 𐨪 𐨫 𐨬 𐨭 𐨮 𐨯 𐨱",
    );

    // No jha, various nasals missing
    assert_transliterate(
        slp1,
        Slp1,
        Limbu,
        "ᤁ ᤂ ᤃ ᤄ ᤅ ᤆ ᤇ ᤈ ᤈ ᤏ ᤋ ᤌ ᤍ ᤎ ᤏ ᤋ ᤌ ᤍ ᤎ ᤏ ᤐ ᤑ ᤒ ᤓ ᤔ ᤕ ᤖ ᤗ ᤘ ᤙ ᤙ ᤛ ᤜ",
    );

    // No distinction between ca/cha and others.
    assert_transliterate(
        slp1,
        Slp1,
        MeeteiMayek,
        "ꯀ ꯈ ꯒ ꯘ ꯉ ꯆ ꯆ ꯖ ꯓ ꯅ ꯇ ꯊ ꯗ ꯙ ꯅ ꯇ ꯊ ꯗ ꯙ ꯅ ꯄ ꯐ ꯕ ꯚ ꯃ ꯌ ꯔ ꯂ ꯋ ꯁ ꯁ ꯁ ꯍ",
    );

    // No distinction between sa-s.
    assert_transliterate(
        slp1,
        Slp1,
        OlChiki,
        concat!(
            "ᱠᱚ ᱠᱷᱚ ᱜᱚ ᱜᱷᱚ ᱶᱚ ᱪᱚ ᱪᱷᱚ ᱡᱚ ᱡᱷᱚ ᱧᱚ ᱴᱚ ᱴᱷᱚ ᱰᱚ ᱰᱷᱚ ᱬᱚ ",
            "ᱛᱚ ᱛᱷᱚ ᱫᱚ ᱫᱷᱚ ᱱᱚ ᱯᱚ ᱯᱷᱚ ᱵᱚ ᱵᱷᱚ ᱢᱚ ᱭᱚ ᱨᱚ ᱞᱚ ᱣᱚ ᱥᱚ ᱥᱚ ᱥᱚ ᱦᱚ"
        ),
    );

    // Missing two na-s and two sa-s
    assert_one_way_pairwise(
        (Slp1, slp1),
        &[(
            GunjalaGondi,
            "𑵱 𑵲 𑵶 𑵷 𑶄 𑵻 𑵼 𑶀 𑶁 𑵺 𑵽 𑵾 𑶂 𑶃 𑵺 𑵳 𑵴 𑵸 𑵹 𑵺 𑶅 𑶆 𑵮 𑵯 𑵰 𑵬 𑶈 𑵵 𑵭 𑶉 𑶉 𑶉 𑶇",
        )],
    );

    // No distinction between Ta / ta
    assert_one_way_pairwise(
        (Slp1, slp1),
        &[(
            Cham,
            "ꨆ ꨇ ꨈ ꨉ ꨋ ꨌ ꨍ ꨎ ꨏ ꨑ ꨓ ꨔ ꨕ ꨖ ꨘ ꨓ ꨔ ꨕ ꨖ ꨘ ꨚ ꨜ ꨝ ꨞ ꨠ ꨢ ꨣ ꨤ ꨥ ꨦ ꨦ ꨧ ꨨ",
        )],
    );
}

// Test only Latin schemes, since most Brahmic schemes will just use a candrabindu here.
#[test]
fn sanskrit_nasal_semivowels() {
    // Example from https://list.indology.info/pipermail/indology/2023-October/058252.html
    let deva_text = "त्रील्ँलोकान्";

    assert_two_way_pairwise(&[
        (Devanagari, deva_text),
        (Iast, "trīlm̐lokān"),
        (Iso15919, "trīlm̐lōkān"),
        (Slp1, "trIl~lokAn"),
    ]);

    // Alternate for IAST.
    // TODO: or should this be preferred?
    assert_transliterate("trīl̃lokān", Iast, Devanagari, deva_text);
}

#[test]
fn sanskrit_symbols() {
    assert_two_way_pairwise(&[
        (HarvardKyoto, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Iast, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Itrans, "0 1 2 3 4 5 6 7 8 9 | || .a"),
        (Slp1, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Velthuis, "0 1 2 3 4 5 6 7 8 9 | || .a"),
        (Wx, "0 1 2 3 4 5 6 7 8 9 . .. Z"),
        // Indic
        (Assamese, "০ ১ ২ ৩ ৪ ৫ ৬ ৭ ৮ ৯ । ॥ ঽ"),
        (Bengali, "০ ১ ২ ৩ ৪ ৫ ৬ ৭ ৮ ৯ । ॥ ঽ"),
        (Bhaiksuki, "𑱐𑱃𑱑𑱃𑱒𑱃𑱓𑱃𑱔𑱃𑱕𑱃𑱖𑱃𑱗𑱃𑱘𑱃𑱙𑱃𑱁𑱃𑱂𑱃𑱀"),
        (Cham, "꩐ ꩑ ꩒ ꩓ ꩔ ꩕ ꩖ ꩗ ꩘ ꩙ ꩝ ꩞ '"),
        (Devanagari, "० १ २ ३ ४ ५ ६ ७ ८ ९ । ॥ ऽ"),
        (Dogra, "० १ २ ३ ४ ५ ६ ७ ८ ९ । ॥ ऽ"),
        (Grantha, "௦ ௧ ௨ ௩ ௪ ௫ ௬ ௭ ௮ ௯ । ॥ 𑌽"),
        // TODO: confirm correct behavior of Gunjala Gondi avagraha.
        (GunjalaGondi, "𑶠 𑶡 𑶢 𑶣 𑶤 𑶥 𑶦 𑶧 𑶨 𑶩 . .. ऽ"),
        (Gujarati, "૦ ૧ ૨ ૩ ૪ ૫ ૬ ૭ ૮ ૯ । ॥ ઽ"),
        (Gurmukhi, "੦ ੧ ੨ ੩ ੪ ੫ ੬ ੭ ੮ ੯ । ॥ ऽ"),
        (Javanese, "꧐ ꧑ ꧒ ꧓ ꧔ ꧕ ꧖ ꧗ ꧘ ꧙ ꧈ ꧉ '"),
        (Kaithi, "० १ २ ३ ४ ५ ६ ७ ८ ९ 𑃀 𑃁 ऽ"),
        (Kannada, "೦ ೧ ೨ ೩ ೪ ೫ ೬ ೭ ೮ ೯ । ॥ ಽ"),
        (Khmer, "០ ១ ២ ៣ ៤ ៥ ៦ ៧ ៨ ៩ ។ ៕ ៜ"),
        (Khudawadi, "𑋰 𑋱 𑋲 𑋳 𑋴 𑋵 𑋶 𑋷 𑋸 𑋹 । ॥ ऽ"),
        (Limbu, "᥆ ᥇ ᥈ ᥉ ᥊ ᥋ ᥌ ᥍ ᥎ ᥏ । ॥ '"),
        (Malayalam, "൦ ൧ ൨ ൩ ൪ ൫ ൬ ൭ ൮ ൯ । ॥ ഽ"),
        (MasaramGondi, "𑵐 𑵑 𑵒 𑵓 𑵔 𑵕 𑵖 𑵗 𑵘 𑵙 । ॥ ऽ"),
        (MeeteiMayek, "꯰ ꯱ ꯲ ꯳ ꯴ ꯵ ꯶ ꯷ ꯸ ꯹ ꯫ ꯫꯫ '"),
        (Modi, "𑙐 𑙑 𑙒 𑙓 𑙔 𑙕 𑙖 𑙗 𑙘 𑙙 𑙁 𑙂 ऽ"),
        (Mon, "၀ ၁ ၂ ၃ ၄ ၅ ၆ ၇ ၈ ၉ ၊ ။ '"),
        (Nandinagari, "೦ ೧ ೨ ೩ ೪ ೫ ೬ ೭ ೮ ೯ । ॥ 𑧡"),
        (Newa, "𑑐 𑑑 𑑒 𑑓 𑑔 𑑕 𑑖 𑑗 𑑘 𑑙 𑑋 𑑌 𑑇"),
        (Odia, "୦ ୧ ୨ ୩ ୪ ୫ ୬ ୭ ୮ ୯ । ॥ ଽ"),
        (OlChiki, "᱐ ᱑ ᱒ ᱓ ᱔ ᱕ ᱖ ᱗ ᱘ ᱙ ᱾ ᱿ '"),
        (Saurashtra, "꣐ ꣑ ꣒ ꣓ ꣔ ꣕ ꣖ ꣗ ꣘ ꣙ ꣎ ꣏ ఽ"),
        // (Soyombo, "0 1 2 3 4 5 6 7 8 9 𑪛 𑪜 '"),
        (TaiTham, "᪐ ᪑ ᪒ ᪓ ᪔ ᪕ ᪖ ᪗ ᪘ ᪙ ᪨ ᪩ '"),
        (Takri, "𑛀 𑛁 𑛂 𑛃 𑛄 𑛅 𑛆 𑛇 𑛈 𑛉 । ॥ ऽ"),
        (Telugu, "౦ ౧ ౨ ౩ ౪ ౫ ౬ ౭ ౮ ౯ । ॥ ఽ"),
        (Thai, "๐ ๑ ๒ ๓ ๔ ๕ ๖ ๗ ๘ ๙ ฯ ๚ '"),
        (Tibetan, "༠་༡་༢་༣་༤་༥་༦་༧་༨་༩་།་༎་྅"),
        (Tirhuta, "𑓐 𑓑 𑓒 𑓓 𑓔 𑓕 𑓖 𑓗 𑓘 𑓙 । ॥ 𑓄"),
    ]);

    // Only dandas
    assert_two_way_pairwise(&[(Slp1, ". .."), (ZanabazarSquare, "𑩂 𑩃")]);
}

#[test]
fn sanskrit_basic_sentences() {
    let slp1_text = concat!(
        "nArAyaRaM namaskftya naraM cEva narottamam . ",
        "devIM sarasvatIM cEva tato jayamudIrayet .. 1 .."
    );
    assert_two_way_pairwise(&[
        (BarahaSouth, "nArAyaNaM namaskRutya naraM chaiva narOttamam | dEvIM sarasvatIM chaiva tatO jayamudIrayEt || 1 ||",),
        (HarvardKyoto, "nArAyaNaM namaskRtya naraM caiva narottamam . devIM sarasvatIM caiva tato jayamudIrayet .. 1 ..",),
        (Iast, "nārāyaṇaṃ namaskṛtya naraṃ caiva narottamam . devīṃ sarasvatīṃ caiva tato jayamudīrayet .. 1 .."),
        (Iso15919, "nārāyaṇaṁ namaskr̥tya naraṁ caiva narōttamam . dēvīṁ sarasvatīṁ caiva tatō jayamudīrayēt .. 1 .."),
        (Itrans, "nArAyaNaM namaskRRitya naraM chaiva narottamam | devIM sarasvatIM chaiva tato jayamudIrayet || 1 ||"),
        (Slp1, slp1_text),
        (Velthuis, "naaraaya.na.m namask.rtya nara.m caiva narottamam | devii.m sarasvatii.m caiva tato jayamudiirayet || 1 ||"),
        (Wx, "nArAyaNaM namaskqwya naraM cEva narowwamam . xevIM sarasvawIM cEva wawo jayamuxIrayew .. 1 .."),
        // Indic
        (Balinese, "ᬦᬵᬭᬵᬬᬡᬂ ᬦᬫᬲ᭄ᬓᬺᬢ᭄ᬬ ᬦᬭᬂ ᬘᬿᬯ ᬦᬭᭀᬢ᭄ᬢᬫᬫ᭄ ᭞ ᬤᬾᬯᬷᬂ ᬲᬭᬲ᭄ᬯᬢᬷᬂ ᬘᬿᬯ ᬢᬢᭀ ᬚᬬᬫᬸᬤᬷᬭᬬᬾᬢ᭄ ᭟ ᭑ ᭟"),
        (Bhaiksuki, "𑰡𑰯𑰨𑰯𑰧𑰜𑰽𑱃𑰡𑰦𑰭𑰿𑰎𑰴𑰝𑰿𑰧𑱃𑰡𑰨𑰽𑱃𑰓𑰹𑰪𑱃𑰡𑰨𑰺𑰝𑰿𑰝𑰦𑰦𑰿𑱃𑱁𑱃𑰟𑰸𑰪𑰱𑰽𑱃𑰭𑰨𑰭𑰿𑰪𑰝𑰱𑰽𑱃𑰓𑰹𑰪𑱃𑰝𑰝𑰺𑱃𑰕𑰧𑰦𑰲𑰟𑰱𑰨𑰧𑰸𑰝𑰿𑱃𑱂𑱃𑱑𑱃𑱂"),
        (Brahmi, "𑀦𑀸𑀭𑀸𑀬𑀡𑀁 𑀦𑀫𑀲𑁆𑀓𑀾𑀢𑁆𑀬 𑀦𑀭𑀁 𑀘𑁃𑀯 𑀦𑀭𑁄𑀢𑁆𑀢𑀫𑀫𑁆 𑁇 𑀤𑁂𑀯𑀻𑀁 𑀲𑀭𑀲𑁆𑀯𑀢𑀻𑀁 𑀘𑁃𑀯 𑀢𑀢𑁄 𑀚𑀬𑀫𑀼𑀤𑀻𑀭𑀬𑁂𑀢𑁆 𑁈 𑁧 𑁈"),
        (Burmese, "နာရာယဏံ နမသ္ကၖတျ နရံ စဲဝ နရောတ္တမမ် ၊ ဒေဝီံ သရသွတီံ စဲဝ တတော ဇယမုဒီရယေတ် ။ ၁ ။"),
        (Devanagari, "नारायणं नमस्कृत्य नरं चैव नरोत्तमम् । देवीं सरस्वतीं चैव ततो जयमुदीरयेत् ॥ १ ॥"),
        (Dogra, "𑠝𑠬𑠤𑠬𑠣𑠘𑠷 𑠝𑠢𑠩𑠹𑠊𑠱𑠙𑠹𑠣 𑠝𑠤𑠷 𑠏𑠴𑠦 𑠝𑠤𑠵𑠙𑠹𑠙𑠢𑠢𑠹 । 𑠛𑠳𑠦𑠮𑠷 𑠩𑠤𑠩𑠹𑠦𑠙𑠮𑠷 𑠏𑠴𑠦 𑠙𑠙𑠵 𑠑𑠣𑠢𑠯𑠛𑠮𑠤𑠣𑠳𑠙𑠹 ॥ १ ॥"),
        (Grantha, "𑌨𑌾𑌰𑌾𑌯𑌣𑌂 𑌨𑌮𑌸𑍍𑌕𑍃𑌤𑍍𑌯 𑌨𑌰𑌂 𑌚𑍈𑌵 𑌨𑌰𑍋𑌤𑍍𑌤𑌮𑌮𑍍 । 𑌦𑍇𑌵𑍀𑌂 𑌸𑌰𑌸𑍍𑌵𑌤𑍀𑌂 𑌚𑍈𑌵 𑌤𑌤𑍋 𑌜𑌯𑌮𑍁𑌦𑍀𑌰𑌯𑍇𑌤𑍍 ॥ ௧ ॥"),
        (Gujarati, "નારાયણં નમસ્કૃત્ય નરં ચૈવ નરોત્તમમ્ । દેવીં સરસ્વતીં ચૈવ તતો જયમુદીરયેત્ ॥ ૧ ॥"),
        (Javanese, "ꦤꦴꦫꦴꦪꦟꦁ ꦤꦩꦱ꧀ꦏꦽꦠꦾ ꦤꦫꦁ ꦕꦻꦮ ꦤꦫꦺꦴꦠ꧀ꦠꦩꦩ꧀ ꧈ ꦢꦺꦮꦷꦁ ꦱꦫꦱ꧀ꦮꦠꦷꦁ ꦕꦻꦮ ꦠꦠꦺꦴ ꦗꦪꦩꦸꦢꦷꦫꦪꦺꦠ꧀ ꧉ ꧑ ꧉"),
        (Kaithi, "𑂢𑂰𑂩𑂰𑂨𑂝𑂁 𑂢𑂧𑂮𑂹𑂍𑂹𑂩𑂱𑂞𑂹𑂨 𑂢𑂩𑂁 𑂒𑂶𑂫 𑂢𑂩𑂷𑂞𑂹𑂞𑂧𑂧𑂹 𑃀 𑂠𑂵𑂫𑂲𑂁 𑂮𑂩𑂮𑂹𑂫𑂞𑂲𑂁 𑂒𑂶𑂫 𑂞𑂞𑂷 𑂔𑂨𑂧𑂳𑂠𑂲𑂩𑂨𑂵𑂞𑂹 𑃁 १ 𑃁"),
        (Kannada, "ನಾರಾಯಣಂ ನಮಸ್ಕೃತ್ಯ ನರಂ ಚೈವ ನರೋತ್ತಮಮ್ । ದೇವೀಂ ಸರಸ್ವತೀಂ ಚೈವ ತತೋ ಜಯಮುದೀರಯೇತ್ ॥ ೧ ॥"),
        (Kharoshthi, "𐨣𐨌𐨪𐨌𐨩𐨞𐨎 𐨣𐨨𐨯𐨿𐨐𐨃𐨟𐨿𐨩 𐨣𐨪𐨎 𐨕𐨅𐨌𐨬 𐨣𐨪𐨆𐨟𐨿𐨟𐨨𐨨𐨿 𐩖 𐨡𐨅𐨬𐨁𐨌𐨎 𐨯𐨪𐨯𐨿𐨬𐨟𐨁𐨌𐨎 𐨕𐨅𐨌𐨬 𐨟𐨟𐨆 𐨗𐨩𐨨𐨂𐨡𐨁𐨌𐨪𐨩𐨅𐨟𐨿 𐩗 𐩀 𐩗"),
        (Khmer, "នារាយណំ នមស្ក្ឫត្យ នរំ ចៃវ នរោត្តមម៑ ។ ទេវីំ សរស្វតីំ ចៃវ តតោ ជយមុទីរយេត៑ ៕ ១ ៕"),
        (Khudawadi, "𑋑𑋠𑋙𑋠𑋘𑋌𑋟 𑋑𑋗𑋝𑋪𑊺𑋪𑋙𑋡𑋍𑋪𑋘 𑋑𑋙𑋟 𑋀𑋦𑋛 𑋑𑋙𑋧𑋍𑋪𑋍𑋗𑋗𑋪 । 𑋏𑋥𑋛𑋢𑋟 𑋝𑋙𑋝𑋪𑋛𑋍𑋢𑋟 𑋀𑋦𑋛 𑋍𑋍𑋧 𑋂𑋘𑋗𑋣𑋏𑋢𑋙𑋘𑋥𑋍𑋪 ॥ 𑋱 ॥"),
        // (Limbu, "ᤏᤠᤖᤠᤕᤏᤱ ᤏᤔᤛ᤻ᤁᤪᤢᤋᤩ ᤏᤖᤱ ᤆᤤᤘ ᤏᤖᤥᤳᤋᤔᤶ । ᤍᤣᤘᤡ᤺ᤱ ᤛᤖᤛᤫᤋᤡ᤺ᤱ ᤆᤤᤘ ᤋᤋᤥ ᤈᤕᤔᤢᤍᤡ᤺ᤖᤕᤣᤳ ॥ ᥇ ॥"),
        (Malayalam, "നാരായണം നമസ്കൃത്യ നരം ചൈവ നരോത്തമമ് । ദേവീം സരസ്വതീം ചൈവ തതോ ജയമുദീരയേത് ॥ ൧ ॥"),
        (MasaramGondi, "𑴟𑴱𑴦𑴱𑴥𑴚𑵀 𑴟𑴤𑴫𑵅𑴌𑴶𑴛𑵅𑴥 𑴟𑴦𑵀 𑴑𑴼𑴨 𑴟𑴦𑴽𑴛𑵅𑴛𑴤𑴤𑵄 । 𑴝𑴺𑴨𑴳𑵀 𑴫𑴦𑴫𑵅𑴨𑴛𑴳𑵀 𑴑𑴼𑴨 𑴛𑴛𑴽 𑴓𑴥𑴤𑴴𑴝𑴳𑴦𑴥𑴺𑴛𑵄 ॥ 𑵑 ॥"),
        (Modi, "𑘡𑘰𑘨𑘰𑘧𑘜𑘽 𑘡𑘦𑘭𑘿𑘎𑘵𑘝𑘿𑘧 𑘡𑘨𑘽 𑘓𑘺𑘪 𑘡𑘨𑘻𑘝𑘿𑘝𑘦𑘦𑘿 𑙁 𑘟𑘹𑘪𑘲𑘽 𑘭𑘨𑘭𑘿𑘪𑘝𑘲𑘽 𑘓𑘺𑘪 𑘝𑘝𑘻 𑘕𑘧𑘦𑘳𑘟𑘲𑘨𑘧𑘹𑘝𑘿 𑙂 𑙑 𑙂"),
        (Mon, "နာရာယဏံ နမသ္ကၖတျ နရံ စဲဝ နရောတ္တမမ် ၊ ဒေဝဳံ သရသွတဳံ စဲဝ တတော ဇယမုဒဳရယေတ် ။ ၁ ။"),
        (Nandinagari, "𑧁𑧑𑧈𑧑𑧇𑦼𑧞 𑧁𑧆𑧍𑧠𑦮𑧖𑦽𑧠𑧇 𑧁𑧈𑧞 𑦳𑧛𑧊 𑧁𑧈𑧜𑦽𑧠𑦽𑧆𑧆𑧠 । 𑦿𑧚𑧊𑧓𑧞 𑧍𑧈𑧍𑧠𑧊𑦽𑧓𑧞 𑦳𑧛𑧊 𑦽𑦽𑧜 𑦵𑧇𑧆𑧔𑦿𑧓𑧈𑧇𑧚𑦽𑧠 ॥ ೧ ॥"),
        (Newa, "𑐣𑐵𑐬𑐵𑐫𑐞𑑄 𑐣𑐩𑐳𑑂𑐎𑐺𑐟𑑂𑐫 𑐣𑐬𑑄 𑐔𑐿𑐰 𑐣𑐬𑑀𑐟𑑂𑐟𑐩𑐩𑑂 𑑋 𑐡𑐾𑐰𑐷𑑄 𑐳𑐬𑐳𑑂𑐰𑐟𑐷𑑄 𑐔𑐿𑐰 𑐟𑐟𑑀 𑐖𑐫𑐩𑐸𑐡𑐷𑐬𑐫𑐾𑐟𑑂 𑑌 𑑑 𑑌"),
        (Odia, "ନାରାଯଣଂ ନମସ୍କୃତ୍ଯ ନରଂ ଚୈଵ ନରୋତ୍ତମମ୍ । ଦେଵୀଂ ସରସ୍ଵତୀଂ ଚୈଵ ତତୋ ଜଯମୁଦୀରଯେତ୍ ॥ ୧ ॥"),
        (Saurashtra, "ꢥꢵꢬꢵꢫꢠꢀ ꢥꢪꢱ꣄ꢒꢺꢡ꣄ꢫ ꢥꢬꢀ ꢗꣀꢮ ꢥꢬꣂꢡ꣄ꢡꢪꢪ꣄ ꣎ ꢣꢿꢮꢷꢀ ꢱꢬꢱ꣄ꢮꢡꢷꢀ ꢗꣀꢮ ꢡꢡꣂ ꢙꢫꢪꢸꢣꢷꢬꢫꢿꢡ꣄ ꣏ ꣑ ꣏"),
        (Sharada, "𑆤𑆳𑆫𑆳𑆪𑆟𑆁 𑆤𑆩𑆱𑇀𑆑𑆸𑆠𑇀𑆪 𑆤𑆫𑆁 𑆖𑆽𑆮 𑆤𑆫𑆾𑆠𑇀𑆠𑆩𑆩𑇀 𑇅 𑆢𑆼𑆮𑆵𑆁 𑆱𑆫𑆱𑇀𑆮𑆠𑆵𑆁 𑆖𑆽𑆮 𑆠𑆠𑆾 𑆘𑆪𑆩𑆶𑆢𑆵𑆫𑆪𑆼𑆠𑇀 𑇆 𑇑 𑇆"),
        (Siddham, "𑖡𑖯𑖨𑖯𑖧𑖜𑖽 𑖡𑖦𑖭𑖿𑖎𑖴𑖝𑖿𑖧 𑖡𑖨𑖽 𑖓𑖹𑖪 𑖡𑖨𑖺𑖝𑖿𑖝𑖦𑖦𑖿 𑗂 𑖟𑖸𑖪𑖱𑖽 𑖭𑖨𑖭𑖿𑖪𑖝𑖱𑖽 𑖓𑖹𑖪 𑖝𑖝𑖺 𑖕𑖧𑖦𑖲𑖟𑖱𑖨𑖧𑖸𑖝𑖿 𑗃 1 𑗃"),
        (TaiTham, "ᨶᩣᩁᩣᨿᨱᩴ ᨶᨾᩈ᩠ᨠ᩺ᩂᨲ᩠ᨿ ᨶᩁᩴ ᨧᩱᩅ ᨶᩁᩮᩣᨲ᩠ᨲᨾᨾ᩺ ᪨ ᨴᩮᩅᩦᩴ ᩈᩁᩈ᩠ᩅᨲᩦᩴ ᨧᩱᩅ ᨲᨲᩮᩣ ᨩᨿᨾᩩᨴᩦᩁᨿᩮᨲ᩺ ᪩ ᪑ ᪩"),
        (Tamil, "நாராயணம்ʼ நமஸ்க்ருʼத்ய நரம்ʼ சைவ நரோத்தமம் . தே³வீம்ʼ ஸரஸ்வதீம்ʼ சைவ ததோ ஜயமுதீ³ரயேத் .. 1 .."),
        (Telugu, "నారాయణం నమస్కృత్య నరం చైవ నరోత్తమమ్ । దేవీం సరస్వతీం చైవ తతో జయముదీరయేత్ ॥ ౧ ॥"),
        (Thai, "นารายณํ นมสฺกฺฤตฺย นรํ ไจว นโรตฺตมมฺ ฯ เทวีํ สรสฺวตีํ ไจว ตโต ชยมุทีรเยตฺ ๚ ๑ ๚"),
        (Tirhuta, "𑒢𑒰𑒩𑒰𑒨𑒝𑓀 𑒢𑒧𑒮𑓂𑒏𑒵𑒞𑓂𑒨 𑒢𑒩𑓀 𑒔𑒻𑒫 𑒢𑒩𑒼𑒞𑓂𑒞𑒧𑒧𑓂 । 𑒠𑒹𑒫𑒲𑓀 𑒮𑒩𑒮𑓂𑒫𑒞𑒲𑓀 𑒔𑒻𑒫 𑒞𑒞𑒼 𑒖𑒨𑒧𑒳𑒠𑒲𑒩𑒨𑒹𑒞𑓂 ॥ 𑓑 ॥"),
    ]);

    // Non-reversible due to vocalic r/rr/l/ll.
    assert_one_way_pairwise(
        (Slp1, slp1_text),
        &[
            (
                Takri,
                "𑚝𑚭𑚤𑚭𑚣𑚘𑚫 𑚝𑚢𑚨𑚶𑚊𑚶𑚤𑚮𑚙𑚶𑚣 𑚝𑚤𑚫 𑚏𑚳𑚦 𑚝𑚤𑚴𑚙𑚶𑚙𑚢𑚢𑚶 । 𑚛𑚲𑚦𑚯𑚫 𑚨𑚤𑚨𑚶𑚦𑚙𑚯𑚫 𑚏𑚳𑚦 𑚙𑚙𑚴 𑚑𑚣𑚢𑚰𑚛𑚯𑚤𑚣𑚲𑚙𑚶 ॥ 𑛁 ॥",
            ),
            (
                ZanabazarSquare,
                "𑨝𑨊𑨫𑨊𑨪𑨘𑨸 𑨝𑨢𑨰𑩇𑨋𑩇𑨫𑨉𑨙𑩇𑨪 𑨝𑨫𑨸 𑨣𑨄𑨊𑨭 𑨝𑨫𑨆𑨙𑩇𑨙𑨢𑨢𑨴 𑩂 𑨛𑨄𑨭𑨁𑨊𑨸 𑨰𑨫𑨰𑩇𑨭𑨙𑨁𑨊𑨸 𑨣𑨄𑨊𑨭 𑨙𑨙𑨆 𑨥𑨪𑨢𑨂𑨛𑨁𑨊𑨫𑨪𑨄𑨙𑨴 𑩃 1 𑩃",
            ),
        ],
    );

    // Non-reversible due to y-nukta and no virama.
    assert_transliterate(
        slp1_text,
        Slp1,
        Assamese,
        "নাৰায়ণং নমস্কৃত্য নৰং চৈৱ নৰোত্তমম্ । দেৱীং সৰস্ৱতীং চৈৱ ততো জয়মুদীৰয়েৎ ॥ ১ ॥",
    );

    // Non-reversible due to no v/b distinction.
    assert_transliterate(
        slp1_text,
        Slp1,
        Bengali,
        "নারায়ণং নমস্কৃত্য নরং চৈব নরোত্তমম্ । দেবীং সরস্বতীং চৈব ততো জয়মুদীরয়েৎ ॥ ১ ॥",
    );

    // Non-reversible due to no Ta/ta distinctions.
    assert_transliterate(
        slp1_text,
        Slp1,
        Cham,
        "ꨘꨩꨣꨩꨢꨘꩌ ꨘꨠꩋꨆꨴꨮꨓꨳ ꨘꨣꩌ ꨌꨰꨥ ꨘꨣꨯꩅꨓꨠꩌ ꩝ ꨕꨯꨮꨥꨫꩌ ꨧꨣꨧꨶꨓꨫꩌ ꨌꨰꨥ ꨓꨓꨯ ꨎꨢꨠꨭꨕꨫꨣꨢꨯꨮꩅ ꩞ ꩑ ꩞",
    );

    // Non-reversible due to various na-s and sa-s.
    assert_transliterate(
        slp1_text,
        Slp1,
        GunjalaGondi,
        "𑵺𑶊𑶈𑶊𑵬𑵺𑶕 𑵺𑵰𑶉𑶗𑵱𑶗𑶈𑶍𑵳𑶗𑵬 𑵺𑶈𑶕 𑵻𑶑𑵭 𑵺𑶈𑶓𑵳𑶗𑵳𑵰𑵰 . 𑵸𑶐𑵭𑶌𑶕 𑶉𑶈𑶉𑶗𑵭𑵳𑶌𑶕 𑵻𑶑𑵭 𑵳𑵳𑶓 𑶀𑵬𑵰𑶍𑵸𑶌𑶈𑵬𑶐𑵳 .. 𑶡 ..",
    );

    // Non-reversible due to no vocalic r/rr/l/ll
    // TODO: how to use ADDAK?
    /*
    assert_transliterate(
        slp1_text,
        Slp1,
        Gurmukhi,
        "ਨਾਰਾਯਣੰ ਨਮਸ੍ਕ੍ਰੁਤ੍ਯ ਨਰੰ ਚੈਵ ਨਰੋੱਤਮਮ੍ । ਦੇਵੀਂ ਸਰਸ੍ਵਤੀਂ ਚੈਵ ਤਤੋ ਜਯਮੁਦੀਰਯੇਤ੍ ॥ 1 ॥",
    );
    */

    // Non-reversible due to ca/cha, vocalic r/rr/l/ll, etc.
    assert_transliterate(
        slp1_text,
        Slp1,
        MeeteiMayek,
        "ꯅꯥꯔꯥꯌꯅꯪ ꯅꯃꯁ꯭ꯛꯔꯨꯠꯌ ꯅꯔꯪ ꯆꯩꯋ ꯅꯔꯣꯠꯇꯃꯝ ꯫ ꯗꯦꯋꯤꯪ ꯁꯔꯁ꯭ꯋꯇꯤꯪ ꯆꯩꯋ ꯇꯇꯣ ꯖꯌꯃꯨꯗꯤꯔꯌꯦꯠ ꯫꯫ ꯱ ꯫꯫",
    );

    // Non-reversible due to no "sa" distinctions.
    assert_transliterate(
        slp1_text,
        Slp1,
        OlChiki,
        concat!(
            "ᱱᱟᱨᱟᱭᱚᱬᱚᱝ ᱱᱚᱢᱚᱥᱠᱨᱩᱛᱭᱚ ᱱᱚᱨᱚᱝ ᱪᱚᱤᱣᱚ ᱱᱚᱨᱳᱛᱛᱚᱢᱚᱢ ᱾ ",
            "ᱫᱮᱣᱤᱻᱝ ᱥᱚᱨᱚᱥᱣᱚᱛᱤᱻᱝ ᱪᱚᱤᱣᱚ ᱛᱚᱛᱳ ᱡᱚᱭᱚᱢᱩᱫᱤᱻᱨᱚᱭᱮᱛ ᱿ ᱑ ᱿"
        ),
    );

    // Non-reversible due to no virama.
    /*
    assert_transliterate(
        slp1_text,
        Slp1,
        Soyombo,
        "𑩯𑩛𑩼𑩛𑩻𑩪𑪖 𑩯𑩴𑪁𑪙𑩜𑩙𑩫𑪙𑩻 𑩯𑩼𑪖 𑩵𑩗𑩾 𑩯𑩼𑩖𑩫𑪘𑩴𑩴 𑪛 𑩭𑩔𑩾𑩑𑩛𑪖 𑪁𑩼𑪁𑪙𑩾𑩫𑩑𑩛𑪖 𑩵𑩗𑩾 𑩫𑩫𑩖 𑩷𑩻𑩴𑩒𑩭𑩑𑩛𑩼𑩻𑩔𑩫 𑪜 1 𑪜",
    );
    */

    // Non-reversible due to b/v and no virama.
    assert_transliterate(
        slp1_text,
        Slp1,
        Tibetan,
        "ནཱརཱཡཎཾ་ནམསྐྲྀཏྱ་ནརཾ་ཙཻབ་ནརོཏྟམམ་།་དེབཱིཾ་སརསྭཏཱིཾ་ཙཻབ་ཏཏོ་ཛཡམུདཱིརཡེཏ་༎་༡་༎",
    );
}

// Sanskrit (Vedic)
// ----------------

#[test]
fn vedic_svaras() {
    // Svarita and anudatta
    assert_two_way_pairwise(&[
        (BarahaSouth, "aq a#"),
        (Itrans, r"a\_ a\'"),
        (Slp1, r"a\ a^"),
        // Indic
        (Assamese, "অ॒ অ॑"),
        (Bengali, "অ॒ অ॑"),
        (Devanagari, "अ॒ अ॑"),
        (Grantha, "𑌅॒ 𑌅᳴"),
        (Gujarati, "અ॒ અ॑"),
        (Kannada, "ಅ॒ ಅ॑"),
        (Malayalam, "അ॒ അ॑"),
        (Odia, "ଅ॒ ଅ॑"),
        (Sharada, "𑆃॒ 𑆃॑"),
        (Telugu, "అ॒ అ॑"),
    ]);

    // Dirgha svarita
    assert_two_way_pairwise(&[
        (BarahaSouth, "a$"),
        // Indic
        (Assamese, "অ᳚"),
        (Bengali, "অ᳚"),
        (Devanagari, "अ᳚"),
        (Grantha, "𑌅॑"),
        (Kannada, "ಅ᳚"),
        (Malayalam, "അ᳚"),
        (Odia, "ଅ᳚"),
        (Sharada, "𑆃᳚"),
        (Telugu, "అ᳚"),
    ]);
}

#[test]
fn vedic_svaras_with_ayogavahas() {
    assert_two_way_pairwise(&[
        (BarahaSouth, "aqH a#H a$H aqM a#M a$M"),
        (Itrans, r#"a\_H a\'H a\"H a\_M a\'M a\"M"#),
        // Indic
        (Assamese, "অঃ॒ অঃ॑ অঃ᳚ অং॒ অং॑ অং᳚"),
        (Bengali, "অঃ॒ অঃ॑ অঃ᳚ অং॒ অং॑ অং᳚"),
        (Devanagari, "अः॒ अः॑ अः᳚ अं॒ अं॑ अं᳚"),
        (Grantha, "𑌅𑌃॒ 𑌅𑌃᳴ 𑌅𑌃॑ 𑌅𑌂॒ 𑌅𑌂᳴ 𑌅𑌂॑"),
        (Gujarati, "અઃ॒ અઃ॑ અઃ᳚ અં॒ અં॑ અં᳚"),
        (Kannada, "ಅಃ॒ ಅಃ॑ ಅಃ᳚ ಅಂ॒ ಅಂ॑ ಅಂ᳚"),
        (Malayalam, "അഃ॒ അഃ॑ അഃ᳚ അം॒ അം॑ അം᳚"),
        (Odia, "ଅଃ॒ ଅଃ॑ ଅଃ᳚ ଅଂ॒ ଅଂ॑ ଅଂ᳚"),
        (Telugu, "అః॒ అః॑ అః᳚ అం॒ అం॑ అం᳚"),
    ]);
}

#[test]
fn vedic_candrabindu_virama() {
    assert_two_way_pairwise(&[
        (Assamese, "অৼ"),
        (Bengali, "অৼ"),
        (Devanagari, "अꣳ"),
        (Malayalam, "അഄ"),
        (Grantha, "𑌅𑍞"),
        (Newa, "𑐀𑑟"),
    ]);
}

#[test]
fn vedic_upadhmaniya_and_jihvamuliya() {
    assert_two_way_pairwise(&[
        (Iso15919, "kaẖ kaḫ"),
        (Slp1, "kaZ kaV"),
        // Indic
        (Brahmi, "𑀓𑀃 𑀓𑀄"),
        (Devanagari, "कᳵ कᳶ"),
        (Kannada, "ಕೱ ಕೲ"),
        (Newa, "𑐎𑑠 𑐎𑑡"),
        (Sharada, "𑆑𑇂 𑆑𑇃"),
        (Soyombo, "𑩜𑪄 𑩜𑪅"),
        (Tibetan, "ཀྈ་ཀྉ"),
    ]);
}

#[test]
fn vedic_anusvaras() {
    assert_two_way_pairwise(&[(Devanagari, "कꣳ कꣴ"), (Grantha, "𑌕𑍞 𑌕𑍟")]);
}

#[test]
fn vedic_consonants() {
    assert_two_way_pairwise(&[
        (BarahaSouth, "La Lha"),
        (HarvardKyoto, "La Lha"),
        (Iast, "ḻa ḻha"),
        (Iso15919, "ḷa ḷha"),
        (Itrans, "La Lha"),
        (Slp1, "La |a"),
        (Velthuis, "La Lha"),
        (Wx, "lYa lYha"),
        // Indic
        (Balinese, "ᬮ᬴ ᬮ᬴᭄ᬳ"),
        (Bengali, "ল় ল়্হ"),
        (Brahmi, "𑀴 𑀴𑁆𑀳"),
        (Burmese, "ဠ ဠှ"),
        (Devanagari, "ळ ळ्ह"),
        (Grantha, "𑌳 𑌳𑍍𑌹"),
        (Gujarati, "ળ ળ્હ"),
        (GunjalaGondi, "𑵿 𑵿𑶗𑶇"),
        (Javanese, "ꦭ꦳ ꦭ꦳꧀ꦲ"),
        (Kannada, "ಳ ಳ್ಹ"),
        (Malayalam, "ള ള്ഹ"),
        (MasaramGondi, "𑴭 𑴭𑵅𑴬"),
        (Modi, "𑘯 𑘯𑘿𑘮"),
        (Odia, "ଳ ଳ୍ହ"),
        (Saurashtra, "ꢳ ꢳ꣄ꢲ"),
        (Sharada, "𑆭 𑆭𑇀𑆲"),
        (Siddham, "𑖩𑗀 𑖩𑗀𑖿𑖮"),
        (Sinhala, "ළ ළ්හ"),
        (TaiTham, "ᩊ ᩊ᩠ᩉ"),
        (Takri, "𑚥𑚷 𑚥𑚷𑚶𑚩"),
        (Telugu, "ళ ళ్హ"),
    ]);
}

// Other
// -----

#[test]
fn short_e_and_o_vowels() {
    assert_two_way_pairwise(&[
        (BarahaSouth, "e ke o ko"),
        (Iso15919, "e ke o ko"),
        // Indic
        (Devanagari, "ऎ कॆ ऒ कॊ"),
        (Kannada, "ಎ ಕೆ ಒ ಕೊ"),
        (Malayalam, "എ കെ ഒ കൊ"),
        (Tamil, "எ கெ ஒ கொ"),
        (Telugu, "ఎ కె ఒ కొ"),
    ]);
}

#[test]
fn nukta_consonants() {
    assert_two_way_pairwise(&[
        (Itrans, "qa Ka Ga za .Da .Dha fa Ya Ra"),
        (Iso15919, "qa k͟ha ġa za ṛa ṛha fa ẏa ṟa"),
        (Velthuis, "qa .kha .ga za Ra Rha fa .ya ^ra"),
        // Indic
        (Assamese, "ক় খ় গ় জ় ড় ঢ় ফ় য় ৰ়"),
        (Bengali, "ক় খ় গ় জ় ড় ঢ় ফ় য় র়"),
        (Devanagari, "क़ ख़ ग़ ज़ ड़ ढ़ फ़ य़ ऱ"),
        (Dogra, "𑠊𑠺 𑠋𑠺 𑠌𑠺 𑠑𑠺 𑠫 𑠗𑠺 𑠟𑠺 𑠣𑠺 𑠤𑠺"),
        // There is no documentated usage of Grantha nukta, but include it anyway for completion's
        // sake.
        (Grantha, "𑌕𑌼 𑌖𑌼 𑌗𑌼 𑌜𑌼 𑌡𑌼 𑌢𑌼 𑌫𑌼 𑌯𑌼 𑌰𑌼"),
        (Gujarati, "ક઼ ખ઼ ગ઼ જ઼ ડ઼ ઢ઼ ફ઼ ય઼ ર઼"),
        (Gurmukhi, "ਕ਼ ਖ਼ ਗ਼ ਜ਼ ੜ ਢ਼ ਫ਼ ਯ਼ ਰ਼"),
        (Kaithi, "𑂍𑂺 𑂎𑂺 𑂏𑂺 𑂔𑂺 𑂚 𑂜 𑂤𑂺 𑂨𑂺 𑂩𑂺"),
        // Aksharamukha has gha + nukta, but surely this is ga + nukta?
        (Khudawadi, "𑊺𑋩 𑊻𑋩 𑊼𑋩 𑋂𑋩 𑋊 𑋋𑋩 𑋓𑋩 𑋘𑋩 𑋙𑋩"),
        (Kannada, "ಕ಼ ಖ಼ ಗ಼ ಜ಼ ಡ಼ ಢ಼ ಫ಼ ಯ಼ ಱ"),
        (MasaramGondi, "𑴌𑵂 𑴍𑵂 𑴎𑵂 𑴓𑵂 𑴘𑵂 𑴙𑵂 𑴡𑵂 𑴥𑵂 𑴦𑵂"),
        (Newa, "𑐎𑑆 𑐏𑑆 𑐐𑑆 𑐖𑑆 𑐜𑑆 𑐝𑑆 𑐦𑑆 𑐫𑑆 𑐬𑑆"),
        (Odia, "କ଼ ଖ଼ ଗ଼ ଜ଼ ଡ଼ ଢ଼ ଫ଼ ୟ ର଼"),
        (Sharada, "𑆑𑇊 𑆒𑇊 𑆓𑇊 𑆘𑇊 𑆝𑇊 𑆞𑇊 𑆦𑇊 𑆪𑇊 𑆫𑇊"),
        (Siddham, "𑖎𑗀 𑖏𑗀 𑖐𑗀 𑖕𑗀 𑖚𑗀 𑖛𑗀 𑖣𑗀 𑖧𑗀 𑖨𑗀"),
        (Takri, "𑚊𑚷 𑚋𑚷 𑚌𑚷 𑚑𑚷 𑚪 𑚗𑚷 𑚟𑚷 𑚣𑚷 𑚤𑚷"),
        (Telugu, "క఼ ఖ఼ గ఼ జ఼ డ఼ ఢ఼ ఫ఼ య఼ ఱ"),
        (Tirhuta, "𑒏𑓃 𑒐𑓃 𑒑𑓃 𑒖𑓃 𑒛𑓃 𑒜𑓃 𑒤𑓃 𑒨𑓃 𑒩𑓃"),
    ]);
}

#[test]
fn symbol_abbreviation_sign() {
    assert_two_way_pairwise(&[
        (Assamese, "\u{09fd}"),
        (Bengali, "\u{09fd}"),
        (Devanagari, "\u{0970}"),
        (Dogra, "\u{1183b}"),
        (Gujarati, "\u{0af0}"),
        (Gurmukhi, "\u{0a76}"),
        (Kaithi, "\u{110bb}"),
        (Modi, "\u{11643}"),
        (Newa, "\u{1144f}"),
        (Sharada, "\u{111c7}"),
        (Takri, "\u{116b9}"),
        (Tirhuta, "\u{114c6}"),
    ]);
}

#[test]
fn symbol_om() {
    assert_two_way_pairwise(&[
        (Devanagari, "ॐ"),
        (Grantha, "𑍐"),
        (Gujarati, "ૐ"),
        (GunjalaGondi, "𑶘"),
        (Newa, "𑑉"),
        (Sharada, "\u{1118f}\u{11180}"),
        (Tamil, "ௐ"),
        (Tibetan, "ༀ"),
        (Tirhuta, "𑓇"),
    ]);
}

// Scheme-specific tests
// =====================

#[test]
fn baraha_defaults_and_alternates() {
    let round_trip = |x| {
        let deva = t(x, BarahaSouth, Devanagari);
        t(&deva, Devanagari, BarahaSouth)
    };

    assert_eq!(
        "A I U au kha gha cha Cha jha pha bha va sha ha",
        round_trip("aa ee oo ou Ka Ga ca Ca Ja Pa Ba wa Sa ~ha")
    );
}

#[test]
fn baraha_accents() {
    let assert_has =
        |expected, actual| assert_transliterate(expected, BarahaSouth, Devanagari, actual);
    assert_has("a# aq", "अ॑ अ॒");
    assert_has("aQ aV a$", "अ\u{1cd2} अ\u{1cdd} अ\u{1cda}");
    // TODO: aW
}

#[test]
fn burmese_subjoined_consonants() {
    assert_two_way_pairwise(&[
        (
            Slp1,
            concat!(
                "nka nKa nga nGa nNa nca nCa nja nJa nYa nwa nWa nqa nQa nRa ",
                "nta nTa nda nDa nna npa nPa nba nBa nma nya nra nla nva nSa nza nsa nha"
            ),
        ),
        (
            Burmese,
            "န္က န္ခ န္ဂ န္ဃ န္င န္စ န္ဆ န္ဇ န္ဈ န္ဉ န္ဋ န္ဌ န္ဍ န္ဎ န္ဏ န္တ န္ထ န္ဒ န္ဓ န္န န္ပ န္ဖ န္ဗ န္ဘ န္မ နျ နြ န္လ နွ န္ၐ န္ၑ န္သ နှ",
        ),
    ]);
}

#[test]
fn devanagari_nuktas_joined() {
    let separate = "क़ ख़ ग़ ज़ ड़ ढ़ फ़ य़";
    let joined = "\u{0958} \u{0959} \u{095a} \u{095b} \u{095c} \u{095d} \u{095e} \u{095f}";
    assert_ne!(separate, joined);

    // Encouraged per Unicode spec. (nuktas separate)
    assert_two_way_pairwise(&[
        (Devanagari, separate),
        (Itrans, "qa Ka Ga za .Da .Dha fa Ya"),
    ]);

    // Discouraged per Unicode spec. (nuktas joined)
    assert_transliterate(joined, Devanagari, Itrans, "qa Ka Ga za .Da .Dha fa Ya");
}

#[test]
fn devanagari_prishthamatra() {
    let assert_has =
        |expected, actual| assert_transliterate(expected, Devanagari, HarvardKyoto, actual);

    // U+094E is "DEVANAGARI VOWEL SIGN PRISHTHAMATRA E". For usage, see the spec comments on
    // U+094E.
    assert_has("क\u{0947}\u{094e}", "kai");
    assert_has("क\u{093e}\u{094e}", "ko");
    assert_has("क\u{094b}\u{094e}", "kau");
}

#[test]
fn grantha_two_part_vowels() {
    let assert_has = |expected, actual| assert_transliterate(expected, Grantha, Devanagari, actual);

    let ko = "को";
    // Default
    assert_has("𑌕\u{1134b}", ko);
    // Permitted by Unicode spec -- see spec comments on U+1134B
    assert_has("𑌕\u{11347}\u{1133e}", ko);

    let kau = "कौ";
    // Default
    assert_has("𑌕\u{1134c}", kau);
    // Permitted by Unicode spec -- see spec comments on U+1134C
    assert_has("𑌕\u{11347}\u{11357}", kau);
    // Also support just "GRANTHA AU LENGTH MARK" which is permissible as a shorthand
    assert_has("𑌕\u{11357}", kau);
}

#[test]
fn iast_unicode_variants() {
    assert_supports_unicode_variants(
        Scheme::Iast,
        &[
            "ā", "ī", "ū", "ṛ", "ṝ", "ḷ", "ḹ", "ṃ", "ḥ", "ṅ", "ñ", "ṭ", "ḍ", "ṇ", "ś", "ṣ",
        ],
    );
}

#[test]
fn iso_15919_bug_no_greedy_match_on_nfd() {
    assert_transliterate("ai\u{0304} au\u{0304}", Iso15919, Devanagari, "अई अऊ");
    assert_transliterate(
        "agh\u{0323} agh\u{0331} agh\u{032e}",
        Iso15919,
        Devanagari,
        "अग्ः अग्ᳵ अग्ᳶ",
    );
}

#[test]
fn iso_15919_colon_separator() {
    // Consonants
    assert_two_way_pairwise(&[
        (
            Iso15919,
            "k:ha g:ha c:ha j:ha ṭ:ha ḍ:ha t:ha d:ha p:ha b:ha",
        ),
        (Slp1, "kha gha cha jha wha qha tha dha pha bha"),
        (Devanagari, "क्ह ग्ह च्ह ज्ह ट्ह ड्ह त्ह द्ह प्ह ब्ह"),
        (Kannada, "ಕ್ಹ ಗ್ಹ ಚ್ಹ ಜ್ಹ ಟ್ಹ ಡ್ಹ ತ್ಹ ದ್ಹ ಪ್ಹ ಬ್ಹ"),
    ]);

    // Consonants with marks
    assert_two_way_pairwise(&[
        (
            Iso15919,
            "k:hā g:hā c:hā j:hā ṭ:hā ḍ:hā t:hā d:hā p:hā b:hā",
        ),
        (Slp1, "khA ghA chA jhA whA qhA thA dhA phA bhA"),
        (Devanagari, "क्हा ग्हा च्हा ज्हा ट्हा ड्हा त्हा द्हा प्हा ब्हा"),
        (Kannada, "ಕ್ಹಾ ಗ್ಹಾ ಚ್ಹಾ ಜ್ಹಾ ಟ್ಹಾ ಡ್ಹಾ ತ್ಹಾ ದ್ಹಾ ಪ್ಹಾ ಬ್ಹಾ"),
    ]);

    // Consonants with viramas
    assert_two_way_pairwise(&[
        (Iso15919, "k:h g:h c:h j:h ṭ:h ḍ:h t:h d:h p:h b:h"),
        (Slp1, "kh gh ch jh wh qh th dh ph bh"),
        (Devanagari, "क्ह् ग्ह् च्ह् ज्ह् ट्ह् ड्ह् त्ह् द्ह् प्ह् ब्ह्"),
        (Kannada, "ಕ್ಹ್ ಗ್ಹ್ ಚ್ಹ್ ಜ್ಹ್ ಟ್ಹ್ ಡ್ಹ್ ತ್ಹ್ ದ್ಹ್ ಪ್ಹ್ ಬ್ಹ್"),
    ]);

    // Vowels
    assert_two_way_pairwise(&[
        (Iso15919, "a:i a:u ka:i ka:u"),
        (Slp1, "ai au kai kau"),
        (Devanagari, "अइ अउ कइ कउ"),
        (Kannada, "ಅಇ ಅಉ ಕಇ ಕಉ"),
    ]);

    // Regular colons -- ignore
    // TODO: what's the best policy for handling these?
    assert_two_way_pairwise(&[
        (Iso15919, "a: ka: k: a:ā k:ta"),
        (Slp1, "a: ka: k: a:A k:ta"),
        (Devanagari, "अ: क: क्: अ:आ क्:त"),
        (Kannada, "ಅ: ಕ: ಕ್: ಅ:ಆ ಕ್:ತ"),
    ]);
}

#[test]
fn iso_15919_tamil_aytam() {
    assert_transliterate("ஃ", Tamil, Iso15919, "ḳ");
    assert_transliterate("\u{1e33}", Iso15919, Tamil, "ஃ");
    assert_transliterate("k\u{0323}", Iso15919, Tamil, "ஃ");
}

#[test]
fn iso_15919_unicode_variants() {
    assert_supports_unicode_variants(
        Scheme::Iso15919,
        &[
            "ā", "ī", "ū", "ṁ", "ḥ", "ẖ", "ḫ", "ṅ", "ñ", "ṭ", "ḍ", "ṇ", "ś", "ṣ",
        ],
    );
}

#[test]
fn itrans_alternates() {
    fn assert_same(items: &[&str]) {
        let mut lipika = Lipika::new();
        let default = items[0];
        let deva = lipika.transliterate(default, Itrans, Devanagari);

        for x in &items[1..] {
            let x_deva = lipika.transliterate(x, Itrans, Devanagari);
            assert_eq!(deva, x_deva, "{default} ({deva}) != {x} ({x_deva})");
        }
    }

    assert_same(&["A kA", "aa kaa"]);
    assert_same(&["I kI", "ii kii", "ee kee"]);
    assert_same(&["U kU", "uu kuu", "oo koo"]);
    assert_same(&["RRi kRRi", "R^i kR^i"]);
    assert_same(&["RRI kRRI", "R^I kR^I"]);
    assert_same(&["LLi kLLi", "L^i kL^i"]);
    assert_same(&["LLI kLLI", "L^I kL^I"]);

    // Anusvara and candrabindu
    assert_same(&["aM", "a.m", "a.n"]);
    assert_same(&["a.N", "a{\\m+}"]);

    // Consonants
    assert_same(&["~Na", "N^a"]);
    assert_same(&["ca", "cha"]);
    assert_same(&["~na", "JNa"]);
    // TODO: source for Ca?
    assert_same(&["Cha", "chha", "Ca"]);
    assert_same(&["va", "wa"]);
    assert_same(&["Sha", "Sa", "shha"]);
    assert_same(&["za", "Ja"]);
    // TODO: L / ld?

    // Clusters
    assert_same(&["kSha", "kSa", "kshha", "xa"]);
    assert_same(&["j~na", "GYa", "dnya"]);
    assert_same(&["OM", "AUM"]);

    // Punctuation
    assert_same(&[".a", "~"]);
    assert_same(&["|", "."]);
    assert_same(&["||", ".."]);
}

#[test]
fn itrans_zero_width_joiner() {
    assert_transliterate("bara_u", Itrans, Devanagari, "बरउ");
    assert_transliterate("k{}Shetra", Itrans, Devanagari, "क्\u{200d}षेत्र");
}

#[test]
fn itrans_backslash_escape() {
    assert_transliterate("\\nara", Itrans, Devanagari, "nअर");
    assert_transliterate("na\\ra", Itrans, Devanagari, "नrअ");
    assert_transliterate("nara\\", Itrans, Devanagari, "नर");
}

#[test]
fn javanese_medial_consonants() {
    assert_two_way_pairwise(&[(Slp1, "nya nra"), (Javanese, "ꦤꦾ ꦤꦿ")]);
}

#[test]
fn kannada_unicode_variants() {
    assert_supports_unicode_variants(Scheme::Kannada, &["ಕೀ", "ಕೇ", "ಕೈ", "ಕೊ", "ಕೋ"]);
}

#[test]
fn khmer_sign_robat() {
    assert_two_way_pairwise(&[(Slp1, "kra kara rka arka kara"), (Khmer, "ក្រ ករ ក៌ អក៌ ករ")]);
}

#[test]
fn malayalam_chillus() {
    // Our chillu support is currently limited to when we transliterate *from* Malayalam.
    let pairs = &[
        ("അൺക", "aRka", "अण्क"),
        ("അൻക", "anka", "अन्क"),
        ("അർക", "arka", "अर्क"),
        ("അൽക", "alka", "अल्क"),
    ];
    for (input, slp1, devanagari) in pairs {
        assert_transliterate(input, Malayalam, Slp1, slp1);
        assert_transliterate(input, Malayalam, Devanagari, devanagari);
    }
}

#[test]
fn malayalam_modern_and_archaic_au() {
    // Modern sign au
    assert_two_way_pairwise(&[(Slp1, "kO"), (Malayalam, "കൗ")]);

    // Archaic sign au
    assert_transliterate("കൌ", Malayalam, Slp1, "kO");
}

#[test]
fn masaram_gondi_conjuncts() {
    assert_two_way_pairwise(&[(Slp1, "kza kzA"), (MasaramGondi, "𑴮 𑴮𑴱")]);
    assert_two_way_pairwise(&[(Slp1, "jYa jYA"), (MasaramGondi, "𑴯 𑴯𑴱")]);
    assert_two_way_pairwise(&[(Slp1, "tra trA"), (MasaramGondi, "𑴰 𑴰𑴱")]);
}

#[test]
fn saurashtra_ksha_with_zwj() {
    assert_two_way_pairwise(&[(Slp1, "kza"), (Saurashtra, "ꢒ꣄\u{200d}ꢰ")]);
}

#[test]
fn sharada_om() {
    // Encouraged per Unicode spec. (SHARADA LETTER O + SHARADA SIGN CANDRABINDU)
    assert_two_way_pairwise(&[(Devanagari, "ॐ"), (Sharada, "\u{1118f}\u{11180}")]);

    // Discouraged per Unicode spec. (SHARADA OM)
    assert_transliterate("\u{111c4}", Sharada, Devanagari, "ॐ");
}

#[test]
fn tamil_superscript() {
    assert_two_way_pairwise(&[
        (
            Slp1,
            "ga gA gi gI gu gU gf gF gx gX ge gE go gO gaM gaH gIM gIH devI",
        ),
        (
            Tamil,
            "க³ கா³ கி³ கீ³ கு³ கூ³ க்³ருʼ க்³ரூʼ க்³லுʼ க்³லூʼ கே³ கை³ கோ³ கௌ³ க³ம்ʼ க³꞉ கீ³ம்ʼ கீ³꞉ தே³வீ",
        ),
    ]);
}

#[test]
fn tamil_unicode_variants() {
    assert_supports_unicode_variants(Scheme::Tamil, &["ஔ", "கொ", "கொ", "கௌ"]);
}

#[test]
fn thai_preceding_vowels() {
    assert_two_way_pairwise(&[(Slp1, "ke kE ko kO"), (Thai, "เก ไก โก เกา")]);
}

#[test]
fn telugu_unicode_variants() {
    assert_supports_unicode_variants(Scheme::Telugu, &["కై"]);
}

#[test]
fn tibetan_ba_va() {
    // TODO: why is this change made? For now, enforce for consistency with Aksharamukha default
    // behavior.
    assert_transliterate("bava", Slp1, Tibetan, "བབ");
    assert_transliterate("nbanva", Slp1, Tibetan, "ནྦནྭ");
}

#[test]
fn tibetan_subjoined_consonants() {
    assert_two_way_pairwise(&[(Slp1, "nGa"), (Tibetan, "ནྒྷ")]);
    assert_two_way_pairwise(&[(Slp1, "kf kF kx kX"), (Tibetan, "ཀྲྀ་ཀྲཱྀ་ཀླྀ་ཀླཱྀ")]);

    assert_two_way_pairwise(&[
        (
            // Use "nba" instead of "nva" because Tibetan does not have a "v" character.
            Slp1,
            concat!(
                "nka nKa nga nGa nNa nca nCa nja nJa nYa nwa nWa nqa nQa nRa ",
                "nta nTa nda nDa nna npa nPa nba nBa nma nya nra nla nba nSa nza nsa nha"
            ),
        ),
        (
            Tibetan,
            "ནྐ་ནྑ་ནྒ་ནྒྷ་ནྔ་ནྩ་ནྪ་ནྫ་ནྫྷ་ནྙ་ནྚ་ནྛ་ནྜ་ནྜྷ་ནྞ་ནྟ་ནྠ་ནྡ་ནྡྷ་ནྣ་ནྤ་ནྥ་ནྦ་ནྦྷ་ནྨ་ནྱ་ནྲ་ནླ་ནྦ་ནྴ་ནྵ་ནྶ་ནྷ",
        ),
    ]);
}

#[test]
fn velthuis_basic() {
    let assert_has =
        |expected, actual| assert_transliterate(expected, Velthuis, Devanagari, actual);
    // Tests are from "Devanagari for TeX", Anshuman Pandey, 2019: Table 1"

    // Signs
    // TODO: ".." --> "."
    assert_has(
        ".m / .h .a & ~a ~o .o | || ^ra @ #",
        "\u{902} \u{901} \u{903} \u{93d} \u{94d} \u{945} \u{949} ॐ । ॥ ऱ ॰ ॱ",
    );
    // Extended consonants
    assert_has("qa .kha .ga za Ra Rha fa", "क़ ख़ ग़ ज़ ड़ ढ़ फ़");
}

// Other bugs
// ----------

/// Tests that skipping unmappable content doesn't land within a char boundry.
#[test]
fn test_mixed_content() {
    assert_transliterate("saMskftam 漢語", HarvardKyoto, Devanagari, "संस्क्फ़्तम् 漢語");
}
