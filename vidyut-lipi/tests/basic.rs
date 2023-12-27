use vidyut_lipi::Scheme::*;
use vidyut_lipi::{transliterate, Mapping, Scheme};

fn assert_transliterate(input: &str, from: Scheme, to: Scheme, expected: &str) {
    let mapping = Mapping::new(from, to);
    let actual = transliterate(input, &mapping);
    assert_eq!(expected, actual, "t(\"{input}\", {from:?}, {to:?})");
}

/// Transliterates all input strings against each other.
///
/// This function assums that all strings provided are lossless.
fn assert_exhaustive_pairwise(examples: &[(Scheme, &str)]) {
    for (from, input) in examples {
        for (to, expected) in examples {
            let mapping = Mapping::new(*from, *to);
            let actual = transliterate(input, &mapping);
            assert_eq!(*expected, actual, "t(\"{input}\", {from:?}, {to:?})");
        }
    }
}

#[test]
fn sanskrit_independent_vowels() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "a A i I u U R RR lR lRR e ai o au"),
        (Iast, "a ā i ī u ū ṛ ṝ ḷ ḹ e ai o au"),
        (Iso19519, "a ā i ī u ū r̥ r̥̄ l̥ l̥̄ ē ai ō au"),
        (Itrans, "a A i I u U RRi RRI LLi LLI e ai o au"),
        (Slp1, "a A i I u U f F x X e E o O"),
        (Velthuis, "a aa i ii u uu .r .R .l .L e ai o au"),
        // Indic
        (Balinese, "ᬅ ᬆ ᬇ ᬈ ᬉ ᬊ ᬋ ᬌ ᬍ ᬎ ᬏ ᬐ ᬑ ᬒ"),
        (Bengali, "অ আ ই ঈ উ ঊ ঋ ৠ ঌ ৡ এ ঐ ও ঔ"),
        (Brahmi, "𑀅 𑀆 𑀇 𑀈 𑀉 𑀊 𑀋 𑀌 𑀍 𑀎 𑀏 𑀐 𑀑 𑀒"),
        (Burmese, "အ အာ ဣ ဤ ဥ ဦ ၒ ၓ ၔ ၕ ဧ အဲ ဩ ဪ"),
        (Devanagari, "अ आ इ ई उ ऊ ऋ ॠ ऌ ॡ ए ऐ ओ औ"),
        (Grantha, "𑌅 𑌆 𑌇 𑌈 𑌉 𑌊 𑌋 𑍠 𑌌 𑍡 𑌏 𑌐 𑌓 𑌔"),
        (Gujarati, "અ આ ઇ ઈ ઉ ઊ ઋ ૠ ઌ ૡ એ ઐ ઓ ઔ"),
        (Javanese, "ꦄ ꦄꦴ ꦆ ꦇ ꦈ ꦈꦴ ꦉ ꦉꦴ ꦊ ꦋ ꦌ ꦍ ꦎ ꦎꦴ"),
        (Kannada, "ಅ ಆ ಇ ಈ ಉ ಊ ಋ ೠ ಌ ೡ ಏ ಐ ಓ ಔ"),
        (Malayalam, "അ ആ ഇ ഈ ഉ ഊ ഋ ൠ ഌ ൡ ഏ ഐ ഓ ഔ"),
        (Odia, "ଅ ଆ ଇ ଈ ଉ ଊ ଋ ୠ ଌ ୡ ଏ ଐ ଓ ଔ"),
        (Sharada, "𑆃 𑆄 𑆅 𑆆 𑆇 𑆈 𑆉 𑆊 𑆋 𑆌 𑆍 𑆎 𑆏 𑆐"),
        (Sinhala, "අ ආ ඉ ඊ උ ඌ ඍ ඎ ඏ ඐ ඒ ඓ ඕ ඖ"),
        (Telugu, "అ ఆ ఇ ఈ ఉ ఊ ఋ ౠ ఌ ౡ ఏ ఐ ఓ ఔ"),
    ]);
}

