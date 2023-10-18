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

for k0, k1, sp in [('A0', 'A1', 321), ('A0', 'B0', 322), ('A1', 'B0', 323), 
                   ('A0', 'B1', 324), ('A1', 'B1', 325), ('B0', 'B1', 326)]:
    
    plt.subplot(sp)

    for impl in dwells.keys():
        
        dwell = dwells[impl]
        wf0 = dwell[k0]
        wf1 = dwell[k1]
        N = min(len(wf0), len(wf1))
        
        phase_cmp = [wf0[i] * np.conj(wf1[i]) for i in range(N)]
        
        plt.title('%s vs %s'%(k0, k1))
        plt.hist([np.angle(x) for x in phase_cmp], bins=75, ec='k', alpha=0.7, label=impl)

    plt.xlim((-np.pi, np.pi))
    
    
        
        
        
        