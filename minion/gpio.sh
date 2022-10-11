#!/usr/bin/env bash

sudo chown root:$USER /dev/gpiomem
sudo chmod g+rw /dev/gpiomem
sudo chown root:$USER /dev/gpiochip0
sudo chmod g+rw /dev/gpiochip0
ls -l /dev/g*