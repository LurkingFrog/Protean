# Diesel Subscription

This is one of the primary use-cases for Patchwork. Data is going to be pulled into a cache via diesel and
then stored in a local Patchwork enabled cache. When new data comes in, the cache is updated and generates
a list Patches, one for each incoming record that actually generated a change.

The initial run of this is going to be specifically for Fishhead Labs, as that is the germ of the idea. I'll
make it generic when I get closer to done and am ready to spin it off into its own project.
