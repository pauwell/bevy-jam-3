# Fungal Fury

My entry for **Bevy Jam #3**. 

---

## Build 💻
Run dev:
```
$ cargo run
```

Build release:
```
$ cargo build --release
```

Build wasm:
```
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/fungal_fury.wasm
```

---

## Assets & Tools 🖼️
Sprites, sounds and music created by me in the span of the jam.
Tools used:
 - Asesprite 
 - Ableton Live 10
 - Jsfxr (https://sfxr.me/)

---

## Theme Of The Jam ⭐
**Side Effects**
*Fungal Fury* explores this theme by having the player eat mushrooms that have negative consequences, while also being the only weapon against the creatures' nests.

--- 

## Story 📚

You're wandering through an unfamiliar, dense forest when you come across a group of people who are in dire need of your assistance. Strange, slimy creatures have taken over the path to their home. They explain that you must eliminate the creatures' nests, of which there are three in total. However, the only way to destroy the nests is by using some peculiar mushrooms that are consumed by those beings. You must defeat the creatures by utilizing their own abilities against them, but be cautious of the mushrooms' consequences...

---

## Goal Of The Game 🥅
You win the game by destroying all three spawners in the forest. Those spawners can only be damaged by attacking it with your sword while being under the influence of one of the mushrooms dropped by defeated enemies.

--- 

## Controls ⌨️
```Arrow Keys``` - Walk around
```Space``` - Talk to NPCs
```D``` - Use magic attack
```S``` - Use melee attack
```P``` - Pause/ Resume

---

## Mushroom Effects 🍄
Each mushroom has one of the following effects when being picked up:
 - You walk slower
 - You can't use magic
 - You can't use melee
 - You lose half your current lifepoints
 - Your lifepoints are refilled

--- 
## NPCs 🧑‍🤝‍🧑

### Marcus 👨‍🦱
During visits, Marcus has the ability to heal the player. Even though he doesn't talk much, he's a friendly guy.

### Andrea 👱‍♀️
Although Andrea is of small height, she has a big heart. She is very grateful for being rescued from the creatures so she can finally head back home.

### Piet 👨‍🦳
Like you, Piet is a wanderer who ended up stranded here due to the creatures.

### Cat 🐱
Small and adorable.

