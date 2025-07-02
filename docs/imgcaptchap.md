<div id="imgcaptchap"></div>

# imgcaptchap

<div id="Types"></div>

## Types

<div id="Geometry"></div>

## Geometry

Represents the area which contains text in a CAPTCHA.

<details>
<summary>Raw Type</summary>

```luau
--- Represents the area which contains text in a CAPTCHA.
type Geometry = {
	--- The minimum x coordinate of the area which contains text (inclusive).
	---
	--- Max size: u32
	left: number,

	--- The maximum x coordinate of the area which contains text (inclusive).
	---
	--- Max size: u32
	right: number,

	--- The minimum y coordinate of the area which contains text (inclusive).
	---
	--- Max size: u32
	top: number,

	--- The maximum y coordinate of the area which contains text (inclusive).
	--- 
	--- Max size: u32
	bottom: number
}
```

</details>

<div id="left"></div>

### left

The minimum x coordinate of the area which contains text (inclusive).



Max size: u32

[number](#number)

<div id="right"></div>

### right

The maximum x coordinate of the area which contains text (inclusive).



Max size: u32

[number](#number)

<div id="top"></div>

### top

The minimum y coordinate of the area which contains text (inclusive).



Max size: u32

[number](#number)

<div id="bottom"></div>

### bottom

The maximum y coordinate of the area which contains text (inclusive).



Max size: u32

[number](#number)

<div id="SerdeColor"></div>

## SerdeColor

Represents a color in RGB format.

<details>
<summary>Raw Type</summary>

```luau
--- Represents a color in RGB format.
type SerdeColor = {
	--- The red component of the color (0-255)
	r: number,

	--- The green component of the color (0-255)
	g: number,

	--- The blue component of the color (0-255)
	b: number
}
```

</details>

<div id="r"></div>

### r

The red component of the color (0-255)

[number](#number)

<div id="g"></div>

### g

The green component of the color (0-255)

[number](#number)

<div id="b"></div>

### b

The blue component of the color (0-255)

[number](#number)

<div id="ColorInvertFilter"></div>

## ColorInvertFilter

Filter that inverts the colors of an image.

<details>
<summary>Raw Type</summary>

```luau
--- Filter that inverts the colors of an image.
type ColorInvertFilter = {
	filter: "ColorInvert"
}
```

</details>

<div id="filter"></div>

### filter

```luau
"ColorInvert"
```

<div id="CowFilter"></div>

## CowFilter

Filter that generates a CAPTCHA with a specified number of cows (circles/other curvical noise)

<details>
<summary>Raw Type</summary>

```luau
--- Filter that generates a CAPTCHA with a specified number of cows (circles/other curvical noise)
type CowFilter = {
	filter: "Cow",

	--- The minimum radius of the cows
	---
	--- Max size: u32
	min_radius: number,

	--- The maximum radius of the cows
	---
	--- Max size: u32
	max_radius: number,

	--- The number of cows to generate
	---
	--- Max size: u32
	n: number,

	--- Whether to allow duplicate cows
	allow_duplicates: boolean,

	--- The geometry of the area which contains text
	geometry: Geometry?
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Cow"
```

<div id="min_radius"></div>

### min_radius

The minimum radius of the cows



Max size: u32

[number](#number)

<div id="max_radius"></div>

### max_radius

The maximum radius of the cows



Max size: u32

[number](#number)

<div id="n"></div>

### n

The number of cows to generate



Max size: u32

[number](#number)

<div id="allow_duplicates"></div>

### allow_duplicates

Whether to allow duplicate cows

[boolean](#boolean)

<div id="geometry"></div>

### geometry

The geometry of the area which contains text

*This field is optional and may not be specified*

[Geometry](#Geometry)?

<div id="DotFilter"></div>

## DotFilter

Filter that creates a specified number of dots

<details>
<summary>Raw Type</summary>

```luau
--- Filter that creates a specified number of dots
type DotFilter = {
	filter: "Dot",

	--- The number of dots to generate
	---
	--- Max size: u32
	n: number,

	--- The minimum radius of the dots
	---
	--- Max size: u32
	min_radius: number,

	--- The maximum radius of the dots
	---
	--- Max size: u32
	max_radius: number
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Dot"
```

<div id="n"></div>

### n

The number of dots to generate



Max size: u32

[number](#number)

<div id="min_radius"></div>

### min_radius

The minimum radius of the dots



Max size: u32

[number](#number)

<div id="max_radius"></div>

### max_radius

The maximum radius of the dots



Max size: u32

[number](#number)

<div id="GridFilter"></div>

## GridFilter

Filter that creates a grid (horizontal/vertical lines with a specified gap in X and Y direction)



(think graph paper)

<details>
<summary>Raw Type</summary>

```luau
--- Filter that creates a grid (horizontal/vertical lines with a specified gap in X and Y direction)
---
--- (think graph paper)
type GridFilter = {
	filter: "Grid",

	--- The Y gap between the vertical lines
	---
	--- Max size: u32
	y_gap: number,

	--- The X gap between the horizontal linesPrimitives.u32
	---
	--- Max size: u32
	x_gap: number
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Grid"
```

<div id="y_gap"></div>

### y_gap

The Y gap between the vertical lines



Max size: u32

[number](#number)

<div id="x_gap"></div>

### x_gap

The X gap between the horizontal linesPrimitives.u32



Max size: u32

[number](#number)

<div id="LineFilter"></div>

## LineFilter

Draw lines/rectangles on the screen

<details>
<summary>Raw Type</summary>

```luau
--- Draw lines/rectangles on the screen
type LineFilter = {
	filter: "Line",

	--- Point 1, must be an array of two numbers
	---
	--- Max size: f32
	p1: {number},

	--- Point 2, must be an array of two numbers
	---
	--- Max size: f32
	p2: {number},

	--- The thickness of the line
	---
	--- Max size: u32
	thickness: number,

	--- The color of the line
	color: SerdeColor
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Line"
```

<div id="p1"></div>

### p1

Point 1, must be an array of two numbers



Max size: f32

{[number](#number)}

<div id="p2"></div>

### p2

Point 2, must be an array of two numbers



Max size: f32

{[number](#number)}

<div id="thickness"></div>

### thickness

The thickness of the line



Max size: u32

[number](#number)

<div id="color"></div>

### color

The color of the line

[SerdeColor](#SerdeColor)

<div id="NoiseFilter"></div>

## NoiseFilter

Adds some random noise at a specified probability

<details>
<summary>Raw Type</summary>

```luau
--- Adds some random noise at a specified probability
type NoiseFilter = {
	filter: "Noise",

	--- The probability of adding noise
	---
	--- Max size: f32
	prob: number
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Noise"
```

<div id="prob"></div>

### prob

The probability of adding noise



Max size: f32

[number](#number)

<div id="RandomLineFilter"></div>

## RandomLineFilter

Creates a random line somewhere

<details>
<summary>Raw Type</summary>

```luau
--- Creates a random line somewhere
type RandomLineFilter = {
	filter: "RandomLine"
}
```

</details>

<div id="filter"></div>

### filter

```luau
"RandomLine"
```

<div id="WaveFilter"></div>

## WaveFilter

Creates a wave in a given direction (horizontal/vertical)

<details>
<summary>Raw Type</summary>

```luau
--- Creates a wave in a given direction (horizontal/vertical)
type WaveFilter = {
	filter: "Wave",

	--- The frequency of the wave
	---
	--- Max size: f32
	f: number,

	--- The amplitude of the wave
	---
	--- Max size: f32
	amp: number,

	--- The direction of the wave
	d: "horizontal" | "vertical"
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Wave"
```

<div id="f"></div>

### f

The frequency of the wave



Max size: f32

[number](#number)

<div id="amp"></div>

### amp

The amplitude of the wave



Max size: f32

[number](#number)

<div id="d"></div>

### d

The direction of the wave

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"horizontal"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"vertical"
```

</details>

<div id="Filter"></div>

## Filter

Represents a filter that can be applied to an image.

<details>
<summary>Raw Type</summary>

```luau
--- Represents a filter that can be applied to an image.
type Filter = ColorInvertFilter | CowFilter | DotFilter | GridFilter | LineFilter | NoiseFilter | RandomLineFilter | WaveFilter
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

[ColorInvertFilter](#ColorInvertFilter)

</details>

<details>
<summary>Variant 2</summary>

[CowFilter](#CowFilter)

</details>

<details>
<summary>Variant 3</summary>

[DotFilter](#DotFilter)

</details>

<details>
<summary>Variant 4</summary>

[GridFilter](#GridFilter)

</details>

<details>
<summary>Variant 5</summary>

[LineFilter](#LineFilter)

</details>

<details>
<summary>Variant 6</summary>

[NoiseFilter](#NoiseFilter)

</details>

<details>
<summary>Variant 7</summary>

[RandomLineFilter](#RandomLineFilter)

</details>

<details>
<summary>Variant 8</summary>

[WaveFilter](#WaveFilter)

</details>

<div id="CaptchaConfig"></div>

## CaptchaConfig

Captcha configuration

<details>
<summary>Raw Type</summary>

```luau
--- Captcha configuration
type CaptchaConfig = {
	--- The number of characters the CAPTCHA should have (0-255)
	char_count: number,

	--- The list of filters
	filters: {Filter},

	--- The size of the viewbox to render the CAPTCHA in.
	--- (first element is width, second element is height)
	---
	--- Max size: u32
	viewbox_size: {number},

	--- At what index of CAPTCHA generation should a viewbox be created at.
	---
	--- Max size: u32
	set_viewbox_at_idx: number
}
```

</details>

<div id="char_count"></div>

### char_count

The number of characters the CAPTCHA should have (0-255)

[number](#number)

<div id="filters"></div>

### filters

The list of filters

{[Filter](#Filter)}

<div id="viewbox_size"></div>

### viewbox_size

The size of the viewbox to render the CAPTCHA in.

(first element is width, second element is height)



Max size: u32

{[number](#number)}

<div id="set_viewbox_at_idx"></div>

### set_viewbox_at_idx

At what index of CAPTCHA generation should a viewbox be created at.



Max size: u32

[number](#number)

<div id="Captcha"></div>

## Captcha

<details>
<summary>Raw Type</summary>

```luau
type Captcha = {
	--- The text of the CAPTCHA.
	text: string,

	--- The image of the CAPTCHA (Luau buffer).
	image: buffer
}
```

</details>

<div id="text"></div>

### text

The text of the CAPTCHA.

[string](#string)

<div id="image"></div>

### image

The image of the CAPTCHA (Luau buffer).

[buffer](#buffer)

<div id="Plugin"></div>

## Plugin

<details>
<summary>Raw Type</summary>

```luau
type Plugin = {
	--- @yields
	---
	--- Creates a new CAPTCHA with the given configuration.
	Create: (config: CaptchaConfig) -> Captcha
}
```

</details>

<div id="Create"></div>

### Create

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a new CAPTCHA with the given configuration.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a new CAPTCHA with the given configuration.
Create: (config: CaptchaConfig) -> Captcha
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="config"></div>

##### config

[CaptchaConfig](#CaptchaConfig)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Captcha](#Captcha)