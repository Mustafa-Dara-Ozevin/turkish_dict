# Turkish definiton Dictinory made in Egui

it uses [Turkish Language Society](https://sozluk.gov.tr/)'s api point (https://sozluk.gov.tr/gts?ara={}) which I am not sure if it is even suppose to be public because there is zero documentation
for it and only place it is mentioned is a random tweet. but since it uses official resources it should be as accurate as it gets.

![Screenshot_20230327_152634](https://user-images.githubusercontent.com/63582793/227951012-76a77c0d-4199-47fb-b265-9009905538d5.png)

## Issues

* Typing word's with "I" causes api call to fail
* Blocking api call makes the program unresponsive while it is getting processed
* UI elements are not dynamic and breaks at lower application sizes

## Platforms

Should be cross-platform although I only tested it for Linux


## Licence
MIT, see LICENCE for details
