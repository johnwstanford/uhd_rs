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
filename = '/home/john/SDR/aviation_vhf/uhd/flight_2020_09_27/1601219410/output_127.00MHz_20.0Msps_gain60.0dB_sc16.dat'

N_SAMPLES = 1e6

_, tail = os.path.split(filename)
tail_split = tail.split('_')
center_freq = float(tail_split[1][:-3])*1.0e6
rate_sps = float(tail_split[2][:-4])*1.0e6

f_in = open(filename, 'rb')

assert(tail_split[4] == 'sc16.dat')
buffer = f_in.read(4)
while len(buffer) == 4 and len(data) < N_SAMPLES:
    re, im = struct.unpack('hh', buffer)
    data.append(re + 1j*im)
    buffer = f_in.read(4)
    
f_in.close()

plot(data, tail, center_freq, rate_sps)