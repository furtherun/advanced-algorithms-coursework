# -*- coding: utf-8 -*-
"""
Created on Tue Jun  7 12:44:02 2022

@author: Furtherun

Draw for big integer multplation
"""

import numpy as np
import matplotlib.pyplot as plt

xlist = []
ylist1 = []
ylist2 = []
ylist3 = []

with open("datas/data1.csv") as f:
    for line in f:
        x, y1, y2, y3 = line.split(',')
        xlist.append(np.log10(float(x)))
        ylist1.append(np.log10(float(y1)))
        ylist2.append(np.log10(float(y2)))
        ylist3.append(np.log10(float(y3)))

fig1 = plt.figure()

plt.plot(xlist, ylist1, 'o--', color='red', label='origin:mult')
plt.plot(xlist, ylist2, 'o--', color='blue', label='origin:mult_recur')
plt.plot(xlist, ylist3, 'o--', color='green', label='origin:mult_recur_pro')

plt.legend(loc='upper left')

plt.savefig('results/result1_py.png')

xlist_better = []
ylist1_better = []
ylist2_better = []
ylist3_better = []

with open("datas/data2.csv") as f:
    for line in f:
        x, y1, y2, y3 = line.split(',')
        xlist_better.append(np.log10(float(x)))
        ylist1_better.append(np.log10(float(y1)))
        ylist2_better.append(np.log10(float(y2)))
        ylist3_better.append(np.log10(float(y3)))

fig2 = plt.figure()

plt.plot(xlist_better, ylist1_better, 'o--', color='pink', label='better:mult')
plt.plot(xlist_better, ylist2_better, 'o--', color='lightblue', label='better:mult_recur')
plt.plot(xlist_better, ylist3_better, 'o--', color='lightgreen', label='better:mult_recur_pro')

plt.legend(loc='upper left')

plt.savefig('results/result2_py.png')

fig3 = plt.figure()

plt.plot(xlist, ylist1, 'o--', color='red', label='origin:mult')
plt.plot(xlist, ylist2, 'o--', color='blue', label='origin:mult_recur')
plt.plot(xlist, ylist3, 'o--', color='green', label='origin:mult_recur_pro')
plt.plot(xlist_better, ylist1_better, 'o--', color='pink', label='better:mult')
plt.plot(xlist_better, ylist2_better, 'o--', color='lightblue', label='better:mult_recur')
plt.plot(xlist_better, ylist3_better, 'o--', color='lightgreen', label='better:mult_recur_pro')

plt.legend(loc='upper left')

plt.savefig('results/result_cmp_py.png')