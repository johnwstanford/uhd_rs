#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Sun Oct 15 23:29:17 2023

@author: john
"""


import numpy as np
import matplotlib.pyplot as plt

import twinrx_common

dwells = twinrx_common.load_dwells('..')

for k0, k1 in [('A0', 'A1'), ('A0', 'B0'), ('A0', 'B1')]:

    points = []
    
    for dwell_num in dwells.keys():
        
        dwell = dwells[dwell_num]
        wf0 = dwell[k0]
        wf1 = dwell[k1]
        N = min(len(wf0), len(wf1))
        
        phase_cmp = [wf0[i] * np.conj(wf1[i]) for i in range(N)]
        mu, sig = twinrx_common.rotating_fit(phase_cmp)
        points.append((dwell_num, mu, sig))
        
    points.sort(key=lambda x: x[0])
        
    x = [i for i, _, _ in points]
    mean_lines = plt.plot(x, [mu for _, mu, _ in points], label='%s-%s'%(k0, k1))
    c = mean_lines[0].get_c()
    
    y_lo = [mu-sg for _, mu, sg in points]
    y_hi = [mu+sg for _, mu, sg in points]
    plt.plot(x, y_lo, color=c, ls='dashed', lw=0.5)
    plt.plot(x, y_hi, color=c, ls='dashed', lw=0.5)
    plt.gca().fill_between(x, y_lo, y_hi, color=c, alpha=0.5)
    
plt.legend()
plt.ylabel('Relative Phase [radians]')
    
    
        
        
        
        