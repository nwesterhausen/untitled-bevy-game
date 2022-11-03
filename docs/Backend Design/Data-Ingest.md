# Data Ingest Files

The information about the world, spells, and all other systems gets loaded into the game database at game creation. Tentatively, the database will be specific to a savegame, and can be loaded itself (without regen) on game load. This also lets the world be dynamic and have permanent effects from character chocies.

## File Layout

The general format includes an unique id, the system, ordinal information, author, description and version.
