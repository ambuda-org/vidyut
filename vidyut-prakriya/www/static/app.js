import init, { Vidyut, Gana, Lakara, Prayoga, Purusha, Vacana, Pada, Sanadi } from "/static/wasm/vidyut_prakriya.js";


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
    },

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

    setActiveDhatu(s) {
        this.activeDhatu = this.dhatus.find(d => d.code === s);
        // Scroll position might be off if the user has scrolled far down the dhatu list.
        window.scrollTo({ top: 0 });
    },

    setActivePada(p) {
        this.activePada = p;
        this.prakriya = this.createPrakriya();
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

    createPrakriya() {
        if (!this.activePada) {
            return null;
        }

        const pada = this.activePada;
        let allPrakriyas = this.vidyut.derive(
            pada.dhatu.code,
            pada.lakara,
            pada.prayoga,
            pada.purusha,
            pada.vacana,
            null,
            pada.sanadi,
        );

        return allPrakriyas.find((p) => p.text == pada.text);
    },

    show(s) {
        this.activeTab = s;
    },

    deva(s) {
        return Sanscript.t(s, 'slp1', 'devanagari');
    },

    devaNoSvara(s) {
        return Sanscript.t(removeSlpSvaras(s), 'slp1', 'devanagari');
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
                let prakriyas = this.vidyut.derive(
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
                    console.log(p.text);
                    seen.add(p.text);

                    pvPadas.push({
                        text: p.text,
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

    // From https://github.com/alpinejs/alpine/discussions/1205
    document.querySelectorAll('[x-component]').forEach(component => {
        const componentName = `x-${component.getAttribute('x-component')}`
        class Component extends HTMLElement {
            connectedCallback() {
                this.append(component.content.cloneNode(true))
            }

            data() {
                const attributes = this.getAttributeNames()
                const data = {}
                attributes.forEach(attribute => {
                    data[attribute] = this.getAttribute(attribute)
                })
                return data
            }
        }
        customElements.define(componentName, Component)
    })
});
