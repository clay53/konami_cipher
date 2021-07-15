# Konami Cipher
A ciphering algorithm based on the Konami Code and the Vigenere Cipher for Tech Roulette 2021 Project 3, Love Triangle, Module 5.

## The model

### Konami -> Byte (u8)
|Value|Up|Down|LeftRightLeftRight|BA|Start|
|-|-|-|-|-|-|
|0|^^|vv|<><>|ba|Start|
|1|^^^|vvv|<>|bb||
|2|^|v|<<|aa||
|3|{Empty}|{Empty}|>>|ab||
|Multiplier|1|4|16|64|Terminator

### Formats
|Usage | Storage Type|
|- | -|
|Plain Message | String|
|Encrypted Message | Konami formatted bytes|
|Key | Konami formatted byte offsets|

### Conversion
Similar to the Vigenere Cipher, the key is used as an offset to the plain message except instead of being limited to a set of charcters, any data can be encoded by using variations of Konami code that is translated to 8 bit binary. To do this, the 4 variable sections of the Konami code are broken into 4 different variations along with the "Start" value as the obligatory terminator for each string of Konami code to have 256 variations (4^4=256).