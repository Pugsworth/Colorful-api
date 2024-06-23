# Colorful Palette API
The focus of this program is to host an API for generating images of color palettes.
I thought about it when cataloging some interesting color palettes and wanted a "live preview".

Ideally when in a stable state, I want to push it to Fly.io or something similar.


## Design
I chose Rust after trying out Go and finding that it's a pretty terrible language, even if great for server-sided applications (like this).
I wanted something not only fast in responding to queries, but also as fast as possible for generating and serving the actual image data. Rust accomplishes this very well!
The API design mimics that of CLI-style argument parsing and consumption.
The primary "endpoint" is that of taking a list of colors and outputting an image.
`palette/?colors=col1,col2,col3,etc...`



# API
These are mainly in a state of idea or prototype at the moment.
## Legend
### Square Brackets
`[a,b,c...]` denotes a list of items separated by comma. The brackets are not included in the actual url.
e.g. `?colors=EFEAA1,AABBAA,1213AB`
### Angle Brackets
`<1-10>` denotes a valid range of values.
e.g. `?size=<1px-32px>` means that for the parameter `size`, the valid values are 1px to 32px.
### Curly Brackets
`{hue,sat,...}` denotes a map or key-value table.
e.g. `?adjust=hue=+1,sat=-10`
This one is subject to change.
### Parenthesis
`(a|b)` denotes an option between one or the other.
e.g. `?(order|sort)=...` means that the parameter name can either be `order` or `sort`.
### Regex
`/[0-9]/` denotes that the value matches this regex pattern.
e.g. `/#?[a-fA-F0-9]{3,}/` means to match any hex color of at least 3 characters consisting of a-f, A-F, or 0-9. Optionally prefixed with a '#' character.


- List of colors `?colors=[xxx,xxx,xxx,...]`
List of colors to use to generate.

- Image resolution `?res=<1px-32px>`
Force the output resolution in some capacity. This could be used to set the "size" of the individual color blocks. When combined, the image would have a certain resolution that makes sense, i.e. 32px blocks with six colors gives a **192x32px** image.  
⚠ Requires limiting
ℹ A secondary mode for inverting the logic and setting the overall size would be very beneficial.

- Block size `?bs=<1..128>`
The pixel size of the color blocks within the image. This is exclusive with `res`.
This is going to limited to something like '64px' or '128px' since there's no reason to ever need to generate palette images *that* big. At some point, it just becomes an attack vector on the API.

- Adjustments `?adjust={hue,sat,val,...}`
Apply basic adjustments to the entire color palette. This would allow shifting the colors around  
⚠ Requires smart limits for wrapping or reaching the edges.

- Palette layouts `?layout=[strip,grid,mason,...]`
Output image can be in a special layout for a more aesthetic appearance or to help better convey the palette.

- Labeling `?label={color,gravity,font,format}`
Apply labels to the color blocks. The parameters would allow for tweaking how and where the label is placed.  
⚠ Requires minimum block sizes.
ℹ Pixel fonts could be very useful here.

- Presets `?preset=[material,bootstrap, ...]`
Provide common preset palettes from other software or standards.  
ℹ Extra parameters specific to this will be necessary for things such as limiting color quantity or selecting versions.

- Order `?(order|sort)=[hue,lum,...]`
Rearranging or sorting the input colors into a format that makes more sense depending on application. For example, sorting by luminance or hue.  
ℹ Figure out what kind of color sorting algorithms exist and pick a couple that make sense. Don't go overboard here!

- Seed `?seed=/[0-9]+/`
Some functions might have an intrinsic randomness that can be chosen from a seed.  
❔ This might not even be necessary.


## What this API is not
While I want this to have a good amount of functionality, there are a couple things I don't want this to do at all. Some of these might be good projects to implement as a separate API in the future.

- Generation/AI/Completion
Right now, I have no interest in this being a tool to generate palettes with or without AI or to Complete partial palettes.

- Blending
Blending the color palette into a smaller or larger one. This some-what fits into the **generation** functionality, but it's a bit more specific.

- Storage
I don't intend to actually store any information given the purposes of re-use or fetching for the user. All palettes will be generated dynamically and cache may only be considered if it would actually improve performance anywhere (don't think it would!)
