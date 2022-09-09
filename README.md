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

General Flow:

- **Command**: Some entity (generally the player) wants to do something.  For the player, this can be in or out of character.  This is indicated by a `HasCommand` component.  A system will act on these directly (OOC commands) or translate these into Actions (IC commands).
- **Action**: Some entity attempts to do something.  This is always in-character.  This will fire a sequence of **Checks** and **Events**.
  - **Checks**: Checks whether the action should go through.
    - **ShouldPerformAction**: Checks whether the entity _should_ do something.  For instance, if an action was enqueued to attack a goblin, but the goblin is dead.  This is the entity having a chance to abort its own action.
    - **CanPerformAction**: Checks whether the entity _can_ do something.  Continuing the above example, an action was enqueued to attack a goblin, but the player has no arms.  This is the game enforcing rules preventing the expressed intent.
    - **TryPerformAction**: Checks whether the entity can _successfully_ do something.
  - **Events**: Informs the entity and/or others of the progress of the attempted action.
    - **CouldNotPerformAction**: Event informs the entity that the entity could not do something.  This is generally (always?) invisible to other entities.
    - **WillPerformAction**: Event informs the entity and others that the entity will do something.  If an action was enqueued to attack a goblin, the goblin gets a chance to duck, or block, or make a saving throw or whatever.  A sufficiently fast and powerful entity can prevent actions of another (though this should be generally uncommon and determined by skill, etc).
    - **WillFailToPerformAction**: Event informs the entity and others that the entity is in the process of failing to perform an action.
    - **DidPerformAction**: Event informs the entity and others that the entity did do something.  Messages like "You hit the goblin!" or "The player hits you!", or informing the entity that the player pickpocketed them sloppily and they should retaliate.
    - **DidFailToPerformAction**: Event informs the entity and others that the entity failed to do something.  Messages like "You swing at the goblin, but miss!".
- **Effect**: Any sort of effect on the game world or anything in it.  Damage, unlocking a door, a bird singing (which dispatches events), etc.
  

## Current Status

At present, you can navigate between rooms... provided you created the connections between those rooms in a rather painstaking process, lol.

```
Spawn Room
This is just a nondescript room.

> ne

Northeast Room
This is just a nondescript room.

> sw

Spawn Room
This is just a nondescript room.

>
```
