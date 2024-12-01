extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;

#[test]
fn sk_2678() {
    let kandu = d("kaRqUY", Kandvadi);
    assert_has_lat(&[], &kandu, &["kaRqUyati", "kaRqUyate"]);

    let mantu = d("mantu", Kandvadi);
    assert_has_lat(&[], &mantu, &["mantUyati"]);

    let mantu = d("mantuY", Kandvadi);
    assert_has_lat(&[], &mantu, &["mantUyati", "mantUyate"]);

    let valgu = d("valgu", Kandvadi);
    assert_has_lat(&[], &valgu, &["valgUyati"]);

    let as_ = d("asu~", Kandvadi);
    assert_has_lat(&[], &as_, &["asyati"]);

    let asu = d("asU", Kandvadi);
    assert_has_lat(&[], &asu, &["asUyati"]);

    let asu = d("asUY", Kandvadi);
    assert_has_lat(&[], &asu, &["asUyati", "asUyate"]);

    let lew = d("lew", Kandvadi);
    assert_has_lat(&[], &lew, &["lewyati"]);
    assert_has_lut(&[], &lew, &["lewitA"]);

    let low = d("low", Kandvadi);
    assert_has_lat(&[], &low, &["lowyati"]);
    assert_has_lut(&[], &low, &["lowitA"]);

    let iras = d("iras", Kandvadi);
    assert_has_lat(&[], &iras, &["irasyati"]);

    let iraj = d("iraj", Kandvadi);
    assert_has_lat(&[], &iraj, &["irajyati"]);

    let ira = d("iraY", Kandvadi);
    assert_has_lat(&[], &ira, &["Iryati", "Iryate"]);

    let medha = d("meDA", Kandvadi);
    assert_has_lat(&[], &medha, &["meDAyati"]);

    let kushubha = d("kuzuBa", Kandvadi);
    assert_has_lat(&[], &kushubha, &["kuzuByati"]);

    let sukha = d("suKa", Kandvadi);
    assert_has_lat(&[], &sukha, &["suKyati"]);

    let duhkha = d("duHKa", Kandvadi);
    assert_has_lat(&[], &duhkha, &["duHKyati"]);

    let lekha = d("leKa", Kandvadi);
    assert_has_lat(&[], &lekha, &["leKyati"]);

    let lita = d("liwa", Kandvadi);
    assert_has_lat(&[], &lita, &["liwyati"]);

    let mahi = d("mahIN", Kandvadi);
    assert_has_lat(&[], &mahi, &["mahIyate"]);

    let uras = d("uras", Kandvadi);
    assert_has_lat(&[], &uras, &["urasyati"]);
}
