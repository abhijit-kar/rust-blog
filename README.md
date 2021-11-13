# rust-blog
Experiment to create Blog powered by Rust, Actix Web, &amp; Tera.

## Features:
1. Fast because of Actix Web & Rust.
2. Fastest dynamic templating engine, using Tera.
3. Pulldown Cmark for markdown.
4. Live watch of folder using Notify, Arc & RwLock magic.
5. All the guarantees of Rust.

## Disadvantage:
1. Rust is slow to develop with.
2. If bad template is passed to Rust program, it poisons the RwLock and any other process accessing it fails.
3. Rust's compiler becomes hard to decipher when error occurs in Tokio Runtime.
4. Rust is verbose.
5. Lots of imports.
6. Lots of type defs.

## Alternative:
1. It's better to not reinvent the wheel.
2. Loose / Duck typing is faster to develop with.
3. If I don't have to import stuffs, it would be great.
4. Live reload should be a no brainer.
5. Powerful html aware templating.

> Hence chosing Elixir and Phoenix framework going forward.
