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

import init, { VidyutLipi as VidyutWasm, Scheme } from "/static/wasm/vidyut_lipi.js";

// ===================================================
// vidyut-lipi
// ===================================================

class Vidyut {
  // Call `init()` before calling this so that you initialize the WASM environment.
  constructor() {
    this.wasm = VidyutWasm.init();
    console.log("Constructed Vidyut.");
  }

  transliterate(input, from, to) {
    return this.wasm.transliterate(input, from, to);
  }
}

// ===================================================
// App
// ===================================================

let vowels = [
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

let marks = [
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

let consonants = [
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

let schemes = [
  Scheme.Devanagari,
  Scheme.Bengali,
  Scheme.Brahmi,
  Scheme.Grantha,
  Scheme.Gujarati,
  Scheme.Gurmukhi,
  Scheme.Kannada,
  Scheme.Malayalam,
  Scheme.Oriya,
  Scheme.Sinhala,
  Scheme.Tamil,
  Scheme.Telugu,
  Scheme.Tibetan,

  Scheme.HarvardKyoto,
  Scheme.Iast,
  Scheme.Itrans,
  Scheme.Slp1,
  Scheme.Velthuis,
];

let schemeNames = {
  [Scheme.Devanagari]: "Devanagari",
  [Scheme.Bengali]: "Bengali",
  [Scheme.Brahmi]: "Brahmi",
  [Scheme.Grantha]: "Grantha",
  [Scheme.Gujarati]: "Gujarati",
  [Scheme.Gurmukhi]: "Gurmukhi",
  [Scheme.Kannada]: "Kannada",
  [Scheme.Malayalam]: "Malayalam",
  [Scheme.Oriya]: "Oriya",
  [Scheme.Sinhala]: "Sinhala",
  [Scheme.Tamil]: "Tamil",
  [Scheme.Telugu]: "Telugu",
  [Scheme.Tibetan]: "Tibetan",

  [Scheme.HarvardKyoto]: "Harvard-Kyoto",
  [Scheme.Iast]: "IAST",
  [Scheme.Itrans]: "ITRANS",
  [Scheme.Slp1]: "SLP1",
  [Scheme.Velthuis]: "Velthuis",
}

const App = () => ({
  activeTab: null,

  simpleInput: "",
  simpleFrom: "HarvardKyoto",
  simpleTo: "Devanagari",

  multiInput: "",
  multiFrom: "HarvardKyoto",

  async init() {
    this.activeTab = 'simple';
    await this.initVidyut();
  },

  async initVidyut() {
    await init();
    this.vidyut = new Vidyut();
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

  simpleOutput() {
    const input = this.simpleInput;
    const from = Scheme[this.simpleFrom];
    const to = Scheme[this.simpleTo];
    if (!this.vidyut) {
      return "";
    }
    return this.vidyut.transliterate(input, from, to);
  },

  multiOutput(toScheme) {
    const input = this.multiInput;
    const from = Scheme[this.multiFrom];
    if (!this.vidyut) {
      return "";
    }
    return this.vidyut.transliterate(input, from, toScheme);
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
    return this.vidyut.transliterate(text, Scheme.Slp1, Scheme.Devanagari);
  },

  async soundTable() {
    await this.initVidyut();
    let allTables = [];
    [vowels, marks, consonants, symbols].forEach((group) => {
      let table = [];
      schemes.forEach((toScheme) => {
        let name = schemeNames[toScheme];
        let row = [];
        group.forEach((sound) => {
          let out = this.vidyut.transliterate(sound, Scheme.Devanagari, toScheme);
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
  }
});

window.Scheme = Scheme;
window.Vidyut = Vidyut;

// Initialize the app.
window.addEventListener('alpine:init', async () => {
  Alpine.data("app", App)
});
