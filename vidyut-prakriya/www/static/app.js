import init, { Krt, Vidyut, Gana, Lakara, Prayoga, Purusha, Vacana, Pada, Sanadi, Linga, Vibhakti } from "/static/wasm/vidyut_prakriya.js";

// Krts that create ordinary nouns.
const NOMINAL_KRTS = [
    Krt.GaY,
    Krt.lyuw,
    Krt.Rvul,
];

// Krts that are generally called *participles*.
const PARTICIPLE_KRTS = [
    Krt.tavya,
    Krt.anIyar,
    Krt.yat,
    Krt.Ryat,

    Krt.Satf,
    Krt.SAnac,

    Krt.kta,
    Krt.ktavatu,

    Krt.kvasu,
    Krt.kAnac,
];

// Krts that create avyayas.
const AVYAYA_KRTS = [
    Krt.tumun,
    Krt.ktvA,
];

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

function setParam(url, key, value) {
    if (value) {
        url.searchParams.set(key, value);
    } else {
        url.searchParams.delete(key);
    }
}

function createKrdantasFrom(vidyut, dhatu, krtList) {
    let results = [];

    krtList.forEach((krt) => {
        let padas = [];
        const prakriyas = vidyut.derive_krdantas(
            dhatu.code,
            krt
        );
        prakriyas.forEach((p) => {
            padas.push({
                text: p.text,
                type: "krt",
                dhatu,
                krt,
            });
        });
        results.push({
            title: Krt[krt],
            padas,
        });
    });
    return results;
}

function removeSlpSvaras(s) {
    return s.replaceAll(/[\^\\]/g, '');
}

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

async function loadVidyut() {
    await init();

    const resp = await fetch("/static/data/dhatupatha.tsv");
    const text = await resp.text();

    return {
        vidyut: Vidyut.init(text),
        dhatus: parseDhatus(text),
    }
}

