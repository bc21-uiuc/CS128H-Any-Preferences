# About

## W.B.E. Audio

- Benjamin Chen | bc21

- Korn Jiamsripong | kornj2

- Sankar Gopalkrishna | sankarg2

- Sean Lu | seanlu2


# Project Introduction

  We create an aural browser engine. Basically, it takes the HTML and CSS and generates audio based on that. We chose to make this because we thought it was cool idea we were leaning towards ands seems reasonably challenging to make.

![browser components](https://3fxtqy18kygf3on3bu39kh93-wpengine.netdna-ssl.com/wp-content/uploads/2019/11/BrowserEngine.png)

# System Overview

  The browser engine is what works under the hood of web browsers like Firefox and Chrome to take the web page and display it. There are many components to a browser engine, such as the HTML parser and the CSS parser. This code will implement the DOM structure to handle the different languages that can act as its inputs. Browser engines typically display a visual image, but ours will generate audio instead. 

# Possible Challenges
  
  Difficulty to find resources for audio/speech synthesis/etc, especially given that it's Rust and not a more common language. Ensuring the DOM works well with the different languages will likely be a challenge. Also, we will have to figure out generally how to represent the objects and stuff as audio. For example, images. Depending on the method of conversion, we may have trouble with working with a database, or have trouble algorithmically creating sounds.

# References

  Sean found this: https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html.


