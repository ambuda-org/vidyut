# script to convert consts file to devanagari json.
# this is just a convenience script. Further editing required.

# process a line and return a hashmap with token, and enc.
# unicode prefix is passed in - to help generate maps for
# different languages.
# if a language specific codepoint does not map to devanagari
# or has a extra elements, then they need to be manually inserted.
# Also, the constants don't have an indicator as to their types
# so that has to be manually edited to.
# This is a dumb script just to reduce RSI but not by much.
import sys
import json

def process_line(cpprfx, ln):
    if 'pub' in ln:
        fls = ln.split()
        if len(fls) >= 6:
            tmap = {}
            tok = fls[2].strip()[:-1] # remove the :
            tmap["token"] = tok
            tval = fls[5].strip()[2:-1] # remove leading 0x and ;
            if 'VEDIC' in tok:
                tval = '\\u1C' + tval
            else :
                tval = '\\u'+cpprfx+tval

            # set tval to 
            if 'useunicode' in ln:
                if len(fls) >= 8:
                    useunifls = fls[8].split(':')
                    if len(useunifls) >= 2:
                        uflds = useunifls[1].split()
                        tval = ''
                        for u in uflds:
                            tval = tval + '\\'+u
                
            encar = []
            encar.append(tval)
            tmap["enc"] = encar
            tmap["ttype"] = "replaceme"
            if 'ttype' in ln:
                try:
                    tfls = fls[7].strip()
                    tt = tfls.split(':')
                    tmap['ttype'] = tt[1]
                except:
                    print(ln)
                
            return tmap
        else:
            return {}
    else:
        return {}

def process_file(inf, ofl,prfx) :
    with open(inf,'r') as infl:
        lns = infl.readlines()
        encdngs = []
        for ln in lns:
            if 'pub const' in ln:
                encdngs.append(process_line(prfx,ln))
        mymap = {}
        mymap["comment"] = "Automatically generated file, edit as necessary"
        mymap["implicit"] = "EditMe"
        mymap["encodings"] = encdngs
        jsout = json.dumps(mymap,indent=4)
        with open(ofl,"w") as outfl:
            outfl.write(jsout)
            outfl.flush()
            outfl.close()

            
if __name__ == "__main__":
    if len(sys.argv) == 4:
        process_file(sys.argv[1], sys.argv[2], sys.argv[3])
    else :
        print("Usage python consts_to_json.py consts.rs xx.json prfx")