const App = () => ({
    ganas: Gana,
    lakaras: Lakara,
    prayogas: Prayoga,
    purushas: Purusha,
    vacanas: Vacana,

    activeTab: 'tin',

    // All dhatus.
    dhatus: [],
    // The selected dhatu.
    activeDhatu: null,
    // The selected pada for the selected dhatu.
    activePada: null,
    // The prakriya for the selected pada.
    prakriya: null,

    // UI options
    // ----------
    // The desired prayoga.
    prayoga: null,
    // The desired sanAdi pratyaya.
    sanadi: null,
    // A filter to apply to the dhatu list.
    dhatuFilter: null,

    async init() {
        const data = await loadVidyut();
        this.vidyut = data.vidyut;
        this.dhatus = data.dhatus;

        // TODO: set state earlier. But, our current implemenation needs to
        // wait for the dhatus to load so that we can set activeDhatu.
        this.readUrlState();

        this.$watch('activeDhatu', (value) => {
            this.updateUrlState();
        });
        this.$watch('tab', (value) => {
            this.updateUrlState();
        });
    },

    // Mutators

    readUrlState() {
        const params = new URLSearchParams(window.location.search);
        const dhatuCode = params.get('dhatu');
        const tab = params.get('tab');
        if (tab) {
            this.setTab(tab);
        }
        if (dhatuCode) {
            this.setActiveDhatu(dhatuCode);
        }
    },

    updateUrlState() {
        const url = new URL(window.location.href);
        let dhatuCode = null;
        if (this.activeDhatu) {
            dhatuCode = this.activeDhatu.code;
        }
        setParam(url, "dhatu", dhatuCode);
        setParam(url, "tab", this.activeTab);
        // TODO: sanadi, prayoga, etc.

        history.replaceState(null, document.title, url.toString());
    },

    setActiveDhatu(s) {
        this.activeDhatu = this.dhatus.find(d => d.code === s);
        // Scroll position might be off if the user has scrolled far down the dhatu list.
        window.scrollTo({ top: 0 });
    },

    setActivePada(p) {
        this.activePada = p;
        this.prakriya = this.createPrakriya();
        window.scrollTo({ top: 0 });
    },

    clearActivePada() {
        this.activePada = null;
        this.prakriya = null;
    },

    clearActiveDhatu() {
        // Breaks if we clear `activeDhatu` last -- not sure why. So, clear it first.
        this.activeDhatu = null;
        this.tinantas = null;
        this.clearActivePada();
    },

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
            console.log('filter is ', filter);
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

    createPrakriya() {
        if (!this.activePada) {
            return null;
        }

        const pada = this.activePada;
        let allPrakriyas = [];
        if (pada.type === "tin") {
            allPrakriyas = this.vidyut.derive_tinantas(
                pada.dhatu.code,
                pada.lakara,
                pada.prayoga,
                pada.purusha,
                pada.vacana,
                null,
                pada.sanadi,
            );
        } else if (pada.type === "krt") {
            allPrakriyas = this.vidyut.derive_krdantas(
                pada.dhatu.code,
                pada.krt
            );
        }

        return allPrakriyas.find((p) => p.text == pada.text);
    },

    deva(s) {
        return Sanscript.t(s, 'slp1', 'devanagari');
    },

    devaNoSvara(s) {
        return Sanscript.t(removeSlpSvaras(s), 'slp1', 'devanagari');
    },

    async sutraText(rule) {
        const text = (await sutras)[rule];
        return text ? this.deva(text) : '';
    },

    entryString(entries) {
        let str = entries.map(x => x.text).join(', ');
        return this.deva(str);
    },

    createParadigm(args) {
        const { dhatu, lakara, prayoga, pada, sanadi } = args;

        let purushas = Object.values(Purusha).filter(Number.isInteger);
        let vacanas = Object.values(Vacana).filter(Number.isInteger);

        let paradigm = [];
        for (const purusha in purushas) {
            for (const vacana in vacanas) {
                let prakriyas = this.vidyut.derive_tinantas(
                    dhatu.code,
                    lakara,
                    prayoga,
                    purusha,
                    vacana,
                    pada,
                    sanadi,
                );

                let pvPadas = [];
                let seen = new Set();
                prakriyas.forEach((p) => {
                    if (seen.has(p.text)) {
                        return;
                    }
                    seen.add(p.text);

                    pvPadas.push({
                        text: p.text,
                        type: "tin",
                        dhatu,
                        lakara,
                        prayoga,
                        purusha,
                        vacana,
                        pada,
                        sanadi,
                    });
                });

                if (pvPadas.length === 0) {
                    return [];
                }

                paradigm.push(pvPadas);
            }
        }

        return paradigm;
    },

    createKrdantas() {
        if (this.activeDhatu === null) {
            return [];
        }

        const dhatu = this.activeDhatu;
        return [
            createKrdantasFrom(this.vidyut, dhatu, NOMINAL_KRTS),
            createKrdantasFrom(this.vidyut, dhatu, PARTICIPLE_KRTS),
            createKrdantasFrom(this.vidyut, dhatu, AVYAYA_KRTS),
        ];
    },

    createTinantas() {
        if (this.activeDhatu === null) {
            return [];
        }

        const dhatu = this.activeDhatu;
        const lakaras = Object.values(Lakara).filter(Number.isInteger);
        const tinPadas = Object.values(Pada).filter(Number.isInteger);
        const prayoga = this.prayoga !== null ? this.prayoga : Prayoga.Kartari;
        const sanadi = this.sanadi || null;;

        let results = [];
        for (const lakara in lakaras) {
            let laResults = {
                title: Lakara[lakara],
            };

            for (const tinPada in tinPadas) {
                const padaKey = Pada[tinPada];
                const paradigm = this.createParadigm({
                    dhatu,
                    lakara,
                    prayoga,
                    pada: tinPada,
                    sanadi,
                });

                if (paradigm.length !== 0) {
                    laResults[padaKey] = paradigm;
                }
            }
            results.push(laResults);
        }

        return results;
    },
});

window.Lakara = Lakara;
window.Prayoga = Prayoga;
window.Sanadi = Sanadi;

window.addEventListener('alpine:init', () => {
    Alpine.data("app", App)
});
