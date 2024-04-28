# power law analysis scripts
import powerlaw
import csv 
import os 
import numpy as np 
import matplotlib.pyplot as plt 

data = genfromtxt('py-powerlaw-import.txt')
results = powerlaw.Fit(data)
print(results.power_law.alpha)
print(results.power_law.xmin)
R, p = results.distribution_compare('power_law', 'lognormal')



####
# fit = powerlaw.Fit(data, discrete=True, estimate_discrete=False)
# print(fit.xmin)
# print(fit.alpha)
# print(fit.power_law._pdf_discrete_normalizer)
# print(fit.distribution_compare('power_law', 'lognormal'))
