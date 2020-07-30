# Protean

Rust structs are notably mercurial in temperament. When they go and mutate on you, now you can check what
the differences between them are.

Protean is focused on two primary traits:

- **Patchwork** Adding diff functionality to a given struct with the ability to generate and apply patches
- **Historic** Add a revertible history to a struct, usable for features like transactional rollback

The goal of this project is to get rid of a lot of the boilerplate around checking to see if the fields
inside of a struct have been changed. I'm still figuring out scope, so I'll throw the kitchen sink at it
until I can whittle down/separate things.

## Current scope

- **trait Historic** Add a history of updates as a history. Requires Patchwork, no pub fields.
  - **Macro integration** How does this work with the other macros (Serde comes to mind)
  - **fn list_history** Get a list of the patches applied vector of struct Patch
  - **fn pop() -> Patch** Mutate the struct to the state it was before the last patch was applied
- **trait Patchwork** -> Able to generate and apply patches
  - **fn patch(Patch)** run a patch against
  - **patch!(struct, key, action)** for creating/running a simple key/value patch. Useful for Historic which
    requires getters/setters
  - **Getters/Setters** Since the fields are private by necessity, access must be provided by getters and
    setters. Possible implementation: [getset](https://github.com/Hoverbear/getset/)
  - **fn diff(struct1, struct2) -> Patch** Compare and return the differences between
