# 2023 Day 24: Part 2

## Problem Definition

Given hailstone paths  

$$
H_n(t) = H^0_n + t \partial H_n
$$

Find a rock path  

$$
 R(t) = R_0 + t \partial R
$$

$$
\text{such that } \forall n \space \exists \space t_n \text{ such that } R(t_n) = H_n(t_n)
$$  

I.e. The rock impacts all hailstones.

## Solution

Considering $n=0:$

$$
R(t_0) = H_0(t_0)
$$  

$$
R_0 + t_0\partial R = H^0_0 + t_0 \partial H_0
$$  

$$
\implies R_0 - H^0_0 = t_0(\partial H_0 - \partial R)
$$

$$
\implies R_0 - H_0^0 \parallel \partial H_0 - \partial R
$$

$$
\implies C_0 := R_0 - H_0^0 \times \partial H_0 - \partial R = 0
$$

Giving us 3 equations

$$
(C_0)_x = 0
$$

$$
(C_0)_y = 0
$$

$$
(C_0)_z = 0
$$

Computing $(C_0)_x$:

$$
\text{let} \space P^0 := R_0 - H_0^0
$$

$$
V^0 := \partial H_0 - \partial R
$$

So

$$
P^0 \times V^0 = \begin{vmatrix}
i & j & k \\
P_x^0 & P_y^0 & P_z^0 \\
V_x^0 & V_y^0 & V_z^0 \\
\end{vmatrix}
$$

$$
\text Giving:
$$

$$
P^0_yV^0_z = P_z^0V_y^0
$$

$$
P_x^0V_z^0 = P_z^0V_x^0
$$

$$
P_x^0V_y^0 = P_y^0V_x^0
$$

So $P_aV_b = P_bV_a\space\forall\space a \neq b$

Expanding:

$$
P_a^0V_b^0 = P_b^0V_a^0
$$

$$
\implies (R_{0a} - H_{0a}^0)(\partial H_{0b} - \partial R_b) = (R_{0b} - H_{0b}^0)(\partial H_{0a} - \partial R_a)
$$

$$
\implies R_{0a}\partial H_{0b} - R_{0a}\partial R_b - H_{0a}^0\partial H_{0b} + H_{0a}^0\partial R_b =..
$$
$$
..= R_{0b}\partial H_{0a} - R_{0b}\partial R_a - H_{0b}^0\partial H_{0a} + H_{0b}^0\partial R_a
$$

Repeating this with $n=1$ we get 3 more equations.
Subtracting each from their corresponding $n=0$ equation non-linear components are eliminated.

$$
\text{LHS } \implies R_{0a}\partial H_{0b} - R_{0a}\partial R_b - H_{0a}^0\partial H_{0b} + H_{0a}^0\partial R_b ...
$$
$$
... - R_{0a}\partial H_{1b} + R_{0a}\partial R_b + H_{1a}^0\partial H_{1b} - H_{1a}^0\partial R_b
$$

$$
= R_{0a}( \partial H_{0b} - \partial H_{1b} ) + \partial R_b ( H_{0a}^0 - H_{1a}^0) - H_{0a}^0\partial H_{0b}  + H_{1a}^0\partial H_{1b}
$$

$$
\text{RHS } \implies R_{0b}\partial H_{0a} - R_{0b}\partial R_a - H_{0b}^0\partial H_{0a} + H_{0b}^0\partial R_a ...
$$
$$
... - R_{0b}\partial H_{1a} + R_{0b}\partial R_a + H_{1b}^0\partial H_{1a} - H_{1b}^0\partial R_a
$$

$$
= R_{0b}( \partial H_{0a} - \partial H_{1a} ) + \partial R_a ( H_{0b}^0 - H_{1b}^0) - H_{0b}^0\partial H_{0a} + H_{1b}^0\partial H_{1a}
$$

Moving unknowns $R_{0a}, R_{0b}, R_{0c}, \partial R_a, \partial R_b, \partial R_c$ to the LHS:

