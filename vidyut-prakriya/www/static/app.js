/**
 * A simple demo interface for vidyut-prakriya.
 *
 *
 * Outline
 * =======
 *
 * Focus on `App`, which contains the main code. We use the Alpine framework,
 * which you can think of as a lightweight version of Vue.
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

import init, { BaseKrt, Vidyut as VidyutWasm, Gana, Lakara, Prayoga, Purusha, Vacana, DhatuPada, Sanadi, Linga, Vibhakti } from "/static/wasm/vidyut_prakriya.js";

// ===================================================
// vidyut-prakriya
// ===================================================

// Turn the TSV file sutrapatha.tsv into a map.
function parseSutras(tsv) {
    let sutras = {};
    tsv.split(/\r?\n/).forEach(line => {
        const [id, text] = line.split(/\t/);
        sutras[id] = text;
    });
    return sutras;
}
const sutras = fetch("/static/data/sutrapatha.tsv").then(resp => resp.text()).then(text => parseSutras(text));

// Parse a dhatupatha string into separate objects.
function parseDhatus(text) {
    let dhatus = [];
    text.split(/\r?\n/).forEach((line) => {
        const [code, upadesha, artha] = line.split(/\t/);
        if (!!code && code !== 'code') {
            dhatus.push({
                code,
                upadesha,
                upadeshaQuery: removeSlpSvaras(upadesha),
                artha
            });
        }
    });
    return dhatus;
}

class Vidyut {
    // Call `init()` before calling this so that you initialize the WASM environment.
    constructor(dhatupatha) {
        this.wasm = VidyutWasm.init(dhatupatha);
        this.dhatus = parseDhatus(dhatupatha);
        console.log("constructed Vidyut.");
    }


    deriveTinantas(tinanta) {
        // For argument order, see wasm.rs.
        return this.wasm.deriveTinantas(
            tinanta.dhatu.code,
            tinanta.lakara,
            tinanta.prayoga,
            tinanta.purusha,
            tinanta.vacana,
            tinanta.pada,
            tinanta.sanadi || null,
            tinanta.upasarga || null,
        );
    }

    deriveSubantas(subanta) {
        // For argument order, see wasm.rs.
        return this.wasm.deriveSubantas(
            subanta.pratipadika,
            subanta.linga,
            subanta.vibhakti,
            subanta.vacana,
        );
    }

    deriveKrdantas(krdanta) {
        // For argument order, see wasm.rs.
        return this.wasm.deriveKrdantas(
            krdanta.dhatu.code,
            krdanta.krt,
            krdanta.sanadi || null,
            krdanta.upasarga || null,
        )
    }
}

// ===================================================
// Frontend
// ===================================================

// Krts that create ordinary nouns.
const NOMINAL_KRTS = [
    BaseKrt.GaY,
    BaseKrt.lyuw,
    BaseKrt.Rvul,
    BaseKrt.tfc,
    BaseKrt.kvip,
];

// Krts that are generally called *participles*.
const PARTICIPLE_KRTS = [
    BaseKrt.tavya,
    BaseKrt.anIyar,
    BaseKrt.yat,
    BaseKrt.Ryat,

    BaseKrt.Satf,
    BaseKrt.SAnac,

    BaseKrt.kta,
    BaseKrt.ktavatu,

    BaseKrt.kvasu,
    BaseKrt.kAnac,
];

// Krts that create avyayas.
const AVYAYA_KRTS = [
    BaseKrt.tumun,
    BaseKrt.ktvA,
    BaseKrt.Ramul,
];

// What to call these params in the URL.
const Params = {
    Dhatu: "dhatu",
    Tab: "tab",
    DhatuPada: "pada",
    Prayoga: "prayoga",
    Sanadi: "sanadi",
    ActivePada: "activePada",
    Upasarga: "upasarga",
}

function setParam(url, key, value) {
    if (value) {
        url.searchParams.set(key, value);
    } else {
        url.searchParams.delete(key);
    }
}

function removeSlpSvaras(s) {
    return s.replaceAll(/[\^\\]/g, '');
}

const App = () => ({
    activeTab: 'about',

    // Dhatus
    // ------
    dhatus: [],
    // The selected dhatu.
    activeDhatu: null,
    // The selected pada for the selected dhatu.
    activePada: null,
    // The prakriya for the selected pada.
    dhatuPrakriya: null,

    // Subantas
    // --------
    supActivePratipadika: null,
    supParadigm: null,
    supPrakriya: null,
    // A filter to apply to the sup list.
    supFilter: null,

    // UI options
    // ----------
    // The desired prayoga.
    prayoga: null,
    // The desired upasarga.
    upasarga: null,
    // The desired sanAdi pratyaya.
    sanadi: null,
    // A filter to apply to the dhatu list.
    dhatuFilter: null,

    // data
    sutras: {},

    // Transliteration script (devanagari, iast, telugu, etc.)
    script: 'devanagari',

    async init() {
        // Initialize WASM environment.
        await init();

        const resp = await fetch("/static/data/dhatupatha.tsv");
        const dhatupatha = await resp.text();

        // Vidyut needs its own copy of the dhatupatha.
        this.vidyut = new Vidyut(dhatupatha);
        console.log("initialized vidyut-prakriya WASM bindings.");

        this.dhatus = this.vidyut.dhatus;

        // TODO: set state earlier. But, our current implemenation needs to
        // wait for the dhatus to load so that we can set activeDhatu.
        this.readUrlState();

        // Save important properties to the URL when they change.
        this.$watch('activeDhatu', (value) => {
            this.updateUrlState();
        });
        this.$watch('tab', (value) => {
            this.updateUrlState();
        });
        this.$watch('sanadi', (value) => {
            this.updateUrlState();
        });
        this.$watch('prayoga', (value) => {
            this.updateUrlState();
        });
        this.$watch('upasarga', (value) => {
            this.updateUrlState();
        });
        this.$watch('activePada', (value) => {
            this.updateUrlState();
        });

        this.sutras = await sutras;
    },

    // Mutators

    // Load the application state from the URL, if applicable.
    readUrlState() {
        const params = new URLSearchParams(window.location.search);
        const dhatuCode = params.get(Params.Dhatu);
        const tab = params.get(Params.Tab);
        const prayoga = params.get(Params.Prayoga);
        const upasarga = params.get(Params.Upasarga);
        const sanadi = params.get(Params.Sanadi);
        const activePada = params.get(Params.ActivePada);

        console.log(`realUrlState, prayoga=${prayoga}, upasarga=${upasarga}, sanadi=${sanadi}`);
        if (tab) {
            this.setTab(tab);
        }
        if (prayoga) {
            this.prayoga = parseInt(prayoga);
        }
        if (upasarga) {
            this.upasarga = upasarga;
        }
        if (sanadi) {
            this.sanadi = parseInt(sanadi);
        }
        if (dhatuCode) {
            this.setActiveDhatu(dhatuCode);
        }
        if (activePada) {
            this.setActivePada(JSON.parse(activePada));
        }
    },

    // Encode the current application state in the URL so that it can be
    // referenced later.
    updateUrlState() {
        const url = new URL(window.location.href);
        let dhatuCode = null;
        if (this.activeDhatu) {
            dhatuCode = this.activeDhatu.code;
        }
        setParam(url, Params.Dhatu, dhatuCode);
        setParam(url, Params.Tab, this.activeTab);
        setParam(url, Params.Prayoga, this.prayoga);
        setParam(url, Params.Sanadi, this.sanadi);
        setParam(url, Params.Upasarga, this.upasarga);
        if (this.activePada) {
            setParam(url, Params.ActivePada, JSON.stringify(this.activePada));
        } else {
            setParam(url, Params.ActivePada, null);
        }

        console.log("updateUrlState to: ", url.href);
        history.replaceState(null, document.title, url.toString());
    },

    // Set the active dhatu (and show its forms)
    setActiveDhatu(s) {
        this.activeDhatu = this.dhatus.find(d => d.code === s);
        // Scroll position might be off if the user has scrolled far down the dhatu list.
        window.scrollTo({ top: 0 });
    },

    // Set the active pada (and show its prakriya)
    setActivePada(p) {
        this.activePada = p;
        if (p.type === "subanta") {
            this.supPrakriya = this.createPrakriya();
            console.log(this.supPrakriya);
        } else {
            this.dhatuPrakriya = this.createPrakriya();
        }
        window.scrollTo({ top: 0 });
    },

    // Create the active pada (and show all forms for the dhatu)
    clearActivePada() {
        this.activePada = null;
        this.dhatuPrakriya = null;
    },

    // Clear the active dhatu (and show the full dhatu list).
    clearActiveDhatu() {
        // Breaks if we clear `activeDhatu` last -- not sure why. So, clear it first.
        this.activeDhatu = null;
        this.tinantas = null;
        this.sanadi = null;
        this.prayoga = null;
        this.clearActivePada();
    },

    // Set the app's active tab.
    setTab(s) {
        // Reset the prakriya so that we don't display a krt pratyaya for tin, etc.
        // The proper fix is to have separate prakriyas for each tab.
        this.clearActivePada();
        this.activeTab = s;
    },

    // Computed properties

    tab(s) {
        if (s === this.activeTab) {
            return "font-bold p-2 bg-sky-100 rounded text-sky-800";
        } else {
            return "";
        }
    },

    /** A filtered list of dhatus according to a user query. */
    filteredDhatus() {
        if (this.dhatuFilter !== null) {
            let filter = Sanscript.t(this.dhatuFilter, 'devanagari', 'slp1');
            let hkFilter = Sanscript.t(this.dhatuFilter, 'hk', 'slp1');
            return this.dhatus.filter(d =>
                d.code.includes(filter)
                || d.upadeshaQuery.includes(filter)
                || d.artha.includes(filter)
                || d.upadeshaQuery.includes(hkFilter)
                || d.artha.includes(hkFilter)
            );
        } else {
            return this.dhatus;
        }
    },

    filteredSupPratipadikas() {
        if (this.supFilter !== null) {
            const slpFilter = Sanscript.t(this.supFilter, 'devanagari', 'slp1');
            const hkFilter = Sanscript.t(this.supFilter, 'hk', 'slp1');
            return this.supPratipadikas().filter(s =>
                s.text.includes(slpFilter) || s.text.includes(hkFilter)
            );
        } else {
            return this.supPratipadikas();
        }
    },

    createPrakriya() {
        if (!this.activePada) {
            return null;
        }

        const pada = this.activePada;
        let allPrakriyas = [];
        if (pada.type === "tinanta") {
            allPrakriyas = this.vidyut.deriveTinantas(pada.args);
        } else if (pada.type === "krdanta") {
            allPrakriyas = this.vidyut.deriveKrdantas(pada.args);
        } else if (pada.type === "subanta") {
            allPrakriyas = this.vidyut.deriveSubantas(pada.args);
        }

        return allPrakriyas.find((p) => p.text == pada.text);
    },

    // Render the given SLP1 text in Devanagari.
    deva(s) {
        return Sanscript.t(s, 'slp1', this.script);
    },

    // Render the given SLP1 text in Devanagari without svara marks.
    devaNoSvara(s) {
        return Sanscript.t(removeSlpSvaras(s), 'slp1', this.script);
    },

    sutraText(rule) {
        const text = this.sutras[rule];
        return text ? this.deva(text) : '';
    },

    entryString(entries) {
        let str = entries.map(x => x.text).join(', ');
        return this.deva(str);
    },

    stepClasses(step) {
        const code = step.rule;
        let minor = new Set(["1.3.1", "1.3.2", "1.3.3", "1.3.4", "1.3.5", "1.3.6", "1.3.7", "1.3.8", "1.3.9", "1.2.45", "1.2.46", "3.4.114", "1.1.43", "1.4.14",
            "1.4.58", "1.4.59", "1.4.60", "1.4.80", "3.1.32", "6.1.4", "6.1.5", "8.4.68", "3.4.113", "2.3.48", "1.4.17", "2.3.49", "1.4.7",
        ]);
        if (minor.has(code)) {
            return ["opacity-40"];
        } else {
            return [];
        }
    },

    renderStepResult(step) {
        let res = "";
        step.result.forEach((val, i) => {
            if (res.length !== 0) {
                res += ' <span class="text-sm text-gray-400">+</span> ';
            }
            let text = Sanscript.t(removeSlpSvaras(val), 'slp1', this.script);
            if (i === step.active) {
              text = `<span class="text-red-700">${text}</span>`
            }
            res += text;
        })
        return res;
    },

    /// Create all tinantas allowed by the given `args`.
    createTinantaParadigm(args) {
        const { dhatu, lakara, prayoga, pada, sanadi, upasarga } = args;

        let purushas = Object.values(Purusha).filter(Number.isInteger);
        let vacanas = Object.values(Vacana).filter(Number.isInteger);

        let paradigm = [];
        for (const purusha in purushas) {
            let row = [];
            for (const vacana in vacanas) {
                const args = {
                    dhatu,
                    lakara,
                    prayoga,
                    purusha,
                    vacana,
                    pada,
                    sanadi,
                    upasarga,
                };
                let prakriyas = this.vidyut.deriveTinantas(args);

                let cell = [];
                let seen = new Set();
                prakriyas.forEach((p) => {
                    if (seen.has(p.text)) {
                        return;
                    }
                    seen.add(p.text);
                    cell.push({
                        text: p.text,
                        type: "tinanta",
                        args
                    });
                });

                if (cell.length === 0) {
                    return [];
                }

                row.push(cell);
            }
            paradigm.push(row);
        }

        return paradigm;
    },

    // Get a nice human-readable name for the given lakara.
    getLakaraTitle(value) {
        const mapping = {
            "Lat": "law",
            "Lit": "liw",
            "Lut": "luw",
            "Lrt": "lfw",
            "Let": "lew",
            "Lot": "low",
            "Lan": "laN",
            "VidhiLin": "viDi-liN",
            "AshirLin": "ASIr-liN",
            "Lun": "luN",
            "Lrn": "lfN",
        };
        const text = mapping[Lakara[value]];
        return this.deva(text);
    },

    getLingaTitle(value) {
        const mapping = {
            [Linga.Pum]: "puMliNgaH",
            [Linga.Stri]: "strIliNgaH",
            [Linga.Napumsaka]: "napuMsakaliNgaH",
        };
        const text = mapping[value];
        return this.deva(text || "missing");
    },

    supPratipadikas() {
        return [
            {text: "a", linga: Linga.Pum},
            {text: "deva", linga: Linga.Pum},
            {text: "rAma", linga: Linga.Pum},
            {text: "sarva", linga: Linga.Pum},
            {text: "viSva", linga: Linga.Pum},
            {text: "i", linga: Linga.Pum},
            {text: "kavi", linga: Linga.Pum},
            {text: "hari", linga: Linga.Pum},
            {text: "kavi", linga: Linga.Pum},
            {text: "hari", linga: Linga.Pum},
            {text: "saKi", linga: Linga.Pum},
            {text: "pati", linga: Linga.Pum},
            {text: "tri", linga: Linga.Pum},
            {text: "u", linga: Linga.Pum},
            {text: "SamBu", linga: Linga.Pum},
            {text: "guru", linga: Linga.Pum},
            {text: "krozwu", linga: Linga.Pum},
            {text: "hUhU", linga: Linga.Pum},
            {text: "pitf", linga: Linga.Pum},
            {text: "jAmAtf", linga: Linga.Pum},
            {text: "BrAtf", linga: Linga.Pum},
            {text: "go", linga: Linga.Pum},
            {text: "rE", linga: Linga.Pum},
            {text: "glO", linga: Linga.Pum},
            {text: "janO", linga: Linga.Pum},
            {text: "rAjan", linga: Linga.Pum},
            {text: "yajvan", linga: Linga.Pum},
            {text: "aryaman", linga: Linga.Pum},
            {text: "brahman", linga: Linga.Pum},
            {text: "SarNgin", linga: Linga.Pum},
            {text: "Atman", linga: Linga.Pum},
            {text: "guRin", linga: Linga.Pum},
            {text: "Svan", linga: Linga.Pum},
            {text: "paTin", linga: Linga.Pum},
            {text: "yaSasvin", linga: Linga.Pum},
            {text: "pUzan", linga: Linga.Pum},
            {text: "yuvan", linga: Linga.Pum},
            {text: "maTin", linga: Linga.Pum},
            {text: "arvan", linga: Linga.Pum},
            {text: "fBukzin", linga: Linga.Pum},
            {text: "ftvij", linga: Linga.Pum},
            {text: "tyad", linga: Linga.Pum},
            {text: "tad", linga: Linga.Pum},
            {text: "yad", linga: Linga.Pum},
            {text: "etad", linga: Linga.Pum},
            {text: "yuzmad", linga: Linga.Pum},
            {text: "asmad", linga: Linga.Pum},
            {text: "kim", linga: Linga.Pum},
            {text: "idam", linga: Linga.Pum},
            {text: "adas", linga: Linga.Pum},

            {text: "u", linga: Linga.Stri},
            {text: "tad", linga: Linga.Stri},
            {text: "yad", linga: Linga.Stri},
            {text: "etad", linga: Linga.Stri},
            {text: "kim", linga: Linga.Stri},
            {text: "idam", linga: Linga.Stri},
            {text: "adas", linga: Linga.Stri},
            {text: "svasf", linga: Linga.Stri},
            {text: "mAtf", linga: Linga.Stri},
            {text: "duhitf", linga: Linga.Stri},
            {text: "go", linga: Linga.Stri},
            {text: "dyo", linga: Linga.Stri},
            {text: "rE", linga: Linga.Stri},
            {text: "nO", linga: Linga.Stri},

            {text: "Pala", linga: Linga.Napumsaka},
            {text: "puzpa", linga: Linga.Napumsaka},
            {text: "sarva", linga: Linga.Napumsaka},
            {text: "viSva", linga: Linga.Napumsaka},
            {text: "eka", linga: Linga.Napumsaka},
            {text: "pUrva", linga: Linga.Napumsaka},
            {text: "para", linga: Linga.Napumsaka},
            {text: "avara", linga: Linga.Napumsaka},
            {text: "dakziRa", linga: Linga.Napumsaka},
            {text: "uttara", linga: Linga.Napumsaka},
            {text: "apara", linga: Linga.Napumsaka},
            {text: "aDara", linga: Linga.Napumsaka},
            {text: "hfdaya", linga: Linga.Napumsaka},
            {text: "asTi", linga: Linga.Napumsaka},
            {text: "sakTi", linga: Linga.Napumsaka},
            {text: "akzi", linga: Linga.Napumsaka},
            {text: "maDu", linga: Linga.Napumsaka},
            {text: "ambu", linga: Linga.Napumsaka},
            {text: "vasu", linga: Linga.Napumsaka},
            {text: "aSru", linga: Linga.Napumsaka},
            {text: "laGu", linga: Linga.Napumsaka},
            {text: "bahu", linga: Linga.Napumsaka},
            {text: "vastu", linga: Linga.Napumsaka},
            {text: "pIlu", linga: Linga.Napumsaka},
            {text: "sAnu", linga: Linga.Napumsaka},
            {text: "yakft", linga: Linga.Napumsaka},
            {text: "tad", linga: Linga.Napumsaka},
            {text: "yad", linga: Linga.Napumsaka},
            {text: "etad", linga: Linga.Napumsaka},
            {text: "kim", linga: Linga.Napumsaka},
            {text: "idam", linga: Linga.Napumsaka},
            {text: "adas", linga: Linga.Napumsaka},
        ];
    },

    setPratipadika(pratipadika) {
        this.supActivePratipadika = pratipadika;
        this.supParadigm = this.createSubantaParadigm();
    },

    clearSupPratipadika() {
        this.supParadigm = null;
        this.supActivePratipadika = null;
    },

    createSubantaParadigm() {
        const vibhaktis = Object.values(Vibhakti).filter(Number.isInteger);
        const vacanas = Object.values(Vacana).filter(Number.isInteger);
        const pratipadika = this.supActivePratipadika;

        const vibhaktiTitles = {
            [Vibhakti.Prathama]: "praTamA",
            [Vibhakti.Dvitiya]: "dvitIyA",
            [Vibhakti.Trtiya]: "tftIyA",
            [Vibhakti.Caturthi]: "caturTI",
            [Vibhakti.Panchami]: "paYcamI",
            [Vibhakti.Sasthi]: "zazWI",
            [Vibhakti.Saptami]: "saptamI",
            [Vibhakti.Sambodhana]: "samboDanam",
        };

        let paradigm = [];
        vibhaktis.forEach((vibhakti) => {
            let row = [];
            vacanas.forEach((vacana) => {
                const args = {
                    pratipadika: pratipadika.text,
                    linga: pratipadika.linga,
                    vibhakti: vibhakti,
                    vacana: vacana,
                };
                const prakriyas = this.vidyut.deriveSubantas(args);

                let vvPadas = [];
                prakriyas.forEach((p) => {
                    let text = p.text;
                    let displayText = text;
                    // Use `==` for int/string cast.
                    if (vibhakti == Vibhakti.Sambodhana) {
                        displayText = "he " + text;
                    }
                    vvPadas.push({
                        displayText,
                        text,
                        type: "subanta",
                        args,
                    })
                })
                row.push(vvPadas);
            });

            paradigm.push({
                title: vibhaktiTitles[vibhakti],
                cells: row,
            });
        });

        return paradigm;
    },


    /// Creates a grid of krdantas to show for a specific dhatu.
    createKrdantas() {
        if (this.activeDhatu === null) {
            return [];
        }

        const dhatu = this.activeDhatu;
        const upasarga = this.upasarga;
        const sanadi = this.sanadi;

        let ret = [];
        [NOMINAL_KRTS, PARTICIPLE_KRTS, AVYAYA_KRTS].forEach((list) => {
            let table = [];
            list.forEach((krt) => {
                const args = {
                    dhatu,
                    krt,
                    upasarga,
                    sanadi,
                };

                const prakriyas = this.vidyut.deriveKrdantas(args)
                let padas = [];
                prakriyas.forEach((p) => {
                    padas.push({
                        text: p.text,
                        type: "krdanta",
                        args
                    });
                });

                table.push({
                    title: BaseKrt[krt],
                    padas,
                });
            });
            ret.push(table);
        });
        return ret;
    },

    createAllTinantas() {
        if (this.activeDhatu === null) {
            return [];
        }

        const dhatu = this.activeDhatu;
        const lakaras = Object.values(Lakara).filter(Number.isInteger);
        const tinPadas = Object.values(DhatuPada).filter(Number.isInteger);
        const prayoga = this.prayoga !== null ? this.prayoga : Prayoga.Kartari;
        const sanadi = this.sanadi || null;;
        const upasarga = this.upasarga || null;;

        console.log("createAllTinantas", prayoga, this.sanadi, upasarga);
        let results = [];
        for (const lakara in lakaras) {
            let laResults = {
                title: this.getLakaraTitle(lakara),
            };

            for (const tinPada in tinPadas) {
                const padaKey = DhatuPada[tinPada];
                const paradigm = this.createTinantaParadigm({
                    dhatu,
                    lakara,
                    prayoga,
                    pada: tinPada,
                    sanadi,
                    upasarga,
                });

                if (paradigm.length !== 0) {
                    laResults[padaKey] = paradigm;
                }
            }
            results.push(laResults);
        }

        console.log(results);
        return results;
    },
});

window.Lakara = Lakara;
window.Prayoga = Prayoga;
window.Sanadi = Sanadi;

// Initialize the app.
window.addEventListener('alpine:init', () => {
    Alpine.data("app", App)
});
