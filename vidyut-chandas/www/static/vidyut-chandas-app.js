import init, { Chandas } from "/static/wasm/vidyut_chandas.js";

const vrttas = fetch("/static/data/meters.tsv").then(resp => resp.text());

const App = () => ({
  // The tab to display.
  activeTab: "demo",

  // Page transliteration script (devanagari, iast, telugu, etc.)
  script: 'devanagari',

  // App transliteration scripts.
  from: "hk",
  to: "devanagari",

  // The user's text.
  text: null,

  // The results.
  results: {},

  async init() {
    // Initialize WASM environment.
    await init();
    const vrttasText = await vrttas;
    this.chandas = Chandas.init(vrttasText);

    this.$watch('text', (text) => this.classify(text));
  },

  // Render the given SLP1 text in Devanagari.
  deva(s) {
    return Sanscript.t(s, 'slp1', this.script);
  },

  trans(s) {
    return Sanscript.t(s, this.from, this.to);
  },

  transSLP1(s) {
    return Sanscript.t(s, 'slp1', this.to);
  },

  // Computed properties

  schemes() {
    // Using viydut-lipi is too tedious here, so just use Sanscript schemes.
    return [
      { value: "itrans", text: "ITRANS" },
      { value: "hk", text: "Harvard-Kyoto" },
      { value: "slp1", text: "SLP1" },
      { value: "iast", text: "IAST" },
      { value: "devanagari", text: "Devanagari" },
      { value: "kannada", text: "Kannada" },
      { value: "malayalam", text: "Malayalam" },
      { value: "telugu", text: "Telugu" },
    ]
  },

  akshara(a) {
    if (a.weight === "G") {
      return "bg-slate-300";
    }
  },

  tab(s) {
    if (s === this.activeTab) {
      return "font-bold p-2 bg-sky-100 rounded text-sky-800";
    } else {
      return "";
    }
  },

  classify(text) {
    if (text) {
      const slp1 = Sanscript.t(text, this.from, 'slp1');
      this.results = this.chandas.classify(slp1);
    } else {
      this.results = {};
    }

    console.log("result:", this.results);
  },

});

// Initialize the app.
window.addEventListener('alpine:init', () => {
  Alpine.data("app", App)
});
