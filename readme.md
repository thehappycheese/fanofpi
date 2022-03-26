# fanofpi

A rust service to drive a PWM fan.

Apparently it uses the `Channel::Pwm0` pin. Refer to `rppal` documentation and
the raspberry pi documentation to find out what that means.

Note that `dtoverlay=pwm` must be added somewhere in /boot/config.txt for this
to work. Note that this will prevent the analog audio output from working. See
more details about setup here
<https://docs.rs/rppal/latest/rppal/pwm/index.html>

Also note that software PWM is used by the linux kernel... I dont know what the
performance implications of that are. Seems like a bummer to me if you want
maximim performance out of your PI.

## Hardware

For the fan I used a level shifter was required. Note that in the diagram below, the fan shares the same ground as the PI.


```
                           .───.
                          /     \
                        ┌┴───────┴┐    
                        │         │
                        │  2N7000 │
                        │         │
                        │ S  G  D │
                        └─┬──┬──┬─┘
┌─────────────┐           │  │  │         ┌─────────┐
│ GPIO18 PWM0 ├───────────┤  │  ├─────────┤ FAN PWM │
└─────────────┘           │  │  │         └─────────┘
┌─────────────┐           │  │  │         ┌─────────┐
│     3.3V    ├──┬─<10kΩ>─┘  │  └─<10kΩ>──┤    5V   │
└─────────────┘  └───────────┘            └─────────┘

┌────────────────────────────────────────────────────────────────────────┐
│ ┌───────┬──────┬─┬─┬─┬───────────────┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐     │
│ │       │ 4) 5V│ │ │ │12) GPIO18 PWM0│ │ │ │ │ │ │ │ │ │ │ │ │ │ │     │
│ ├───────┼──────┼─┼─┼─┼───────────────┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─┤     │
│ │1) 3.3V│      │ │ │ │               │ │ │ │ │ │ │ │ │ │ │ │ │ │ │     │
│ └───────┴──────┴─┴─┴─┴───────────────┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘     │
│                                 .~~.   .~~.                            │
│                                '. \ ' ' / .'                           │
│                                 .~ .~~~..~.                            │
│                                : .~.'~'.~. :                           │
│                               ~ (   ) (   ) ~                          │
│                              ( : '~'.~.'~' : )                         │
│                               ~ .~ (   ) ~. ~                          │
│                                (  : '~' :  )                           │
│                                 '~ .~~~. ~'                            │
│                                     '~'                                │
└────────────────────────────────────────────────────────────────────────┘
```

# Install as Service

```bash
echo TODO: I forgot how I got systemd to run this on startup :(
```

