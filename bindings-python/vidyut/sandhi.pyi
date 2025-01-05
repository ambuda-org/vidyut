from dataclasses import dataclass
from pathlib import Path
from typing import List

@dataclass
class Split:
    first: str
    second: str
    is_valid: bool

class Splitter:
    @staticmethod
    def from_csv(path: Path | str) -> Splitter:
        pass
    def split_at(self, text: str, index: int) -> List[Split]:
        pass
