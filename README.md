# rusty_ray
ray tracing and cpu rasterization in rust

## TODO
- add procedural texture [failing]

## notes
here are some notes about doing this in rust and comparing a similar implementation in cpp

### floating point errors
- in general cpp vs rust code handles floating point math differently perhaps because of integer coeercion or something else
    - when we calculate if light visible from a point on the plane, we calculate what objects come in the path from the intersection point to the light. This intersection point should have t_value > 0. for object to be in line of sight. In cpp, this t_value is -ve and in rust we get a positive epsilon. 
    - ideally this t_value should be 0. or rather this point of intersection should not be considered as it is the same plane, the right way to handle it would be to send a ray epsilon away from the point
    - this is somewhat equivalent to having that t_value > epsilon to be considered a interaction (???)
    - similar error happens and creates artifacts when we try to calculate the reflected ray, we need to keep it epsilon away from the surface

### y-axis convention
- sometimes we have a situation where the image buffer origin is at top-left but in cpp the matrix and image construction happens from bottom-left. These can cause weird stuff to happen like orientation and light mismatch?? 

### other
- a lot of errors were happening due to the incorrect default initialization of the AlignedBox3d - default macro would assign 0 to all Vec3 but that's wrong. 