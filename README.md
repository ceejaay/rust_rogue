Getting familiar with Rust by following a roguelike tutorial.

Questions:
    What is a stuct?
    Is struct like an object?
    Do you have to create the struct then create the code to create the struct in the actual code?
    Does `mut` mean mutable?
    What is `blit` Is it tcod or Rust?
    What is `impl`?
    What's going on with the `&`? Is that a pointer?
    What is `pub fn` Public function?
    Have no idea what's going on here



    ```match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }```




    I think this is a tcod thing, not so much a Rust thing. But I'm not sure what's going oin with the 

   '' match foo {
    barfoo } =>
    { foobar }`



What is `vec`?
this is from the tutorial.

>The vec! macro is a shortcut that creates a Vec and fills it with values. For example, vec!['a'; 42] would create a Vec containing the letter 'a' 42 times. We do the same trick above to build a column of tiles and then build the map of those columns.
