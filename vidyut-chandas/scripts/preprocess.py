import sys
from pathlib import Path

def process_data(data, ignore_last_syllable=False):
    metre_db = {}

    for metre_data in data:
        name, scheme_str = metre_data.split('\t')
        scheme = scheme_str.split('/')
        
        if ignore_last_syllable:
            scheme = [s[:-1] for s in scheme]

        if len(scheme) == 1:
            padas = '/'.join([scheme[0]] * 4)
        elif len(scheme) == 2:
            padas = f"{scheme[0]}/{scheme[1]}/{scheme[0]}/{scheme[1]}"
        elif len(scheme) == 4:
            padas = '/'.join(scheme)
        else:
            print(f'ERROR: Scheme {name} is not in the correct format!')
        
        if padas in metre_db:
            metre_db[padas] += '/' + name
        else:
            metre_db[padas] = name

    return metre_db

def write_processed_file(metre_db, filename='processed_vrttas.tsv'):
    with open(filename, 'w') as f:
        lines = []

        for padas, names in metre_db.items():
            padas_list = padas.split('/')
            name = names.split('/')[0]

            if all(p == padas_list[0] for p in padas_list):
                line = f"{name}\t{padas_list[0]}"
            elif padas_list[:2] == padas_list[2:]:
                line = f"{name}\t{padas_list[0]}/{padas_list[1]}"
            else:
                line = f"{name}\t{padas_list[0]}/{padas_list[1]}/{padas_list[2]}/{padas_list[3]}"

            lines.append(line)

        f.write('\n'.join(lines))

def print_duplicates(duplicates, title='DUPLICATES'):
    print(f'{title}: ')
    print('---')
    for idx, duplicate in enumerate(duplicates):
        print(f"{idx+1})\t{duplicate}")
    print('---')

source = Path(sys.argv[1])
with open(source) as f:
    data = f.read().splitlines()


# Check for exact duplicates
metre_db = process_data(data)
duplicates = [value for _key, value in metre_db.items() if '/' in value]
print_duplicates(duplicates, 'PROPER DUPLICATES')

# Process and write without duplicates
write_processed_file(metre_db)

print('WRITING: into processed_vrttas.tsv')
print('---')

# Check for duplicates (allowing last syllable of each pada to be different)
metre_db_loose = process_data(data, ignore_last_syllable=True)
duplicates_loose = [value for _key, value in metre_db_loose.items() if '/' in value]
print_duplicates(duplicates_loose, 'LOOSE DUPLICATES (Only last syllable of pada is different)')