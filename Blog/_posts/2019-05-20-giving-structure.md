---
title: "Giving Structure"
date: 2019-05-20
---

This post is technically being written on the 21st, but I still want to reflect on what happened yesterday
because I think it was pretty important. 

I started the day in a good mood off the progress from the day before. I woke up early and was ready to go.
My initial goal at this point was to get basic drawing of the Level struct going so I could start getting 
familiar with whatever graphics library I was going to use. 

After a little bit of digging I decided to try and use Piston as my graphics backend. After spending a little
bit of time trying to parse the pieces of example code they had (https://github.com/PistonDevelopers/piston-examples)
I decided my time would be better spend a little bit of time working through one of their examples. I started
working on the Sudoku example: (https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/sudoku). 

This Sudoku tutorial was great because it showed a good example a Model, View, Controller design pattern along 
with how to divide each into different modules. While running through the example, I remembered a video I had 
seen on rust module structure and gave that a second watch (https://www.youtube.com/watch?v=3MQUdXRB1Gc). These
two sources got me to spend the time and provide structure to my code before continuing on. 

By noon this is where i had ended up on my file structure:

```
├── entity
│   ├── mod.rs
│   └── tile.rs
├── input.rs
├── level
│   ├── level_model.rs
│   └── mod.rs
├── lib.rs
├── main.rs
├── misc
│   ├── mod.rs
│   └── random.rs
└── position.rs
```

By the time I merged this pull request I was very happy, because I had been able to reduce my main.rs from 377
lines of code to 14. No longer was everything in one large mess of a file, but rather things were more well 
organized. There's still some things I'm not super happy with, such as the whole misc folder and position.rs 
standing alone, but that's not too hard to fix later. 

One of my gripes that I had had with Rust when I first tried it, was that I couldn't find a nice way to split
my work across multiple files. I'm not sure if I missed the memo or if something was changed but this set up
works very well. 

From my main.rs I'm able to use all modules listed in lib.rs by importing rust_game as an external crate like
any other. 

```rust
extern crate rust_game;
```

From any one modules, I can access things from other modules by using 'use crate'.
```rust 
// src/level/level_model.rs
use crate::position::Pos;
use crate::entity::tile::Tile;
use crate::misc::random::{Seed,RNG,from_seed, next_u32};
```

The only thing that this file structure requires of me is to maintain the lib.rs and mod.rs files. The mod.rs 
files are very useful because they let me present my code differently than how it is actually organized internally.

For example accessing the Level structure through use statements follows a slightly different structure than the
file structure. If I followed the file structure exactly, the level struct could be used after the following 
statement:

```rust
extern crate rust_game;
use rust_game::level::level_model::Level
```

Since my plan was to have a LevelController struct in level_controller.rs, and a LevelView struct in level_view.rs,
way of accessing code would hide the connection between the Level, LevelController, and the LevelView. What I 
wanted was the following:

```rust
extern crate rust_game;
use rust_game::level::{Level, LevelController, LevelView};
```

To get this behaviour there are some tricks you can use in the appropriate mod.rs file. 

```rust
// src/level/mod.rs
pub use self::level_model::Level;
pub use self::level_controller::LevelController;
pub use self::level_view::LevelView;
mod level_model;
mod level_controller;
mod level_view;
```

This allows for the desired behaviour as the individual modules are all private, but the level module uses the desired
structures publically, so they're all usable directly from level.

File structure aside, I spent the next hour and a half setting up the LevelController and LevelView structures for 
future use. At this point I was following the principles behind the Sudoku tutorial and applying it to my problem 
instead. Nothing too exciting happened here.

After having my Model, View, Controller ready for my level, I was ready to start doing the basic graphics I wanted to 
do at the beginning of the day. Again, I used the Sudoku tutorial as a guide to get things going. I departed from the 
tutorial shortly after they started drawing the squares of the board. After understanding how to set everything up, it 
wasn't to hard to create a draw function for the LevelView which would iterate over all the tiles in a Map, and draw
each one. 

Now it was 3pm and I finally had a way to visualize the levels I was creating, so I started thinking over my next steps.
It was probably a bit of an ambitious idea to create a Player entity and have some dude start moving around, but that's
what I was shooting for. The fact that I'm writing this on the 21st instead of the 20th is a good indication of how this
went. 

My plan was to follow the same Model, View, Controller design that I had for level for the Player as well. Additionally 
I was planning on defining a number of traits which could be defined for different entities, such as Moving, Collideable,
and Drawable. When I started to work on the different Player structs, I noticed that there was a lot of repeated features
between the Player structs and the Level structs. I then started thinking that it would be a good idea to define Model, 
View, and Controller each as different traits. 

I thought this was going to be an easy thing to do, but some subtleties of rust made me learn a bunch before I could get 
it done. This is how I had things planned in my head.

```rust 
pub trait Model {
    fn new() -> Self;
}

pub trait Controller {
    fn new(model: Model) -> Self;
    fn event<E: GenericEvent>(&mut self, e: &E);
}

pub trait View {
    fn draw<G: Graphics>(&self, controller: &Controller, c: &Context, g: &mut G);
}
```

In my head, I could easily define that a generic Controller had to passed to the implementation of the draw method. This 
idea also ignored the fact that I wanted to be able to pass input arguments to a Model. After some experimentation I 
realized that what I wanted could be done, but it required some more specifics in order to compile. 

```rust
pub trait Model<T> {
    fn new(init: T) -> Self;
}

pub trait Controller<T, M: Model<T>> {
    fn new(model: M) -> Self;
    fn event<E: GenericEvent>(&mut self, e: &E);
}

pub trait View<T, M: Model<T>, C: Controller<T,M>> {
    fn draw<G: Graphics>(&self, controller: &C, c: &Context, g: &mut G);
} 
```

By the end of the day this is what I had figured out would work. In essence, implementations of Model, View, and Controller
have to be linked together, so the compiler can correctly assign the correct amount of memory and understand how what 
functions the Controller has available. I'm not certain about how I feel about this implementation, as it couples the Model,
View, and Controller together more than I would like. That being said, I'm considering each implementation of a Model, View,
and Controller as a singular module, so maybe the coupling isn't too bad. 

Having figured out a solution to my earlier problem, I called it a night before implementing it. Overall it felt like another
very productive day, where the project continued to take on even more shape. 

Thats all for "today's" log.

Robbie