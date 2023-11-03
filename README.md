# performance_aware_space_shooter

## Written Assignment

I decided to make this assignment in Rust using SDL2 and specs. I have some previous (very limited) experience with rust
and I wanted to take the oppurtunity to learn more about it and it's usage in creating games.

Making this game in a hardware efficient manner is something that rust is very efficient in handling by it's own nature.
The languare itself is fairly accessible and easy to write once you get a hang of certain rust concepts like lifetimes,
borrowing and ownership, these are definitely concepts that could be used better in my project but are lacking slighly because
of my limited experience. The compiler is extremely powerful and capable of converting what feel like high-level code
into extremely effiecient binaries. Rust does not feature a garbage collector but is very efficient in memory management
and easy to use in a memory efficient way. The compiler is also incredibly intelligent in handling abstractions, which are
handled at compile time. This results in very little runtime overhead for handling these concepts. These are just a few
of the concepts that make Rust an incredibly efficient and memory safe language for creating programs with great scalability
that can run until your machine starts to rust ;)

I made this game using the ECS architecture provided with specs, which is a very interesting contrast to object oriented
game programming. It was challanging to try not thinking about the data the game required in a non-object mindset, but after
spending some time doing this, I quite enjoy this way of working. The game logic itself revolves a lot around iterating 
over data that is collections of component based on structs and dealing with the information stored in them to create systems
that make the game interactable. I think this makes systems quite readable, and easy to work with. It also makes the systems 
limited to only the data required to make them work, instead of references to entire complex objects.

Using a the profiler in VS on the naive implementation of the game I decide to work iteratively. All profiling was done by
attaching the VS profiler to the process started by running "cargo run", and not an exe.

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

I created a fairly simple implementation of a texture manager, keeping a hashmap of cached textures that I could fetch for the
render calls, using the name field of the renderable component as the key, this made rendering textures way more efficient.
I didnt run any profiling before the texture manager unfortunately but it was obvious fps wise that creating new ones each call
was an extremely heavy operation when dealing with several textures.

The get_texture function from my manager is still one of the heavier operations of my render function, even though it's fetching
data from a hashmap, which can be done without iterating from all its content. I've had a hard time interpreting why this function
represents about 15% of the cpu time spent in the render function (compared to the second most heavy operation of presenting the canvas at 7%).
After trying to research it a bit I realize that one of the reasons might be the complexity of the SDL texture data structure.

I assume create_texture_from_surface() works similarly to create_texture(), and I would be wise to find a solution
where I don't call this every render call. This will be my first fix. The function is used for drawing text on
the screen, so a simple solution would be just to remove text. However my goal is to keep the program itself as
close to the first implementation as possible, at least the user experience of it, so I will try to optimise this
instead of simply deleting the feature.

I also wanted to try to optimize my colission handling. My first implementation was simply doing some math operations to check overlaps
for every asteroid in the game. These systems are handled by the dispatcher and because of this I coould not inspect the specific
function calls in the profiler, or they were logged under different names. I tried measuring the performance using my fps counter instead.
I implemented a concept where each asteroid had a "quadrant" enum value in its component that gets updated along with it's movement
based on its position on the screen, I then compare this value with the quadrant of the player and only check colission conditions with asteroids
in the same quadrant. This seems to have bumped my fps up slightly, but given the simplicity of the colission logic versus the operation of checking
the quadrant for each asteroid versus the player this "optimization" was rather minimal. At least it was interesting to learn from!

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

