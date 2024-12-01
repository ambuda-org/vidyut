use crate::args::Lakara;
use crate::core::Tag as T;
use crate::core::Term;
use crate::core::{Code, Prakriya};
use crate::it_samjna;

fn add_la(rule: Code, p: &mut Prakriya, i: usize, lakara: Lakara, la: &str) {
    p.run(rule, |p| {
        let mut la = Term::make_upadesha(la);
        la.lakara = Some(lakara);
        la.add_tags(&[T::Pratyaya, T::La]);
        p.insert_after(i, la)
    });
    it_samjna::run(p, i + 1).expect("should always succeed");
}

/// Runs rules that add a lakAra to the prakriya.
///
/// Lakaras are replaced by one of two different kinds of pratyayas:
///
/// 1. Tin pratyayas. The results are tinantas (bhavati, gacchati, etc.)
/// 2. Krt pratyayas. The results are krdantas (bhavat, jagmivas, etc.)
pub fn run(p: &mut Prakriya, la: Lakara) {
    let i = match p.find_last_with_tag(T::Dhatu) {
        Some(i) => i,
        None => return,
    };

    match la {
        Lakara::Lat => add_la("3.2.123", p, i, la, "la~w"),
        Lakara::Lit => add_la("3.2.115", p, i, la, "li~w"),
        Lakara::Lut => add_la("3.3.15", p, i, la, "lu~w"),
        Lakara::Lrt => add_la("3.3.13", p, i, la, "lf~w"),
        Lakara::Let => add_la("3.4.7", p, i, la, "le~w"),
        Lakara::Lot => add_la("3.3.162", p, i, la, "lo~w"),
        Lakara::Lan => add_la("3.2.111", p, i, la, "la~N"),
        Lakara::AshirLin => {
            p.add_tag(T::Ashih);
            add_la("3.3.173", p, i, la, "li~N");
        }
        Lakara::VidhiLin => add_la("3.3.161", p, i, la, "li~N"),
        Lakara::Lun => add_la("3.2.110", p, i, la, "lu~N"),
        Lakara::Lrn => add_la("3.3.139", p, i, la, "lf~N"),
    };
}
