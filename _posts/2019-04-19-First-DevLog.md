---
title: "First Devlog"
date: 2019-04-19
---

Today was the first day actually sitting down and getting stuff done.

I started with the GoL implementation from http://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust. 
This was a very good find as it introduced me to HashMaps and showed me a very clean implementation.

The first thing I tried to tackle was the creation of the board. In retospect, I should have had more
smaller commits documenting the changes I made. Nevertheless, I did some experimenting to figure out
which random number generators could be seeded to help make testing consistent in the future. This 
was actually very good for gettign me back into the groove of Rust and it's documentation. 

I think my implementation of the random seed generator is fine for the moment, but I should return to
it at somepoint in the future.

The original GoL implementation assumed that the only thing within the "Colony" was the "Cells". I  
was aiming to have the Colony hold different types of Cells. As such, it made sense to go from a  
HashSet to a HashMap for the Colony, since the value of the key would matter. 

I also changed a few names around early on. Colony became Board, Cell became Pos (short for position)
and Cell became an enum to hold the different types of entries which could be placed into the board. 

Early on I got some command line argument parsing. I had forgotten that cargo directly passed in any 
arguments after --. I think I can make pretty good use of this later for testing and whatnot. For now,
all the parsing is still in the main, which I should change. 

So by about 2:20pm I had added command line parsing and the Cell enum working. At this point the Cell had 
3 variations: Floor, Wall, and Cust(i32). Floors and Walls were fairly self evident, but the Custom
variation was my initial plan for how to handle the different connected components of the "Graph" that's 
being created. This had felt like a pretty good first step, and I'm happy with how the Cell enum worked
out. It changed names a few times, but the content didn't change from here.

At this point. I could tell that the code was already starting to feel a little cumbersome and it was 
getting to be difficult to handle. I decided that it would be in my best interest to clean up and comment
the code that I had, and try my best to keep it up in the future. After about 40 mins, I had some nicely 
commented code which will help keep me sane.

By 3:00pm I was wanting to clean out main as much as made sense. This would ultimately take me the rest of
the day to get it to where I wanted it, but the progress was very good. First I noticed that all the world
generation was happening in main. My first step was to separate the world generation into a function:
generate_map(). This was also when I changed the name from Board to Map, and when I filled in the rest of 
the Map with floors Tiles. Oh yeah, Cells changed to Tiles; I had fun with naming today.

After an hour of tea, lunch, and coding, the generate_map() function was ready, and I wanted to tackle
something that was bugging me. I was getting concerned with how I was going to handle path-finding since
there were all these different connected components at the time. 

I ended up implementing a flood-fill algorithm which would find all the Tile::Floor spaces which were 
reachable from a given starting position. I was really impressed with how well the match syntax handled 
this. The hardest challenge I faced at this time was differenciating between different Tile::Cust(i32) by 
their value, and this was easily resolved with match.

```rust
    match Map.get(Pos) {
        Some(Tile::Cust(val)) if val == target_val => { /* ... */ },
        _ => ()
    }
```

By the end I had figured out a three step process to have only one connected component in a Map. First I 
found all the connected components using the flood-fill and replaced the Tile::Floor with a Tile::Cust(i32)
where the value was the ith connected component. By traversing every position of the Map, it was easy to 
identify when to start a new flood-fill because every Tile::Floor would represent a previously unreached 
position. Next I figured out which connected component was the largest. Then, I finally traversed the Map 
again and filled in any Tile::Cust(i32) which wasn't from the largetst connected component with Tile::Wall
and replaced each of the Tile::Cust(i32) which were elements from the largest connected component with 
Tile::Floor. 

After finishing that, it was about 6:30 and I took a break for dinner. I came back at around 8:30 to work 
on converting the Map to a struct from a HashMap. My thought process was that it would be useful to collect
information such as the width and height of the level with the Map itself. 

This proved to be very challengin and ultimately, not the right approach. That being said, it wasn't a waste
of time. I learnt a lot about how iterators worked and how to implement traits from other modules. The 
problem with the idea of converting the Map into a struct was more an issue of perspective rather than a 
design flaw I think. Here's what I was trying to do:

```rust
    struct Map {
        tiles: HashMap<Pos, Tile>
        width: i32,
        height: i32
    }
```

The issue is that I then wanted to try and iterate over a Map, or collect() over a Map. This led down some
rabbit holes of implementing iterators and collections, only to realize that another perspective would make
more sense:

```rust
    struct Level {
        map: Map,
        width: i32, 
        height: i32,
        rng: ChaChaRng
    }
```

By going up a level of abstraction, it was making a lot more sense to me as to how everything was going to 
fit together. Suddenly all the operations I wanted to perform on Maps were still easy because I wasn't 
trying to shoe-horn in functionallity where it wasn't needed. This also gave me a much nicer way to create
a global random number generator. After bashing my head against the problem for a few hours, everything was
now sitting nicely with me. 

By structuring my code in this way, it also gave me a logical place to put all the world (or Level) generating
code. The last thing I did today was to implement Level, and move all the Game of Life specific stuff inside
the Level implementation. As such by the end of today, I have level generation into one line of code:

```rust
    let lvl: Level = Level::new(width, height, iters, seed);
```

Overall today has been quite the success. I feel as if I've made a bunch of progress and have gotten to be a 
lot more familiar with rust, which should only help increase the speed of development.

Tomorrow is a holiday, so I should be able to get a bunch more work done. I think my next step is to start 
looking at graphics. I should also do some more formal planning now that things are slowly starting to shape
up. 

Thats all for tonight.
Robbie
