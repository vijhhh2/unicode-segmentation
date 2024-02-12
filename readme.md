# Unicode Segmentation

##### This library provides node binding for rust package [Unicode Segmentation Crate](https://crates.io/crates/unicode-segmentation)

## Installation
```shell
npm install -s unicode-segmentation
```

## Usage
```typescript
import { graphemes } from "unicode_segmentation";

const stringToSplit = "Lorem 😂😂 ipsum 🕵️‍♂️dolor adipiscing😇😇🤙 elit, sed do eiusmod🥰 tempor 😤😤🏳️‍🌈."

const result = graphemes(stringToSplit)

console.log(result);
/* Output
[
  'L',  'o',  'r',  'e',   'm', ' ', '😂', '😂',
  ' ',  'i',  'p',  's',   'u', 'm', ' ',  '🕵️‍♂️',
  'd',  'o',  'l',  'o',   'r', ' ', 'a',  'd',
  'i',  'p',  'i',  's',   'c', 'i', 'n',  'g',
  '😇', '😇', '🤙', ' ',   'e', 'l', 'i',  't',
  ',',  ' ',  's',  'e',   'd', ' ', 'd',  'o',
  ' ',  'e',  'i',  'u',   's', 'm', 'o',  'd',
  '🥰', ' ',  't',  'e',   'm', 'p', 'o',  'r',
  ' ',  '😤', '😤', '🏳️‍🌈', '.'
]
*/
```

This library exposes all functions from the [Unicode Segmentation Crate](https://crates.io/crates/unicode-segmentation). For more detailed documentation please visit this [link](https://unicode-rs.github.io/unicode-segmentation/unicode_segmentation/trait.UnicodeSegmentation.html) 