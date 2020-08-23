# albhed

A translator for the Al Bhed language.

Al Bhed is the language spoken by the Al Bhed people in the land of Spira,
the main setting of the game Final Fantasy X.

This library doesn't attempt to detect English loanwords that are present in
Al Bhed at the time of the game, as in airship/fiend/etc.

## Usage

Converting Al Bhed into English:
```rust
let al_bhed = "Oui yvnyet uv dra cay?";
let english = al_bhed.chars().filter_map(albhed::from_al_bhed).collect::<String>();
assert_eq!("You afraid of the sea?", english);
```

Converting English into Al Bhed:
```rust
let english = "Beware of Sandstorms!";
let al_bhed = english.chars().filter_map(albhed::to_al_bhed).collect::<String>();
assert_eq!("Pafyna uv Cyhtcdunsc!", al_bhed);
```

License: MIT