#[test]
fn sanskrit_dependent_vowels_and_virama() {
    assert_exhaustive_pairwise(&[
        (
            HarvardKyoto,
            "ka kA ki kI ku kU kR kRR klR klRR ke kai ko kau k",
        ),
        (Iast, "ka kā ki kī ku kū kṛ kṝ kḷ kḹ ke kai ko kau k"),
        (Iso19519, "ka kā ki kī ku kū kr̥ kr̥̄ kl̥ kl̥̄ kē kai kō kau k"),
        (
            Itrans,
            "ka kA ki kI ku kU kRRi kRRI kLLi kLLI ke kai ko kau k",
        ),
        (Slp1, "ka kA ki kI ku kU kf kF kx kX ke kE ko kO k"),
        (
            Velthuis,
            "ka kaa ki kii ku kuu k.r k.R k.l k.L ke kai ko kau k",
        ),
        // Indic
        (Balinese, "ᬓ ᬓᬵ ᬓᬶ ᬓᬷ ᬓᬸ ᬓᬹ ᬓᬺ ᬓᬻ ᬓᬼ ᬓᬽ ᬓᬾ ᬓᬿ ᬓᭀ ᬓᭁ ᬓ᭄"),
        (Bengali, "ক কা কি কী কু কূ কৃ কৄ কৢ কৣ কে কৈ কো কৌ ক্"),
        (Brahmi, "𑀓 𑀓𑀸 𑀓𑀺 𑀓𑀻 𑀓𑀼 𑀓𑀽 𑀓𑀾 𑀓𑀿 𑀓𑁀 𑀓𑁁 𑀓𑁂 𑀓𑁃 𑀓𑁄 𑀓𑁅 𑀓𑁆"),
        (Burmese, "က ကာ ကိ ကီ ကု ကူ ကၖ ကၗ ကၘ ကၙ ကေ ကဲ ကော ကော် က်"),
        (Devanagari, "क का कि की कु कू कृ कॄ कॢ कॣ के कै को कौ क्"),
        (Grantha, "𑌕 𑌕𑌾 𑌕𑌿 𑌕𑍀 𑌕𑍁 𑌕𑍂 𑌕𑍃 𑌕𑍄 𑌕𑍢 𑌕𑍣 𑌕𑍇 𑌕𑍈 𑌕𑍋 𑌕𑍗 𑌕𑍍"),
        (Gujarati, "ક કા કિ કી કુ કૂ કૃ કૄ કૢ કૣ કે કૈ કો કૌ ક્"),
        (Javanese, "ꦏ ꦏꦴ ꦏꦶ ꦏꦷ ꦏꦸ ꦏꦹ ꦏꦽ ꦏ꧀ꦉꦴ ꦏ꧀ꦊ ꦏ꧀ꦋ ꦏꦺ ꦏꦻ ꦏꦺꦴ ꦏꦻꦴ ꦏ꧀"),
        (Kannada, "ಕ ಕಾ ಕಿ ಕೀ ಕು ಕೂ ಕೃ ಕೄ ಕೢ ಕೣ ಕೇ ಕೈ ಕೋ ಕೌ ಕ್"),
        (Malayalam, "ക കാ കി കീ കു കൂ കൃ കൄ കൢ കൣ കേ കൈ കോ കൌ ക്"),
        (Odia, "କ କା କି କୀ କୁ କୂ କୃ କୄ କୢ କୣ କେ କୈ କୋ କୌ କ୍"),
        (Sharada, "𑆑 𑆑𑆳 𑆑𑆴 𑆑𑆵 𑆑𑆶 𑆑𑆷 𑆑𑆸 𑆑𑆹 𑆑𑆺 𑆑𑆻 𑆑𑆼 𑆑𑆽 𑆑𑆾 𑆑𑆿 𑆑𑇀"),
        (Sinhala, "ක කා කි කී කු කූ කෘ කෲ කෟ කෳ කේ කෛ කෝ කෞ ක්"),
        (Telugu, "క కా కి కీ కు కూ కృ కౄ కౢ కౣ కే కై కో కౌ క్"),
    ]);
}

#[test]
fn sanskrit_ayogavahas() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "aM aH"),
        (Iast, "aṃ aḥ"),
        (Iso19519, "aṁ aḥ"),
        (Itrans, "aM aH"),
        (Slp1, "aM aH"),
        (Velthuis, "a.m a.h"),
        (Wx, "aM aH"),
        // Indic
        (Balinese, "ᬅᬂ ᬅᬄ"),
        (Bengali, "অং অঃ"),
        (Brahmi, "𑀅𑀁 𑀅𑀂"),
        (Burmese, "အံ အး"),
        (Devanagari, "अं अः"),
        (Grantha, "𑌅𑌂 𑌅𑌃"),
        (Javanese, "ꦄꦁ ꦄꦃ"),
        (Kannada, "ಅಂ ಅಃ"),
        (Malayalam, "അം അഃ"),
        (Odia, "ଅଂ ଅଃ"),
        (Sharada, "𑆃𑆁 𑆃𑆂"),
        (Sinhala, "අං අඃ"),
        (Telugu, "అం అః"),
    ]);
}

