Advent of Code, Rust edition
============================

2023 I finally got myself around to code some Rust taking part in Advent of Code.

Here's some random notes:

* Day 6 DONE...and its January already.
  * Well...I did manage to bork my OS install by running updates and not having power plugged in. And not having my dotfiles under version control I lost over two weeks worth of freetime setting things up again. But yeah, gonna still finish! But before day 7, I'll put the dotfiles under control!
* Day 5 part 2 is really heavy to run. I'm probably missing something.
  * I missed the fact that I could just map the range edges instead of every number in the range.
* Do some additional tests to explore that front
* Extract stuff to libraries shared by parts 1 and 2
  * First try in day 5
* .iter() vs .into_iter() what the heck? .collect()?
  * So the general idea is that .iter() is shorthand for into_iter() where the returned iterator is by reference.
* Read about usize, and about primitives in general.
* Setup debugging

Here's some random things I could not get working and decided to figure out later.
Maybe they're stupid things to do, but that's somewhat besides the point.

.. code-block:: rust
    :linenos:

    // This does not even pass compilation,
    // but the idea is to feed the results
    // of iter.take_while(...) to this function
    // as something that implements IntoIterator
    // yielding string slices (&str),
    // and this function returning a vector of
    // string-slices back.
    // The returned value could also be a vector
    // of strings, which, if I understand correctly,
    // would simplify handling the lifetimes.
    // And it seems, indeed, that lifetimes
    // are the reason why this wooshes me so hard
    // right now.
    fn my_fn<I>(i: I) -> Vec<&str>
    where I: IntoIterator<Item = &str>
    {
        // This is just an example
        i.into_iter().map(|s| s.trim()).collect()
    }

    fn main() {
        let input = include_str("input.txt")
        let trimmed = my_fn(input.lines().take_while(|line| !line.is_empty()));
    }


