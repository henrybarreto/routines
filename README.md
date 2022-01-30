[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental)

![Routines' logo](logo.png)
Routines is an application check events and execute actions based on it.

## Configuration

Routines is configured through JSON file like this, what contains a list of events to be monitored.

**Example:**
```json
 {
   "events":[
        {
            "name": "battery_level",
            "condition": ">",
            "value": 50,
            "execute":"battery-saver.sh"
        },
        {
            "name": "cpu_frequency",
            "condition": "<",
            "value": "2000000",
            "execute":"cpu-governor-change.sh"
        }
   ]
}
```

...