#[test]
fn sanskrit_consonants_non_vedic() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "ka kha ga gha Ga ca cha ja jha Ja Ta Tha Da Dha Na ta tha da dha na pa pha ba bha ma ya ra la va za Sa sa ha",),
        (Iast, "ka kha ga gha ṅa ca cha ja jha ña ṭa ṭha ḍa ḍha ṇa ta tha da dha na pa pha ba bha ma ya ra la va śa ṣa sa ha"),
        (Iso19519, "ka kha ga gha ṅa ca cha ja jha ña ṭa ṭha ḍa ḍha ṇa ta tha da dha na pa pha ba bha ma ya ra la va śa ṣa sa ha"),
        (Itrans, "ka kha ga gha ~Na cha Cha ja jha ~na Ta Tha Da Dha Na ta tha da dha na pa pha ba bha ma ya ra la va sha Sha sa ha"),
        (Slp1, "ka Ka ga Ga Na ca Ca ja Ja Ya wa Wa qa Qa Ra ta Ta da Da na pa Pa ba Ba ma ya ra la va Sa za sa ha"),
        (Velthuis, "ka kha ga gha \"na ca cha ja jha ~na .ta .tha .da .dha .na ta tha da dha na pa pha ba bha ma ya ra la va \"sa .sa sa ha"),
        (Wx, "ka Ka ga Ga fa ca Ca ja Ja Fa ta Ta da Da Na wa Wa xa Xa na pa Pa ba Ba ma ya ra la va Sa Ra sa ha"),
        // Indic
        (Balinese, "ᬓ ᬔ ᬕ ᬖ ᬗ ᬘ ᬙ ᬚ ᬛ ᬜ ᬝ ᬞ ᬟ ᬠ ᬡ ᬢ ᬣ ᬤ ᬥ ᬦ ᬧ ᬨ ᬩ ᬪ ᬫ ᬬ ᬭ ᬮ ᬯ ᬰ ᬱ ᬲ ᬳ"),
        (Burmese, "က ခ ဂ ဃ င စ ဆ ဇ ဈ ဉ ဋ ဌ ဍ ဎ ဏ တ ထ ဒ ဓ န ပ ဖ ဗ ဘ မ ယ ရ လ ဝ ၐ ၑ သ ဟ"),
        (Javanese, "ꦏ ꦑ ꦒ ꦓ ꦔ ꦕ ꦖ ꦗ ꦙ ꦚ ꦛ ꦜ ꦝ ꦞ ꦟ ꦠ ꦡ ꦢ ꦣ ꦤ ꦥ ꦦ ꦧ ꦨ ꦩ ꦪ ꦫ ꦭ ꦮ ꦯ ꦰ ꦱ ꦲ"),
        (Brahmi, "𑀓 𑀔 𑀕 𑀖 𑀗 𑀘 𑀙 𑀚 𑀛 𑀜 𑀝 𑀞 𑀟 𑀠 𑀡 𑀢 𑀣 𑀤 𑀥 𑀦 𑀧 𑀨 𑀩 𑀪 𑀫 𑀬 𑀭 𑀮 𑀯 𑀰 𑀱 𑀲 𑀳"),
        (Devanagari, "क ख ग घ ङ च छ ज झ ञ ट ठ ड ढ ण त थ द ध न प फ ब भ म य र ल व श ष स ह"),
        (Grantha, "𑌕 𑌖 𑌗 𑌘 𑌙 𑌚 𑌛 𑌜 𑌝 𑌞 𑌟 𑌠 𑌡 𑌢 𑌣 𑌤 𑌥 𑌦 𑌧 𑌨 𑌪 𑌫 𑌬 𑌭 𑌮 𑌯 𑌰 𑌲 𑌵 𑌶 𑌷 𑌸 𑌹"),
        (Gujarati, "ક ખ ગ ઘ ઙ ચ છ જ ઝ ઞ ટ ઠ ડ ઢ ણ ત થ દ ધ ન પ ફ બ ભ મ ય ર લ વ શ ષ સ હ"),
        (Kannada, "ಕ ಖ ಗ ಘ ಙ ಚ ಛ ಜ ಝ ಞ ಟ ಠ ಡ ಢ ಣ ತ ಥ ದ ಧ ನ ಪ ಫ ಬ ಭ ಮ ಯ ರ ಲ ವ ಶ ಷ ಸ ಹ"),
        (Malayalam, "ക ഖ ഗ ഘ ങ ച ഛ ജ ഝ ഞ ട ഠ ഡ ഢ ണ ത ഥ ദ ധ ന പ ഫ ബ ഭ മ യ ര ല വ ശ ഷ സ ഹ"),
        (Odia, "କ ଖ ଗ ଘ ଙ ଚ ଛ ଜ ଝ ଞ ଟ ଠ ଡ ଢ ଣ ତ ଥ ଦ ଧ ନ ପ ଫ ବ ଭ ମ ଯ ର ଲ ଵ ଶ ଷ ସ ହ"),
        (Sharada, "𑆑 𑆒 𑆓 𑆔 𑆕 𑆖 𑆗 𑆘 𑆙 𑆚 𑆛 𑆜 𑆝 𑆞 𑆟 𑆠 𑆡 𑆢 𑆣 𑆤 𑆥 𑆦 𑆧 𑆨 𑆩 𑆪 𑆫 𑆬 𑆮 𑆯 𑆰 𑆱 𑆲"),
        (Sinhala, "ක ඛ ග ඝ ඞ ච ඡ ජ ඣ ඤ ට ඨ ඩ ඪ ණ ත ථ ද ධ න ප ඵ බ භ ම ය ර ල ව ශ ෂ ස හ"),
        (Telugu, "క ఖ గ ఘ ఙ చ ఛ జ ఝ ఞ ట ఠ డ ఢ ణ త థ ద ధ న ప ఫ బ భ మ య ర ల వ శ ష స హ"),
    ]);
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
fn itrans_alternates() {
    let assert_identical = |x, y| {
        let mapping = Mapping::new(Itrans, Devanagari);
        let deva_x = transliterate(x, &mapping);
        let deva_y = transliterate(y, &mapping);
        assert_eq!(deva_x, deva_y, "{x} ({deva_x}) != {y} ({deva_y})");
    };
    assert_identical("A I U RRi RRI LLi LLI", "aa ii uu R^i R^I L^i L^I");
    assert_identical(
        "kA kI kU kRRi kRRI kLLi kLLI",
        "kaa kii kuu kR^i kR^I kL^i kL^I",
    );
    assert_identical("I U", "ee oo");
    assert_identical("kI kU", "kee koo");
    assert_identical("aM aM", "a.m a.n");
    assert_identical("~Na", "N^a");
    assert_identical("ca", "cha");
    assert_identical("Cha Cha", "Ca chha");
    assert_identical("va", "wa");
    assert_identical("Sha Sha", "Sa shha");
    assert_identical("kSha kSha kSha", "kSa kshha xa");
    assert_identical("j~na j~na", "GYa dnya");
    assert_identical("OM", "AUM");
    assert_identical(".a | ||", "~ . ..");
    assert_identical("za", "Ja");
    // TODO: assert_identical("a{\\m+}", "a.h.N");
}

