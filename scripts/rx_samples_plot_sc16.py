#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Fri Sep 25 21:14:09 2020

@author: john
"""


import os
import struct

import numpy as np

from plot_signal_summary import plot

data = []
filename = '../output_95.00MHz_1.00e7sps.dat'

N_SAMPLES = 1e5

_, tail = os.path.split(filename)
tail_split = tail.split('_')
center_freq = float(tail_split[1][:-3])*1.0e6
rate_sps = float(tail_split[2][:-4].replace('sps', ''))

f_in = open(filename, 'rb')

buffer = f_in.read(4)
while len(buffer) == 4 and len(data) < N_SAMPLES:
    re, im = struct.unpack('hh', buffer)
    data.append(re + 1j*im)
    buffer = f_in.read(4)
    
f_in.close()

plot(data, tail, center_freq, rate_sps)