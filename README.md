
# Indexed Square Root Algorithm

* Designed by **[**p4lm4d3v**](https://github.com/p4lm4d3v/)** in `2022`
* Implemented in **[**Rust**](https://www.rust-lang.org/)** on `08.07.2024.`

## Benchmark

I tested this algorithm on the number set `[4, 1000000]` and calculated the **minimum difference** between this algorithm and the standard `sqrt()` function implemented for the type [`f64`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.sqrt) in [Rust](https://www.rust-lang.org/).

         ![](min_max.png)

## Algorithm

I designed this algorithm in my second year of high school when I was 16.
I was bored in class, started playing with numbers, and came up with this.

This algorithm is **not perfectly accurate**, but it was not designed to be.
It’s meant to **quickly approximate the square root of a natural number** that does **not** have a natural root.

### Step 1 — Define `z`

We define the number `z` whose root we want to calculate:

```
z ∈ N  and  z > 4
```

### Step 2 — Find surrounding perfect squares

Define the closest numbers that have natural roots around `z`:

```
x < z  and  sqrt(x) ∈ N     (below z)
y > z  and  sqrt(y) ∈ N     (above z)
```

### Step 3 — Generate range between x and y

Create a range from `x` to `y`, **not including them**:

```
range(x, y) = x+1, x+2, ..., y-2, y-1
```

### Step 4 — Define n as the range length

Let `n` be the length of that range:

```
n = length(range(x, y))
```

We can also express it as:

```
n = 2 * sqrt(y) - 3
```

### Step 5 — Index the range (zero-based)

```
index(x + 1) = 0
index(x + 2) = 1
index(x + 3) = 2
...
index(y - 2) = n - 2
index(y - 1) = n - 1
```

So the range and its indices are:

```
range(x, y):       x+1, x+2, ..., y-1
indices:           0, 1, ..., n-1
```

### Step 6 — Find the index of z in that range

To find where `z` lies between `x` and `y`:

```
idx = index of z in range
```

A simple way to calculate it:

```
range[0] = x + 1
idx = z - range[0]
idx = z - (x + 1)
idx = z - x - 1
```

### Step 7 — Logical bounds

Since:

```
sqrt(z) > sqrt(x)  and  sqrt(z) < sqrt(y)
```

This also implies:

```
z > x  and  z < y
```

### Step 8 — Final formula

We can now approximate the square root of z as:

```
sqrt(z) ≈ sqrt(x) + (idx / n)
```

Simplified:

```
sqrt(z) ≈ sqrt(x) + (z - x - 1) / (2 * sqrt(y) - 3)
```

## Disclaimer & Conclusion

Given the constraints, numbers ≤ 4 cannot be used because their surrounding roots overlap.

This algorithm can be seen as **a form of linear interpolation**.
Values between perfect squares are approximated linearly based on their distance between those perfect squares.

## Examples

### Example 1

```
z = 12
x = 9
y = 16

sqrt(12) ≈ sqrt(9) + (12 - 9 - 1) / (2 * sqrt(16) - 3)
sqrt(12) ≈ 3 + (2) / (8 - 3)
sqrt(12) ≈ 3 + 0.4
sqrt(12) ≈ 3.4

sqrt(12) = 3.46410161513775 (real root)
error = +0.06410161513775 
```

### Example 2

```
z = 34
x = 25
y = 36

sqrt(34) ≈ sqrt(25) + (34 - 25 - 1) / (2 * sqrt(36) - 3)
sqrt(34) ≈ 5 + (8) / (12 - 3)
sqrt(34) ≈ 5 + 0.8888888889
sqrt(34) ≈ 5.8888888889

sqrt(34) = 5.8309518948453 (real root)
error = -0.05793699405
```



### Example 3

```
z = 69
x = 64
y = 81

sqrt(69) ≈ sqrt(64) + (69 - 64 - 1) / (2 * sqrt(81) - 3)
sqrt(69) ≈ 8 + (4) / (18 - 3)
sqrt(69) ≈ 8 + 0.2666666667
sqrt(69) ≈ 8.2666666667

sqrt(69) = 8.30662386291808 (real root)
error = +0.0399571962
```



### Example 4

```
z = 95
x = 81
y = 100

sqrt(95) ≈ sqrt(81) + (95 - 81 - 1) / (2 * sqrt(100) - 3)
sqrt(95) ≈ 9 + (13) / (20 - 3)
sqrt(95) ≈ 9 + 0.7647058824
sqrt(95) ≈ 9.7647058824

sqrt(95) = 9.74679434480896 (real root)
error = -0.0179115376
```


