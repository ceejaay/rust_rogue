Getting familiar with Rust by following a roguelike tutorial.

Questions:
    What is a stuct?

 Is struct like an object?

Do you have to create the struct then create the code to create the struct in the actual code?

Does `mut` mean mutable? A: `mut` is mutable. Variables in Rust are immutable by default. So to have a variable change you
have to make it mutable with `mut`

What is `blit` Is it tcod or Rust?

What is `impl`?

 What's going on with the `&`? Is that a pointer? A: Yes, this is a pointer or a reference. It references the thing in
 memory so you don't have to copy it.



What is `pub fn` Public function?

What is `usize`??

What is `i32` is it a signed integer or an unsigned integer?

How do you loop through arrays?

How does Rust know when you've reached the end of an array?

What does the `&` mean? Is that a pointer?


I think this is how you create an object.
1. you have to describe what the object is. What are it's properties.
2. Then you have to write the function that creates the object.
3. Any functions the object needs need to be written.


I forgot what a closure is?


What is derive? How does it work on `struct`?

What does the `*` mean?

How does `match` work?

    
Have no idea what's going on here



    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }


I think this is a tcod thing, not so much a Rust thing. But I'm not sure what's going oin with the 

    match foo {
        barfoo } =>
    { foobar }



What is `vec`?

this is from the tutorial.

>The vec! macro is a shortcut that creates a Vec and fills it with values. For example, vec!['a'; 42] would create a Vec containing the letter 'a' 42 times. We do the same trick above to build a column of tiles and then build the map of those columns.




