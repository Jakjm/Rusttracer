# Rusttracer
This is a command line program written in Rust that uses software rendering to produce
a highly realistic image of a 3D scene.
First, it reads a description of the scene to be rendered from a file.
Next, it calculates the colour of each pixel from the perspective of the point of an 
eye, located at the origin (0,0,0), that is looking towards the negative Z axis.
Finally, these colours are processed and written into .png and .ppm image files.


![Example rendered image](SampleOutputFiles/example2.png)
![Example rendered image](SampleOutputFiles/example3.png)

You can compile the program using Cargo: 

    cargo build --release

To run the program after compiling, you can run the following:

    cargo run --release input-file 

Where input-file is a file that describes the 3D scene for the program to render into .ppm and .png files
