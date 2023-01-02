import init, { Vidyut, Gana, Lakara, Prayoga, Purusha, Vacana, Sanadi } from "/static/wasm/vidyut_prakriya.js";


function parseDhatus(text) {
    let dhatus = [];
    text.split(/\r?\n/).forEach((line) => {
        const [code, upadesha, artha] = line.split(/\t/);
        if (!!code && code !== 'code') {
            dhatus.push({ code, upadesha, artha });
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

    async init() {
        const data = await loadVidyut(); 
        this.vidyut = data.vidyut;
        this.dhatus = data.dhatus;
        console.log("initialized");
    },

    tab(s) {
        if (s === this.activeTab) {
            return "font-bold p-2 bg-sky-100 rounded text-sky-800";
        } else {
            return "";
        }
    },

    setActiveDhatu(s) {
        this.activeDhatu = this.dhatus.find(d => d.code === s);
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
        return Sanscript.t(s, 'slp1', 'devanagari').replace(/[\^\\]/, '');
    },

    entryString(entries) {
        let str = entries.map(x => x.text).join(', ');
        return this.deva(str);
    },

    createTinantas() {
        if (this.activeDhatu === null) {
            return [];
        }

        const dhatu = this.activeDhatu;

        let lakaras = Object.values(Lakara).filter(Number.isInteger);
        let purushas = Object.values(Purusha).filter(Number.isInteger);
        let vacanas = Object.values(Vacana).filter(Number.isInteger);
      
        let prayoga = this.prayoga !== null ? this.prayoga : Prayoga.Kartari;
        let sanadi = this.sanadi || null;;

        let groups = {};
        for (const lakara in lakaras) {
            let lakaraPadas = [];
            for (const purusha in purushas) {
                for (const vacana in vacanas) {
                    let pvPadas = [];
                    let prakriyas = this.vidyut.derive(
                        dhatu.code,
                        lakara,
                        prayoga,
                        purusha,
                        vacana,
                        sanadi,
                    );
                    prakriyas.forEach((p) => {
                        pvPadas.push({
                            text: p.text,
                            dhatu,
                            lakara,
                            prayoga,
                            purusha,
                            vacana,
                            sanadi,
                        });
                    });
                    lakaraPadas.push(pvPadas);
                }
            }
            groups[Lakara[lakara]] = lakaraPadas;
        }
        return groups;
    },
});

window.Lakara = Lakara;
window.Prayoga = Prayoga;
window.Sanadi = Sanadi;
window.addEventListener('alpine:init', () => {
    Alpine.data("app", App)
});
