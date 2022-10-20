## I've archived this because I've shifted my efforts to a total rewrite.  Check out the new hotness [here](https://github.com/ndouglas/hornvale/).

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

- Multiple levels of movement (macroscopic [traveling across the plains] and microscopic [moving between rooms in a house or dungeon]), moved between via `enter`/`exit` (the town/the castle/whatever)
- Heavy focus on procedural generation.  If it can't be procedurally generated, it shouldn't be in the game.
- Weather systems/celestial phenomenae/etc.  See _Minecraft_, etc.
- Very, very complex NPCs.  See _Dwarf Fortress_.
- Idling.  You should be able to just sit and watch interactions between NPCs.
- Persistence.  I'd like NPCs, the player, etc, to have measureable, visible effects on their environment.  If goblins deforest an area, that should mean something within the world of the game.
- Extensibility.  I want this to be data-oriented, very well-factored, and easy to extend.
- Make macros for action events.
- Add event subscriber struct.
- Add event handler struct??? IDK.
- Figure out what to do with message formats.
- Make message macros for general errors, etc.
- I think text effects are out, but it'd be nice to demo some good formatting.
- Add a mob!  We need something else animate so we can start working with event reactions, etc.
- Change the spammer to be a little more interesting, maybe.  It might be time to start writing weather systems or whatever.
- Start poaching ideas from TADS, Inform, ADoM, NetHack, AtP, etc.
- Start dealing with idea of energy/metabolism/speed/etc.
- Pass through command parsing to player object, room, items in room, etc for commands that only make sense with a specific contextual element.
- Try to simplify systems.
- Add Results to Commands.
- Moving back to ECS, now that I've sampled the alternative, lol.
- Move messages to the IO module.
- Attributes.
- Species.
- Faction.
- Character build process.  I like the tug-the-threads model of AtP II.
  - Species should give the attribute starting values, skill bonuses/penalties.  (Ogres will suck at lockpicking, faerie at taking hits)
  - No class.  I want a Skyrim-style classless system.
  - Sex/Gender is an interesting question.  I kinda wanna have it and kinda wanna not have it.
  - Star Sign.  Random attribute/skill/whatever bonuses and penalties.
    - It'd be cool to automatically generate these.  For instance, we have 57 possible signs.  The world is generated and picks 8-17 of these, then the character is assigned one.
  - Talents.  Skill bonuses.
- Initial descriptions for entities, objects, and rooms.  Once the initial description has been displayed, it will not be shown again.
- Triggers? (when an in-game message appears, execute a command)
- Prompt customizability?
- Spatial hashing? lol
- onTake()
- onDrop()
- onEat()
- onExamine()
- onTaste()
- Shift to using callbacks rather than the event queue.
- Aliases for objects/entities/rooms.
- before, on, after actions.
- Possibly use tui-input instead of tui-textarea.

## Current Status

_Super_ early development.  No, earlier than that.

At present, you can navigate between rooms... provided you created the connections between those rooms in a rather painstaking process, lol.

<img width="803" alt="Screen Shot 2022-09-18 at 5 16 12 PM" src="https://user-images.githubusercontent.com/1318579/190928457-e553472e-5388-486f-aa5e-1c43f26d23c7.png">

Possible future architecture:
- Hornvale: Frontend and Game Logic.
- Goldengrove: Narrative PCG and tools.
- Lasthearth: Embedded scripting language.
- Breakwater: Environmental PCG and tools.  
- Brownhollow: Life/factional/economic PCG and tools.

General roadmap:
- Complete the interpreter-based embedded scripting language.  This is mostly a learning project and preparation for a more solid project to follow.
- Reimplement scripting language as a bytecode interpreter as a separate project (Lasthearth).
- Expand core functionality so that I can basically re-implement Zork.  This is the main focus of the Hornvale project.
- Shift environmental work into a separate project (Breakwater).  I'm interested in both roguelikes and text adventures, and possibly mixing the two together.  It'd be neat to work out common concepts and make the two interoperable and interchangeable.  I like the idea of simulating weather too, and I think that fits in here too.
- Shift NPC elements into a separate project (Brownhollow).  This is one of my primary areas of interest; I'd like to be able to sort of simulate history.  We might only see a very thin slice of this in a given game, but I'm enchanted by the idea of being able to observe NPCs interacting with one another.
- Build some narrative PCG elements in a separate project (Goldengrove).  I'm deeply interested in the idea of PCG narratives.  Of course, these can inform NPCs, and inform environments, and vice-versa.
