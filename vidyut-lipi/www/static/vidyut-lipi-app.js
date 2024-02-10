/**
 * A simple demo interface for vidyut-lipi.
 *
 *
 * Constraints
 * ===========
 * - This demo is served on GitHub pages. So, no databases -- everything should
 *   be done client side!
 * - This demo should use our wasm build's public API.
 * - Although this is a production site, the stakes are low -- do things the
 *   hacky way if that fixes the problem.
 */

import init, { transliterate, detect, Scheme } from "/static/wasm/vidyut_lipi.js";

// ===================================================
// App
// ===================================================

let CHEAT_SHEET = [
  "a A i I u U R RR lR lRR e ai o au aM aH",
  "ka kha ga gha Ga ca cha ja jha Ja",
  "Ta Tha Da Dha Na ta tha da dha na",
  "pa pha ba bha ma ya ra la va",
  "za Sa sa ha La kSa jJa",
  "0 1 2 3 4 5 6 7 8 9 . .. ' OM",
];


let VOWELS = [
  "अ",
  "आ",
  "इ",
  "ई",
  "उ",
  "ऊ",
  "ऋ",
  "ॠ",
  "ऌ",
  "ॡ",
  "ए",
  "ऐ",
  "ओ",
  "औ",
];

let MARKS = [
  "क",
  "का",
  "कि",
  "की",
  "कु",
  "कू",
  "कृ",
  "कॄ",
  "कॢ",
  "कॣ",
  "के",
  "कै",
  "को",
  "कौ",
  "क्"
];

let CONSONANTS = [
  "क",
  "ख",
  "ग",
  "घ",
  "ङ",
  "च",
  "छ",
  "ज",
  "झ",
  "ञ",
  "ट",
  "ठ",
  "ड",
  "ढ",
  "ण",
  "त",
  "थ",
  "द",
  "ध",
  "न",
  "प",
  "फ",
  "ब",
  "भ",
  "म",
  "य",
  "र",
  "ल",
  "व",
  "श",
  "ष",
  "स",
  "ह",
  "ळ"
];

let symbols = [
  "०",
  "१",
  "२",
  "३",
  "४",
  "५",
  "६",
  "७",
  "८",
  "९",
  "।",
  "॥",
  "ऽ"
];

let orderedSchemes = [
  Scheme.Devanagari,
  Scheme.Assamese,
  Scheme.Balinese,
  Scheme.Bengali,
  Scheme.Bhaiksuki,
  Scheme.Brahmi,
  Scheme.Burmese,
  Scheme.Cham,
  Scheme.Dogra,
  Scheme.Grantha,
  Scheme.Gujarati,
  Scheme.GunjalaGondi,
  Scheme.Gurmukhi,
  Scheme.Javanese,
  Scheme.Kaithi,
  Scheme.Kannada,
  Scheme.Khmer,
  Scheme.Khudawadi,
  Scheme.Limbu,
  Scheme.Malayalam,
  Scheme.MasaramGondi,
  Scheme.MeeteiMayek,
  Scheme.Modi,
  Scheme.Nandinagari,
  Scheme.Newa,
  Scheme.Odia,
  Scheme.OlChiki,
  Scheme.Saurashtra,
  Scheme.Sharada,
  Scheme.Siddham,
  Scheme.Sinhala,
  Scheme.TaiTham,
  Scheme.Takri,
  Scheme.Tamil,
  Scheme.Telugu,
  Scheme.Thai,
  Scheme.Tibetan,
  Scheme.Tirhuta,
  Scheme.ZanabazarSquare,

  Scheme.BarahaSouth,
  Scheme.HarvardKyoto,
  Scheme.Iast,
  Scheme.Iso15919,
  Scheme.Itrans,
  Scheme.Slp1,
  Scheme.Velthuis,
  Scheme.Wx,
];

