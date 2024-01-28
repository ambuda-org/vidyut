use vidyut_lipi::Scheme::*;
use vidyut_lipi::{Lipika, Scheme};

/// A quick alias for transliterating.
fn t(input: &str, from: Scheme, to: Scheme) -> String {
    let mut lipika = Lipika::new();
    lipika.transliterate(input, from, to)
}

fn assert_transliterate(input: &str, from: Scheme, to: Scheme, expected: &str) {
    let actual = t(input, from, to);
    let e_codes: Vec<_> = expected.chars().map(|c| c as u32).collect();
    let a_codes: Vec<_> = actual.chars().map(|c| c as u32).collect();
    assert_eq!(
        *expected, actual,
        "input: {input} ({from:?} --> {to:?})

expected: {}
          {e_codes:x?}

actual:   {}
          {a_codes:x?}
",
        expected, actual
    );
}

/// Transliterates all input strings against each other.
///
/// Use this function if round-trip transliteration is lossless.
fn assert_two_way_pairwise(examples: &[(Scheme, &str)]) {
    for (from, input) in examples {
        for (to, expected) in examples {
            // Also test the case where "from == to." In this case, we should return the original
            // input string as-is.
            let actual = t(input, *from, *to);
            let e_codes: Vec<_> = expected.chars().map(|c| c as u32).collect();
            let a_codes: Vec<_> = actual.chars().map(|c| c as u32).collect();
            assert_eq!(
                *expected, actual,
                "input: {input} ({from:?} --> {to:?})

expected: {}
          {e_codes:x?}

actual:   {}
          {a_codes:x?}
",
                expected, actual
            );
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
        // input string as-is.
        let actual = t(input, from, *to);
        assert_eq!(*expected, actual, "t(\"{input}\", {from:?}, {to:?})");
    }

    assert_two_way_pairwise(examples);
}

// Sanskrit (Basic)
// ----------------

#[test]
fn sanskrit_independent_vowels() {
    assert_two_way_pairwise(&[
        (BarahaSouth, "a A i I u U Ru RU ~lu ~lU E ai O au"),
        (HarvardKyoto, "a A i I u U R RR lR lRR e ai o au"),
        (Iast, "a ā i ī u ū ṛ ṝ ḷ ḹ e ai o au"),
        (Iso15919, "a ā i ī u ū r̥ r̥̄ l̥ l̥̄ ē ai ō au"),
        (Itrans, "a A i I u U RRi RRI LLi LLI e ai o au"),
        (Slp1, "a A i I u U f F x X e E o O"),
        (Velthuis, "a aa i ii u uu .r .R .l .L e ai o au"),
        (Wx, "a A i I u U q Q L LV e E o O"),
        // Indic
        (Balinese, "ᬅ ᬆ ᬇ ᬈ ᬉ ᬊ ᬋ ᬌ ᬍ ᬎ ᬏ ᬐ ᬑ ᬒ"),
        (Bengali, "অ আ ই ঈ উ ঊ ঋ ৠ ঌ ৡ এ ঐ ও ঔ"),
        (Brahmi, "𑀅 𑀆 𑀇 𑀈 𑀉 𑀊 𑀋 𑀌 𑀍 𑀎 𑀏 𑀐 𑀑 𑀒"),
        (Burmese, "အ အာ ဣ ဤ ဥ ဦ ၒ ၓ ၔ ၕ ဧ အဲ ဩ ဪ"),
        // (Cham, "ꨀ ꨀꨩ ꨁ ꨁꨩ ꨂ ꨂꨩ ꨣꨮ ꨣꨮꨩ ꨤꨮ ꨤꨮꨩ ꨃ ꨄ ꨅ ꨀꨯꨱ"),
        (Devanagari, "अ आ इ ई उ ऊ ऋ ॠ ऌ ॡ ए ऐ ओ औ"),
        (Grantha, "𑌅 𑌆 𑌇 𑌈 𑌉 𑌊 𑌋 𑍠 𑌌 𑍡 𑌏 𑌐 𑌓 𑌔"),
        (Gujarati, "અ આ ઇ ઈ ઉ ઊ ઋ ૠ ઌ ૡ એ ઐ ઓ ઔ"),
        (Javanese, "ꦄ ꦄꦴ ꦆ ꦇ ꦈ ꦈꦴ ꦉ ꦉꦴ ꦊ ꦋ ꦌ ꦍ ꦎ ꦎꦴ"),
        (Kannada, "ಅ ಆ ಇ ಈ ಉ ಊ ಋ ೠ ಌ ೡ ಏ ಐ ಓ ಔ"),
        (Khmer, "អ អា ឥ ឦ ឧ ឩ ឫ ឬ ឭ ឮ ឯ ឰ ឱ ឳ"),
        (Malayalam, "അ ആ ഇ ഈ ഉ ഊ ഋ ൠ ഌ ൡ ഏ ഐ ഓ ഔ"),
        (Modi, "𑘀 𑘁 𑘂 𑘃 𑘄 𑘅 𑘆 𑘇 𑘈 𑘉 𑘊 𑘋 𑘌 𑘍"),
        (Newa, "𑐀 𑐁 𑐂 𑐃 𑐄 𑐅 𑐆 𑐇 𑐈 𑐉 𑐊 𑐋 𑐌 𑐍"),
        (Odia, "ଅ ଆ ଇ ଈ ଉ ଊ ଋ ୠ ଌ ୡ ଏ ଐ ଓ ଔ"),
        (Saurashtra, "ꢂ ꢃ ꢄ ꢅ ꢆ ꢇ ꢈ ꢉ ꢊ ꢋ ꢍ ꢎ ꢐ ꢑ"),
        (Sharada, "𑆃 𑆄 𑆅 𑆆 𑆇 𑆈 𑆉 𑆊 𑆋 𑆌 𑆍 𑆎 𑆏 𑆐"),
        (Sinhala, "අ ආ ඉ ඊ උ ඌ ඍ ඎ ඏ ඐ ඒ ඓ ඕ ඖ"),
        (Tamil, "அ ஆ இ ஈ உ ஊ ருʼ ரூʼ லுʼ லூʼ ஏ ஐ ஓ ஔ"),
        (Telugu, "అ ఆ ఇ ఈ ఉ ఊ ఋ ౠ ఌ ౡ ఏ ఐ ఓ ఔ"),
        (Thai, "อ อา อิ อี อุ อู ฤ ฤๅ ฦ ฦๅ เอ ไอ โอ เอา"),
        (Tirhuta, "𑒁 𑒂 𑒃 𑒄 𑒅 𑒆 𑒇 𑒈 𑒉 𑒊 𑒋 𑒌 𑒍 𑒎"),
        (Tibetan, "ཨ་ཨཱ་ཨི་ཨཱི་ཨུ་ཨཱུ་རྀ་རཱྀ་ལྀ་ལཱྀ་ཨེ་ཨཻ་ཨོ་ཨཽ"),
    ]);

    // Scripts with no vocalic L
    assert_two_way_pairwise(&[
        (Slp1, "a A i I u U f F e E o O"),
        (Siddham, "𑖀 𑖁 𑖂 𑖃 𑖄 𑖅 𑖆 𑖇 𑖊 𑖋 𑖌 𑖍"),
    ]);

    // Scripts with no vocalic R or L
    assert_two_way_pairwise(&[
        (Slp1, "a A i I u U e E o O"),
        (Gurmukhi, "ਅ ਆ ਇ ਈ ਉ ਊ ਏ ਐ ਓ ਔ"),
    ]);
}

#[test]
fn sanskrit_dependent_vowels() {
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
        (Slp1, "ka kA ki kI ku kU kf kF kx kX ke kE ko kO"),
        (
            Velthuis,
            "ka kaa ki kii ku kuu k.r k.R k.l k.L ke kai ko kau",
        ),
        (Wx, "ka kA ki kI ku kU kq kQ kL kLV ke kE ko kO"),
        // Indic
        (Balinese, "ᬓ ᬓᬵ ᬓᬶ ᬓᬷ ᬓᬸ ᬓᬹ ᬓᬺ ᬓᬻ ᬓᬼ ᬓᬽ ᬓᬾ ᬓᬿ ᬓᭀ ᬓᭁ"),
        (Bengali, "ক কা কি কী কু কূ কৃ কৄ কৢ কৣ কে কৈ কো কৌ"),
        (Brahmi, "𑀓 𑀓𑀸 𑀓𑀺 𑀓𑀻 𑀓𑀼 𑀓𑀽 𑀓𑀾 𑀓𑀿 𑀓𑁀 𑀓𑁁 𑀓𑁂 𑀓𑁃 𑀓𑁄 𑀓𑁅"),
        (Burmese, "က ကာ ကိ ကီ ကု ကူ ကၖ ကၗ ကၘ ကၙ ကေ ကဲ ကော ကော်"),
        // (Cham, "ꨆ ꨆꨩ ꨆꨪ ꨆꨫ ꨆꨭ ꨆꨭꨩ ꨆꨴꨮ ꨆꨴꨮꨩ ꨆꨵꨮ ꨆꨵꨮꨩ ꨆꨯꨮ ꨆꨰ ꨆꨯ ꨆꨯꨱ"),
        (Devanagari, "क का कि की कु कू कृ कॄ कॢ कॣ के कै को कौ"),
        (Grantha, "𑌕 𑌕𑌾 𑌕𑌿 𑌕𑍀 𑌕𑍁 𑌕𑍂 𑌕𑍃 𑌕𑍄 𑌕𑍢 𑌕𑍣 𑌕𑍇 𑌕𑍈 𑌕𑍋 𑌕𑍌"),
        (Gujarati, "ક કા કિ કી કુ કૂ કૃ કૄ કૢ કૣ કે કૈ કો કૌ"),
        (Javanese, "ꦏ ꦏꦴ ꦏꦶ ꦏꦷ ꦏꦸ ꦏꦹ ꦏꦽ ꦏ꧀ꦉꦴ ꦏ꧀ꦊ ꦏ꧀ꦋ ꦏꦺ ꦏꦻ ꦏꦺꦴ ꦏꦻꦴ"),
        (Kannada, "ಕ ಕಾ ಕಿ ಕೀ ಕು ಕೂ ಕೃ ಕೄ ಕೢ ಕೣ ಕೇ ಕೈ ಕೋ ಕೌ"),
        (Khmer, "ក កា កិ កី កុ កូ ក្ឫ ក្ឬ ក្ឭ ក្ឮ កេ កៃ កោ កៅ"),
        (Malayalam, "ക കാ കി കീ കു കൂ കൃ കൄ കൢ കൣ കേ കൈ കോ കൌ"),
        (Modi, "𑘎 𑘎𑘰 𑘎𑘱 𑘎𑘲 𑘎𑘳 𑘎𑘴 𑘎𑘵 𑘎𑘶 𑘎𑘷 𑘎𑘸 𑘎𑘹 𑘎𑘺 𑘎𑘻 𑘎𑘼"),
        (Newa, "𑐎 𑐎𑐵 𑐎𑐶 𑐎𑐷 𑐎𑐸 𑐎𑐹 𑐎𑐺 𑐎𑐻 𑐎𑐼 𑐎𑐽 𑐎𑐾 𑐎𑐿 𑐎𑑀 𑐎𑑁"),
        (Odia, "କ କା କି କୀ କୁ କୂ କୃ କୄ କୢ କୣ କେ କୈ କୋ କୌ"),
        (Saurashtra, "ꢒ ꢒꢵ ꢒꢶ ꢒꢷ ꢒꢸ ꢒꢹ ꢒꢺ ꢒꢻ ꢒꢼ ꢒꢽ ꢒꢿ ꢒꣀ ꢒꣂ ꢒꣃ"),
        (Sharada, "𑆑 𑆑𑆳 𑆑𑆴 𑆑𑆵 𑆑𑆶 𑆑𑆷 𑆑𑆸 𑆑𑆹 𑆑𑆺 𑆑𑆻 𑆑𑆼 𑆑𑆽 𑆑𑆾 𑆑𑆿"),
        (Sinhala, "ක කා කි කී කු කූ කෘ කෲ කෟ කෳ කේ කෛ කෝ කෞ"),
        (Tamil, "க கா கி கீ கு கூ க்ருʼ க்ரூʼ க்லுʼ க்லூʼ கே கை கோ கௌ"),
        (Telugu, "క కా కి కీ కు కూ కృ కౄ కౢ కౣ కే కై కో కౌ"),
        (Thai, "ก กา กิ กี กุ กู กฺฤ กฺฤๅ กฺฦ กฺฦๅ เก ไก โก เกา"),
        (Tibetan, "ཀ་ཀཱ་ཀི་ཀཱི་ཀུ་ཀཱུ་ཀྲྀ་ཀྲཱྀ་ཀླྀ་ཀླཱྀ་ཀེ་ཀཻ་ཀོ་ཀཽ"),
        (Tirhuta, "𑒏 𑒏𑒰 𑒏𑒱 𑒏𑒲 𑒏𑒳 𑒏𑒴 𑒏𑒵 𑒏𑒶 𑒏𑒷 𑒏𑒸 𑒏𑒹 𑒏𑒻 𑒏𑒼 𑒏𑒾"),
    ]);

    // Scripts without vocalic L
    assert_two_way_pairwise(&[
        (Slp1, "ka kA ki kI ku kU kf kF ke kE ko kO"),
        (Siddham, "𑖎 𑖎𑖯 𑖎𑖰 𑖎𑖱 𑖎𑖲 𑖎𑖳 𑖎𑖴 𑖎𑖵 𑖎𑖸 𑖎𑖹 𑖎𑖺 𑖎𑖻"),
    ]);

    // Scripts without vocalic R or L
    assert_two_way_pairwise(&[
        (Slp1, "ka kA ki kI ku kU ke kE ko kO"),
        (Gurmukhi, "ਕ ਕਾ ਕਿ ਕੀ ਕੁ ਕੂ ਕੇ ਕੈ ਕੋ ਕੌ"),
    ]);
}

