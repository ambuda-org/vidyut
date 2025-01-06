.. py:currentmodule:: vidyut.lipi

Transliterating with `vidyut.lipi`
==================================

`vidyut-lipi` is a transliteration library for Sanskrit and Pali that also
supports many of the scripts used within the Indosphere. Our main goals with
this module are correctness, completeness, and consistency.

`vidyut-lipi` is generally comparable to other Python-based transliterators
like `Aksharamukha`_ and `indic_transliteration`_ but has specific advantages
for certain use cases. For details on how `vidyut.lipi` differs from these
transliterators, see the :ref:`Features` section below.

.. _indic_transliteration: https://github.com/indic-transliteration/indic_transliteration_py
.. _Aksharamukha: https://github.com/virtualvinodh/aksharamukha-python


Quickstart
----------

Transliteration is simple and straightforward:

.. testcode::

    from vidyut.lipi import transliterate, Scheme

    text = "tapaHsvAdhyAyanirataM tapasvI vAgvidAM varam"
    output = transliterate(text, Scheme.HarvardKyoto, Scheme.Devanagari)
    print(output)

Output:

.. testoutput::

    तपःस्वाध्यायनिरतं तपस्वी वाग्विदां वरम्

If the input scheme is not known, you can use :func:`detect` to guess it:

.. testcode::

    from vidyut.lipi import detect, transliterate, Scheme

    text = "tapaHsvAdhyAyanirataM tapasvI vAgvidAM varam"

    # NOTE: `detect` returns `None` if no scheme could be found.
    source = detect(text)
    assert source == Scheme.HarvardKyoto

    output = transliterate(text, source, Scheme.Devanagari)

`vidyut.lipi` supports a wide variety of transliteration schemes. For example:

.. testcode::

    text = "tapaHsvAdhyAyanirataM tapasvI vAgvidAM varam"
    for dest in Scheme.choices():
        output = transliterate(text, Scheme.HarvardKyoto, dest)
        print(f"{dest}: {output}")

Output:

