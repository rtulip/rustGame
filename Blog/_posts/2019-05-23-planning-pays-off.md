---
title: "Planning Pays Off"
date: 2019-05-23
---

So last we left off, I was running up against some roadblocks because I hadn't planned things out very much. 
I had some loose ideas as to how I was wanting to structure things, but ultimately I was noticing I was 
beginning to struggle to move forward because I wasn't sure how the different pieces were going to interact.

The first thing I did was hash out some real loose requirements in terms of what game components I was going 
to need, as well as some justification as to why I wanted them. The final list was as follows:
* Walls
* Floors
* Player
* Enemy
* Beacon
* Tower
* Bullet
* Sword

After understanding what my game needed in order to function as a minimum viable product, I tried to figure 
out what the best way to structure my game was going to be. The technique I found the most useful was to 
think in terms of pseudocode. This isn't nessisarily the best technique, but I found it to be really helpful
to get me to think in terms of how the code is going to ultimately be used. 

Where I started was with my main program. I thought back to when I did the file restructuring and how clean 
it was. 

```rust
fn main() {
    let game = Game::new();
    game.run();
}
```

From here I fleshed out exactly what would make up the Game struct. 

```rust 
struct Game {
    controller: GameController
}

impl Game {
    fn new() -> Self;
    fn run(&self);
}
``` 

The Game struct would be responsible for starting the game loop, and would dispatch events to the GameController.
Notably there was only going to be one GameController now. There was a good article I read 
(https://dewitters.com/model-view-controller-for-games/) which provided some nice insight as to how to do MVC in
games. Part of the reason I was wanting to do all this planning was that my initial plan required me to create 3
different structures for every single entity I wanted to create. After finishing the initial Player draft it was 
apparent that that was going to be too much. 

Now there would be a singular GameController which had access to both the GameView and the GameModel. It would 
handle user input and any other events. It would also trigger a game tick (the unit of time).

```rust 
struct GameController {
    model: GameModel,
    view: GameView,
}

impl GameController {
    fn new() -> Self;
    fn handle_event(&self, event: &e);
    fn tick(&self);
}
```

The GameView, unsurprisingly, would only be responsible for drawing the GameModel. One major difference from the
earlier design was that the GameView would now take a GameModel reference to draw rather than a GameController.
Something that had bugged me even early on was that I would pass the GameController, which would in turn get the
model to draw. The article listed above suggested the View should directly see the Model. This approach made more
sense to me, so I carried through with the decision.

```rust 
struct GameViewSettings { /* ... */ }

struct GameView {
    settings: GameViewSettings,
}

impl GameView {
    fn new() -> Self;
    fn draw(&self, model: &GameModel);
}
```

The GameModel didn't get too fleshed out at this point. I knew there were going to be a bunch of methods I'd realize
I'd need as I go. I'll need to keep an eye on the structure of the GameModel in the future, but for now, I've been
doing whatever's been needed as it's come up.

The next thing I started thinking about was how some of these functions were going to work. This helped me plan out 
some traits which would let me use the code the way I wanted. The first funcion I thought about was the draw function
for the GameView.

```rust 
fn draw(&self, model: &GameModel){
    for entity in model.enties {
        match entity.shape {
            Shape::Circle => { /* ... */ },
            Shape::Rectangle => { /* ... */ },
            _ => {}
        }  
    }
}
```

I was thinking that it would be great if I could define the Shape of an entity, but have everything else related to 
how it gets drawn be handled by the GameView. This led me to this plan for the Shape trait:

```rust
enum ShapeType {
    Circle,
    Rectangle,
}

impl ShapeType {
    fn draw(x,y,size, rotation,color);
}

trait Shape {
    type ShapeVariant: ShapeType;
}
```

The idea was that the ShapeTrait would only define the associated type which would define which type of shape the 
implementing struct was. The type itself would have an implemented draw function. The only flaw with this plan is 
that instances of an enum don't count as types for the ShapeVariant. This was pretty easy to work around, by 
introducing an empty struct for the types as well. This work around had the added benefit that the different structs
could have different inputs for the draw function.

The last two things I wanted to plan out were what defined an entity and how to give entities state. Neither was too 
hard to figure out a plan for.

```rust 
trait Entity: Shape {
    fn tick(&self);
}

trait State: Entity {
    type StateEnum;
    fn change_state(&self, new_state: StateEnum);
}
```

The rest of the day I was able to spend implementing this design, removing all the redundant code. By the very end I 
started work on input handling and was able to get the Player to follow the mouse when the "W" key is pressed and back
away from the mouse when the "S" key is pressed.

I was expecting input handling to be a much harder process, but it really wasn't too hard at all because I had spent 
the time to create a plan. I need to be careful to keep an eye on how I approach development going forward, to make 
sure I don't start loosing sight of the structure and organization I have at the moment. 

Anyways, it's late now. So thats all for today. 

Robbie