let schemeNames = {
  [Scheme.Devanagari]: "Devanagari",
  [Scheme.Assamese]: "Assamese",
  [Scheme.Balinese]: "Balinese",
  [Scheme.Bengali]: "Bengali",
  [Scheme.Bhaiksuki]: "Bhaiksuki",
  [Scheme.Brahmi]: "Brahmi",
  [Scheme.Burmese]: "Burmese",
  [Scheme.Cham]: "Cham",
  [Scheme.Dogra]: "Dogra",
  [Scheme.Grantha]: "Grantha",
  [Scheme.Gujarati]: "Gujarati",
  [Scheme.GunjalaGondi]: "Gunjala Gondi",
  [Scheme.Gurmukhi]: "Gurmukhi",
  [Scheme.Javanese]: "Javanese",
  [Scheme.Kaithi]: "Kaithi",
  [Scheme.Kannada]: "Kannada",
  [Scheme.Kharoshthi]: "Kharoshthi",
  [Scheme.Khmer]: "Khmer",
  [Scheme.Khudawadi]: "Khudawadi",
  [Scheme.Limbu]: "Limbu",
  [Scheme.Malayalam]: "Malayalam",
  [Scheme.MasaramGondi]: "Masaram Gondi",
  [Scheme.MeeteiMayek]: "Meetei Mayek",
  [Scheme.Modi]: "Modi",
  [Scheme.Mon]: "Mon",
  [Scheme.Nandinagari]: "Nandinagari",
  [Scheme.Newa]: "Newa (Nepal Bhasa)",
  [Scheme.Odia]: "Odia",
  [Scheme.OlChiki]: "Ol Chiki",
  [Scheme.Saurashtra]: "Saurashtra",
  [Scheme.Sharada]: "Sharada",
  [Scheme.Siddham]: "Siddham",
  [Scheme.Sinhala]: "Sinhala",
  [Scheme.Soyombo]: "Soyombo",
  [Scheme.TaiTham]: "Tai Tham",
  [Scheme.Takri]: "Takri",
  [Scheme.Tamil]: "Tamil",
  [Scheme.Telugu]: "Telugu",
  [Scheme.Thai]: "Thai",
  [Scheme.Tibetan]: "Tibetan",
  [Scheme.Tirhuta]: "Tirhuta",
  [Scheme.ZanabazarSquare]: "Zanabazar Square",

  [Scheme.BarahaSouth]: "Baraha (Southern)",
  [Scheme.HarvardKyoto]: "Harvard-Kyoto",
  [Scheme.Iast]: "IAST",
  [Scheme.Iso15919]: "ISO 15919",
  [Scheme.Itrans]: "ITRANS",
  [Scheme.Slp1]: "SLP1",
  [Scheme.Velthuis]: "Velthuis",
  [Scheme.Wx]: "WX",
}

const App = () => ({
  activeTab: null,

  simpleInput: "",
  simpleFrom: "HarvardKyoto",
  simpleTo: "Devanagari",

  multiInput: "",
  multiFrom: "HarvardKyoto",

  detectInput: "",

  async init() {
    this.activeTab = 'simple';
    await this.initVidyut();
  },

  async initVidyut() {
    await init();
    this.vidyut = "initialized";
  },

  tab(s) {
    if (s === this.activeTab) {
      return "font-bold p-2 bg-sky-100 rounded text-sky-800";
    } else {
      return "";
    }
  },

  setTab(tab) {
    this.activeTab = tab;
  },

  // Schemes, mapping from strings to numeric values.
  schemes() {
    let ints = Object.values(Scheme).filter(Number.isInteger);

    let data = [];
    ints.forEach((i) => {
      data.push({ text: schemeNames[i], value: Scheme[i] });
    });
    return data;
  },

  async deva(text) {
    await this.initVidyut();
    return transliterate(text, Scheme.Slp1, Scheme.Devanagari);
  },

  // Simple
  // ------

  simpleOutput() {
    const input = this.simpleInput;
    const from = Scheme[this.simpleFrom];
    const to = Scheme[this.simpleTo];
    if (!this.vidyut) {
      return "";
    }
    return transliterate(input, from, to);
  },

  swapSimple() {
    const oldOutput = this.simpleOutput();
    const oldFrom = this.simpleFrom;
    this.simpleInput = oldOutput;
    this.simpleFrom = this.simpleTo;
    this.simpleTo = oldFrom;
  },

  simpleHighlightOutput(e) {
    e.target.select();
  },

  async simpleCheatSheet() {
    const simpleFrom = this.simpleFrom;
    const simpleTo = this.simpleTo;
    await this.initVidyut();
    let rows = [];
    CHEAT_SHEET.forEach((group) => {
      let row = [];
      group.split(/\s+/).forEach((s) => {
        let from = transliterate(s, Scheme.HarvardKyoto, Scheme[simpleFrom]);
        let to = transliterate(s, Scheme.HarvardKyoto, Scheme[simpleTo]);
        row.push({ from, to })
      });
      rows.push(row);
    });
    return rows;
  },

  // Multi
  // ------

  multiOutput(toScheme) {
    const input = this.multiInput;
    const from = Scheme[this.multiFrom];
    if (!this.vidyut) {
      return "";
    }
    return transliterate(input, from, toScheme);
  },

  // Grid
  // -----

  async soundTable() {
    await this.initVidyut();

    let allTables = [];
    [VOWELS, MARKS, CONSONANTS, symbols].forEach((group) => {
      let table = [];
      orderedSchemes.forEach((toScheme) => {
        let name = schemeNames[toScheme];
        let row = [];
        group.forEach((sound) => {
          let out = transliterate(sound, Scheme.Devanagari, toScheme);
          row.push(out);
        });
        table.push({
          name,
          values: row,
        });
      });
      allTables.push(table);
    });

    return allTables;
  },

  // Detect
  // ------

  detectResult() {
    const input = this.detectInput;
    if (!this.vidyut || input === "") {
      return "(none)";
    }
    // TODO: consider filing alpine.js bug for when logic is like:
    //
    // if (input === "") {
    //   return "(none)";
    // } else {
    //   let scheme = ...;
    //   return ...;
    // }
    let scheme = detect(input);
    return Scheme[scheme];
  }

});

window.Scheme = Scheme;

// Initialize the app.
window.addEventListener('alpine:init', async () => {
  Alpine.data("app", App)
});
