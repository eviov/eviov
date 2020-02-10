Project walkthrough.

This article guides through all aspects (including technical and gameplay) of
this project.

Note that this file is intended as technical reference, not a gameplay guide.

# Project structure
The client is a static page running a WASM module.
The source code of the WASM module is located in the /client directory.

The client may connect to arbitrary servers.
Different types of servers expose different gamemodes,
and the common code they share is located in the /server directory.
Different servers may connect to each other (details explained below),
forming a possibly-disconnected graph.
Various gamemodes are implemented in crated under the /games directory.

There are other direct/indirect library crates in the root directory
for code that are shared between server and client.

# Game mechanics
The primary focus of this game is the mechanics arising from gravitational pull
in a 2D world.

## Hierarchical orbital systems
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

## Object transfer
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
System of the new parent through the `intra::EntityTransfer` message.
If the child is a large body, the old parent System should send an
`intra::StartTransfer` message before starting the transfer,
and the new parent System should send an `intra::EndTransfer` message
after completing the transfer setup.
If the new System wants to immediately transfer the child object to another
System (e.g. if the g-field of one of the child large bodies of the new parent
intersects with the g-field of the old parent),
it should perform the new transfer after sending the first `intra::EndTransfer`
message.

## Netsplits
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

## Distant view
Players have infinite Field of View, i.e. they can zoom out indefinitely.
However, due to diffraciton of light,
objects further away have reduced "resolution",
i.e. players can only see bigger objects at a distance.

> (The following feature might not be implemented)
>
> In addition, due to relativity, objects seen far away would delay in time.
> For example, if a spacecraft spawns at a location 8*c* away,
> it would appear on the client after 8 seconds.

# Protocol
In this section, a "node" refers to a party that can send or receive messages.
A node can be a client, the communications manager of a server process, a
System coroutine, or "Hub servers".
A Hub server is a logical server that handles non-physical events,
such as player score, load balancing, etc.

All communications go through the `Router`, which manages the internal and
external communication routing.

For each communication, there is a "server" endpoint and a "client" endpoint.
Before the connection is created, the server first listens at a "node address"
(an object ID for internal, a server address plus an object ID for external).
The client requests through the router (or two routers for external) to connect
to a server.

Data are exchanged in units of messages.
Messages of the similar type are grouped as the same endpoint of a "protocol".
Multiple protocols can be grouped as a "protocol aggregate",
which shares the same identification information and reuses the connection.

When an external connection is created, the server sends a challenge query to
the client.
This can be an empty query, but it must be sent even if empty.
The client then sends a reply (based on the query or otherwise).
If the server accepts the query, it replies with a `ChallengeResult::Ok`
message, otherwise `ChallengeResult::Fail` and closes the connection.
Then, protocol-specific messages can be transferred through the aggregate
wrapper packets.

## Time synchronization
Time synchronization is used to synchronize the "game time" between processes.
Game time is represented as number of "ticks" (1 tick = 20ms, 50 ticks = 1s)
since the "Universal Epoch", which is the instant when the first node in the
network started.

Since nodes in the same process have the same system clock,
Time synchronization only happens between nodes of different processes/machines.
In addition, to avoid overloading,
the server side of time synchronization can be System coroutines.

Clients should only perform time synchronization with System servers.

### Request
The request contains a random `id` field.

### Response
The response contains the `id` from the request,
as well as the `time`, the current game time.

## Client-system connection
Client-system connection is the wrapper protocol for multiple types
("channels")
of communication between the client and Systems.
Systems in the same process communicate with the same client using the same
websocket.

### Observer channel
The observer channel allows authorized clients to listen to events happening
within the System.

#### Handshake
When a client is allowed to start listening to events in a System
(most likely authorized by the parent or child of the System),
the System and the authorizer indirectly exchange an
`intra::AllowObserve` message.
`intra::AllowObserve::Response::token` is forwarded to the client,
which uses the token to perform observer handshake to this System
through the `cs::obs::Handshake::Request` message.

#### Accept
If the System accepts the handshake, it responds with a
`cs::obs::Handshake::Response` message.
The response message contains the components of the center large body of the
System, as well as the external traits of its children.

#### Event
The `cs::obs::Event` message encapsulates events happening in the System.
For example, `cs::obs::EventContent::Accel` sets the gravity-independent
acceleration of a child object.

#### Termination
When the authorization to listen to events is revoked,
the authorizer sends an `intra::RevokeObserve` message to the System.
Then the System would send a `cs::obs::Close` message to the client and close
the connection.

### Controller channel
For gamemodes where the player controls a private object ("spacecraft"),
the controller channel allows the player to toggle spacecraft controls.

Controller channel takes place bteween the controlling client and the System
that handles the spacecraft.
If the spacecraft is transferred to another System,
the Controller channel is closed, and the client should create one in the new System.

Unresolved: what if two transfers happen in a short period of time?

#### Handshake
A client gains access to a spacecraft by providing its ID and password
through the `cs::ctrl::Handshake::Request` message.
The password is a random `u64` generated by the server,
and provided to the client most likely during game join.

#### Accept
If the System accepts the handshake, it responds with a
`cs::ctrl::Handshake::Response` message.
This message provides information of various controls of the spacecraft.

#### Control
The `cs::ctrl::Control` message encapsulates client actions to update the
spacecraft controls.

The `cs::ctrl::Update` message encapsulates events in the System
(such as changes in energy levels)
that only the rocket controller should receive.

## Client-hub connection
This connection exchanges information about general public data and
player-specific data, such as online server list, accumulated score, etc.

## Intra-system connection
This connection exchanges information about object transfers.
Since object transfer can only occur between parent and children,
the connection only exists at this level.

### Object transfer

### Observer authorization

## System-hub connection
This connection allows Systems to retrieve and update persistent data about
players, such as player accumulated score.

# Gamemodes
Adding more gamemodes is a late stage of development.
In early stage, only the simplest Basic FFA mode is implemented.

## Basic FFA
Players join in one-time sessions (no progress is retained).
Players are attached to a specific "spacecraft" (a type of small body),
and control its acceleration in a restricted range.
The spacecraft can produce "missiles" (another type of small body),
ejected with a specific initial velocity.

### Durability
Both spacecrafts and missiles have "durability" (a type of scalar value).
They disappear when durability reaches zero.

Missiles lose durability over time.
Spacecrafts also lose durability over time (at an quadratic increasing rate)
if there is no player controlling it.

When two objects collide (this requires better definition),
their durability reduce at a different ratio,
as defined by their "resistance value".
In Basic mode, all spacecrafts have the same resistance value,
and all missiles have the same resistance value.

### Environment
Around a Sun, there are four planets, initially equally spaced in angle,
with different radii from the Sun.
Each planet has 2-3 children.

## Evo FFA
Similar to Basic FFA, but there exists various classes of spacecrafts,
allowing variation in acceleration range, rotation speed, durability,
resistance value and missile type.

Different missile types also vary in ejection velocity, durability, resistance
value, and possibly other logic.

All players start with the basic class.
Spacecrafts can pick up floating space debris to increase their "energy level",
which can be used to transform the spacecraft into other classes.
Energy level can also be increased by destroying other spacecrafts.

## Expert FFA
Similar to Evo FFA, but there is an additional constraint of fuel.
All operations consume fuel, and the player has to pick up hydrogen and oxygen
orbs in space in order to refill the fuel.

# Client display
