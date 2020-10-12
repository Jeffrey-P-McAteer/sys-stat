#!/usr/bin/env python3

import sys
import csv
import datetime
import pprint
import random

import matplotlib.pyplot as plt # python3 -m pip install --user matplotlib
import numpy as np # python3 -m pip install --user numpy

if __name__ == '__main__':
  status_csv = "/tmp/status.csv" if len(sys.argv) < 2 else sys.argv[1]
  print("reading from status_csv={}".format(status_csv))

  system_data = {
    # Format:
    # "system name": [(timestamp in epoch seconds, up status true/false, latency milliseconds)]
  }

  min_timestamp = 999999999999999999999.0
  max_timestamp = 0.0
  with open(status_csv) as csvfile:
    reader = csv.reader(csvfile)
    for row in reader:
      sys_name = row[1].strip()
      if not sys_name in system_data:
        system_data[sys_name] = []
      
      time_epoch_s = int( datetime.datetime.fromisoformat(row[0].split(".")[0]).timestamp() )
      up_or_down = "t" in row[2]
      latency_ms = int(row[3])
      
      system_data[sys_name].append(
        (time_epoch_s, up_or_down, latency_ms)
      )

      if time_epoch_s < min_timestamp:
        min_timestamp = time_epoch_s
      if time_epoch_s > max_timestamp:
        max_timestamp = time_epoch_s
        


  # debugging
  #pprint.PrettyPrinter(indent=4).pprint(system_data)

  fig = plt.figure()
  ax = plt.axes()

  ax.set_xlabel('time (epoch seconds)')
  ax.set_ylabel('latency (milliseconds)')

  colors = ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd', '#8c564b', '#e377c2', '#7f7f7f', '#bcbd22', '#17becf']

  for sys_name, sys_data in system_data.items():
    color = colors.pop()
    x = [x[0] for x in sys_data if x[1]]
    y = [x[2] for x in sys_data if x[1]]
    plt.scatter(x, y, color=color, label=sys_name)

    x = [x[0] for x in sys_data]
    y = [x[2] for x in sys_data]
    plt.plot(x, y, color=color, label='_nolegend_')

    x = [x[0] for x in sys_data if not x[1]]
    y = [x[2] for x in sys_data if not x[1]]
    plt.scatter(x, y, color='red', marker='X', label='_nolegend_')

  ax.legend()
  plt.show()

