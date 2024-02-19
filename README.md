# Piw Pew
multiplayer game (2024)
## Network Events
| Occurences | Event |
|--|--|
once | NET_WORLD_MAP
once | NET_WORLD_PLAYERS
once | NET_PLAYER_JOINED
never (forgor) | NET_PLAYER_GRID_POSITION
tick | NET_PLAYER_WORLD_POSITION
tick | NET_PLAYER_ORIENTATION_ANGLE

Onces the player joins:

```mermaid
graph TD;
    C0[Client] -->|Joined| Server;
    Server -->|Initial Data| C0;
    Server --> C1[Client];
    Server --> C2[Client];
    Server --> C3[Client];
```