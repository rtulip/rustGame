---
title: "Powering Away"
date: 2019-06-05
---

Well its been a while since I did one of these. Clearly my plan of doing these nightly hasn't really worked out. So this'll probably be a 
long one, because a lot has happened since the last blog. Lets go chronologically I guess.

Last we left off, I had just done a major overhaul to the organization of the code base and just gotten input handling to work. At that point,
the player would follow the mouse. This was a launching off point for getting a bunch of basic gameplay implemented. To get my game to a 
feature complete state, the following things were needed:
1. Player Collision Detection 
    * Need to collide with walls, and enemies
2. An attack animation for the player
    * Have to detect collisions between the attack and enemies
3. A Beacon entity
    * Acts as the point to defend in the game
4. A basic enemy implementation
5. A means to spawn enemies
6. A resource entity
    * To be dropped by enemies on death
    * Have to be picked up by the player
    * Can be spent to build towers
6. A basic tower implementation 
    * Has to shoot bullets at the enemies it sees
    * Requires a resource to be built

Starting on the 25th of May, I powered away at these problems. All the collision detection was done on the fly, in a very janky manner, but it got
the job done. I made a mental note that collision detection was something to be revisited in the future, and I'll talk more about this later.

For the player, colliding with the walls was fairly simple. I looked at the surrounding tiles, and if the player overlapped with any Wall tiles, 
then the player was shifted the smallest distance to resolve the collision. Other means of correcting the collision resulted in strange warping 
across patches of walls. It does mean that the game takes two game ticks to fully resolve a collision, but it was a simple robust solution. Unfortunately,
this also means the player collides with the walls as if it was a square.

By the end of the 28th I'd added Beacons, Enemies, Resources, an attack animation for the Player. None of this was too challenging because of the 
framework I'd set myself up in. Some of the hardest challenges I faced at this time was the collision detection (mostly it was just very janky) and
the A* pathfinding for the Enemies. Ultimately, not too much of note happened at this time, apart from a lot of work getting done.

Actually, as an aside, for the first time I had someone I don't know contribue to a project of mine. Shout out to maxdietrich for updating some piston
dependencies and cleaning up some warnings. It might not seem like much, but getting that pull request was a very cool moment. 

Anyways, at this point, the only major feature I was missing was creating the towers. The problem was that my plan for drawing the towers involved 
drawing a circle with a rectangle. As it stood with my design, I realized that the Shape trait I had created didn't support compound shapes very well. 
there was no elegant way for me to have something implement Shape and have two shapes be drawn. This required a redesign of my rendering system.

Before I get into the details of how the new rendering system works, I want to take a moment to recognize the 'cost' of making a change like this. In
my studies, its been repeated time and time again, that the 'cost' of fixing an issue increases exponentially with the project life. It's been stressed
that making large design changes well into development takes a lot of time and resources in comparison to making the same change earlier on during design.
Having to do this rendering redesign is a pretty perfect example of how this idea takes place in practice. Having just finished creating enemies, beacons, 
resources, and attack animations, changing how I render my objects requires updating all of the above and more. If I had recognized the issue before spending
the time creating all of these entities, I could have saved myself a lot of work. 

Going forward, I can only expect the project to get bigger and more complex, So even for small milestones, I should spend more time planning before powering
out a solution. 

That aside, let's dive into how rendering works now. When I started this design, I knew I wanted to draw the enetities independently of controlling the entities to follow the Model View Controller pattern. That being said, there were elements of bleed between the model and the view. For instance, the
Player entity had a position as part of its structure. This served no purpose except for positioning the model for drawing. This made me wonder how much did the model and the view need to be separated?

The conclusion I came to was that it was reasonable for an object to understand and define how it would be drawn, but the drawing would have to be controlled by something else. This felt very similar to having an entity implement Shape only it was more clear and easy to understand. 

Thus I removed the Shape trait and introduced the Draw trait and the GenericShape struct, and the ShapeVariant enum.

```rust

pub trait Draw {
    fn draw<G: Graphics>(&self, c: &context, g: &mut G);
}
```

The Draw trait is fairly simple, all it requires is a function defining how to draw the implementor.

```rust
pub enum ShapeVariant {
    Rect {width: f64, height: f64},
    Circle {size: f64, radius: f64}
}
```

I wish I had appreciated that you can have structs as enum variants when I did this the first time. This allowed me to define features of different shapes in the enum. This has proven to be a much, much prettier solution than the empty structs I was using earlier. 

```rust
pub struct GenericShape {
    shape: ShapeVariant,
    color: Color,
    position: Point2,
    rotation: Option<f64>,
    offset: Option<Point2>,
}
```

The GenericShape struct is meant to serve as a building block for creating more complex shapes. Each shape has a variant (rectangle or circle), a color, a 
position, an optional rotation, and an optional offset (which is applied after rotation). A struct implementing Draw can easily have conjugate shapes by having
multiple GenericShapes and calling draw() for each in the appropriate order. 

Oh yeah, obviously GenericShape implements Draw.

```rust
impl Draw for GenericShape {

    fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        let mut transform = c.transfor;
        
        // translate to self.position
        transform = transform.trans(self.position.x, self.position.y);
        // rotate if needed
        match self.rotation {
            Some(rad) => {
                transform = transform.rot_rad(rad);
            },
            None => (),
        }
        // translate by offset if needed
        match self.offset {
            Some(offset) => {
                transform = transform.trans(offset.x, offset.y);
            },
            None => (),
        }

        match self.shape {
            ShapeVariant::Circle{size, radius} => {
                // Draw the dircle
                Rectangle::new_round(self.color, radius).draw(
                    [0.0,0.0,size,size],
                    &c.draw_state, 
                    transform, 
                    g
                )
            },
            ShapeVariant::Rect{width, height} => {
                // Draw the rectangle
                Rectangle::new(self.color).draw(
                    [0.0,0.0,width,height], 
                    &c.draw_state, 
                    transform, 
                    g
                )
            }
        }

    }

}

