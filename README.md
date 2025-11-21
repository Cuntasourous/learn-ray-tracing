# Ray Tracer - Documentation

A physically-based ray tracer written in Rust that renders realistic 3D scenes with ray tracing, materials, and lighting effects.

## Table of Contents

1. [Features](#features)
2. [Getting Started](#getting-started)
3. [Creating Objects](#creating-objects)
4. [Working with Materials](#working-with-materials)
5. [Controlling the Camera](#controlling-the-camera)
6. [Adjusting Brightness](#adjusting-brightness)
7. [Rendering](#rendering)

---

## Features

This ray tracer implements the following features:

### Core Ray Tracing
- **Ray Casting**: Sends rays from the camera through each pixel to calculate color
- **Recursive Ray Bouncing**: Rays bounce off surfaces up to a maximum depth for realistic light simulation
- **Anti-Aliasing**: Multiple samples per pixel reduce jagged edges for smoother results

### Materials
- **Lambertian (Diffuse)**: Realistic matte surfaces that scatter light in all directions
- **Metal (Reflective)**: Shiny reflective surfaces with optional fuzziness for brushed metal effects
- **Dielectric (Transparent)**: Glass-like transparent materials with refraction and Fresnel reflection

### Rendering
- **Scene Management**: Add multiple objects to a scene with automatic intersection testing
- **Aspect Ratio Control**: Render at any desired aspect ratio (16:9, 4:3, square, etc.)
- **Variable Quality**: Adjust samples per pixel and bounce depth for quality vs. performance tradeoffs

---

## Getting Started

### Building and Running

```bash
# Build the project
cargo build --release

# Run and output to PPM file
cargo run --release > image.ppm
```

### Basic Scene Structure

All scene setup happens in the `main()` function. Here's the general workflow:

```rust
fn main() {
    // 1. Configure image dimensions and rendering quality
    const IMAGE_WIDTH: i32 = 800;
    const IMAGE_HEIGHT: i32 = 600;
    const SAMPLES_PER_PIXEL: i32 = 100;  // More = better quality, slower
    const MAX_DEPTH: i32 = 50;            // Bounce depth for ray reflections

    // 2. Create the world (scene)
    let mut world = HittableList::new();
    
    // 3. Add objects to the scene (see next sections)
    
    // 4. Create and configure the camera
    let cam = Camera::new(...);
    
    // 5. The render loop runs automatically
}
```

---

## Creating Objects

Currently, the ray tracer supports **Spheres**. Below is how to create them and how to add other object types.

### Creating a Sphere

Spheres are the primary geometric primitive in this ray tracer.

```rust
use std::rc::Rc;
use sphere::Sphere;
use material::{Lambertian, Metal, Dielectric};
use vec3::Point3;
use color::Color;

// Create a material first
let material = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));

// Create a sphere at center (0, 0, -1) with radius 0.5
let sphere = Sphere::new(
    Point3::new(0.0, 0.0, -1.0),  // Center position (x, y, z)
    0.5,                            // Radius
    material                        // Material reference
);

// Add to world
world.add(Box::new(sphere));
```

**Sphere Parameters:**
- **Center**: A 3D point `Point3::new(x, y, z)` specifying the sphere's center
- **Radius**: Positive floating-point number defining the sphere's size
- **Material**: A material that controls how the sphere reflects/scatters light

### Example: Creating Multiple Spheres

```rust
let mut world = HittableList::new();

// Ground sphere (large, bottom)
let ground_mat = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
world.add(Box::new(Sphere::new(
    Point3::new(0.0, -100.5, -1.0),
    100.0,
    ground_mat,
)));

// Center sphere (matte blue)
let center_mat = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
world.add(Box::new(Sphere::new(
    Point3::new(0.0, 0.0, -1.0),
    0.5,
    center_mat,
)));

// Right sphere (metallic)
let metal_mat = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3));
world.add(Box::new(Sphere::new(
    Point3::new(1.0, 0.0, -1.0),
    0.5,
    metal_mat,
)));

// Left sphere (glass)
let glass_mat = Rc::new(Dielectric::new(1.5));
world.add(Box::new(Sphere::new(
    Point3::new(-1.0, 0.0, -1.0),
    0.5,
    glass_mat.clone(),
)));
```

### Adding New Object Types (Future Enhancement)

To add new object types (cube, cylinder, plane):

1. **Create a new file** (e.g., `cube.rs`)
2. **Implement the `Hittable` trait** with a `hit()` method
3. **Add the module** to `main.rs`
4. **Create instances** and add to the world

Example structure for a new object:

```rust
// In cube.rs
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct Cube {
    min: Point3,
    max: Point3,
    mat: Rc<dyn Material>,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, mat: Rc<dyn Material>) -> Cube {
        Cube { min, max, mat }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Implement ray-box intersection logic here
        // ...
    }
}
```

---

## Working with Materials

Materials control how objects interact with light. Three materials are available:

### 1. Lambertian (Diffuse/Matte)

Creates realistic matte surfaces that scatter light uniformly in all directions.

```rust
use material::Lambertian;
use color::Color;
use std::rc::Rc;

let material = Rc::new(Lambertian::new(Color::new(r, g, b)));
```

**Parameters:**
- **Color** `(r, g, b)`: Values from 0.0 to 1.0 for red, green, blue
  - `(1.0, 0.0, 0.0)` = Pure red
  - `(0.5, 0.5, 0.5)` = Medium gray
  - `(1.0, 1.0, 1.0)` = White
  - `(0.0, 0.0, 0.0)` = Black (absorbs all light)

**Use Cases:**
- Wall surfaces
- Paper, fabric
- Wood, concrete
- Most real-world non-shiny surfaces

**Example:**
```rust
let wood = Rc::new(Lambertian::new(Color::new(0.8, 0.7, 0.6)));
let brick = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.2)));
```

### 2. Metal (Reflective)

Creates shiny reflective surfaces. The fuzz parameter controls how "brushed" the metal appears.

```rust
use material::Metal;
use color::Color;
use std::rc::Rc;

let material = Rc::new(Metal::new(Color::new(r, g, b), fuzz));
```

**Parameters:**
- **Color** `(r, g, b)`: The metal's color (typically grayscale for realism)
- **Fuzz** (0.0 to 1.0): Surface roughness
  - `0.0` = Mirror-like perfect reflection
  - `0.3` = Slightly brushed metal
  - `1.0` = Very rough, almost diffuse

**Use Cases:**
- Polished metal
- Mirrors
- Shiny surfaces

**Example:**
```rust
let polished_gold = Rc::new(Metal::new(Color::new(1.0, 0.8, 0.0), 0.0));
let brushed_aluminum = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.5));
let rough_steel = Rc::new(Metal::new(Color::new(0.5, 0.5, 0.5), 0.8));
```

### 3. Dielectric (Transparent/Glass)

Creates glass-like transparent surfaces with refraction and Fresnel effects.

```rust
use material::Dielectric;
use std::rc::Rc;

let material = Rc::new(Dielectric::new(index_of_refraction));
```

**Parameters:**
- **Index of Refraction** (IOR): Controls how much light bends when entering the material
  - `1.0` = Air/vacuum (no bending)
  - `1.5` = Glass
  - `1.33` = Water
  - `2.42` = Diamond

**Use Cases:**
- Window glass
- Water
- Diamonds
- Transparent plastic

**Example:**
```rust
let window_glass = Rc::new(Dielectric::new(1.5));
let water = Rc::new(Dielectric::new(1.33));
let diamond = Rc::new(Dielectric::new(2.42));
```

**Note:** To create a hollow sphere (like a soap bubble), place a smaller sphere with negative radius inside a larger one:

```rust
let hollow_glass = Rc::new(Dielectric::new(1.5));

// Outer surface
world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, hollow_glass.clone())));

// Inner surface (negative radius acts as inside-out sphere)
world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), -0.45, hollow_glass)));
```

---

## Controlling the Camera

The camera determines what and how the scene is viewed. It's highly configurable.

### Camera Creation

```rust
use camera::Camera;
use vec3::{Point3, Vec3};

let cam = Camera::new(
    Point3::new(-2.0, 2.0, 1.0),    // lookfrom: Where the camera is positioned
    Point3::new(0.0, 0.0, -1.0),    // lookat: What point the camera looks at
    Vec3::new(0.0, 1.0, 0.0),       // vup: "Up" direction (typically Y-axis)
    20.0,                            // vfov: Vertical field of view in degrees
    ASPECT_RATIO,                    // aspect_ratio: Width/Height ratio
);
```

### Camera Parameters Explained

#### 1. **lookfrom** (Camera Position)
The 3D coordinates where the camera is placed.

```rust
// Camera above and to the left
Point3::new(-2.0, 2.0, 1.0)
//           x     y    z

// High altitude view
Point3::new(0.0, 5.0, 0.0)

// Ground level, behind the scene
Point3::new(0.0, 0.0, 3.0)
```

#### 2. **lookat** (Target Point)
The 3D point the camera aims at.

```rust
Point3::new(0.0, 0.0, -1.0)   // Look at center of scene
Point3::new(1.0, 0.5, -1.0)   // Look at a different object
```

#### 3. **vup** (Up Direction)
Defines which direction is "up" for the camera. Almost always `(0, 1, 0)` (Y-axis).

```rust
Vec3::new(0.0, 1.0, 0.0)   // Standard: Y is up
Vec3::new(0.0, 0.0, 1.0)   // Roll camera so Z is up
```

#### 4. **vfov** (Vertical Field of View)
The vertical viewing angle in degrees. Lower values = zoomed in, higher values = wide angle.

```rust
20.0   // Telephoto lens (zoomed)
50.0   // Standard/normal lens
90.0   // Wide angle lens
120.0  // Very wide angle
```

#### 5. **aspect_ratio** (Image Proportions)
Width divided by height. Common values:

```rust
16.0 / 9.0   // Widescreen (1280x720, 1920x1080)
4.0 / 3.0    // Standard (800x600)
1.0          // Square (512x512)
```

### Camera Examples

#### Example 1: Standard View
```rust
const ASPECT_RATIO: f64 = 16.0 / 9.0;
let cam = Camera::new(
    Point3::new(0.0, 0.0, 0.0),      // Camera at origin
    Point3::new(0.0, 0.0, -1.0),     // Looking forward
    Vec3::new(0.0, 1.0, 0.0),        // Y is up
    50.0,                             // Normal FOV
    ASPECT_RATIO,
);
```

#### Example 2: Bird's Eye View (Top-Down)
```rust
let cam = Camera::new(
    Point3::new(0.0, 10.0, 0.0),     // Camera high above
    Point3::new(0.0, 0.0, -1.0),     // Looking down at scene
    Vec3::new(0.0, 1.0, 0.0),
    60.0,
    16.0 / 9.0,
);
```

#### Example 3: Side View
```rust
let cam = Camera::new(
    Point3::new(5.0, 0.0, 0.0),      // Camera to the right
    Point3::new(0.0, 0.0, 0.0),      // Looking at center
    Vec3::new(0.0, 1.0, 0.0),
    40.0,
    16.0 / 9.0,
);
```

#### Example 4: Cinematic Angled View
```rust
let cam = Camera::new(
    Point3::new(-3.0, 2.5, 2.0),     // Offset position
    Point3::new(0.0, 0.5, -1.0),     // Look at slightly raised point
    Vec3::new(0.0, 1.0, 0.0),
    35.0,
    16.0 / 9.0,
);
```

#### Example 5: Macro/Zoom View
```rust
let cam = Camera::new(
    Point3::new(0.0, 0.0, 0.1),      // Very close to scene
    Point3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 1.0, 0.0),
    10.0,                             // Very narrow FOV = zoomed
    16.0 / 9.0,
);
```

---

## Adjusting Brightness

Brightness in ray tracing is controlled through several mechanisms:

### 1. Material Color (Primary Method)

Darker colors = less light reflected = darker appearance. Lighter colors = brighter appearance.

```rust
// Very dark (absorbs most light)
let dark = Rc::new(Lambertian::new(Color::new(0.1, 0.1, 0.1)));

// Medium brightness
let medium = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

// Very bright (reflects most light)
let bright = Rc::new(Lambertian::new(Color::new(0.9, 0.9, 0.9)));
```

### 2. Number of Samples Per Pixel

More samples reduce noise and produce cleaner, sometimes brighter-looking images.

```rust
const SAMPLES_PER_PIXEL: i32 = 10;   // Fast, noisy
const SAMPLES_PER_PIXEL: i32 = 100;  // Good quality
const SAMPLES_PER_PIXEL: i32 = 1000; // Very clean (slow)
```

**Effect:** Each sample takes a random ray through the pixel. More samples average out noise, revealing the true brightness value.

### 3. Ray Bounce Depth

More bounces allow light to scatter further, potentially brightening the scene. The background (sky) is the ultimate light source.

```rust
const MAX_DEPTH: i32 = 10;   // Fewer bounces = darker (light lost early)
const MAX_DEPTH: i32 = 50;   // Standard (light simulates reality)
const MAX_DEPTH: i32 = 100;  // More bounces = potentially brighter
```

### 4. Background Color

The sky/background color acts as the scene's light source. Modify this in the `ray_color` function:

```rust
// In main.rs, ray_color function:

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // ... hit detection ...
    
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    
    // This creates a gradient from blue to white
    // Change these colors to adjust overall brightness:
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) +  // White at top
    t * Color::new(0.5, 0.7, 1.0)             // Light blue at horizon
}
```

**To make the scene brighter:**
```rust
// Brighter sky (more light)
(1.0 - t) * Color::new(1.0, 1.0, 1.0) +
t * Color::new(0.8, 0.8, 1.0)
```

**To make the scene darker:**
```rust
// Darker sky (less light)
(1.0 - t) * Color::new(0.5, 0.5, 0.5) +
t * Color::new(0.2, 0.3, 0.5)
```

### 5. Complete Brightness Control Example

```rust
// In main.rs
const SAMPLES_PER_PIXEL: i32 = 200;    // Higher = cleaner/brighter
const MAX_DEPTH: i32 = 75;              // More bounces = more light

// Create bright materials
let bright_ground = Rc::new(Lambertian::new(Color::new(0.9, 0.9, 0.9)));
let bright_sphere = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.8)));

// Use white light metals
let shiny = Rc::new(Metal::new(Color::new(0.95, 0.95, 0.95), 0.0));

// Render to a bright scene
```

---

## Quick Reference

### Creating Your First Scene

```rust
use std::rc::Rc;
use sphere::Sphere;
use material::{Lambertian, Metal};
use hittable_list::HittableList;
use camera::Camera;
use color::Color;
use vec3::Point3;

fn main() {
    const IMAGE_WIDTH: i32 = 800;
    const IMAGE_HEIGHT: i32 = 600;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // Create world
    let mut world = HittableList::new();

    // Add ground
    let ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground)));

    // Add sphere
    let material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material)));

    // Create camera
    let cam = Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        50.0,
        16.0 / 9.0,
    );

    // Render (main loop handles this)
}
```

---

## Troubleshooting

**Scene appears too dark:** Increase `SAMPLES_PER_PIXEL`, `MAX_DEPTH`, or brighten material colors.

**Scene is too noisy:** Increase `SAMPLES_PER_PIXEL` (more samples = smoother).

**Rendering takes too long:** Reduce `SAMPLES_PER_PIXEL` or `MAX_DEPTH`, or use smaller image dimensions.

**Objects don't appear:** Check camera `lookat` points at your objects, and materials are properly assigned.

---

## Further Reading

For more information about ray tracing concepts, see:
- [Ray Tracing in One Weekend](https://raytracing.github.io/) - The foundational reference for this project
- [Physically Based Rendering](https://pbrt.org/) - Advanced rendering concepts
