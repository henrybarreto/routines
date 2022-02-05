[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental)

![Routines' logo](logo.png)
Routines is an application check events and execute actions based on it.

## Configuration

Routines is configured through RON file like this, what contains a list of events to be monitored.

**Example:**
```ron
[
    Event(
        name: "battery_level",
        condition: Great,
        value: "50",
        execute: "battery_saver.sh"
    ),
    Event(
        name: "cpu_frequency",
        condition: Less,
        value: "2000000",
        execute: "cpu_governor_saver.sh"
    ),
]
```

...