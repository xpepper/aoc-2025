# Day 9: Movie Theater

You slide down the firepole into the North Pole base movie theater. The tiled floor has a handful of red tiles, given as `x,y` coordinates (origin at the top left, `x` increasing to the right, `y` increasing downward).

Using **any two red tiles as opposite corners of an axis-aligned rectangle**, find the largest possible rectangle area. Rectangle area counts every tile inside the box, including the corners: `area = (|x1 - x2| + 1) * (|y1 - y2| + 1)`.

## Example

Sample red tiles:

```
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
```

Visualized (`#` = red tile, `.` = empty):

```
..............
.......#...#..
..............
..#....#......
..............
..#......#....
..............
.........#.#..
..............
```

One rectangle with maximum area uses corners `2,5` and `11,1`, giving an area of `50`:

```
..............
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..............
.........#.#..
..............
```

> **Part 1** — Given the list of red tile coordinates, what is the largest rectangle area you can form using two red tiles as opposite corners?

> **Part 2** — (Not tackled yet.) Rectangles must stay entirely inside the region filled by red tiles and the green boundary traced between consecutive red tiles.
