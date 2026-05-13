use crate::core::operators as op;
use crate::core::{Prakriya, Tag as T};

/// Run rules that add enclitics (for yuzmad/asmad)
///
/// (8.1.20 - 8.1.25)
fn try_enclitics(p: &mut Prakriya) -> Option<()> {
    let i = p.find_last_with_tag(T::Pratipadika)?;
    let i_sup = p.find_next_where(i, |t| !t.is_nyap_pratyaya())?;

    let anga = p.get(i)?;
    if !anga.has_u_in(&["yuzmad", "asmad"]) {
        return None;
    }

    let is_asmad = anga.has_u("asmad");
    let is_yuzmad = anga.has_u("yuzmad");

    // Enclitic variants for dative/genitive/accusative pronouns.
    let (i_sup_start, i_sup_end, is_v2, is_v4, is_v6, is_ekavacana, is_dvivacana, is_bahuvacana) = {
        let sup_view = p.pratyaya(i_sup)?;
        let sup = sup_view.last();
        (
            sup_view.start(),
            sup_view.end(),
            sup.has_tag(T::V2),
            sup.has_tag(T::V4),
            sup.has_tag(T::V6),
            sup.has_tag(T::Ekavacana),
            sup.has_tag(T::Dvivacana),
            sup.has_tag(T::Bahuvacana),
        )
    };

    let set_sup_text = |p: &mut Prakriya, text: &str| {
        if text.is_empty() {
            p.set(i_sup_start, op::luk);
        } else {
            p.set(i_sup_start, |t| t.set_text(text));
        }
        for j in (i_sup_start + 1)..=i_sup_end {
            p.set(j, op::luk);
        }
    };

    if is_v4 || is_v6 {
        if is_ekavacana {
            p.optional_run("8.1.22", |p| {
                if is_asmad {
                    p.set(i, op::text("me"));
                } else if is_yuzmad {
                    p.set(i, op::text("te"));
                }
                set_sup_text(p, "");
            });
        } else if is_dvivacana {
            p.optional_run("8.1.20", |p| {
                if is_asmad {
                    p.set(i, op::text("nO"));
                } else if is_yuzmad {
                    p.set(i, op::text("vAm"));
                }
                set_sup_text(p, "");
            });
        } else if is_bahuvacana {
            p.optional_run("8.1.21", |p| {
                if is_asmad {
                    p.set(i, op::text("nas"));
                } else if is_yuzmad {
                    p.set(i, op::text("vas"));
                }
                set_sup_text(p, "");
            });
        }
    } else if is_v2 {
        if is_ekavacana {
            p.optional_run("8.1.23", |p| {
                if is_asmad {
                    p.set(i, op::text("mA"));
                } else if is_yuzmad {
                    p.set(i, op::text("tvA"));
                }
                set_sup_text(p, "");
            });
        } else if is_dvivacana {
            p.optional_run("8.1.20", |p| {
                if is_asmad {
                    p.set(i, op::text("nO"));
                } else if is_yuzmad {
                    p.set(i, op::text("vAm"));
                }
                set_sup_text(p, "");
            });
        } else if is_bahuvacana {
            p.optional_run("8.1.21", |p| {
                if is_asmad {
                    p.set(i, op::text("nas"));
                } else if is_yuzmad {
                    p.set(i, op::text("vas"));
                }
                set_sup_text(p, "");
            });
        }
    }
    let t = p.get_mut(i).unwrap();
    t.remove_tag(T::FlagAntyaAcSandhi); // Otherwise this won't trigger 8.3.15  r->s
    Some(())
}

pub fn run(p: &mut Prakriya) {
    // 8.1.16 specifies these occur when not at the start of a pada.
    // In our current single-word prakriya context, this is usually handled by optional_run.
    try_enclitics(p);
}
