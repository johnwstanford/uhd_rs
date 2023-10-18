#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sun Oct 15 23:29:17 2023

@author: john
"""

import os
import struct
import re as regex

import numpy as np
import matplotlib.pyplot as plt

FNAME_RE = regex.compile('([^/]+)_([AB])([01])_([^M]+)MHz_(\\d+)dB_([^M]+)Msps')
MAX_SAMPLES = 200000

channels = dict()

for fname in os.listdir('..'):
    m = FNAME_RE.match(fname)
    if not m:
        continue

    impl = m.group(1)
    if 'twinrx' not in impl:
        continue
    
    slot = m.group(2)
    chan = int(m.group(3))
    center_freq_mhz = float(m.group(4))
    gain_db = m.group(5)
    fs_msps = float(m.group(6))

    f = open('../%s'%fname, 'rb')
    buff = f.read(4)

    waveform = []

    re_sum = 0.0
    im_sum = 0.0

    while len(buff) == 4 and len(waveform) < MAX_SAMPLES:
        re, im = struct.unpack('hh', buff)
        re_sum += re
        im_sum += im
        waveform.append(re + (1j*im))
        buff = f.read(4)
        
    N = len(waveform)
    dc_ofs = (re_sum/N) + 1j*(im_sum/N)
        
    channels['%s%i'%(slot, chan)] = [x-dc_ofs for x in waveform]

for k0, k1, sp in [('A0', 'A1', 321), ('A0', 'B0', 322), ('A1', 'B0', 323), 
                   ('A0', 'B1', 324), ('A1', 'B1', 325), ('B0', 'B1', 326)]:
    
    wf0 = channels[k0]
    wf1 = channels[k1]
    N = min(len(wf0), len(wf1))
    
    phase_cmp = [wf0[i] * np.conj(wf1[i]) for i in range(N)]
    
    plt.subplot(sp)
    plt.title('%s vs %s'%(k0, k1))
    plt.hist([np.angle(x) for x in phase_cmp], bins=75, ec='k')
        
        
        
        
        
        
        