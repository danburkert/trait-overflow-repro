# Trait Resolution Overflow Repro

Reproduces a spooky behavior (bug?) in `rustc` 1.36 which causes a compilation
failure when resolving trait bound requirements. Hopefully chalk will solve
this.
