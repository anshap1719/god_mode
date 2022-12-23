# god_mode

A rust macro to replace your `unsafe` blocks with a more friendlier name in order to not look as bad.

### Usage Examples

Instead of

```
fn main() {
    let mut x = 2.;
    let pointer = &mut x as *mut f64;

    unsafe {
        *pointer = 6.3;
    }

    println!("{}", x);
}
```

just use

```
fn main() {
    let mut x = 2.;
    let pointer = &mut x as *mut f64;

    god_mode! {{
        *pointer = 6.3;
    }}

    println!("{}", x);
}
```