$$
R_{0a}( \partial H_{0b} - \partial H_{1b} ) + \partial R_b ( H_{0a}^0 - H_{1a}^0) - R_{0b}( \partial H_{0a} - \partial H_{1a} ) - \partial R_a ( H_{0b}^0 - H_{1b}^0) = ...
$$
$$
... = H_{1b}^0\partial H_{1a} - H_{0b}^0\partial H_{0a} + H_{0a}^0\partial H_{0b}  - H_{1a}^0\partial H_{1b}
$$

Repeating for $n=2$:

$$
R_{0a}( \partial H_{0b} - \partial H_{2b} ) + \partial R_b ( H_{0a}^0 - H_{2a}^0) - R_{0b}( \partial H_{0a} - \partial H_{2a} ) - \partial R_a ( H_{0b}^0 - H_{2b}^0) = ...
$$
$$
... = H_{2b}^0\partial H_{2a} - H_{0b}^0\partial H_{0a} + H_{0a}^0\partial H_{0b}  - H_{2a}^0\partial H_{2b}
$$

Making our unknowns into a vector:

$$
\vec{X} := \begin{bmatrix}
R_{0x} \\
R_{0y} \\
R_{0z} \\
\partial R_x \\
\partial R_y \\
\partial R_z \\
\end{bmatrix}
$$

Gives a linear system

$$
\matrix{A}\vec{X} = \vec{B}
$$

Where

$$
\matrix{A} := \begin{bmatrix}
(\partial H_{0y} - \partial H_{1y}) & -(\partial H_{0x} - \partial H_{1x}) & 0 & -(H_{0y}^0 - H_{1y }^0) & (H_{0x}^0 - H_{1x}^0) & 0 \\
(\partial H_{0z} - \partial H_{1z}) & 0 & -(\partial H_{0x} - \partial H_{1x}) & -(H_{0z}^0 - H_{1z}^0) & 0 & (H_{0x}^0 - H_{1x}^0) \\
0 & (\partial H_{0z} - \partial H_{1z}) & -(\partial H_{0y} - \partial H_{1y}) & 0 & -(H_{0z}^0 - H_{1z}^0) &  (H_{0y}^0 - H_{1y}^0) \\
(\partial H_{0y} - \partial H_{2y}) & -(\partial H_{0x} - \partial H_{2x}) & 0 & -(H_{0y}^0 - H_{2y }^0) & (H_{0x}^0 - H_{2x}^0) & 0 \\
(\partial H_{0z} - \partial H_{2z}) & 0 & -(\partial H_{0x} - \partial H_{2x}) & -(H_{0z}^0 - H_{2z}^0) & 0 & (H_{0x}^0 - H_{2x}^0) \\
0 & (\partial H_{0z} - \partial H_{2z}) & -(\partial H_{0y} - \partial H_{2y}) & 0 & -(H_{0z}^0 - H_{2z}^0) &  (H_{0y}^0 - H_{2y}^0) \\
\end{bmatrix}
$$

$$
\vec{B} := \begin{bmatrix}
H_{1y}^0\partial H_{1x} - H_{0y}^0\partial H_{0x} + H_{0x}^0\partial H_{0y}  - H_{1x}^0\partial H_{1y} \\
H_{1z}^0\partial H_{1x} - H_{0z}^0\partial H_{0x} + H_{0x}^0\partial H_{0z}  - H_{1x}^0\partial H_{1z} \\
H_{1z}^0\partial H_{1y} - H_{0z}^0\partial H_{0y} + H_{0y}^0\partial H_{0z}  - H_{1y}^0\partial H_{1z} \\
H_{2y}^0\partial H_{2x} - H_{0y}^0\partial H_{0x} + H_{0x}^0\partial H_{0y}  - H_{2x}^0\partial H_{2y} \\
H_{2z}^0\partial H_{2x} - H_{0z}^0\partial H_{0x} + H_{0x}^0\partial H_{0z}  - H_{2x}^0\partial H_{2z} \\
H_{2z}^0\partial H_{2y} - H_{0z}^0\partial H_{0y} + H_{0y}^0\partial H_{0z}  - H_{2y}^0\partial H_{2z} \\
\end{bmatrix}
$$

So the initial rock position and velocity can be found by solving this system.
