# @antiraid/img_captcha

This plugin allows for the creation of text/image CAPTCHA's with customizable filters which can be useful in protecting against bots.

## Types

<div id="type.CaptchaConfig" />

### CaptchaConfig

Captcha configuration. See examples for the arguments

```json
{
  "char_count": 5,
  "filters": [
    {
      "filter": "Noise",
      "prob": 0.1
    },
    {
      "filter": "Wave",
      "f": 4.0,
      "amp": 2.0,
      "d": "horizontal"
    },
    {
      "filter": "Line",
      "p1": [
        1.0,
        0.0
      ],
      "p2": [
        20.0,
        20.0
      ],
      "thickness": 2.0,
      "color": {
        "r": 0,
        "g": 30,
        "b": 100
      }
    },
    {
      "filter": "RandomLine"
    },
    {
      "filter": "Grid",
      "y_gap": 30,
      "x_gap": 10
    },
    {
      "filter": "ColorInvert"
    }
  ],
  "viewbox_size": [
    512,
    512
  ],
  "set_viewbox_at_idx": null
}
```

#### Fields

- `char_count` ([u8](#type.u8)): The number of characters the CAPTCHA should have.
- `filters` ([{any}](#type.any)): See example for the parameters to pass for the filter as well as https://github.com/Anti-Raid/captcha
- `viewbox_size` ([(u32, u32)](#type.(u32, u32))): The size of the viewbox to render the CAPTCHA in.
- `set_viewbox_at_idx` ([Option<usize>](#type.Option<usize>)): At what index of CAPTCHA generation should a viewbox be created at.


## Methods

### new

```lua
function new(config: CaptchaConfig): {u8}
```

Creates a new CAPTCHA with the given configuration.

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



#### Parameters

- `config` ([CaptchaConfig](#type.CaptchaConfig)): The configuration to use for the CAPTCHA.


#### Returns

- `captcha` ([{u8}](#type.u8)): The created CAPTCHA object.