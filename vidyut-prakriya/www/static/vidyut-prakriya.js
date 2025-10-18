// An ergonomic wrapper over web-assembly calls to wasm/vidyut_prakriya.js.
//
// Installation
// ------------
//
// If you want to use this file from `static`, then use the following directory structure:
//
// - static/vidyut-prakriya.js (this file -- high-level API)
// - static/wasm/vidyut_prakriya.js (low-level bindings to wasm file)
// - static/wasm/vidyut_prakriya_bg.wasm (wasm file)
//
// Usage
// -----
//
// ```
// import { initWasm, Vidyut } from "/static/vidyut-prakriya.js";
// await initWasm();
// const vidyut = new Vidyut();
// ```
//
// Examples
// --------
//
// For various examples, see the tests in www/unit-tests.html
// 
// Notes for maintainers
// ---------------------
// 
// wasm enums (Purusha, Vacana, ...) map ints to strings and strings to ints:
// 
// - Vacana.Eka is an int.
// - Vacana[Vacana.Eka] is a string.
//
// Rust generally expects strings, but enums are good for ergonomics and documentation.
// So, do some goofy enum juggling so that clients have a clean API and Rust gets what it needs.
import init, { BaseKrt, Vidyut as VidyutWasm, Lakara, Prayoga, Purusha, Vacana, DhatuPada, Sanadi,
    Linga, Vibhakti, Gana, Antargana } from "./wasm/vidyut_prakriya.js";

export {
    init as initWasm,
    Lakara,
    Prayoga,
    Purusha,
    Vacana,
    DhatuPada,
    Sanadi,
    Linga,
    Vibhakti,
    Gana,
    Antargana,
    BaseKrt as Krt,
};

function isMissing(x) {
    return x === null || x === undefined;
}

function createWasmDhatu({aupadeshika, gana, antargana, sanadi, prefixes}) {
    return {
        aupadeshika,
        gana: Gana[gana],
        antargana: isMissing(antargana) ? null : Antargana[antargana],
        sanadi: (sanadi || []).map(x => Sanadi[x]),
        prefixes: prefixes || [],
    };
}

function createWasmKrdanta({ dhatu, krt, lakara = null, prayoga = null }) {
    return {
        dhatu: createWasmDhatu(dhatu),
        krt: BaseKrt[krt],
        lakara: isMissing(lakara) ? null : Lakara[lakara],
        prayoga: isMissing(prayoga) ? null : Prayoga[prayoga],
    }
}

function createWasmPratipadika({ basic, krdanta }) {
    if (basic) {
        return { basic };
    } else {
        return { krdanta: createWasmKrdanta(krdanta) };
    }
}

// Ergonomic wrapper over vidyut-prakriya wasm build.
//
// This class is used in three ways:
// - as the interface for our wasm unit tests (see unit-tests.html)
// - as the interface for our debugger (see `make debugger`)
// - as sample code for developers who want to use vidyut-prakriya in their projects.
//
// In the methods below, `dhatu` and `pratipadika` are complex objects. Here are some examples
// of each.
//
// Dhatus require `aupadeshika` and `gana` and may optionally take `sanadi`, `prefixes`, or
// `antargana`:
// 
// - { aupadeshika: "BU", gana: Gana.Bhvadi }
// - { aupadeshika: "BU", gana: Gana.Bhvadi, sanadi: [Sanadi.san] }
// - { aupadeshika: "BU", gana: Gana.Bhvadi, prefixes: ["pari"], sanadi: [Sanadi.san] }
// - { aupadeshika: "Gawa~", gana: Gana.Bhvadi, antargana: Antargana.Ghatadi }
//
// Pratipadikas have one key that is either `basic` or `krdanta`. `basic` defines a simple
// pratipadika, and `krdanta` defines a krdanta as an object with two keys `dhatu` and `krt`:
// - { basic: "deva" }
// - { krdanta: { dhatu: { aupadeshika: "BU", gana: Gana.Bhvadi }, krt: Krt.lyuw } }
//
// For more examples, see unit-tests.html in the Vidyut repository.
export class Vidyut {
    // Call `init()` before using Vidyut so that you correctly initialize the WASM environment.
    //
    // Example:
    //
    // ```
    // // NOTE: `init()` is async!
    // await init();
    // const vidyut = new Vidyut();
    // ```
    constructor() {
        this.wasm = VidyutWasm.init();
    }

    /**
     * Derives all dhatus that match the input conditions.
     *
     * dhatu: an object that defines a dhatu. For details, see the comments on `class Vidyut`.
     * linga: a `Linga`
     * vibhakti: a `Vibhakti`
     * vacana: a `Vacana`
     */
    deriveDhatus({ aupadeshika, gana, antargana, sanadi, prefixes }) {
        // For argument order, see wasm.rs.
        return this.wasm.deriveDhatus(createWasmDhatu({
            aupadeshika,
            gana,
            antargana,
            sanadi,
            prefixes
        }));
    }

    /**
     * Derives all krdantas that match the input conditions.
     *
     * dhatu: an object that defines a dhatu. For details, see the comments on `class Vidyut`.
     * krt: a `Krt`.
     *
     * lakara?: (for Satf and SAnac only) the lakAra to use.
     * prayoga?: (for Satf and SAnac only) the prayoga to use.
     */
    deriveKrdantas({ dhatu, krt, lakara = null, prayoga = null }) {
        return this.wasm.deriveKrdantas(createWasmKrdanta({
            dhatu,
            krt,
            lakara,
            prayoga
        }));
    }

    /**
     * Derives all subantas that match the input conditions.
     *
     * pratipadika: an object that defines a pratipadika.
     *              For details, see the comments on `class Vidyut`.
     * linga: a `Linga`
     * vibhakti: a `Vibhakti`
     * vacana: a `Vacana`
     */
    deriveSubantas({ pratipadika, linga, vibhakti, vacana }) {
        // For argument order, see wasm.rs.
        return this.wasm.deriveSubantas({
            pratipadika: createWasmPratipadika(pratipadika),
            linga: Linga[linga],
            vibhakti: Vibhakti[vibhakti],
            vacana: Vacana[vacana],
        });
    }

    /**
     * Derives all tinantas that match the input conditions.
     *
     * dhatu: an object that defines a dhatu. For details, see the comments on `class Vidyut`.
     * lakara: a `Lakara`
     * purusha: a `Purusha`
     * vacana: a `Vacana`
     * pada: a `DhatuPada`
     * sanadi: a list of strings. Valid values are "san", "Ric", "yaN", and "yaNluk".
     * upasargas: a list of strings. For the upasarga "A", pass "AN".
     */
    deriveTinantas({ dhatu, lakara, prayoga, purusha, vacana, pada = null }) {
        return this.wasm.deriveTinantas({
            dhatu: createWasmDhatu(dhatu),
            lakara: Lakara[lakara],
            prayoga: Prayoga[prayoga],
            purusha: Purusha[purusha],
            vacana: Vacana[vacana],
            pada: isMissing(pada) ? null : DhatuPada[pada],
        });
    }
}
