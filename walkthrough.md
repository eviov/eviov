# Project walkthrough
This article guides through all aspects (including technical and gameplay) of
this project.

Note that this file is intended for technical reference, not a gameplay guide.

## Project structure
The client is a static page running a WASM module.
The source code of the WASM module is located in the /client directory.

The client may connect to arbitrary servers.
Different types of servers expose different gamemodes,
and the common code they share is located in the /server directory.
Different servers may connect to each other (details explained below),
forming a possibly-disconnected graph.
Various gamemodes are implemented in crated under the /games directory.

The root directory also exposes a lib crate for code shared between server and
client.

## Game mechanics
The primary focus of this game is the mechanics arising from gravitational pull
in a 2D world.

### Hierarchical orbital systems
This game does not implement real-life n-bodies mechanics.
Instead, apart from the root object (nicknamed the "Sun") which is stationary,
every object (the "child") orbits exactly one other object (the "parent"),
and its motion relative to the parent is only influenced by its parent.
Here, an "object" refers to anything that participates in the orbital motion,
no matter parent or child.
The only visible things that are not "objects" are either pure-cosmetic
(such as the background texture of the game),
or part of an object tightly connected together (such that they move together).

To be precise, each "large body"
(object that can exert gravitational pull on other objects) has a circular
"gravitational field" (abbreviated as "g-field") centered at its body.
All large bodies have a finite radius (except possibly a Sun that should
logically never be a child).
If an object has zero acceleration and stays in the g-field of its parent,
the locus of an object should be an ellipse with a focus at the parent,
and the motion should be consistent with Kepler's laws of planetary motion,
relative to its parent, with a small tolerance of computational error.

A "System" refers to a large body and the external abstraction of its bodies.
Each System is processed in its own coroutine.
Different Systems might be processed on coroutines run on separate threads,
separate processes or even separate machines.

For "small bodies" (objects that are not large bodies),
its logic is entirely handled by the System of its parent body.
For large bodies, it should handle its internal logic,
but the interaction with its parent should be handled by the parent.
The parent should see each large body child as a blackbox,
only knowing its g-field radius.

### Object transfer
When a child escapes the g-field of its parent,
it should be "transferred" to the grandparent (the parent of its parent),
such that the grandparent becomes the new parent of this child.

When a child enters the g-field of another child large body of its parent,
it should be transferred to the child,
such that the other child large body becomes the new parent of this child
(and so the original parent becomes the grandparent).

In that case, all physical logic of the object should become handled by the
new parent.
The System of the original parent should send the data about the child to the
System of the new parent through the `intra::Message::EntityTransfer` message.
If the child is a large body, the old parent System should send an
`intra::Message::StartTransfer` message before starting the transfer,
and the new parent System should send an `intra::Message::EndTransfer` message
after completing the transfer setup.
If the new System wants to immediately transfer the child object to another
System (e.g. if the g-field of one of the child large bodies of the new parent
intersects with the g-field of the old parent),
it should perform the new transfer after sending the first
`intra::Message::EndTransfer` message.

### Netsplits
In the whole network, there might be multiple Suns.
However, since there is no way for objects under different Suns to interact,
generally speaking, we assume there is only one Sun.

When the coroutine running a parent happens to have crashed, a netsplit occurs.
Each of its children becomes a Sun, not connected to other children,
nor to the grandparent system (the parent of the crashed parent).
The new Sun, since it should have a finite g-field, should perform "smooth
collision" (as defined below) on children approaching the boundary of its
g-field.

When the coroutine running a child happens to have crashed,
the child should still be handled as a blackbox,
but the parent should stop transferring objects into the child.
Objects intersecting with the child g-field should perform "smooth collision".

"Smooth collision" is a mechanism similar to car-racing games bumping onto the
roadbank, where the velocity component parallel to the radius is removed,
such that the object only moves along the circumference.

## Protocol
### Client connection

### Intra-server connection

### Time synchronization
