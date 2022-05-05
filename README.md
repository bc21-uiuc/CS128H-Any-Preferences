# About

## W.B.E. Audio

- Benjamin Chen | bc21

- Korn Jiamsripong | kornj2

- Sankar Gopalkrishna | sankarg2

- Sean Lu | seanlu2


# Project Introduction

  We are creating a browser engine that is aural instead of visual like most browsers. Browser engines take HTML and CSS and create the web page. Ours is barebone and just parses the HTML to generate audio.
  
  Below is the structure of a typical browser.

![browser components](https://3fxtqy18kygf3on3bu39kh93-wpengine.netdna-ssl.com/wp-content/uploads/2019/11/BrowserEngine.png)

# System Overview

  The browser engine is what works under the hood of web browsers like Firefox and Chrome. In contrast, the exterior that makes up the URL bar and other features, dependent on the browser itself, is called the chrome. We implement the Document Object Model (DOM) to organize the data in the webpage. This is created from the HTML, for which we create an HTML parser. The data from the DOM is then used to create the audio. Text will be read using speech synthesis, and images might be processed into sounds as well. We are not entirely sure about how to represent the structure and other elements though. We will also implement user interaction, to click on links and “scroll” up and down.

# Possible Challenges
  
  Difficulty to find resources for audio/speech synthesis/etc, especially given that it's Rust and not a more common language. Also, we will have to figure out generally how to represent the objects and stuff as audio. For example, images. Depending on the method of conversion, we may have trouble with working with a database, or have trouble algorithmically creating sounds. Because of the interdependence of the parts of the project, dividing up roles fairly might be difficult.

# References

  This was our initial inspiration: https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html. It also is where we found the image shown in the Project Introduction section.
  https://github.com/ckw017/vader-sentiment-rust
  
  We used code from these libraries to try and handle the audio aspect of the project:
  
    For Text To Speech: https://github.com/ndarilek/tts-rs by Nolan Darilek
  
    For Grayscale Image To Sound: https://github.com/iluvcapra/bwavfile by Jamie Hardt
  


