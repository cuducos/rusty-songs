# Rust songs

Repository to epxlore dynamic dispatch in Rust.

## Context

The idea is to have a `collection` os vinyls and CDs and play them all. If the album is a vinyl, we have to flip it and play the other side befor emoving on to the next album.

The summary of the code is:

* A `trait Album` with a method `play()`
* A `struct Vinyl` implementing `Albums`'s method `play()`, plus a method `flip()`
* A `struct CompactDist` implementing `Albums`'s method `play()`, but no `flip()` method

## Problem

My idea was to have a `collection : Vec<Box<dyn Album>>` and iterate over it, calling `play()` on each album, and flip only on the vinyls, but it fails:

```
error[E0599]: no method named `downcast` found for struct `Box<dyn Album>` in the current scope
  --> src/main.rs:79:34
   |
79 |         if let Ok(vinyl) = album.downcast::<Vinyl>() {
   |                                  ^^^^^^^^ method not found in `Box<dyn Album>`
   |
   = note: the method was found for
           - `Box<(dyn Any + 'static), A>`
           - `Box<(dyn Any + Send + 'static), A>`
           - `Box<(dyn Any + Send + Sync + 'static), A>`
```

What am I doing wrong?

## Reproducing

```console
$ cargo run
```
