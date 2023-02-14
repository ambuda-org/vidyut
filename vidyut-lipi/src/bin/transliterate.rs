// translitera.rs
//

use vidyut_lipi as vl;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Write;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,
    #[arg(short, long)]
    infile: Option<std::path::PathBuf>,
    #[arg(short, long)]
    outfile: Option<std::path::PathBuf>,
    #[arg(short, long)]
    frommap: String,
    #[arg(short, long)]
    tomap: String,
    #[arg(short, long)]
    data: Option<String>,
}

fn main() {

    let mut from_map: vl::TranslationMap =  vl::TranslationMap::default();
    let mut to_map: vl::TranslationMap = vl::TranslationMap::default();

    let args = Args::parse();

    match args.frommap.as_str() {
	"itrans" => {
	    vl::generate_map("itrans.json", &mut from_map);
	},
	"slp1" => {
	    vl::generate_map("slp1.json", &mut from_map);
	},
	"iast" => {
	    vl::generate_map("iast.json", &mut from_map);
	},
	"devanagari" => {
	    vl::generate_map("devanagari.json", &mut from_map);
	},
	"malayalam" => {
	    vl::generate_map("malayalam.json", &mut from_map);
	}
	
	_ => {
	    panic!("Valid from maps are itrans,slp1,iast");
	}
    }
    
    match args.tomap.as_str() {
	"itrans" => {
	    vl::generate_map("itrans.json", &mut to_map);
	},
	"slp1" => {
	    vl::generate_map("slp1.json", &mut to_map);
	},
	"iast" => {
	    vl::generate_map("iast.json", &mut to_map);
	},
	"devanagari" => {
	    vl::generate_map("devanagari.json", &mut to_map);
	},
	"malayalam" => {
	    vl::generate_map("malayalam.json", &mut to_map);
	}
	_ => {
	    panic!("Valid from maps are devanagari and malayalam");
	}
    }

    match args.mode.as_str() {
	"file" => {
	    if args.infile.is_some() && args.outfile.is_some() {
		transliterate_file(args.infile.unwrap(), args.outfile.unwrap(), from_map, to_map);	    
	    }

	},
	"text" => {
	    let mut outstr:String = "".to_string();
	    if args.data.is_some() {
		vl::transliterate(args.data.unwrap(), &from_map, &to_map, &mut outstr);
		println!("{}", outstr);		
	    } else {
		println!("No data available...");
	    }
	},
	_ => {
	    panic!("valid modes are file / text");
	}
    }


    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Right now assume we process only itx files.
// handler functions might b required for individual file types. 
fn transliterate_file<P>(inf: P, outf:P, inmap:vl::TranslationMap, outmap:vl::TranslationMap)
where P: AsRef<Path> {
    let mut enc_on: bool = true;
    let mut lnskip: bool = false;

    let mut file = File::create(outf).expect("create failed");
    
    if let Ok(lines) = read_lines(inf) {
	for line in lines {
	    if let Ok(ln) = line {
		if !ln.starts_with("\\") && !ln.starts_with("%") {
		    if ln.starts_with("#") {
			lnskip = true;
			if ln.starts_with("##") {
			    enc_on = !enc_on;
			}
			    
		    }
		    if lnskip == false {
			let mut outstr: String = "".to_string();
			println!("{}", ln);
			if enc_on {
			    vl::transliterate(ln, &inmap, &outmap, &mut outstr);
			    println!("{}", outstr);
			    file.write_all(outstr.as_bytes()).expect("write_failed.");
			    file.write_all("\r\n".as_bytes()).expect("write_failed.");
			}
		    }
		    lnskip = false;
		}
		    
	    }
	}
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_transliteratescheme_slp1_kvarga() {
	      let mut outstr = "".to_string();
        vl::transliterate_scheme("k K g G N".to_string(),
                                 vl::TranslationScheme::Slp1,
                                 vl::TranslationScheme::Devanagari, &mut outstr);
	      assert_eq!(outstr, "क् ख् ग् घ् ङ्");
    }
    
    #[test]
    fn test_transliterate_slp1_kvarga() {
	      let mut slp1_map: vl::TranslationMap = vl::TranslationMap::default();
	      let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	
        
	      vl::generate_map("slp1.json", &mut slp1_map);
	      vl::generate_map("devanagari.json", &mut devanagari_map);
        
	      
	      let mut outstr = "".to_string();
	      vl::transliterate("k K g G N".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	      assert_eq!(outstr, "क् ख् ग् घ् ङ्");
    }
    
    #[test]
    fn test_transliterate_slp1_kvarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("slp1.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	let mut outstr = "".to_string();
	vl::transliterate("k K g G N".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "क् ख् ग् घ् ङ्");
    }

    
    #[test]
    fn test_transliterate_slp1_cvarga() {
	let mut slp1_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("slp1.json", &mut slp1_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("c C j J Y".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "च् छ् ज् झ् ञ्");
    }

     #[test]
    fn test_transliterate_slp1_cvarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("slp1.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	let mut outstr = "".to_string();
	vl::transliterate("c C j J Y".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "च् छ् ज् झ् ञ्");
    }

    #[test]
    fn test_transliterate_slp1_tvarga() {
	let mut slp1_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("slp1.json", &mut slp1_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("w W q Q R".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "ट् ठ् ड् ढ् ण्");
    }

    #[test]
    fn test_transliterate_slp1_tvarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("slp1.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
		
	
	let mut outstr = "".to_string();
	vl::transliterate("w W q Q R".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "ट् ठ् ड् ढ् ण्");
    }
    
    #[test]
    fn test_transliterate_slp1_thvarga() {
	let mut slp1_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("slp1.json", &mut slp1_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("t T d D n".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "त् थ् द् ध् न्");
    }

    #[test]
    fn test_transliterate_slp1_thvarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("slp1.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
	vl::transliterate("t T d D n".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "त् थ् द् ध् न्");
    }

    
    #[test]
    fn test_transliterate_slp1_pavarga() {
	let mut slp1_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("slp1.json", &mut slp1_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("p P b B m".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "प् फ् ब् भ् म्");
    }


    #[test]
    fn test_transliterate_slp1_pavarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("slp1.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	let mut outstr = "".to_string();
	vl::transliterate("p P b B m".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "प् फ् ब् भ् म्");
    }

    
    #[test]
    fn test_transliterate_itrans_vowels() {
	let mut slp1_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut slp1_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("a A i I u U e ai o O au aM aH".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "अ आ इ ई उ ऊ ए ऐ ओ  औ अं अः");
    }    

    #[test]
    fn test_transliterate_itrans_vowels_ff() {
	let slp1_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
	vl::transliterate("a A i I u U e ai o O au aM aH".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "अ आ इ ई उ ऊ ए ऐ ओ  औ अं अः");
    }    

    #[test]
    fn test_transmap_create() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	
	vl::generate_map("../../itrans.json", &mut itrans_map);
	vl::generate_map("../../devanagari.json", &mut devanagari_map);

    }

    #[test]
    fn test_transliterate_with_escape() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("##var## 1234".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "var १२३४")
    }

    #[test]
    fn test_transliterate_with_escape_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
	vl::transliterate("##var## 1234".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "var १२३४")
    }

    
    #[test]
    fn test_transliterate_with_zwj() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("k()Sha k{}Sha k[]Sha ay{}lAH".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "क्‍ष क्‌ष क्ष अय्‌लाः");
    }

    #[test]
    fn test_transliterate_with_zwj_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	
	let mut outstr = "".to_string();
	vl::transliterate("k()Sha k{}Sha k[]Sha ay{}lAH".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "क्‍ष क्‌ष क्ष अय्‌लाः");
    }


    #[test]
    fn test_transliterate_encoding_on_off() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("##1234##1234".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "1234१२३४");
    }

    #[test]
    fn test_transliterate_encoding_on_off_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	let mut outstr = "".to_string();
	vl::transliterate("##1234##1234".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "1234१२३४");
    }

    
    
    #[test]
    fn test_transliterate_kvarga() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("k kh g gh ~N".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "क् ख् ग् घ् ङ्");
    }

    #[test]
    fn test_transliterate_kvarga_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	let mut outstr = "".to_string();
	vl::transliterate("k kh g gh ~N".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "क् ख् ग् घ् ङ्");
    }

    #[test]
    fn test_transliterate_cvarga_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	let mut outstr = "".to_string();
	vl::transliterate("ch Ch j jh ~n".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "च् छ् ज् झ् ञ्");
    }


    
    #[test]
    fn test_transliterate_cvarga() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("ch Ch j jh ~n".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "च् छ् ज् झ् ञ्");
    }

    #[test]
    fn test_transliterate_tvarga_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	let mut outstr = "".to_string();
	vl::transliterate("T Th D Dh N".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "ट् ठ् ड् ढ् ण्");
    }

    
    #[test]
    fn test_transliterate_tvarga() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("T Th D Dh N".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "ट् ठ् ड् ढ् ण्");
    }

    #[test]
    fn test_transliterate_thvarga_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
	vl::transliterate("t th d dh n".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "त् थ् द् ध् न्");
    }
    
    #[test]
    fn test_transliterate_thvarga() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("t th d dh n".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "त् थ् द् ध् न्");
    }

    #[test]
    fn test_transliterate_pavarga_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	
	let mut outstr = "".to_string();
	vl::transliterate("p ph b bh m".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "प् फ् ब् भ् म्");
    }

    
    #[test]
    fn test_transliterate_pavarga() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("p ph b bh m".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "प् फ् ब् भ् म्");
    }

    #[test]
    fn test_transliterate_vowels_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
	vl::transliterate("a A i I u U e ai o O au aM aH".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "अ आ इ ई उ ऊ ए ऐ ओ  औ अं अः");
    }    

    
    #[test]
    fn test_transliterate_vowels() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("a A i I u U e ai o O au aM aH".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "अ आ इ ई उ ऊ ए ऐ ओ  औ अं अः");
    }    

    #[test]
    fn test_transliterate_numbers_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	
	let mut outstr = "".to_string();
	vl::transliterate("0 1 2 3 4 5 6 7 8 9 10 20 30".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "० १ २ ३ ४ ५ ६ ७ ८ ९ १० २० ३०");
    }    

    
    #[test]
    fn test_transliterate_numbers() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("0 1 2 3 4 5 6 7 8 9 10 20 30".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "० १ २ ३ ४ ५ ६ ७ ८ ९ १० २० ३०");
    }    


    #[test]
    fn test_transliterate_kvowels_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	
	let mut outstr = "".to_string();
	    vl::transliterate("ka kA ki kI ku kU ke kai kau kaM kaH".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "क का कि की कु कू के कै कौ कं कः");
    }

    
    #[test]
    fn test_transliterate_kvowels() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	    vl::transliterate("ka kA ki kI ku kU ke kai kau kaM kaH".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "क का कि की कु कू के कै कौ कं कः");
    }

    #[test]
    fn test_transliterate_itrans_break_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	let mut outstr = "".to_string();
	vl::transliterate("gaii ga_ii gayi gayI".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "गैइ गई गयि गयी");
    }    
    
    #[test]
    fn test_transliterate_itrans_break() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("gaii ga_ii gayi gayI".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "गैइ गई गयि गयी");
    }

    #[test]
    fn test_transliterate_itrans_yetc_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
	vl::transliterate("y r l v w L ld sh Sh shh s h kSh shrI q K G z J f .D .Dh OM AUM".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "य् र् ल् व् व् श् ष् ष् स् ह् क्ष् श्री क़् ख़् ग़् ज़् ज़् फ़् ड़् ढ़् ॐ ॐ");
    }

    #[test]
    fn test_transliterate_itrans_yetc() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("y r l v w L ld sh Sh shh s h kSh shrI q K G z J f .D .Dh OM AUM".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "य् र् ल् व् व् श् ष् ष् स् ह् क्ष् श्री क़् ख़् ग़् ज़् ज़् फ़् ड़् ढ़् ॐ ॐ");
    }

    #[test]
    fn test_transliterate_sriramodantam_1_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	
	let mut outstr = "".to_string();
	vl::transliterate("shrIpatiM praNipatyAhaM shrIvatsA~NkitavakShasaM |".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "श्रीपतिं प्रणिपत्याहं श्रीवत्साङ्कितवक्षसं ।");
    }

    #[test]
    fn test_transliterate_sriramodantam_1() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("shrIpatiM praNipatyAhaM shrIvatsA~NkitavakShasaM |".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "श्रीपतिं प्रणिपत्याहं श्रीवत्साङ्कितवक्षसं ।");
    }

    #[test]
    fn test_transliterate_sriramodantam_2_ff() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
	vl::transliterate("shrIrAmodantamAkhyAsye shrIvAlmIkiprakIrtitaM || 1||".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "श्रीरामोदन्तमाख्यास्ये श्रीवाल्मीकिप्रकीर्तितं ॥ १॥");
    }    
    
    #[test]
    fn test_transliterate_sriramodantam_2() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("shrIrAmodantamAkhyAsye shrIvAlmIkiprakIrtitaM || 1||".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "श्रीरामोदन्तमाख्यास्ये श्रीवाल्मीकिप्रकीर्तितं ॥ १॥");
    }

    #[test]
    fn test_transliterate_sriramodantam_3_ff() {
	let mut itrans_map: vl::TranslationMap = vl::TranslationMap::default();
	let mut devanagari_map: vl::TranslationMap = vl::TranslationMap::default();	

	vl::generate_map("itrans.json", &mut itrans_map);
	vl::generate_map("devanagari.json", &mut devanagari_map);

	
	let mut outstr = "".to_string();
	vl::transliterate("va~nchayitvA tu tAnpaurAnnidrANAnnishi rAghavaH |  ##var## kR^ishAn paurAn".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "वञ्चयित्वा तु तान्पौरान्निद्राणान्निशि राघवः ।  var कृशान् पौरान्");
    }        

    #[test]
    fn test_transliterate_sriramodantam_3() {
	let itrans_map =  vl::TranslationMap::from_file("itrans.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	let mut outstr = "".to_string();
	vl::transliterate("va~nchayitvA tu tAnpaurAnnidrANAnnishi rAghavaH |  ##var## kR^ishAn paurAn".to_string(), &itrans_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "वञ्चयित्वा तु तान्पौरान्निद्राणान्निशि राघवः ।  var कृशान् पौरान्");
    }

    #[test]
    fn test_transliterate_iast_vowels_ff() {
	let slp1_map =  vl::TranslationMap::from_file("iast.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
//	vl::transliterate("a ā i ī u ū e ai o au aṃ aḥ".to_string(), &slp1_map, &devanagari_map, &mut outstr);
//	assert_eq!(outstr, "अ आ इ ई उ ऊ ए ऐ ओ अं अः");

	vl::transliterate("a ā i ī u ū e ai o aṃ aḥ".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "अ आ इ ई उ ऊ ए ऐ ओ अं अः");
	
    }

        #[test]
    fn test_transliterate_iast_kvarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("iast.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	let mut outstr = "".to_string();

	vl::transliterate("k kh g gh ṅ".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "क् ख् ग् घ् ङ्");
    }

         #[test]
    fn test_transliterate_iast_cvarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("iast.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
	
	let mut outstr = "".to_string();

	vl::transliterate("c ch j jh ñ".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "च् छ् ज् झ् ञ्");
    }

        #[test]
    fn test_transliterate_iast_tvarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("iast.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");
		
	
	let mut outstr = "".to_string();
	vl::transliterate("ṭ ṭh ḍ ḍh ṇ".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "ट् ठ् ड् ढ् ण्");
    }

        #[test]
    fn test_transliterate_iast_thvarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("iast.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	
	let mut outstr = "".to_string();
	vl::transliterate("t th d dh n".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "त् थ् द् ध् न्");
    }

    #[test]
    fn test_transliterate_iast_pavarga_ff() {
	let slp1_map =  vl::TranslationMap::from_file("iast.json");
	let devanagari_map = vl::TranslationMap::from_file("devanagari.json");

	let mut outstr = "".to_string();
	vl::transliterate("p ph b bh m".to_string(), &slp1_map, &devanagari_map, &mut outstr);
	assert_eq!(outstr, "प् फ् ब् भ् म्");
    }

    
}


