# Omega space

## Introducing $\omega$

We're going to be working with tiles which have

* unit-length sides
* at angles which are multiples _either_ of $45^\circ$ _or_ or $30^\circ$

We want some underlying geometry engine which

* recognizes these angles
* has convenient functions for rotating directions, adding vectors within those contexts etc
* can do all key calculations in integers
* converts eventually to $(x,y)$ coordinates for display

We call this "omega geometry", specifically we have two geometries,

* $\omega_8$, which has eight directions $45^\circ$ apart
* $\omega_{12}$, which has twelve directions $30^\circ$ apart

## Unit vectors in $\omega$ 8 and 12 spaces

Now it happens that the unit vectors in these spaces can be expressed in a simple form.  In $\omega_8$, our first three unit vectors (at 0, 45 and 90 degrees) are:

$$
\begin{align*}
\omega_{8, 0} &= (1, 0), \\
\omega_{8, 1} &= (\sqrt 2 / 2, \sqrt 2 / 2), \\
\omega_{8, 2} &= (0, 1) \\
\end{align*}
$$

And similarly, for $\omega_{12}$, the first four unit vectors (at 0, 30, 60 and 90 degrees) are:

$$
\begin{align*}
\omega_{12, 0} &= (1, 0), \\
\omega_{12, 1} &= (\sqrt 3 / 2, 1 / 2), \\
\omega_{12, 2} &= (1 / 2, \sqrt 3 / 2), \\
\omega_{12, 3} &= (0, 1) \\
\end{align*}
$$

We can factor this:

$$
\begin{align*}
\omega_{8, 0} &= \tfrac{1}{2}([2, 0] . [1, \sqrt 2], [0, 0] . [1, \sqrt 2]), \\
\omega_{8, 1} &= \tfrac{1}{2}([0, 1] . [1, \sqrt 2], [0, 1] . [1, \sqrt 2]), \\
\omega_{8, 2} &= \tfrac{1}{2}([0, 0] . [1, \sqrt 2], [2, 0] . [1, \sqrt 2]), \\
\omega_{12, 0} &= \tfrac{1}{2}([2, 0] . [1, \sqrt 3], [0, 0] . [1, \sqrt 3]), \\
\omega_{12, 1} &= \tfrac{1}{2}([0, 1] . [1, \sqrt 3], [1, 0] . [1, \sqrt 3]), \\
\omega_{12, 2} &= \tfrac{1}{2}([1, 0] . [1, \sqrt 3], [0, 1] . [1, \sqrt 3]), \\
\omega_{12, 3} &= \tfrac{1}{2}([0, 0] . [1, \sqrt 3], [2, 0] . [1, \sqrt 3]) \\
\end{align*}
$$

And, to put that even more compactly, this suggests that we can represent the first three vectors in $\omega_8$ space simply by these coordinates:

$$
\begin{align*}
\omega_{8, 0} &= ([2, 0], [0, 0]), \\
\omega_{8, 1} &= ([0, 1], [0, 1]), \\
\omega_{8, 2} &= ([0, 0], [2, 0]) \\
\end{align*}
$$

where it's understood that, in $\omega_8$ space, to produce an $(x, y)$ coordinate from coordinates like the above, you dot-multiply each component by the basis $[1, \sqrt 2]$, and then divide by 2.

Similarly, in $\omega_{12}$, we have

$$
\begin{align*}
\omega_{12, 0} &= ([2, 0], [0, 0]), \\
\omega_{12, 1} &= ([0, 1], [1, 0]), \\
\omega_{12, 2} &= ([1, 0], [0, 1]), \\
\omega_{12, 3} &= ([0, 0], [2, 0]) \\
\end{align*}
$$

where here the basis is $[1, \sqrt 3]$ and the divisor is still 2.

This is nice because:

* you can express any point or vector in $\omega$ space in terms of one of these vectors
* you can add vectors to vectors, and vectors to points, in any combination, by adding their components
* you can the do a final conversion into $(x, y)$ as above

## Directions

You can express any direction as number a number $d$, so that

* in $\omega_8$ space, the unit vector in that direction is $\omega_{8, d}$ as above, $d$ is in range $0..7$, and the other unit vectors from 135 to 315 degrees are produced in an obvious way
* in $\omega_{12}$ space, it's similar but with $d$ in range $0..11$

You can also express any _change_ in direction as a number, for example 1 in $\omega_8$ is $45^\circ$ left, while $-1$ is $45^\circ$ right.

## Tiles

You can then express shapes in terms of these changes in direction.  For example,

* in $\omega_{12}$,
  * a hexagon is $[2, 2, 2, 2, 2, 2]$
  * a triangle is $[4, 4, 4]$
  * a square is $[3, 3, 3, 3]$
* in $\omega_8$
  * an octagon is $[1, 1, 1, 1, 1, 1, 1, 1]$
  * a square is $[2, 2, 2, 2]$

So we produce a shape by

* starting at an initial point and an initial direction
* for each turn in the shape vector,
  * proceed one unit in the current direction
  * increment the direction by the next number in the list (mod the number of directions in the space)

And we can see that,

* for convex polygons, like all the above, all the turns are left turns, and therefore positive numbers
* the list of numbers adds up to the number of directions in the space

We could have a shape with outward turns, like the spectre, which in $\omega_12$ has fourteen turns:

* $[2, -3, 2, 3, 2, -3, 2, 0, 2, 3, -2, 3, -2, 3]$

This has negative direction changes for the outwards turns.  The total is still 12.

## Tesselations

A tesselation, then, is a set of placements of tile templates into a model which has an $(x, y)$ coordinate grid.  But, when we place them, we use the $\omega$ coordinate system.  We convert into $(x, y)$ only when needed.

## Interesting multiples

Any multiple of an $\omega_8$ coordinate by $\sqrt 2$ will produce another $\omega_8$ coordinate:

$$
\sqrt 2 [a, b] = [2b, a]
$$

Similarly, in $\omega_{12}$:

$$
\sqrt 3 [a, b] = [3b, a]
$$

With this knowledge, we can support shapes whose sides are _either_ unit vectors or $\sqrt 2$ (in $\omega_8$) or $\sqrt 3$ (in $\omega_{12}$).

## Penrose tilings

Penrose tilings are based on $\omega_{20}$, and require $\phi$, the golden ratio $\tfrac{1}{2}(1 + \sqrt 5)$, as the interesting multiple.

We'll spell out more of the $\omega_{20}$ coordinates in a later update.