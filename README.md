# hfractal
This is a pet project of mine created for the purpose of rendering fractals. Currently only the Mandelbrot set is supported, but I have plans to add
Julia sets and more in the future.

![The Mandelbrot set, centered at (-.75, .1) with width and height 0.1](https://github.com/wlondergan/hfractal/blob/master/sample-render.png)
![](https://github.com/wlondergan/hfractal/blob/master/sample-render2.png)

## Progress
Currently, this program only renders the Mandelbrot set in greyscale without any sophisticated math to reduce oversampling requirements. I have plans
to add some analytical methods to the rendering process (i.e. using derivatives and such) to reduce oversampling and allow for more efficient computation
of points. At some point in the future I plan on adding rendering support for Julia sets. The next step after that will be to try to render zooms of the
Mandelbrot set, although this requires more reading on my part.

## To do's
My current biggest to-do is to parallelize computation: currently it takes my computer about 20 minutes to render a 4000x4000 image, which is far too slow.
After that, I plan to look into error correction algorithms that will allow me to reduce the number of multiple precision computations that occur, which should
dramatically increase performance.
