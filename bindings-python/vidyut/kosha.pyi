from pathlib import Path
from enum import Enum

class PartOfSpeech(Enum):
    Tinanta = None
    Subanta = None
    Avyaya = None

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

class PadaPrayoga(Enum):
    Parasmaipada = None
    AtmanepadaKartari = None
    AtmanepadaNotKartari = None

class Linga(Enum):
    Pum = None
    Stri = None
    Napumsaka = None

class Vibhakti(Enum):
    V1 = None
    V2 = None
    V3 = None
    V4 = None
    V5 = None
    V6 = None
    V7 = None
    Sambodhana = None

@dataclass
class Dhatu:
    text: str

    def __init__(self, text: str):
        pass

@dataclass
class Pratipadika:
    text: str

    def __init__(self, text: str):
        pass

@dataclass
class Pada:
    pos: Optional[PartOfSpeech]
    dhatu: Optional[Dhatu]
    pratipadika: Optional[Pratipadika]
    purusha: Optional[Purusha]
    vacana: Optional[Vacana]
    lakara: Optional[Lakara]
    linga: Optional[Linga]
    vibhakti: Optional[Vibhakti]
    pada_prayoga: Optional[PadaPrayoga]
    is_purvapada: bool

    @staticmethod
    def make_tinanta(
        *,
        dhatu: Dhatu,
        purusha: Purusha,
        vacana: Vacana,
        lakara: Lakara,
        pada_prayoga: PadaPrayoga,
    ) -> Pada:
        pass
    @staticmethod
    def make_subanta(
        *, pratipadika: Pratipadika, linga: Linga, vibhakti: Vibhakti, vacana: Vacana
    ) -> Pada:
        pass
    @staticmethod
    def make_avyaya(*, pratipadika: Pratipadika) -> Pada:
        pass

class Builder:
    def __init__(self, path: Path | str):
        pass
    def insert(self, key: str, pada: Pada):
        pass
    def finish(self):
        pass

class Kosha:
    def __init__(self, path: Path | str):
        pass
    def __contains__(self, key: str) -> bool:
        pass
    def contains_prefix(self, key: str) -> bool:
        pass
    def get_all(self, key: str) -> List[Pada]:
        pass
