.. dashi documentation master file, created by
   sphinx-quickstart on Thu Aug 26 09:09:41 2010.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

===================
dashi documentation
===================

Elaborate data analyses are possible with the functionality offered by the great
`numpy <http://numpy.scipy.org/>`_, `matplotlib
<http://matplotlib.sourceforge.net/>`_, and `pytables
<http://www.pytables.org/moin>`_ libraries. However their support for
HEP-typical problems (histograms, fitting routines,visualization of those,..) is
limited. Dashi is intended to be a thin wrapper around these libraries to
provide some useful tools for these problems without obstructing the user the access
to the underlying libraries and without being a dependency sink.

Installation
============

The easiest way to install dashi is with `pip`::

	pip install https://github.com/emiddell/dashi/zipball/master

.. Tutorials:
    ==========

.. .. toctree::
   :maxdepth: 1

   histograms
   fitting
   data_handling

API Reference
=============

.. toctree::
   :maxdepth: 2

   api/index

Documentation
=============

.. toctree::
   :maxdepth: 1

   data_handling


Examples
========

.. toctree::
   :maxdepth: 1

   examples/hist1d
   examples/hist2d
   examples/fitting
   examples/fitting_hist

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`

