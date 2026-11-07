# Roadmap → v0_02

Ideas for the next version (pick and refine with the developer):

- **Procedural dungeon generation** — replace the hand-built demo map with a
  generated one (e.g. random rooms + corridors, or a drunkard's-walk cave).
- **Sprite/animation assets** — replace colored rectangles with a sprite sheet
  and simple walk animation.
- **Isometric projection** — move from a top-down square grid to a diamond
  (2:1) isometric view, the signature early-Diablo look.
- **Entities/ECS** — introduce a lightweight entity layer (or a full ECS) so
  the player, and later enemies/items, share a common update/draw path.
- **Enemies & combat** — a simple enemy that chases the player and a basic
  attack.
- **Larger maps & a minimap** — bigger generated worlds and a minimap overlay.

Recommended order: procedural generation first — it makes the engine feel
Diablo-like and builds directly on the map/collision work in v0_01.
