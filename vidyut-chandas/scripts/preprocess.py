import sys
from pathlib import Path

source = Path(sys.argv[1])
with open(source) as f:
    data = f.read().splitlines()

metre_db = {}

for metre_data in data:
    entry = metre_data.split('\t')
    assert(len(entry) == 2)
    name = entry[0]
    scheme = entry[1].split('/')
    padas = ""
    if len(scheme) == 1:
        padas += scheme[0] + '/'
        padas += scheme[0] + '/'
        padas += scheme[0] + '/'
        padas += scheme[0]
    elif len(scheme) == 2:
            padas += scheme[0] + '/'
            padas += scheme[1] + '/'
            padas += scheme[0] + '/'
            padas += scheme[1]
    elif len(scheme) == 4:
        padas += scheme[0] + '/'
        padas += scheme[1] + '/'
        padas += scheme[2] + '/'
        padas += scheme[3]
    else:
        print('ERROR: Scheme ' + name + ' is not in format!')
    if padas in metre_db:
        metre_db[padas] += '/' + name
    else:
        metre_db[padas] = name

duplicates = [value for _key, value in metre_db.items() if '/' in value]

print('DUPLICATES: ')
print('---')
for idx, duplicate in enumerate(duplicates):
    print(f"{idx+1})\t{duplicate}")
print('---')


with open('processed_vrttas.tsv', 'w') as f:
    lines = ""
    for scheme, names in metre_db.items():
        line = ""
        padas = scheme.split('/')
        name = names.split('/')[0]

        if all(pada == padas[0] for pada in padas):
            line = f"{name}\t{padas[0]}"
        elif padas[:2] == padas[2:]:
            line = f"{name}\t{padas[0]}\{padas[1]}"
        else:
            line = f"{name}\t{padas[0]}\{padas[1]}\{padas[2]}\{padas[3]}"
        lines += (line) + '\n'
        
    f.write(lines)