"""
Kanidashi - dashi on steroids powered by rust

Dashi is a histogramming library, originally 
developed to work with HEP data.

It provides a sleek, easy to use interface 
to create pretty histograms with matplotlib.
"""

from . import kanidashi_core as _kd

__version__ = _kd.get_version()

# sphinx autodoc needs to know th correct module name
get_version = _kd.get_version 
get_version.__module__ = __name__

get_version_major = _kd.get_version_major
get_version_major.__module__ = __name__ 

get_version_minor = _kd.get_version_minor
get_version_minor.__module__ = __name__ 

get_version_patch = _kd.get_version_patch
get_version_patch.__module__ = __name__

version_at_least  = _kd.version_at_least 
version_at_least.__module__ = __name__ 


from . import histfactory as factory
from .fitting import model,gaussian,poly,leastsq #,poissonllh 
from .visual import visual
from .storage import histsave,histload
from .objbundle import bundle, emptybundle, bundleize

from . import junkbox

import logging
logging.basicConfig(level=logging.INFO)

from . import tests
