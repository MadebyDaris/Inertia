
## Angular Physics
In physics, **torque** and **moment of inertia** are fundamental concepts in rotational dynamics, similar to how force and mass are essential in linear dynamics. Here’s a breakdown of each, how they relate to rotational motion, and how to compute them, especially for a sphere.

---

### 1. **Torque**

**Torque** $\tau$ is a measure of the rotational force applied to an object, causing it to rotate about an axis. It’s essentially the rotational equivalent of a linear force.

- **Formula**: 
  $$
  \tau = r \times F = r \cdot F \cdot \sin(\theta)
  $$
  where:
  - $r$ is the **lever arm** or **distance** from the axis of rotation to the point where the force is applied.
  - $F$ is the **force** applied.
  - $\theta$ is the angle between $r$ and $F$.

- **Intuition**: Torque depends on both the force and where it's applied relative to the axis of rotation. For instance, if you push the edge of a door far from its hinges, it’s easier to open than if you push close to the hinges. The farther from the axis you apply a force, the greater the torque.

#### In Code
If you're applying a force at a distance $r$ from an object’s center of mass, you can calculate torque as a cross product of $r$ and $F$, particularly useful in 3D physics engines.

---

### 2. **Moment of Inertia**

**Moment of Inertia** $I$ is a measure of an object’s resistance to changes in its rotational motion. It’s often called "rotational inertia" and depends on both the **mass** of the object and **how the mass is distributed** relative to the axis of rotation.

- **Formula (General)**:
  $$
  I = \int r^2 \, \text{dm}
  $$
  where:
  - $r$ is the distance of each infinitesimal mass element $\text{dm}$ from the axis of rotation.
  - The integral sums up $r^2 \text{dm}$ over the entire volume of the object.

This formula can be complex for objects with non-uniform shapes or mass distributions, so we use standard formulas for basic shapes.

---

### **Moment of Inertia for a Sphere**

For a **solid sphere** with mass $m$ and radius $r$, rotating about an axis through its center, the moment of inertia is given by:

$$
I_{\text{sphere}} = \frac{2}{5} m r^2
$$

- **Explanation**: The factor $\frac{2}{5}$ accounts for the uniform distribution of mass in a solid sphere. The farther the mass is from the axis, the larger the moment of inertia, meaning the sphere resists changes in rotation more strongly.

For a **hollow sphere** (a spherical shell), the moment of inertia is different because all the mass is concentrated at the radius distance from the center:
$$
I_{\text{hollow sphere}} = \frac{2}{3} m r^2
$$

---

### **Computing Torque and Moment of Inertia in Code for a Sphere**

For a solid sphere in your simulation, if you know the mass $m$ and radius $r$, you can calculate the moment of inertia directly using the formula above. Here’s a basic example in Rust:

```rust
struct Sphere {
    mass: f32,
    radius: f32,
    moment_of_inertia: f32,
}

impl Sphere {
    pub fn new(mass: f32, radius: f32) -> Self {
        let moment_of_inertia = (2.0 / 5.0) * mass * radius * radius;
        Sphere { mass, radius, moment_of_inertia }
    }
}
```

For **applying torque**, suppose you have a force `F` applied at a point `r` from the sphere's center. Calculate the torque using the cross product:

```rust
use nalgebra::Vector3;

fn calculate_torque(force: Vector3<f32>, distance: Vector3<f32>) -> Vector3<f32> {
    distance.cross(&force)  // Cross product to get torque
}
```

This torque vector can then be used to update the angular acceleration (\(\alpha\)):

$$
\alpha = \frac{\tau}{I}
$$

For the sphere, we apply:

```rust
fn update_angular_acceleration(torque: Vector3<f32>, moment_of_inertia: f32) -> Vector3<f32> {
    torque / moment_of_inertia  // Divide by moment of inertia to get angular acceleration
}
```


## Some other forces to consider
#### Conceptual Considerations
1. Realism: In real space, there is no friction, so a damping effect simulates the loss of energy due to hypothetical interactions (like radiation pressure, very weak gravitational interactions, or even tiny impacts from micro-particles). If your simulation is purely theoretical or for a specific educational purpose, it might make sense to include it as a way to illustrate energy dissipation.

2. Impact on Simulation: The value of the damping coefficient $(c)$ will dictate how quickly the velocity decreases. A very small value will make the effect negligible over short periods, while a larger value will cause rapid deceleration. Adjust it according to how you want your bodies to behave in the simulation.

Space Scenarios: Consider whether you're simulating a system with realistic conditions. In scenarios where gravitational interactions dominate (like planetary systems), adding a friction-like damping effect may not be necessary. It could be useful, however, in specific contexts, such as simulating spacecraft maneuvers or when studying orbital decay due to drag in very low orbits (though this is not the case in deep space).