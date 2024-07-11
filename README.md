# Ray Tracing

A 2-week object in PPCA of ACMClass-2023

![](images/Book1Final.jpg)

![](images/Book2Final.jpg)

(ps: My `main` function is in a mess, because I'm trying to avoid warning of some fn never used when I need to ignore it sometimes)

---

## Learning Materials

- [*Rust Programming Language*](https://doc.rust-lang.org/book/title-page.html)
- [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
- [_Ray Tracing: The Rest of Your Life_](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)

## How it works

Generally, we use road track to generate rays from our `camera` through the `Viewport` (which decide the pixel of the ray contribute to) to the `world`(where you put all you objects ready to form a picture), and bouncing in the scene to decide what color the ray is, and get the color to the image.

The things in the world are `Hittable`, they use `hit` function to decide whether the ray are hitting it. Also, it has a feature called `Material` to tell how the ray should scatter and its `Texture` to tell what color should add to the ray.

Till now, I have finished `Sphere` `Quad` `Triangle` as origin `Hittable`, and `Hittable_list` `Bvh`(make render faster, read book2 for more detail) as container for original `Hittable`.

So we implement functions like hit recursively.

- [PDF report](PDF_Report.md)

## Advanced Feature

### Multitask

### Support for .obj File

Usage example:

```rust
let obj = read_obj("Haunter.obj", 300.0);
    let obj = RotateY::new(Arc::new(obj), -30.0);
    let obj = Translate::new(Arc::new(obj), &Vec3::new(156.0, 206.0, 300.0));
    world.add(Arc::new(obj));
```

Notice: the obj file should be stored in the `objects/` in **root** of my Ray Tracer

Use tobj to load `.obj` file into `Vec`

[tobj_doc](https://docs.rs/tobj/latest/tobj/)

Use `triangle.rs` to form a poly.

Currently support Lambertian color import.

Example:

![Pokemon Fight](images/PokemonFight.jpg)