#[test]
fn symbols() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Iast, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Itrans, "0 1 2 3 4 5 6 7 8 9 | || .a"),
        (Slp1, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Velthuis, "0 1 2 3 4 5 6 7 8 9 | || .a"),
        (Wx, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        // Indic
        (Bengali, "০ ১ ২ ৩ ৪ ৫ ৬ ৭ ৮ ৯ । ॥ ঽ"),
        (Devanagari, "० १ २ ३ ४ ५ ६ ७ ८ ९ । ॥ ऽ"),
        (Grantha, "௦ ௧ ௨ ௩ ௪ ௫ ௬ ௭ ௮ ௯ । ॥ 𑌽"),
        (Gujarati, "૦ ૧ ૨ ૩ ૪ ૫ ૬ ૭ ૮ ૯ । ॥ ઽ"),
        (Gurmukhi, "੦ ੧ ੨ ੩ ੪ ੫ ੬ ੭ ੮ ੯ । ॥ ऽ"),
        (Kannada, "೦ ೧ ೨ ೩ ೪ ೫ ೬ ೭ ೮ ೯ । ॥ ಽ"),
        (Malayalam, "൦ ൧ ൨ ൩ ൪ ൫ ൬ ൭ ൮ ൯ । ॥ ഽ"),
        (Odia, "୦ ୧ ୨ ୩ ୪ ୫ ୬ ୭ ୮ ୯ । ॥ ଽ"),
        (Telugu, "౦ ౧ ౨ ౩ ౪ ౫ ౬ ౭ ౮ ౯ । ॥ ఽ"),
    ]);
}

