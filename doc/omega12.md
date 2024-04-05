# Hex tiles

Hex tiles form tesselations whose

* angles are all integer multiples of 30 degrees
* lengths are all integer multiples of 1 or $\sqrt 3$

## Directions

There are 12 possible directions.  Unit vectors $\omega_{12}^N$, $N=0..11$, start with

$$
\begin{align*}
\omega_{12}^0 &= [1, 0], \\
\omega_{12}^1 &= [\cos 30^\circ, \sin 30^\circ] \\
&= [\tfrac{1}{2} \sqrt 3, \tfrac{1}{2}], \\
\omega_{12}^2 &= [\cos 60^\circ, \sin 60^\circ] \\
&= [\tfrac{1}{2}, \tfrac{1}{2} \sqrt 3]
\end{align*}
$$

and so on round the circle for the other 9 points.

## Coordinates

We can represent the $[x, y]$ vectors above by producing both $x$ and $y$ from integer multiples of 1 and $\sqrt 3$, and then dividing by 2 afterwards:

$$
\begin{align*}
\omega_{12}^0
&= [1, 0] \\
&= \tfrac{1}{2}[[2 + 0 \sqrt 3], [0 + 0 \sqrt 3]]], \\
\omega_{12}^1
&= [\tfrac{1}{2} \sqrt 3, \tfrac{1}{2}] \\
&= \tfrac{1}{2}[[0 + 1 \sqrt 3], [1 + 0 \sqrt 3]]], \\
\omega_{12}^2
&= [\tfrac{1}{2}, \tfrac{1}{2} \sqrt 3] \\
&= \tfrac{1}{2}[[1 + 0 \sqrt 3], [0 + 1 \sqrt 3]]]
\end{align*}
$$

Or, even more compactly, in an obvious notation,

$$
\begin{align*}
\omega_{12}^0 &= [[2, 0], [0, 0]], \\
\omega_{12}^1 &= [[0, 1], [1, 0]], \\
\omega_{12}^2 &= [[1, 0], [0, 1]] \\
\end{align*}
$$

These are unit distances in each of the first three directions (and it should be obvious how they would continue round the circle).

We could just as well as distances of $\sqrt 3$ in either direction:

$$
\begin{align*}
\sqrt 3 \omega_{12}^0 &= [[0, 2], [0, 0]], \\
\sqrt 3 \omega_{12}^1 &= [[3, 0], [0, 1]], \\
\sqrt 3 \omega_{12}^2 &= [[0, 1], [3, 0]] \\
\end{align*}
$$

It should then be obvious that any combination of unit or $\sqrt 3$ lengths, in any of the $\omega_{12}$ directions, can be added together, by simply adding within this coordinate system.

Such additions have exact integer representations, and so coordinates, potentially reached via different routes, can be compared exactly.

(Such exact comparison would not be possible, if we represented coordinates in floating-point $[x,y]$ pairs and added such pairs successively.)

## Characterizing the notation

So, our system is represented by

* a basis vector $\vec k = [1, \sqrt 3]$, along with a denominator $d = 2$
* any coordinate position or displacement vector $[x, y]$ is computed from $[\vec x, \vec y]$ by taking dot product with $k$ and dividing by $d$: $x = \vec x . \vec k / d, y = \vec y . \vec k / d$
* any length $s$ is obtained from a length-basis vector $\vec l = [1, \sqrt 3]$, such that $s = \vec s . \vec l$

Unit displacements in the 12 basis directions can all be expressed in this coordinate system -- as shown above.

To multiply a displacement $[[x_0, x_1], [y_0, y_1]]$ by a length $[s_0, s_1]$, the result is $[[s_0 x_0 + 3 s_1 x_1, s_0 x_1 + s_1 x_0], [s_0 y_0 + 3 s_1 y_1, s_0 y_1 + s_1 y_0]]$.

Here, $\vec l = \vec k$, but this is a coincidence: we will maintain $\vec l$ and $\vec k$ as distinct parameters.