.. testoutput::

    Assamese: তপঃস্ৱাধ্যায়নিৰতং তপস্ৱী ৱাগ্ৱিদাং ৱৰম্
    Balinese: ᬢᬧᬄᬲ᭄ᬯᬵᬥ᭄ᬬᬵᬬᬦᬶᬭᬢᬂ ᬢᬧᬲ᭄ᬯᬷ ᬯᬵᬕ᭄ᬯᬶᬤᬵᬂ ᬯᬭᬫ᭄
    Bengali: তপঃস্বাধ্যায়নিরতং তপস্বী বাগ্বিদাং বরম্
    Bhaiksuki: 𑰝𑰢𑰾𑰭𑰿𑰪𑰯𑰠𑰿𑰧𑰯𑰧𑰡𑰰𑰨𑰝𑰽𑱃𑰝𑰢𑰭𑰿𑰪𑰱𑱃𑰪𑰯𑰐𑰿𑰪𑰰𑰟𑰯𑰽𑱃𑰪𑰨𑰦𑰿
    Brahmi: 𑀢𑀧𑀂𑀲𑁆𑀯𑀸𑀥𑁆𑀬𑀸𑀬𑀦𑀺𑀭𑀢𑀁 𑀢𑀧𑀲𑁆𑀯𑀻 𑀯𑀸𑀕𑁆𑀯𑀺𑀤𑀸𑀁 𑀯𑀭𑀫𑁆
    Burmese: တပးသွာဓျာယနိရတံ တပသွီ ဝာဂွိဒာံ ဝရမ်
    Cham: ꨓꨚꩍꨧꨶꨩꨖꨳꨩꨢꨘꨪꨣꨓꩌ ꨓꨚꨧꨶꨫ ꨥꨩꨈꨶꨪꨕꨩꩌ ꨥꨣꩌ
    Devanagari: तपःस्वाध्यायनिरतं तपस्वी वाग्विदां वरम्
    Dogra: 𑠙𑠞𑠸𑠩𑠹𑠦𑠬𑠜𑠹𑠣𑠬𑠣𑠝𑠭𑠤𑠙𑠷 𑠙𑠞𑠩𑠹𑠦𑠮 𑠦𑠬𑠌𑠹𑠦𑠭𑠛𑠬𑠷 𑠦𑠤𑠢𑠹
    Grantha: 𑌤𑌪𑌃𑌸𑍍𑌵𑌾𑌧𑍍𑌯𑌾𑌯𑌨𑌿𑌰𑌤𑌂 𑌤𑌪𑌸𑍍𑌵𑍀 𑌵𑌾𑌗𑍍𑌵𑌿𑌦𑌾𑌂 𑌵𑌰𑌮𑍍
    Gujarati: તપઃસ્વાધ્યાયનિરતં તપસ્વી વાગ્વિદાં વરમ્
    GunjalaGondi: 𑵳𑶅𑶖𑶉𑶗𑵭𑶊𑵹𑶗𑵬𑶊𑵬𑵺𑶋𑶈𑵳𑶕 𑵳𑶅𑶉𑶗𑵭𑶌 𑵭𑶊𑵶𑶗𑵭𑶋𑵸𑶊𑶕 𑵭𑶈𑵰
    Gurmukhi: ਤਪਃਸ੍ਵਾਧ੍ਯਾਯਨਿਰਤਂ ਤਪਸ੍ਵੀ ਵਾਗ੍ਵਿਦਾਂ ਵਰਮ੍
    Javanese: ꦠꦥꦃꦱ꧀ꦮꦴꦣꦾꦴꦪꦤꦶꦫꦠꦁ ꦠꦥꦱ꧀ꦮꦷ ꦮꦴꦒ꧀ꦮꦶꦢꦴꦁ ꦮꦫꦩ꧀
    Kaithi: 𑂞𑂣𑂂𑂮𑂹𑂫𑂰𑂡𑂹𑂨𑂰𑂨𑂢𑂱𑂩𑂞𑂁 𑂞𑂣𑂮𑂹𑂫𑂲 𑂫𑂰𑂏𑂹𑂫𑂱𑂠𑂰𑂁 𑂫𑂩𑂧𑂹
    Kannada: ತಪಃಸ್ವಾಧ್ಯಾಯನಿರತಂ ತಪಸ್ವೀ ವಾಗ್ವಿದಾಂ ವರಮ್
    Kharoshthi: 𐨟𐨤𐨏𐨯𐨿𐨬𐨌𐨢𐨿𐨩𐨌𐨩𐨣𐨁𐨪𐨟𐨎 𐨟𐨤𐨯𐨿𐨬𐨁𐨌 𐨬𐨌𐨒𐨿𐨬𐨁𐨡𐨌𐨎 𐨬𐨪𐨨𐨿
    Khmer: តបះស្វាធ្យាយនិរតំ តបស្វី វាគ្វិទាំ វរម៑
    Khudawadi: 𑋍𑋒𑋞𑋪𑋝𑋪𑋛𑋠𑋐𑋪𑋘𑋠𑋘𑋑𑋡𑋙𑋍𑋟 𑋍𑋒𑋝𑋪𑋛𑋢 𑋛𑋠𑊼𑋪𑋛𑋡𑋏𑋠𑋟 𑋛𑋙𑋗𑋪
    Limbu: ᤋᤐᤜ᤻ᤛᤫᤠᤎᤩᤠᤕᤏᤡᤖᤋᤱ ᤋᤐᤛᤫᤡ᤺ ᤘᤠᤃᤫᤡᤍᤠᤱ ᤘᤖᤔ᤻
    Malayalam: തപഃസ്വാധ്യായനിരതം തപസ്വീ വാഗ്വിദാം വരമ്
    MeeteiMayek: ꯇꯄꯍ꯭ꯁ꯭ꯋꯥꯙ꯭ꯌꯥꯌꯅꯤꯔꯇꯪ ꯇꯄꯁ꯭ꯋꯤ ꯋꯥꯒ꯭ꯋꯤꯗꯥꯪ ꯋꯔꯝ
    MasaramGondi: 𑴛𑴠𑵁𑴫𑵅𑴨𑴱𑴞𑵅𑴥𑴱𑴥𑴟𑴲𑴦𑴛𑵀 𑴛𑴠𑴫𑵅𑴨𑴳 𑴨𑴱𑴎𑵅𑴨𑴲𑴝𑴱𑵀 𑴨𑴦𑴤𑵅
    Modi: 𑘝𑘢𑘾𑘭𑘿𑘪𑘰𑘠𑘿𑘧𑘰𑘧𑘡𑘱𑘨𑘝𑘽 𑘝𑘢𑘭𑘿𑘪𑘲 𑘪𑘰𑘐𑘿𑘪𑘱𑘟𑘰𑘽 𑘪𑘨𑘦𑘿
    Mon: တပးသွာဓျာယနိရတံ တပသွဳ ဝာဂွိဒာံ ဝရမ်
    Nandinagari: 𑦽𑧂𑧟𑧍𑧠𑧊𑧑𑧀𑧠𑧇𑧑𑧇𑧁𑧒𑧈𑦽𑧞 𑦽𑧂𑧍𑧠𑧊𑧓 𑧊𑧑𑦰𑧠𑧊𑧒𑦿𑧑𑧞 𑧊𑧈𑧆𑧠
    Newa: 𑐟𑐥𑑅𑐳𑑂𑐰𑐵𑐢𑑂𑐫𑐵𑐫𑐣𑐶𑐬𑐟𑑄 𑐟𑐥𑐳𑑂𑐰𑐷 𑐰𑐵𑐐𑑂𑐰𑐶𑐡𑐵𑑄 𑐰𑐬𑐩𑑂
    Odia: ତପଃସ୍ଵାଧ୍ଯାଯନିରତଂ ତପସ୍ଵୀ ଵାଗ୍ଵିଦାଂ ଵରମ୍
    OlChiki: ᱛᱚᱯᱚᱷᱥᱣᱟᱫᱷᱭᱟᱭᱚᱱᱤᱨᱚᱛᱚᱝ ᱛᱚᱯᱚᱥᱣᱤᱻ ᱣᱟᱜᱣᱤᱫᱟᱝ ᱣᱚᱨᱚᱢ
    Saurashtra: ꢡꢦꢁꢱ꣄ꢮꢵꢤ꣄ꢫꢵꢫꢥꢶꢬꢡꢀ ꢡꢦꢱ꣄ꢮꢷ ꢮꢵꢔ꣄ꢮꢶꢣꢵꢀ ꢮꢬꢪ꣄
    Sharada: 𑆠𑆥𑆂𑆱𑇀𑆮𑆳𑆣𑇀𑆪𑆳𑆪𑆤𑆴𑆫𑆠𑆁 𑆠𑆥𑆱𑇀𑆮𑆵 𑆮𑆳𑆓𑇀𑆮𑆴𑆢𑆳𑆁 𑆮𑆫𑆩𑇀
    Siddham: 𑖝𑖢𑖾𑖭𑖿𑖪𑖯𑖠𑖿𑖧𑖯𑖧𑖡𑖰𑖨𑖝𑖽 𑖝𑖢𑖭𑖿𑖪𑖱 𑖪𑖯𑖐𑖿𑖪𑖰𑖟𑖯𑖽 𑖪𑖨𑖦𑖿
    Sinhala: තපඃස්වාධ්යායනිරතං තපස්වී වාග්විදාං වරම්
    Soyombo: 𑩫𑩰𑪗𑪁𑪘𑩾𑩛𑩮𑪘𑩻𑩛𑩻𑩯𑩑𑩼𑩫𑪖 𑩫𑩰𑪁𑪘𑩾𑩑𑩛 𑩾𑩛𑩞𑪘𑩾𑩑𑩭𑩛𑪖 𑩾𑩼𑩴𑪘
    TaiTham: ᨲᨸᩡᩈ᩠ᩅᩣᨵ᩠ᨿᩣᨿᨶᩥᩁᨲᩴ ᨲᨸᩈ᩠ᩅᩦ ᩅᩣᨣ᩠ᩅᩥᨴᩣᩴ ᩅᩁᨾ᩺
    Takri: 𑚙𑚞𑚬𑚨𑚶𑚦𑚭𑚜𑚶𑚣𑚭𑚣𑚝𑚮𑚤𑚙𑚫 𑚙𑚞𑚨𑚶𑚦𑚯 𑚦𑚭𑚌𑚶𑚦𑚮𑚛𑚭𑚫 𑚦𑚤𑚢𑚶
    Tamil: தப꞉ஸ்வாத்⁴யாயநிரதம்ʼ தபஸ்வீ வாக்³விதா³ம்ʼ வரம்
    Telugu: తపఃస్వాధ్యాయనిరతం తపస్వీ వాగ్విదాం వరమ్
    Thai: ตปห์สฺวาธฺยายนิรตํ ตปสฺวี วาคฺวิทาํ วรมฺ
    Tibetan: ཏཔཿསྭཱདྷྱཱཡནིརཏཾ་ཏཔསྭཱི་བཱགྭིདཱཾ་བརམ
    Tirhuta: 𑒞𑒣𑓁𑒮𑓂𑒫𑒰𑒡𑓂𑒨𑒰𑒨𑒢𑒱𑒩𑒞𑓀 𑒞𑒣𑒮𑓂𑒫𑒲 𑒫𑒰𑒑𑓂𑒫𑒱𑒠𑒰𑓀 𑒫𑒩𑒧𑓂
    ZanabazarSquare: 𑨙𑨞𑨹𑨰𑩇𑨭𑨊𑨜𑩇𑨪𑨊𑨪𑨝𑨁𑨫𑨙𑨸 𑨙𑨞𑨰𑩇𑨭𑨁𑨊 𑨭𑨊𑨍𑩇𑨭𑨁𑨛𑨊𑨸 𑨭𑨫𑨢𑨴
    BarahaSouth: tapaHsvAdhyAyanirataM tapasvI vAgvidAM varam
    HarvardKyoto: tapaHsvAdhyAyanirataM tapasvI vAgvidAM varam
    Iast: tapaḥsvādhyāyanirataṃ tapasvī vāgvidāṃ varam
    Iso15919: tapaḥsvādhyāyanirataṁ tapasvī vāgvidāṁ varam
    Itrans: tapaHsvAdhyAyanirataM tapasvI vAgvidAM varam
    Slp1: tapaHsvADyAyanirataM tapasvI vAgvidAM varam
    Velthuis: tapa.hsvaadhyaayanirata.m tapasvii vaagvidaa.m varam
    Wx: wapaHsvAXyAyanirawaM wapasvI vAgvixAM varam



