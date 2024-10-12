onboard
=======

onboard is the linter for the project management type system introduced in [Rusty Engineering
Management](https://a.co/d/3uLjUPM).

In short, the type system is a way to manage the work of a team.

What it Does
------------

First, navigate to the [example directory](https://github.com/rescrv/onboard/tree/main/example).
This is a complete example of a project that uses onboard.  It has a single p0 objective owned by
the CEO and one p0 objective for each of the three teams.

To run onboard on this example, you can use the following command:

```console
$ onboard example
[✓] graph is a DAG
[✓] ownership is valid
[✓] work scheduling is valid
[✓] priority monotonicity
```

Our linter checks the following properties:

1.  The graph over objectives is a directed acyclic graph (DAG).
2.  Ownership is valid.  There is either an owner or an accountable party listed for each objective.
3.  There are no quantifiably-oversubscribed individuals in the organization.
4.  The priority of objectives is monotonic; that is, no one is answering p0 objectives with p1
    objectives.

Now, if one of the individuals was overloaded, the linter would catch it:

```console
$ onboard example
[✓] graph is a DAG
[✓] ownership is valid
[X] work scheduling is invalid
[✓] priority monotonicity

lint detected:
- too much work for coder1: example/p0.md
```

Anatomy of a Project
--------------------

A project is simply a self-contained directory whose objectives do not reference objectives outside
the project.

The example project has the following structure:

```text
example/
example/p0.md
example/engineering
example/engineering/engineer-it.md
example/marketing
example/marketing/market-it.md
example/product
example/product/product-it.md
```

Each file is a markdown file that contains a single objective.  An objective has the following
structure:

```text
title: Engineer the hell out of it
accountable: coder1
status: planned
size: XXL
priority: 0
parent: ../p0.md

This task requires the engineer to thoroughly investigate
and resolve any issues that arise during development or
deployment. The goal is to push the limits of what is
possible with the technology, even if it means going above
and beyond what is currently considered "normal" engineering
practices.

**Acceptance Criteria:**

* All issues are resolved, regardless of complexity or time
  required
* Solutions are implemented in a way that improves overall
  system performance, scalability, or reliability
* The engineer must demonstrate exceptional problem-solving
  skills and creativity
* Documentation for all changes made is thorough and easy to
  understand

**Notes:**

* Please do not assign this task to anyone without
  discussing it with the team lead first.
* If you are unsure about any aspect of this task, please
  reach out to coder3 for guidance.

This ticket requires a serious amount of engineering
expertise and creativity. The engineer assigned to this task
should be comfortable working outside their normal comfort
zone and should be willing to take calculated risks to
deliver results.
```

It intentionally reeks of nostalgia for HTTP/1.0.  The objective has a title, an accountable party,
a status, a size, a priority, and a parent header.  The parent is the file that contains the
objective that encompasses this objective.  In this case, the parent is `p0.md`.  See the
documentation for the allowable values for this code.

Quick Start
-----------

The central hypotheis of this project is that as software developers we should be building the tools
to govern our own processes.  This is a tool to help manage the work of a team.  But every team will
be different and have different needs:  So to begin with, [fork this
repository](https://github.com/rescrv/onboard/fork) and begin to customize it to your needs.

If you just want to get started locally and are already a rust developer:

```console
$ mkdir -p ~/src/onboard
$ git clone https://github.com/rescrv/onboard ~/src/onboard
$ cd ~/src/onboard
$ cargo run ~/path/to/your/project
```

I intend for this project to be consumable and endlessly forkable.  It's basically throwaway code to
inspire an idea:  We should be building our own tools where we want our tools to reflect our exact
process.

The Type System
---------------

The following is an adapted excerpt from [Rusty Engineering Management](https://a.co/d/3uLjUPM).

The inspiration for the onboard is the Rust programming language's borrow-checker.  In short, the
borrow checker upholds the following invariant:  A value can be referenced with a single mutable
reference or several immutable references, but never both at the same time.  

Onboard is about managing Ownership, Accountability, and Responsibility.  These are the three pieces
of project management that can be programmatically checked.  The central hypothesis of Rusty
Engineering Management is that people will locally align if given the right structure to ensure
their local alignment is in gloal alignment.  With some rules around ownership, accountability, and
responsibility we can "type check" our project management directory hierarchy.

Accountability is like holding a mutable reference in Rust.  The holder of the reference can
delegate it to another location, but cannot do anything with the reference until that delegation is
over.  Similarly, that delegation must come to an end, because the reference is borrowed, not owned.
At the end of scope, it must be returned from where it was delegated, which means it must be
returned to be returned.

Responsibility is like holding an immutable reference in Rust.  The holder of the reference can make
new objects that descend from it and include their own immutable reference, but the immutable
reference says they should not do anything to change the reference they borrowed, except in narrow
and well-defined ways.

Finally, ownership matches cleanly to Rust's ownership model.  In Rust, only an owner can move an
object to somewhere else, and only an owner can drop the object.  If no one else has a reference to
an object, the owner can always obtain a mutable or immutable reference to the object.

To complete the metaphor, the pattern of Rust code most closely aligned to our model is one in which
the delegate of a mutable reference converts it to several immutable references, embeds each within
a sub-objective, and then passes mutable references to these sub-objectives to their subordinates.
The rules work for objects and objectives.

Limitations
-----------

There are two limitations to be aware of with onboard:

* The included code is not a perfectly faithful representation of Rusty Engineering Management.  In
  particular, it doesn't track chains of delegated accountability.  This will change in a future
  release.

* The code is intended to be forked, to be consumed, and to be transformed into a business-specific
  application.  onboard works well for small projects.  I've only used the owner format thus far,
  having no need for the accountability aspect in personal project management.

  There will be bugs, but when you adopt code, you own it.  I've made owning this code easy.
