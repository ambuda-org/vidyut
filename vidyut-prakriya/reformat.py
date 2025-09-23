#!/usr/bin/env python3
import sys
from test.test_unittest.testmock.testhelpers import AnyTest

remainderA:dict = {}
remainderB:dict = {}


def openfile(fname):
    try:
        with open(fname, "r") as file:
            lines = file.readlines()
            return lines
    except FileNotFoundError:
        print("File not found.")
    except IOError as e:
        print(f"I/O error occurred: {e}")
    return []


def handle_only_A(line):
    line = line.replace("<", "").replace(" ", "")
    flds = line.split(",")

    if len(flds) > 2:
        value = flds.pop(0).replace("|"," , ")
        verb_defn = ", ".join(flds)
        if verb_defn in remainderB:
            print(f"| {verb_defn} | {value} | {remainderB.pop(verb_defn)} |")
        else:
            remainderA[verb_defn] = value

def handle_only_B(line):
    line = line.replace(">", "").replace(" ", "")
    flds = line.split(",")

    if len(flds) > 2:
        value = flds.pop(0).replace("|", " , ")
        verb_defn = ", ".join(flds)
        if verb_defn in remainderA:
            print(f"| {verb_defn} | {remainderA.pop(verb_defn)} | {value} |")
        else:
            remainderB[verb_defn] = value

def handle_change(line):
    flds = line.split(" | ")
    if len(flds) != 2:
        print(line)
        return
    flds[0] = flds[0].replace(" ","")
    flds[1] = flds[1].replace(" ","")
    derivation = flds[0].split(",")
    valueA = derivation.pop(0).replace("|"," , ")
    verb_defn = ", ".join(derivation)
    derivation = flds[1].split(",")
    valueB = derivation.pop(0).replace("|"," , ")
    verb_defn2 = ", ".join(derivation)
    if (verb_defn == verb_defn2):
        print(f"| {verb_defn} | {valueA} | {valueB} |")
    elif verb_defn in remainderB:
        print(f"| {verb_defn} | {valueA} | {remainderB.pop(verb_defn)} |")
        remainderB[verb_defn2] = valueB
    elif verb_defn2 in remainderA:
        print(f"| {verb_defn2} | {remainderA.pop(verb_defn2)} | {valueB} | ")
        remainderA[verb_defn] = valueA
    else:
        remainderB[verb_defn2] = valueB
        remainderA[verb_defn] = valueA

def handle_section_header(line):
    print(f"#### {line}")
    print("|Derivation for | After | Before |")
    print("|---------------|---------|-------|")
def handle_line(line):
    line = line.replace("\t", " ").replace("\n", "")
    match line:
        case "Tinantas":
            handle_section_header(line)
        case "Krdantas":
            handle_section_header(line)
        case s if " > " in s :
            handle_only_B(line)
        case s if " < " in s:
            handle_only_A(line)
        case s if " | " in s:
            handle_change(line)
        case _:
            print(line)

def process_file(fname):
    lines = openfile(fname)
    i = 0
    for line in lines:
        handle_line(line)
        i += 1
        if i > 10000:
            break
    for k in remainderA.keys():
        print(f"| {k} | {remainderA[k]} | - |")
    for k in remainderB.keys():
        print(f"| {k} | - | {remainderB[k]} |")

if __name__ == "__main__":
    process_file(sys.argv[1])