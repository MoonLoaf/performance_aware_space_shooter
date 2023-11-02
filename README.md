# performance_aware_space_shooter

-How the implementation makes efficient use of computer hardware

-How the implementation uses a data-oriented method

-How code was optimized by based on findings from using a profiler

## Written Assignment

I decided to make this assignment in Rust using SDL2 and specs. I have some previous (very limited) experience with rust
and I wanted to take the oppurtunity to learn more about it and it's usage in creating games. 

Using a the profiler in VS on the naive implementation of the game I decide to work iteratively

Starting out with checking heaviest functions. Main culprits are obviously main and render,
main referring to the game loop which deals with all runtime logic of the game, including:
render
updating movement
checking collisions and all other events

game update seems surprisingly performant already, probably because of the lightweight nature of the program

Looking further into render I can see the functions that eat the most performance
create_texture_from_surface()
TextureManager::get_texture()

I learned early on in this project that having the texture creator create new instances of textures each render
call is incredibly heavy. This was the very reason for the texture manager. Before the texture manager the program
would slow down significantly, even unplayable levels, whenever more than two textures were being drawn every frame.

I assume create_texture_from_surface() works similarly to create_texture(), and I would be wise to find a solution
where I don't call this every render call. This will be my first fix. The function is used for drawing text on
the screen, so a simple solution would be just to remove text. However my goal is to keep the program itself as
close to the first implementation as possible, at least the user experience of it, so I will try to optimise this
instead of simply deleting the feature.


## Optional reading / Stuff I learned and thought was interesting

### Managing text on screen

I have tried creating one single texture out of all text surfaces but making a single texture out of all of these requires
that the one texture gets drawn in one rect, which screws with the scaling of the individual ones, and can't be re-scaled.

I have tried keeping instances of textures in a hashmap, updating them every 100 frames,
Keeping instances in structs, updating them every 100 frames,

The reason this doesnt work seems to be that the SDL Texture type is owned by the canvas or texture creator that
created it, and since the texture creator that creates this only exists in the function scope, the Textures cannot be
moved to a vector, hashmap or struct outside of the function. 

What I ended up having to do is to create and store my UI elements not in the render function, but in a scope where
I have access to my original texture creator, so in main(). I then pass the vector of (texture, rects) into my 
render function.

### Object pooling

There is a branch in the repo called object pooling which is my first attempt at implementing pooling for the asteroids. 
I definitely think it's doable, but I think I started working on it a bit too late. I also realized after reading in to 
object pooling in ECS that it isn't as useful as it would be in an object-oriented architecture. Also the fact that 
rust isn't garbage collected and is incredibly memory-safe rasults in more efficient memory management for quickly
creating and removing entities and their components during runtime.

I started off with a vector of Entities, but soon realized that if I wanted to keep some sort of collection of my full 
asteroids, rather than just their entities I'd have to create som custom type containing the entity and all its related 
components.

If I were to continue on with the approach of keeping a Vector of just entities I woould have to remove all of the components
whenever an asteeroid entity was returned to the pool, and then re-insert them whenever a new one was created by the pool. 
This system feels ratherover-engineered, since the end result would just be moving a bunch of entities (which from my understanding it pretty much just unique IDs) around and removing/inserting
a bunch of components based on what goes on in the game.

