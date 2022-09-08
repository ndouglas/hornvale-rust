# Hornvale
An experimental prose-based roguelike, written in Rust.

I have two relevant favorite games:
- _After the Plague II_, a Hungarian MUD I played about twenty years ago
- _Zork_ (I-III), the legendary text adventures from Infocom

I've wanted to capture/recreate/whatever the experiences I had with these games for a long time, but I was sort of stuck in a dichotomous way of thinking about it: either I could make a roguelike, which satisfied the dungeon-crawling sort of aspect, or a MUD, which satisfied the prose sort of aspect.

So I thought I might experiment with a prose-based roguelike that incorporates elements from text adventures and MUDs.

Elements this would keep/not keep from each genre (traditionally speaking):
- roguelikes:
  - keep:
    - procedural generation
    - permadeath
    - general complexity and depth of play
    - combat system
  - _not_ keep:
    - the top-down dungeon view
- MUDs:
  - keep:
    - prose descriptions of areas
    - realtime (actions take place in the world regardless of player action)
    - broad focus (multiple areas, factions, quests, plotlines, etc)
    - combat system
  - _not_ keep:
    - multiplayer
    - pre-written world
- text adventures:
  - keep:
    - focus on prose
    - focus on an individual's journey
    - complex, interactive NPCs and devices
  - _not_ keep:
    - narrow focus (anything not directly serving the principal narrative should be discarded)
    - puzzle-based quests (these would tend to lose novelty very quickly)

More specific ideas:

- TA/MUD-like movement (`enter`/`exit`/<Compass Directions (`north`, `ne`)>/<Contextual (`board ship`, `jump off cliff`)>)
- Multiple levels of movement (macroscopic [traveling across the plains] and microscopic [moving between rooms in a house or dungeon]), moved between via `enter`/`exit` (the town/the castle/whatever)
- Heavy focus on procedural generation.  If it can't be procedurally generated, it shouldn't be in the game.
- Weather systems/celestial phenomenae/etc.  See _Minecraft_, etc.
- Very, very complex NPCs.  See _Dwarf Fortress_.
- Idling.  You should be able to just sit and watch interactions between NPCs.
- Persistence.  I'd like NPCs, the player, etc, to have measureable, visible effects on their environment.  If goblins deforest an area, that should mean something within the world of the game.
- Extensibility.  I want this to be data-oriented, very well-factored, and easy to extend.

