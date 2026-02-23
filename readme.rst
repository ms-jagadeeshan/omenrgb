omenrgb
=======

CLI tool for controlling 4-zone RGB keyboards
on Linux using the Omen RGB sysfs driver.

It supports:

- Per-zone color control
- Global color control
- Brightness adjustment (with +/- support)
- Animation mode & speed control
- Presets
- Gradient generator
- Random lighting
- JSON status output

Prerequisites
=============

This CLI requires the ``omen-rgb-keyboard`` kernel driver to be installed first.

Project repository:
https://github.com/alessandromrc/omen-rgb-keyboard

Install the driver::

    git clone https://github.com/alessandromrc/omen-rgb-keyboard.git
    cd omen-rgb-keyboard
    make
    sudo make install

After installation, verify that the sysfs endpoint exists::

    ls /sys/devices/platform/omen-rgb-keyboard/rgb_zones/

You should see entries like::

    all
    brightness
    animation_mode
    animation_speed
    zone00
    zone01
    zone02
    zone03

Installation
===========

Build from source::

    git clone https://github.com/ms-jagadeeshan/omenrgb
    cd omenrgb
    make install
    # or
    # sudo make install

Basic usage
===========
Set individual zones::

    omenrgb set 0 red
    omenrgb set zone1 blue
    omenrgb set zone02 #00ffaa
    omenrgb set 3 f0a

Set all zones::

    omenrgb all white

Supported color formats:

- RRGGBB
- #RRGGBB
- 3-digit shorthand (f0a)
- Named colors (red, green, blue, yellow, cyan, magenta, white, black, gray, orange, purple, pink, lime, teal, maroon, navy, aqua, olive, silver, brown, lavender)

Brightness
----------

Set brightness (0-100)::

    omenrgb brightness 50

Increment / decrement::

    omenrgb brightness +10
    omenrgb brightness -20

Turn off lighting::

    omenrgb brightness 0


Animation Modes
---------------

Available modes:

- static
- breathing
- rainbow
- wave
- pulse
- chase
- sparkle
- candle
- aurora
- disco

Set mode::

    omenrgb animation breathing  7
    omenrgb animation rainbow  -2

Status
======

Human readable::

    omenrgb status

JSON output (script friendly)::

    omenrgb status --json


Presets
=======

Save current state::

    omenrgb preset save gaming

Load preset::

    omenrgb preset load gaming

List presets::

    omenrgb preset list

Presets are stored in::

    ~/.config/omenrgb/


Gradient
========

Generate a 4-zone gradient::

    omenrgb gradient red blue
    omenrgb gradient 00ff00 ff00ff


Random
======

Randomize zones::

    omenrgb random --zones

Randomize animation mode::

    omenrgb random --mode

Both::

    omenrgb random --zones --mode


Others
======

Focus mode::

    omenrgb all 4444FF
    omenrgb brightness 40

Command Result Feedback::

    # Add to .bashrc
    export PROMPT_COMMAND='if [ $? -ne 0 ]; then omenrgb set all red; else omenrgb set all green; fi'

Battery level indicator::

    BAT=$(cat /sys/class/power_supply/BAT0/capacity)

    if [ $BAT -lt 20 ]; then
      omenrgb set all red
    elif [ $BAT -lt 50 ]; then
      omenrgb set all yellow
    else
      omenrgb set all green
    fi


Cron based day/night switching::

    0 8 * * * omenrgb preset load day
    0 22 * * * omenrgb preset load night
