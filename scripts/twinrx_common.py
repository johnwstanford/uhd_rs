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
from scipy import stats

FNAME_RE = regex.compile('([^/]+)_([AB])([01])_([^M]+)MHz_(\\d+)dB_([^M]+)Msps')
IMPL_RE = regex.compile('\\D*(\\d+)')
MAX_SAMPLES = 200000

def rotating_fit(phase_cmp):
    best_fit = (0.0, 2*np.pi)
    for shift_90 in range(4):
        angles = [np.angle(x * pow(1j, shift_90)) for x in phase_cmp]
        
        fit = stats.norm.fit(angles)
        if fit[1] < best_fit[1]:
            best_fit = (fit[0] - (0.5*np.pi*shift_90), fit[1])

    mu, sig = best_fit
    while mu < -np.pi:
        mu += 2*np.pi
    while mu > np.pi:
        mu -= 2*np.pi
    
    return (mu, sig)
    

def load_dwells(path):

    dwells = dict()
    
    for fname in os.listdir(path):
        m = FNAME_RE.match(fname)
        if not m:
            continue
    
        impl = m.group(1)
        m_impl = IMPL_RE.match(impl)
        if not m_impl:
            continue
        
        dwell_num = int(m_impl.group(1))
        
        if dwell_num not in dwells:
            dwells[dwell_num] = dict()
        
        slot = m.group(2)
        chan = int(m.group(3))
        # center_freq_mhz = float(m.group(4))
        # gain_db = m.group(5)
        # fs_msps = float(m.group(6))
    
        f = open('%s/'%path + fname, 'rb')
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
            
        dwells[dwell_num]['%s%i'%(slot, chan)] = [x-dc_ofs for x in waveform]
        
    return dwells

