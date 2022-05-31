# 
This is an incomplete implementation in the Rust language of a ray tracer as described in the book http://raytracerchallenge.com. It covers the first two chapters of the book.

The code is organized as a cargo workspace. The constituent projects are:

- raytracelib: implementations of the Color, Canvas, and Tuple classes from Chapter 1 and 2.

- projectile: an executable that draws a parabola on a canvas and exports it in PPM format. (Based on "Putting it Together" from Chapter 2)

- projectile_native: the same as above but using Rust vector classes instead of Tuple (incomplete)

