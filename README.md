# Maze Performance Grade

This is a simple godot 4 learning project.
The goal is to solve a maze in fewer steps than any of the AIs.
After solving the maze, the path you took to solve the maze is compared to a couple AI's paths.

The game logic is re-implemented in GDScript, C#, and Rust: 
all of the source files have rough equivalents in the other 2 languages.
Only the maze generation and solving code is implemented, and the first person controller is third party.
I hope to use this as a by-example way to answer "How do I translate X from GDScript to C# or Rust?"

[rust scripts root](/rust/src/maze_scripts)

[C# scripts root](/godot/csharp/maze_scripts)

[GDScript scripts root](/godot/gdscript/maze_scripts)


## Gameplay

https://github.com/dsmiller95/MazePerformanceGrade/assets/3212309/492119f6-802d-427e-b04f-4cbc97919a15

## Context

This is coded by someone with some experience, looking to fill in the gaps. If my Rust code is ugly this is why:
- 7 years C#
- 4 years Unity
- 0 days GDscript
- ~1 month toying with Rust

### Feedback

Please give me your code style or performance suggestions if you have any! 
Particularly on the Rust implementation: I have a sneaking suspicion I could have used a full `std::future::Future` impl instead of my hacky coroutine structs.

## Weird

If I noticed something unfamiliar to me during the implementation, I tagged it with `#WEIRD` .
Usually these are features GDScript doesn't have which I've grown accustomed to in C#.
Or missing features/awkward experiences while learning Rust.

[See all `#WEIRD`](https://github.com/search?q=repo%3Adsmiller95%2FMazePerformanceGrade+%23WEIRD&type=code)