#[test]
fn sanskrit_basic_sentences() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "nArAyaNaM namaskRtya naraM caiva narottamam . devIM sarasvatIM caiva tato jayamudIyaret .. 1 ..",),
        (Iast, "nārāyaṇaṃ namaskṛtya naraṃ caiva narottamam . devīṃ sarasvatīṃ caiva tato jayamudīyaret .. 1 .."),
        (Itrans, "nArAyaNaM namaskRRitya naraM chaiva narottamam | devIM sarasvatIM chaiva tato jayamudIyaret || 1 ||"),
        (Slp1, "nArAyaRaM namaskftya naraM cEva narottamam . devIM sarasvatIM cEva tato jayamudIyaret .. 1 .."),
        (Velthuis, "naaraaya.na.m namask.rtya nara.m caiva narottamam | devii.m sarasvatii.m caiva tato jayamudiiyaret || 1 ||"),
        // Indic
        (Devanagari, "नारायणं नमस्कृत्य नरं चैव नरोत्तमम् । देवीं सरस्वतीं चैव ततो जयमुदीयरेत् ॥ १ ॥"),
        (Brahmi, "𑀦𑀸𑀭𑀸𑀬𑀡𑀁 𑀦𑀫𑀲𑁆𑀓𑀾𑀢𑁆𑀬 𑀦𑀭𑀁 𑀘𑁃𑀯 𑀦𑀭𑁄𑀢𑁆𑀢𑀫𑀫𑁆 𑁇 𑀤𑁂𑀯𑀻𑀁 𑀲𑀭𑀲𑁆𑀯𑀢𑀻𑀁 𑀘𑁃𑀯 𑀢𑀢𑁄 𑀚𑀬𑀫𑀼𑀤𑀻𑀬𑀭𑁂𑀢𑁆 𑁈 𑁧 𑁈"),
        (Grantha, "𑌨𑌾𑌰𑌾𑌯𑌣𑌂 𑌨𑌮𑌸𑍍𑌕𑍃𑌤𑍍𑌯 𑌨𑌰𑌂 𑌚𑍈𑌵 𑌨𑌰𑍋𑌤𑍍𑌤𑌮𑌮𑍍 । 𑌦𑍇𑌵𑍀𑌂 𑌸𑌰𑌸𑍍𑌵𑌤𑍀𑌂 𑌚𑍈𑌵 𑌤𑌤𑍋 𑌜𑌯𑌮𑍁𑌦𑍀𑌯𑌰𑍇𑌤𑍍 ॥ ௧ ॥"),
        (Gujarati, "નારાયણં નમસ્કૃત્ય નરં ચૈવ નરોત્તમમ્ । દેવીં સરસ્વતીં ચૈવ તતો જયમુદીયરેત્ ॥ ૧ ॥"),
        (Kannada, "ನಾರಾಯಣಂ ನಮಸ್ಕೃತ್ಯ ನರಂ ಚೈವ ನರೋತ್ತಮಮ್ । ದೇವೀಂ ಸರಸ್ವತೀಂ ಚೈವ ತತೋ ಜಯಮುದೀಯರೇತ್ ॥ ೧ ॥"),
        (Malayalam, "നാരായണം നമസ്കൃത്യ നരം ചൈവ നരോത്തമമ് । ദേവീം സരസ്വതീം ചൈവ തതോ ജയമുദീയരേത് ॥ ൧ ॥"),
        (Odia, "ନାରାଯଣଂ ନମସ୍କୃତ୍ଯ ନରଂ ଚୈଵ ନରୋତ୍ତମମ୍ । ଦେଵୀଂ ସରସ୍ଵତୀଂ ଚୈଵ ତତୋ ଜଯମୁଦୀଯରେତ୍ ॥ ୧ ॥"),
        (Telugu, "నారాయణం నమస్కృత్య నరం చైవ నరోత్తమమ్ । దేవీం సరస్వతీం చైవ తతో జయముదీయరేత్ ॥ ౧ ॥"),
    ]);
}
