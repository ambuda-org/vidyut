from dataclasses import dataclass
from pathlib import Path
from typing import List, Dict

class Prayoga(Enum):
    Kartari = None
    Karmani = None

class Purusha(Enum):
    Prathama = None
    Madhyama = None
    Uttama = None

class Vacana(Enum):
    Eka = None
    Dvi = None
    Bahu = None

class Lakara(Enum):
    Lat = None
    Lit = None
    Lut = None
    Lrt = None
    Let = None
    Lot = None
    Lan = None
    AshirLin = None
    VidhiLin = None
    Lun = None
    Lrn = None

class Linga(Enum):
    Pum = None
    Stri = None
    Napumsaka = None

class Sanadi(Enum):
    San = None
    Yan = None
    Nic = None

# TODO: add more krt-pratyayas
class Krt(Enum):
    ktvA = None
    kta = None

@dataclass
class Dhatu:
    upadesha: str

@dataclass
class Prakriya:
    text: str

class Vyakarana:
    def __init__(self):
        pass
    def derive_tinantas(
        self,
        dhatu: Dhatu,
        prayoga: Prayoga,
        purusha: Purusha,
        vacana: Vacana,
        lakara: Lakara,
        sanadi: Optional[Sanadi] = None,
    ) -> List[Prakriya]:
        pass
    def derive_subantas(self,
        pratipadika: Pratipadika,
        linga: Linga,
        vibhakti: Vibhakti,
        vacana: Vacana,
    ) -> List[Prakriya]:
        pass

class Dhatupatha:
    def __init__(self, path: Path | str):
        pass
    def __getitem__(self, code: str) -> Dhatu:
        pass
