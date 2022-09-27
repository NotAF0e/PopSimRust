import os

os.system("cythonize -b -a popsimc.pyx")

from popsimc import *
