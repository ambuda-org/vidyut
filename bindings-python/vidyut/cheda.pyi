from dataclasses import dataclass
from pathlib import Path
from typing import List, Dict

from vidyut.kosha import Pada

@dataclass
class Token:
    text: str
    lemma: str
    info: Pada

class Chedaka:
    def __init__(self, path: Path | str):
        pass
    def run(self, slp1_text: str) -> List[Token]:
        pass
