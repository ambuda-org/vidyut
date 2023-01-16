*vidyut-lipi* is a work-in-progress transliterator. It is not ready for public use.



Usage
-----



```rust
use vidyut_lipi as pj;

let mut from_map: pj::TranslationMap =  pj::TranslationMap::default();
let mut to_map: pj::TranslationMap = pj::TranslationMap::default();

pj::generate_map("itrans.json", &mut from_map);
pj::generate_map("slp1.json", &mut from_map);

pj::transliterate("k K g G N".to_string(), &slp1_map, &devanagari_map, &mut outstr);
assert_eq!(outstr, "क् ख् ग् घ् ङ्");
```

```shell
# Run transliteration
Usage: transliterate [OPTIONS] --mode <MODE> --frommap <FROMMAP> --tomap <TOMAP>

Options:
  -m, --mode <MODE>        
  -i, --infile <INFILE>    
  -o, --outfile <OUTFILE>  
  -f, --frommap <FROMMAP>  
  -t, --tomap <TOMAP>      
  -d, --data <DATA>        
  -h, --help               Print help information
  -V, --version            Print version information


  mode - text / file 
       - text allows transliteration on command line - single word.
	   - file allows transliteration of whole file. Currently only itrans itx file has been tested.
  infile - input file path in file mode. Not required in text mode.
  outfile - output file path in file mode. Not required in text mode.
  frommap - scheme/language to transliterate from. Valid values are itrans, slp1. 
  tomap - scheme/language to transliterate to. Valid values are itrans 
        - Currently on itrans/slp1 to devanagari/malayalam has been tested. 
  data  - Data to transliterate in text mode. 
        
```

Design:
The transliteration design is quite similar to various other
tools. Scheme specific JSON files maintain encoding<->token
mappings. The tool builds two way hashmaps encoding -> token as well
as token-> encoding, these hashmaps are used to transliterate. 

Essentially a common set of token strings links various schemes
together. The token strings for each character closely follows the
naming scheme used in Devanagari unicode specification (U0900) as well
as Vedic unicode specification (U0D00). A python script in src
directory can be used to create JSON files for individual
languages. Automatically generated files cannot be used directly and
does require further editing and cleanup. 

JSON Data Format

Encodings are maintained as an array of encoding structure. Encoding
structure contains following fields:

	"token": "A String indicating the token this encoding represents."
	"enc": ["this is an array of strings, any one these could be used
             to create an encoding.", "For example itrans allow multiple
             choices aa/A for आ"]
	"ttype": "TType indicates type of token - valid values are
              isvowel, isconsonant, isdigit, issign, isnumeral,
              istransparent, islexicalbreak, isencodingescape"
			  islexicalbreak type are meant to be used in characters
			  that break encoding sequence. e.g. _ in itrans.
			  isencodingescape is meant to be used in cases where
			  sequence of characters is used to enable/disable
			  transliteration e.g ## in itrans.
			  
			  
Apart from the encodings array, the main mapping object has a field
implicit. This field indicates whether a virama or 'a' is
implicit. The unicode encodings have an implicit 'a' incorporated into
the vowels where as the indic roman schemes have an implicit
virama. This field allows the tool to insert/remove virama or 'a' as
required. 
