## Format
use `cargo fmt`

## Clippy
use `cargo clippy`
To ignore warnings, find the the name of the warning in the note line then add it to file:  
`#[allow(clippy::too_many_arguments)]`


## Documentation:
use `cargo doc --no-deps --open`
docs start with ///  can also use /**
supports markdown, lists, links

if linking to same module you can use [`MyStruct`] to link or `[Click Here](PUZZLE)`  
to link to different modules use `[Spwan Thread](std::thread::spawn)`  

For inner documentation, you can use //! or /*!

## Publishing
Make a crates.io account then do `cargo login`  
then run `cargo publish`

## Iterators
Anything that implements into_iter() method will be turned into interator automatically by for loops.
```rs
let v = vec![5,6,7]
v.into_iter().for_each(|num| println!("{}", num));
```
We can also use a for loop but iterators are faster.

Can use iterator adapters like map, filter etc. map can return different type than what it takes.

iterators are lazy, don't do anything until consumed  
for_each, sum(), collect()

```
v.into_iter() //consumes v, returns owned items
v.iter() //returns immutable referernces
v.iter_mut() //returns mutatble references

```
