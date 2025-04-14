# Rusttracer
This is a command line program written in rust that uses software rendering to produce
a highly realistic image of a 3D scene.
First, it reads a description of the scene to be rendered from a file.
Next, it calculates the colour of each pixel from the perspective of the point of an 
eye, located at the origin (0,0,0), that is looking towards the negative Z axis.
Finally, these colours are processed and written into .png and .ppm image files.