#[test]
fn sanskrit_ayogavahas_etc() {
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
        (Balinese, "ᬅᬂ ᬅᬄ ᬅᬁ"),
        (Bengali, "অং অঃ অঁ"),
        (Brahmi, "𑀅𑀁 𑀅𑀂 𑀅𑀀"),
        // (Cham, "ꨀꩌ ꨀꩍ ꨀꩃ"),
        (Devanagari, "अं अः अँ"),
        (Grantha, "𑌅𑌂 𑌅𑌃 𑌅𑌁"),
        (Gujarati, "અં અઃ અઁ"),
        (Javanese, "ꦄꦁ ꦄꦃ ꦄꦀ"),
        (Kannada, "ಅಂ ಅಃ ಅಁ"),
        (Malayalam, "അം അഃ അഁ"),
        (Newa, "𑐀𑑄 𑐀𑑅 𑐀𑑃"),
        (Odia, "ଅଂ ଅଃ ଅଁ"),
        (Saurashtra, "ꢂꢀ ꢂꢁ ꢂꣅ"),
        (Sharada, "𑆃𑆁 𑆃𑆂 𑆃𑆀"),
        (Siddham, "𑖀𑖽 𑖀𑖾 𑖀𑖼"),
        (Tamil, "அம்ʼ அ꞉ அம்ˮ"),
        (Telugu, "అం అః అఁ"),
        (Tibetan, "ཨཾ་ཨཿ་ཨྃ"),
        (Tirhuta, "𑒁𑓀 𑒁𑓁 𑒁𑒿"),
    ]);

    // Scripts without a chandrabindu
    assert_one_way_pairwise(
        (Slp1, "aM aH a~"),
        &[
            (Burmese, "အံ အး အံ"),
            (Khmer, "អំ អះ អំ"),
            (Modi, "𑘀𑘽 𑘀𑘾 𑘀𑘽"),
            (Sinhala, "අං අඃ අං"),
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
        (Balinese, "ᬓ ᬔ ᬕ ᬖ ᬗ ᬘ ᬙ ᬚ ᬛ ᬜ ᬝ ᬞ ᬟ ᬠ ᬡ ᬢ ᬣ ᬤ ᬥ ᬦ ᬧ ᬨ ᬩ ᬪ ᬫ ᬬ ᬭ ᬮ ᬯ ᬰ ᬱ ᬲ ᬳ"),
        (Brahmi, "𑀓 𑀔 𑀕 𑀖 𑀗 𑀘 𑀙 𑀚 𑀛 𑀜 𑀝 𑀞 𑀟 𑀠 𑀡 𑀢 𑀣 𑀤 𑀥 𑀦 𑀧 𑀨 𑀩 𑀪 𑀫 𑀬 𑀭 𑀮 𑀯 𑀰 𑀱 𑀲 𑀳"),
        (Burmese, "က ခ ဂ ဃ င စ ဆ ဇ ဈ ဉ ဋ ဌ ဍ ဎ ဏ တ ထ ဒ ဓ န ပ ဖ ဗ ဘ မ ယ ရ လ ဝ ၐ ၑ သ ဟ"),
        (Devanagari, "क ख ग घ ङ च छ ज झ ञ ट ठ ड ढ ण त थ द ध न प फ ब भ म य र ल व श ष स ह"),
        (Grantha, "𑌕 𑌖 𑌗 𑌘 𑌙 𑌚 𑌛 𑌜 𑌝 𑌞 𑌟 𑌠 𑌡 𑌢 𑌣 𑌤 𑌥 𑌦 𑌧 𑌨 𑌪 𑌫 𑌬 𑌭 𑌮 𑌯 𑌰 𑌲 𑌵 𑌶 𑌷 𑌸 𑌹"),
        (Gujarati, "ક ખ ગ ઘ ઙ ચ છ જ ઝ ઞ ટ ઠ ડ ઢ ણ ત થ દ ધ ન પ ફ બ ભ મ ય ર લ વ શ ષ સ હ"),
        (Javanese, "ꦏ ꦑ ꦒ ꦓ ꦔ ꦕ ꦖ ꦗ ꦙ ꦚ ꦛ ꦜ ꦝ ꦞ ꦟ ꦠ ꦡ ꦢ ꦣ ꦤ ꦥ ꦦ ꦧ ꦨ ꦩ ꦪ ꦫ ꦭ ꦮ ꦯ ꦰ ꦱ ꦲ"),
        (Kannada, "ಕ ಖ ಗ ಘ ಙ ಚ ಛ ಜ ಝ ಞ ಟ ಠ ಡ ಢ ಣ ತ ಥ ದ ಧ ನ ಪ ಫ ಬ ಭ ಮ ಯ ರ ಲ ವ ಶ ಷ ಸ ಹ"),
        (Khmer, "ក ខ គ ឃ ង ច ឆ ជ ឈ ញ ដ ឋ ឌ ឍ ណ ត ថ ទ ធ ន ប ផ ព ភ ម យ រ ល វ ឝ ឞ ស ហ"),
        (Malayalam, "ക ഖ ഗ ഘ ങ ച ഛ ജ ഝ ഞ ട ഠ ഡ ഢ ണ ത ഥ ദ ധ ന പ ഫ ബ ഭ മ യ ര ല വ ശ ഷ സ ഹ"),
        (Modi, "𑘎 𑘏 𑘐 𑘑 𑘒 𑘓 𑘔 𑘕 𑘖 𑘗 𑘘 𑘙 𑘚 𑘛 𑘜 𑘝 𑘞 𑘟 𑘠 𑘡 𑘢 𑘣 𑘤 𑘥 𑘦 𑘧 𑘨 𑘩 𑘪 𑘫 𑘬 𑘭 𑘮"),
        (Newa, "𑐎 𑐏 𑐐 𑐑 𑐒 𑐔 𑐕 𑐖 𑐗 𑐘 𑐚 𑐛 𑐜 𑐝 𑐞 𑐟 𑐠 𑐡 𑐢 𑐣 𑐥 𑐦 𑐧 𑐨 𑐩 𑐫 𑐬 𑐮 𑐰 𑐱 𑐲 𑐳 𑐴"),
        (Odia, "କ ଖ ଗ ଘ ଙ ଚ ଛ ଜ ଝ ଞ ଟ ଠ ଡ ଢ ଣ ତ ଥ ଦ ଧ ନ ପ ଫ ବ ଭ ମ ଯ ର ଲ ଵ ଶ ଷ ସ ହ"),
        (Saurashtra, "ꢒ ꢓ ꢔ ꢕ ꢖ ꢗ ꢘ ꢙ ꢚ ꢛ ꢜ ꢝ ꢞ ꢟ ꢠ ꢡ ꢢ ꢣ ꢤ ꢥ ꢦ ꢧ ꢨ ꢩ ꢪ ꢫ ꢬ ꢭ ꢮ ꢯ ꢰ ꢱ ꢲ"),
        (Sharada, "𑆑 𑆒 𑆓 𑆔 𑆕 𑆖 𑆗 𑆘 𑆙 𑆚 𑆛 𑆜 𑆝 𑆞 𑆟 𑆠 𑆡 𑆢 𑆣 𑆤 𑆥 𑆦 𑆧 𑆨 𑆩 𑆪 𑆫 𑆬 𑆮 𑆯 𑆰 𑆱 𑆲"),
        (Siddham, "𑖎 𑖏 𑖐 𑖑 𑖒 𑖓 𑖔 𑖕 𑖖 𑖗 𑖘 𑖙 𑖚 𑖛 𑖜 𑖝 𑖞 𑖟 𑖠 𑖡 𑖢 𑖣 𑖤 𑖥 𑖦 𑖧 𑖨 𑖩 𑖪 𑖫 𑖬 𑖭 𑖮"),
        (Sinhala, "ක ඛ ග ඝ ඞ ච ඡ ජ ඣ ඤ ට ඨ ඩ ඪ ණ ත ථ ද ධ න ප ඵ බ භ ම ය ර ල ව ශ ෂ ස හ"),
        (Tamil, "க க² க³ க⁴ ங ச ச² ஜ ஜ² ஞ ட ட² ட³ ட⁴ ண த த² த³ த⁴ ந ப ப² ப³ ப⁴ ம ய ர ல வ ஶ ஷ ஸ ஹ"),
        (Telugu, "క ఖ గ ఘ ఙ చ ఛ జ ఝ ఞ ట ఠ డ ఢ ణ త థ ద ధ న ప ఫ బ భ మ య ర ల వ శ ష స హ"),
        (Thai, "ก ข ค ฆ ง จ ฉ ช ฌ ญ ฏ ฐ ฑ ฒ ณ ต ถ ท ธ น ป ผ พ ภ ม ย ร ล ว ศ ษ ส ห"),
        (Tirhuta, "𑒏 𑒐 𑒑 𑒒 𑒓 𑒔 𑒕 𑒖 𑒗 𑒘 𑒙 𑒚 𑒛 𑒜 𑒝 𑒞 𑒟 𑒠 𑒡 𑒢 𑒣 𑒤 𑒥 𑒦 𑒧 𑒨 𑒩 𑒪 𑒫 𑒬 𑒭 𑒮 𑒯"),
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

    // No distinction between Ta / ta
    assert_one_way_pairwise(
        (Slp1, slp1),
        &[
        // (Cham, "ꨆ ꨇ ꨈ ꨉ ꨋ ꨌ ꨍ ꨎ ꨏ ꨑ ꨓ ꨔ ꨕ ꨖ ꨘ ꨓ ꨔ ꨕ ꨖ ꨘ ꨚ ꨜ ꨝ ꨞ ꨠ ꨢ ꨣ ꨤ ꨥ ꨦ ꨦ ꨧ ꨨ"),
        ],
    );
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
        (Bengali, "০ ১ ২ ৩ ৪ ৫ ৬ ৭ ৮ ৯ । ॥ ঽ"),
        // (Cham, "꩐ ꩑ ꩒ ꩓ ꩔ ꩕ ꩖ ꩗ ꩘ ꩙ ꩝ ꩞ '"),
        (Devanagari, "० १ २ ३ ४ ५ ६ ७ ८ ९ । ॥ ऽ"),
        (Grantha, "௦ ௧ ௨ ௩ ௪ ௫ ௬ ௭ ௮ ௯ । ॥ 𑌽"),
        (Gujarati, "૦ ૧ ૨ ૩ ૪ ૫ ૬ ૭ ૮ ૯ । ॥ ઽ"),
        (Gurmukhi, "੦ ੧ ੨ ੩ ੪ ੫ ੬ ੭ ੮ ੯ । ॥ ऽ"),
        (Kannada, "೦ ೧ ೨ ೩ ೪ ೫ ೬ ೭ ೮ ೯ । ॥ ಽ"),
        (Khmer, "០ ១ ២ ៣ ៤ ៥ ៦ ៧ ៨ ៩ ។ ៕ ៜ"),
        (Malayalam, "൦ ൧ ൨ ൩ ൪ ൫ ൬ ൭ ൮ ൯ । ॥ ഽ"),
        (Modi, "𑙐 𑙑 𑙒 𑙓 𑙔 𑙕 𑙖 𑙗 𑙘 𑙙 𑙁 𑙂 ऽ"),
        (Newa, "𑑐 𑑑 𑑒 𑑓 𑑔 𑑕 𑑖 𑑗 𑑘 𑑙 𑑋 𑑌 𑑇"),
        (Odia, "୦ ୧ ୨ ୩ ୪ ୫ ୬ ୭ ୮ ୯ । ॥ ଽ"),
        (Saurashtra, "꣐ ꣑ ꣒ ꣓ ꣔ ꣕ ꣖ ꣗ ꣘ ꣙ ꣎ ꣏ ఽ"),
        (Telugu, "౦ ౧ ౨ ౩ ౪ ౫ ౬ ౭ ౮ ౯ । ॥ ఽ"),
        (Thai, "๐ ๑ ๒ ๓ ๔ ๕ ๖ ๗ ๘ ๙ ฯ ๚ '"),
        (Tibetan, "༠་༡་༢་༣་༤་༥་༦་༧་༨་༩་།་༎་྅"),
        (Tirhuta, "𑓐 𑓑 𑓒 𑓓 𑓔 𑓕 𑓖 𑓗 𑓘 𑓙 । ॥ 𑓄"),
    ]);
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
        (Brahmi, "𑀦𑀸𑀭𑀸𑀬𑀡𑀁 𑀦𑀫𑀲𑁆𑀓𑀾𑀢𑁆𑀬 𑀦𑀭𑀁 𑀘𑁃𑀯 𑀦𑀭𑁄𑀢𑁆𑀢𑀫𑀫𑁆 𑁇 𑀤𑁂𑀯𑀻𑀁 𑀲𑀭𑀲𑁆𑀯𑀢𑀻𑀁 𑀘𑁃𑀯 𑀢𑀢𑁄 𑀚𑀬𑀫𑀼𑀤𑀻𑀭𑀬𑁂𑀢𑁆 𑁈 𑁧 𑁈"),
        (Devanagari, "नारायणं नमस्कृत्य नरं चैव नरोत्तमम् । देवीं सरस्वतीं चैव ततो जयमुदीरयेत् ॥ १ ॥"),
        (Grantha, "𑌨𑌾𑌰𑌾𑌯𑌣𑌂 𑌨𑌮𑌸𑍍𑌕𑍃𑌤𑍍𑌯 𑌨𑌰𑌂 𑌚𑍈𑌵 𑌨𑌰𑍋𑌤𑍍𑌤𑌮𑌮𑍍 । 𑌦𑍇𑌵𑍀𑌂 𑌸𑌰𑌸𑍍𑌵𑌤𑍀𑌂 𑌚𑍈𑌵 𑌤𑌤𑍋 𑌜𑌯𑌮𑍁𑌦𑍀𑌰𑌯𑍇𑌤𑍍 ॥ ௧ ॥"),
        (Gujarati, "નારાયણં નમસ્કૃત્ય નરં ચૈવ નરોત્તમમ્ । દેવીં સરસ્વતીં ચૈવ તતો જયમુદીરયેત્ ॥ ૧ ॥"),
        (Kannada, "ನಾರಾಯಣಂ ನಮಸ್ಕೃತ್ಯ ನರಂ ಚೈವ ನರೋತ್ತಮಮ್ । ದೇವೀಂ ಸರಸ್ವತೀಂ ಚೈವ ತತೋ ಜಯಮುದೀರಯೇತ್ ॥ ೧ ॥"),
        (Khmer, "នារាយណំ នមស្ក្ឫត្យ នរំ ចៃវ នរោត្តមម៑ ។ ទេវីំ សរស្វតីំ ចៃវ តតោ ជយមុទីរយេត៑ ៕ ១ ៕"),
        (Malayalam, "നാരായണം നമസ്കൃത്യ നരം ചൈവ നരോത്തമമ് । ദേവീം സരസ്വതീം ചൈവ തതോ ജയമുദീരയേത് ॥ ൧ ॥"),
        (Modi, "𑘡𑘰𑘨𑘰𑘧𑘜𑘽 𑘡𑘦𑘭𑘿𑘎𑘵𑘝𑘿𑘧 𑘡𑘨𑘽 𑘓𑘺𑘪 𑘡𑘨𑘻𑘝𑘿𑘝𑘦𑘦𑘿 𑙁 𑘟𑘹𑘪𑘲𑘽 𑘭𑘨𑘭𑘿𑘪𑘝𑘲𑘽 𑘓𑘺𑘪 𑘝𑘝𑘻 𑘕𑘧𑘦𑘳𑘟𑘲𑘨𑘧𑘹𑘝𑘿 𑙂 𑙑 𑙂"),
        (Newa, "𑐣𑐵𑐬𑐵𑐫𑐞𑑄 𑐣𑐩𑐳𑑂𑐎𑐺𑐟𑑂𑐫 𑐣𑐬𑑄 𑐔𑐿𑐰 𑐣𑐬𑑀𑐟𑑂𑐟𑐩𑐩𑑂 𑑋 𑐡𑐾𑐰𑐷𑑄 𑐳𑐬𑐳𑑂𑐰𑐟𑐷𑑄 𑐔𑐿𑐰 𑐟𑐟𑑀 𑐖𑐫𑐩𑐸𑐡𑐷𑐬𑐫𑐾𑐟𑑂 𑑌 𑑑 𑑌"),
        (Odia, "ନାରାଯଣଂ ନମସ୍କୃତ୍ଯ ନରଂ ଚୈଵ ନରୋତ୍ତମମ୍ । ଦେଵୀଂ ସରସ୍ଵତୀଂ ଚୈଵ ତତୋ ଜଯମୁଦୀରଯେତ୍ ॥ ୧ ॥"),
        (Saurashtra, "ꢥꢵꢬꢵꢫꢠꢀ ꢥꢪꢱ꣄ꢒꢺꢡ꣄ꢫ ꢥꢬꢀ ꢗꣀꢮ ꢥꢬꣂꢡ꣄ꢡꢪꢪ꣄ ꣎ ꢣꢿꢮꢷꢀ ꢱꢬꢱ꣄ꢮꢡꢷꢀ ꢗꣀꢮ ꢡꢡꣂ ꢙꢫꢪꢸꢣꢷꢬꢫꢿꢡ꣄ ꣏ ꣑ ꣏"),
        (Sharada, "𑆤𑆳𑆫𑆳𑆪𑆟𑆁 𑆤𑆩𑆱𑇀𑆑𑆸𑆠𑇀𑆪 𑆤𑆫𑆁 𑆖𑆽𑆮 𑆤𑆫𑆾𑆠𑇀𑆠𑆩𑆩𑇀 𑇅 𑆢𑆼𑆮𑆵𑆁 𑆱𑆫𑆱𑇀𑆮𑆠𑆵𑆁 𑆖𑆽𑆮 𑆠𑆠𑆾 𑆘𑆪𑆩𑆶𑆢𑆵𑆫𑆪𑆼𑆠𑇀 𑇆 𑇑 𑇆"),
        (Siddham, "𑖡𑖯𑖨𑖯𑖧𑖜𑖽 𑖡𑖦𑖭𑖿𑖎𑖴𑖝𑖿𑖧 𑖡𑖨𑖽 𑖓𑖹𑖪 𑖡𑖨𑖺𑖝𑖿𑖝𑖦𑖦𑖿 𑗂 𑖟𑖸𑖪𑖱𑖽 𑖭𑖨𑖭𑖿𑖪𑖝𑖱𑖽 𑖓𑖹𑖪 𑖝𑖝𑖺 𑖕𑖧𑖦𑖲𑖟𑖱𑖨𑖧𑖸𑖝𑖿 𑗃 1 𑗃"),
        (Tamil, "நாராயணம்ʼ நமஸ்க்ருʼத்ய நரம்ʼ சைவ நரோத்தமம் . தே³வீம்ʼ ஸரஸ்வதீம்ʼ சைவ ததோ ஜயமுதீ³ரயேத் .. 1 .."),
        (Telugu, "నారాయణం నమస్కృత్య నరం చైవ నరోత్తమమ్ । దేవీం సరస్వతీం చైవ తతో జయముదీరయేత్ ॥ ౧ ॥"),
        (Thai, "นารายณํ นมสฺกฺฤตฺย นรํ ไจว นโรตฺตมมฺ ฯ เทวีํ สรสฺวตีํ ไจว ตโต ชยมุทีรเยตฺ ๚ ๑ ๚"),
        (Tirhuta, "𑒢𑒰𑒩𑒰𑒨𑒝𑓀 𑒢𑒧𑒮𑓂𑒏𑒵𑒞𑓂𑒨 𑒢𑒩𑓀 𑒔𑒻𑒫 𑒢𑒩𑒼𑒞𑓂𑒞𑒧𑒧𑓂 । 𑒠𑒹𑒫𑒲𑓀 𑒮𑒩𑒮𑓂𑒫𑒞𑒲𑓀 𑒔𑒻𑒫 𑒞𑒞𑒼 𑒖𑒨𑒧𑒳𑒠𑒲𑒩𑒨𑒹𑒞𑓂 ॥ 𑓑 ॥"),
    ]);

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
fn sanskrit_vedic_accent() {
    // Svarita and anudatta
    assert_two_way_pairwise(&[
        (BarahaSouth, "aq a#"),
        (Itrans, r"a\_ a\'"),
        (Slp1, r"a\ a^"),
        // Indic
        (Bengali, "অ॒ অ॑"),
        (Devanagari, "अ॒ अ॑"),
        (Grantha, "𑌅॒ 𑌅᳴"),
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
fn sanskrit_vedic_consonants() {
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
        (Burmese, "ဠ ဠ်ဟ"),
        (Devanagari, "ळ ळ्ह"),
        (Grantha, "𑌳 𑌳𑍍𑌹"),
        (Gujarati, "ળ ળ્હ"),
        (Javanese, "ꦭ꦳ ꦭ꦳꧀ꦲ"),
        (Kannada, "ಳ ಳ್ಹ"),
        (Malayalam, "ള ള്ഹ"),
        (Odia, "ଳ ଳ୍ହ"),
        (Saurashtra, "ꢳ ꢳ꣄ꢲ"),
        (Sharada, "𑆭 𑆭𑇀𑆲"),
        (Siddham, "𑖩𑗀 𑖩𑗀𑖿𑖮"),
        (Sinhala, "ළ ළ්හ"),
        (Telugu, "ళ ళ్హ"),
    ]);
}

#[test]
fn other_consonants() {
    assert_two_way_pairwise(&[
        (Devanagari, "क़ ख़ ग़ ज़ ड़ ढ़ फ़ य़ ऱ"),
        (Itrans, "qa Ka Ga za .Da .Dha fa Ya Ra"),
        (Iso15919, "qa k͟ha ġa za ṛa ṛha fa ẏa ṟa"),
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

/// Tests that NFC and NFD variants of IAST are transliterated in the same way.
#[test]
fn iast_unicode_variants() {
    use unicode_normalization::UnicodeNormalization;

    let cases = &[
        ("A", "ā"),
        ("I", "ī"),
        ("U", "ū"),
        ("f", "ṛ"),
        ("F", "ṝ"),
        ("x", "ḷ"),
        ("X", "ḹ"),
        ("M", "ṃ"),
        ("H", "ḥ"),
        ("N", "ṅ"),
        ("Y", "ñ"),
        ("w", "ṭ"),
        ("q", "ḍ"),
        ("R", "ṇ"),
        ("S", "ś"),
        ("z", "ṣ"),
    ];

    let mut lipika = Lipika::new();
    for (slp1, iast) in cases {
        let iast_nfc: String = iast.nfc().collect();
        let slp1_out_nfc = lipika.transliterate(&iast_nfc, Scheme::Iast, Scheme::Slp1);
        assert_eq!(&slp1_out_nfc, slp1);

        let iast_nfd: String = iast.nfd().collect();
        assert_ne!(iast_nfc, iast_nfd);

        let slp1_out_nfd = lipika.transliterate(&iast_nfd, Scheme::Iast, Scheme::Slp1);
        assert_eq!(&slp1_out_nfd, slp1);
    }
}

#[test]
fn iso_tamil_aytam() {
    assert_transliterate("ஃ", Tamil, Iso15919, "ḳ");
    assert_transliterate("\u{1e33}", Iso15919, Tamil, "ஃ");
    assert_transliterate("k\u{0323}", Iso15919, Tamil, "ஃ");
}

/// Tests that NFC and NFD variants of ISO-15919 are transliterated in the same way.
#[test]
fn iso_unicode_variants() {
    use unicode_normalization::UnicodeNormalization;

    let cases = &[
        ("A", "ā"),
        ("I", "ī"),
        ("U", "ū"),
        ("M", "ṁ"),
        ("H", "ḥ"),
        ("N", "ṅ"),
        ("Y", "ñ"),
        ("w", "ṭ"),
        ("q", "ḍ"),
        ("R", "ṇ"),
        ("S", "ś"),
        ("z", "ṣ"),
    ];

    let mut lipika = Lipika::new();
    for (slp1, iso) in cases {
        let iso_nfc: String = iso.nfc().collect();
        let slp1_out_nfc = lipika.transliterate(&iso_nfc, Scheme::Iso15919, Scheme::Slp1);
        assert_eq!(&slp1_out_nfc, slp1);

        let iso_nfd: String = iso.nfd().collect();
        assert_ne!(iso_nfc, iso_nfd);

        let slp1_out_nfd = lipika.transliterate(&iso_nfd, Scheme::Iso15919, Scheme::Slp1);
        assert_eq!(&slp1_out_nfd, slp1);
    }
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
fn khmer_sign_robat() {
    assert_two_way_pairwise(&[(Slp1, "kra kara rka arka kara"), (Khmer, "ក្រ ករ ក៌ អក៌ ករ")]);
}

#[test]
fn saurashtra_ksha_with_zwj() {
    assert_two_way_pairwise(&[(Slp1, "kza"), (Saurashtra, "ꢒ꣄\u{200d}ꢰ")]);
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
fn thai_preceding_vowels() {
    assert_two_way_pairwise(&[(Slp1, "ke kE ko kO"), (Thai, "เก ไก โก เกา")]);
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
