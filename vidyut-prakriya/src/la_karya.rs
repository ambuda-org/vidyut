use crate::args::Lakara;
use crate::it_samjna;
use crate::prakriya::{Prakriya, Rule};
use crate::tag::Tag as T;
use crate::term::Term;
use std::error::Error;

fn add_la(rule: Rule, p: &mut Prakriya, i: usize, la: &str) -> Result<(), Box<dyn Error>> {
    let mut la = Term::make_upadesha(la);
    la.add_tag(T::Pratyaya);

    p.insert_after(i, la);
    p.step(rule);
    it_samjna::run(p, i + 1)?;

    Ok(())
}

pub fn run(p: &mut Prakriya, la: Lakara) -> Result<(), Box<dyn Error>> {
    let i = match p.find_last(T::Dhatu) {
        Some(i) => i,
        None => return Ok(()),
    };

    match la {
        Lakara::Lat => add_la("3.3.123", p, i, "la~w")?,
        Lakara::Lit => add_la("3.2.114", p, i, "li~w")?,
        Lakara::Lut => add_la("3.3.15", p, i, "lu~w")?,
        Lakara::Lrt => add_la("3.3.13", p, i, "lf~w")?,
        Lakara::Let => add_la("3.4.7", p, i, "le~w")?,
        Lakara::Lot => add_la("3.3.162", p, i, "lo~w")?,
        Lakara::Lan => add_la("3.2.111", p, i, "la~N")?,
        Lakara::AshirLin => {
            p.add_tag(T::Ashih);
            add_la("3.3.173", p, i, "li~N")?;
        }
        Lakara::VidhiLin => add_la("3.3.161", p, i, "li~N")?,
        Lakara::Lun => add_la("3.2.110", p, i, "lu~N")?,
        Lakara::Lrn => add_la("3.3.139", p, i, "lf~N")?,
    };

    Ok(())
}
