# Protean

Rust structs are notably mercurial in temperament. When they go and mutate on you, now you can check what
the differences between them are.

Protean is focused on two primary traits:

- **Patchwork** Adding diff functionality to a given struct with the ability to generate and apply patches
- **Historic** Add a revertible history to a struct, usable for features like transactional rollback

The goal of this project is to get rid of a lot of the boilerplate around checking to see if the fields
inside of a struct have been changed. I'm still figuring out scope, so I'll throw the kitchen sink at it
until I can whittle down/separate things.

## Patchwok design thoughts

### Traits

- **Patchwork** Base trait for all items that need to implement
  - Iterate fields
  - Compare two of the same types and return a patch (look at git terminology)
  - Apply a patch to update it
- **PatchAction** Describe how the value was transformed
  - Objects can really only update their fields
  - Maps can add/delete keys, and modify values
  - Lists can change order

### Structs

- **Patch** A transformation definition for changing from one state to another

### Impls

- All primitives (see serde for list)
- List types (Vec, HashSet)
- Map Types
- Basic mapable items: Option, Result

### Example

```json
{
  // Simple name
  "Db": {
    // Optional versioning. Some ideas, but the Id would be required
    "version": {
      "id": "Some Uuid",
      // Path
      "type": "Fhl::server::Db",
      "language": "Rust",
      "GitHash": "The latest commit hash of the current branch"
    },

    // Object Actions
    "actions": [{
      "org_map": {
        "version": {},
        // Map actions
        "actions": [
          "Delete": "key",
          "Update": [
            "name": "New Name Here",
            "address": {
              "version": {},
              "actions": [
                "Update": [
                  "line2": "Suite 2123"
                ]
              ]
            }
          ]
        ]
      },
    }]
  }
}

```

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

## Revisited

I need to review what I've done and refactor what goes where (Ugh, this does next to nothing)

- Protean (farrago?)
  - Patchwork (protean/farrago?)
    - model_id guid
    - Diff two of the same object
    - `HashMap<DottedField, (serde<left_value>, serde<right_value>)>`
    - Apply a patch to update an object (allow filters by field name)
  - Patch
    - Patch contains key, if defined
    - Field level filters can be applied to reduce returned patch
    - Patch ID
    - Optional Model ID
  - Historic - Not needed for FHL work.
    - instance of patchwork
    - list of patches applied
    - rollback to prior patch
- Tyrell

  - Replicant: Simple tag/alias for patchwork
  - Creche
    - Map of patchwork/historic structs
    - List of subscriptions
    - List of triggers
  - Storefront

    - Map of Creches
    - Get Stats (Because everything is private)
    - GraphQl based API
    - Send/Receive Patches via API
    - Send/Receive Subscriptions via API

### Order to get what I need for FHL

1. Create Storefront for Google Doc source
2. Add Patchwork gd_submission
3. Create Storefront for postgres
4. Add Patchwork for pg_submission
5. Have pg_storefront subscribe to gd_storefront for gd_submissions (same model_id)
6. **MILESTONE** We should see items added to gd_storefront appear in pg_submissions
7. Create a trigger on pg_storefront on changes to gd_submissions
   1. Trigger should break apart submission
   2. Query and update payments and many-to-many tables
8. Write code to build all new invoices
9. Add GraphQL server in front of pg_storefront for all pg_tables
10. Add code to FHL UI to add

### Things I should consider later

- Patchwork
  - Boundary of the diff (keys, immutable fields)
  - Field validation/filtration
- Patch
  - Optimization - all
- Historic
