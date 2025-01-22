"""Python bindings for Vidyut.

This library defines Python bindings to Vidyut, a Rust project that provides
reliable infrastructure for Sanskrit software. These bindings are thin wrappers
over the underlying Rust code, but we have done what we can to make these
bindings Pythonic.

This library's main modules are:

- `vidyut.chandas`, which analyzes Sanskrit meters..
- `vidyut.cheda`, which segments Sanskrit expressions.
- `vidyut.kosha`, which compactly stores Sanskrit words.
- `vidyut.lipi`, which transliterates Sanskrit text.
- `vidyut.prakriya`, which generates Sanskrit words.
- `vidyut.sandhi`, which splits Sanskrit expressions.

In general, all Vidyut code expects that Sanskrit text uses the SLP1
transliteration format. For details on how this form is defined, see
https://en.wikipedia.org/wiki/SLP1.
"""

from vidyut import vidyut as __mod

__version__ = __mod.__version__


def download_data(path):
    """Downloads Vidyut's linguistic data and saves it to `path`."""
    from io import BytesIO
    import urllib.request
    from zipfile import ZipFile
    url = (
        "https://github.com/ambuda-org/vidyut/releases/download/py-0.4.0/data-0.4.0.zip"
    )
    print(f"Downloading {url} ...")

    resp = urllib.request.urlopen(url)
    archive = ZipFile(BytesIO(resp.read()))
    archive.extractall(path=path)
    print(f"Complete. (Wrote data to `{path}`)")
