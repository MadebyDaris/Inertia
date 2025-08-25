# Raycasting Implementation

This document details the implementation of the raycasting feature in the Inertia engine.

## How it Works

The raycasting system allows for the detection of intersections between a ray and objects in the 3D world. This is useful for a variety of purposes, such as object selection, line-of-sight checks, and eventually, more advanced rendering techniques like ray tracing.

The core of the system is the `Ray` struct, defined in `src/render/ray.rs`. A ray is defined by an `origin` and a `direction` vector. The `direction` vector is always normalized to ensure consistent calculations.

The `PhysicsWorld` struct now has a `cast_ray` method. This method iterates through all the physics objects in the world and checks for intersections with the given ray. It returns an `Option` containing the index of the closest intersected object and the distance to the intersection point.

Each `PhysicsObject` must implement the `intersects` method, which takes a `Ray` and returns an `Option<f32>` representing the distance to the intersection point. If there is no intersection, it returns `None`.

For `AstralBody` objects, which are represented as spheres, the `intersects` method uses a standard sphere-ray intersection formula to determine if the ray hits the sphere.

## Implementation Details

### `Ray` Struct

-   **File:** `src/render/ray.rs`
-   **Struct:** `Ray`
    -   `origin`: `Vector` - The starting point of the ray.
    -   `direction`: `Vector` - The direction of the ray (normalized).

### `PhysicsWorld`

-   **File:** `src/physics/physicsworld.rs`
-   **Method:** `cast_ray(&self, ray: Ray) -> Option<(usize, f32)>`
    -   Iterates through all `children` (physics objects).
    -   Calls the `intersects` method on each object.
    -   Keeps track of the closest intersection found so far.
    -   Returns the index of the closest object and the distance to the intersection.

### `PhysicsObject` Trait

-   **File:** `src/physics/physicsobject.rs`
-   **Method:** `intersects(&self, ray: &Ray) -> Option<f32>`
    -   This method must be implemented by any struct that wants to be a `PhysicsObject`.
    -   It should return the distance to the intersection point if the ray intersects the object, otherwise `None`.

### `AstralBody`

-   **File:** `src/simulation/orbital_simulation/astralBody.rs`
-   **Method:** `intersects(&self, ray: &Ray) -> Option<f32>`
    -   Implements the sphere-ray intersection formula.
    -   Calculates the discriminant of the quadratic equation formed by the ray and the sphere.
    -   If the discriminant is non-negative, there is an intersection, and the method returns the distance to the closest intersection point.

## Summary

The raycasting feature is a powerful addition to the Inertia engine. It provides a flexible and extensible way to perform intersection tests in the 3D world. The current implementation supports sphere-ray intersections, but it can be easily extended to support other shapes by implementing the `intersects` method for different types of `PhysicsObject`s.