.. _features:

Features
--------

Transliterators are essentially identical for common use cases, but they tend
to differ at the edges. Here are some features that distinguish `vidyut.lipi`
for uncommon use cases.

- `vidyut.lipi` has strong support for Grantha, especially Grantha numerals:

.. testcode::

    text = "1 12 123 1234 12345"
    output = transliterate(text, Scheme.HarvardKyoto, Scheme.Grantha)
    assert output == "௧ ௰௨ ௱௨௰௩ ௲௨௱௩௰௪ ௰௨௲௩௱௪௰௫"

- `vidyut.lipi` has strong support for ISO 15919, including the ``:``
  disambiguating separator:

.. testcode::

    text = "नरइति"
    output = transliterate(text, Scheme.Devanagari, Scheme.Iso15919)
    assert output == "nara:iti"

- `vidyut.lipi` is aware of Unicode NFC and NFD forms and normalizes all output
  to NFC.

- `vidyut.lipi` has strong support for *anudātta* and *svarita* accents across a
  variety of schemes:

.. testcode::

    text = "a\\ a^"
    output = transliterate(text, Scheme.Slp1, Scheme.Devanagari)
    assert output == "अ॒ अ॑"

- :func:`detect` is especially robust and has high coverage.

- `vidyut-lipi` is around 6 times faster than `indic_transliteration` and
  around 8 times faster than Aksharamukha. This speed-up does not matter for
  everyday usage, but it can matter for heavy workloads, such as
  transliterating an entire corpus.

