#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Fri Sep 25 21:14:09 2020

@author: john
"""


import numpy as np
import matplotlib.pyplot as plt

def plot(data, title, center_freq, rate_sps):

    # Frequency domain
    fft_size = len(data)
    fft = np.fft.fft(data)
    dt = 1.0 / rate_sps
    
    freqs_negative = [{'freq': (k-fft_size) / (fft_size*dt), 'mag': fft[k] / fft_size} for k in range(int(fft_size/2), fft_size)]
    freqs_positive = [{'freq': k / (fft_size*dt),            'mag': fft[k] / fft_size} for k in range(0, int(fft_size/2))]
    all_freqs = list(filter(lambda f: abs(f['freq']) > 0, freqs_negative + freqs_positive))
    
    plt.clf()
    plt.title(title)
    plt.subplot(311)
    plt.hist([np.real(d) for d in data], ec='k', alpha=0.5, color='#FF7777', label='I')
    plt.hist([np.imag(d) for d in data], ec='k', alpha=0.5, color='#7777FF', label='Q')
    plt.legend()
    
    plt.subplot(312)
    plt.plot([np.real(d) for d in data], 'r-', label='I')
    plt.plot([np.imag(d) for d in data], 'b-', label='Q')
    
    plt.subplot(313)
    plt.plot([f['freq']+center_freq for f in all_freqs], [abs(f['mag']) for f in all_freqs], 'b-')
