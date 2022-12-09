# Day 07

## Key Take-Aways

- In `Iterator::take_while`, the false element that breaks the loop will be consumed
- Use `Peekable::next_if` if you want a solution that retains the false element

I ran into a tricky problem because of this since I was relying on consuming the iterator until the next command (`$`) in the input. The next command always got consumed which completely threw off my results.
