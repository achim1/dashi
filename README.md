# dashi 


Elaborate data analyses are possible with the functionality offered by the great
`numpy <http://numpy.scipy.org/>`_, `matplotlib
<http://matplotlib.sourceforge.net/>`_, and `pytables
<http://www.pytables.org/moin>`_ libraries. However their support for
HEP-typical problems (histograms, fitting routines,visualization of those,..) is
limited. Dashi is intended to be a thin wrapper around these libraries to
provide some useful tools for these problems without obstructing the user the access
to the underlying libraries and without being a dependency sink.

Kanidashi now is dashi on steroids. With the leverage of the powerful rust programming 
language, performance bottlenecks within dashi are addressed. Especially for very large 
datasets, where we are unable to hold numpy arrays for the whole dataset in memory, 
using the `.fill()` methods on the histograms directly can be a solution. 
However, that can be a bit of a peformance bottleneck. 

Kanidashi tries to address this. Using the powerful ndhistogram rust library as a 
backend, all the critical infrastructure of dashi has been factored out while keeping 
the frontend, user interface as well as the interface to matplotlib the same. 

This allows for a performance boost of XX for a XX dataset using `.fill()` 

The project is currently in an early alpha stage, for production use the original 
dashi library is recommended. 

# Installation

The easiest way to install dashi is with `pip`::

	pip install kanidashi

# Documentation

