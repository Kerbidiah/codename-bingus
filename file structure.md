# File Structure
## Bingo Board Project (.BingoProject)
A RON serialized file that has the name and a list of all bingo items.

## Bingo Board (.BingoBoard)
A RON serialized file (see struct definition for BingoBoard)

## Bingo Game (internal just a folder)
* folder titled by the name of the project
* a .BingoBoard file
* "assets" folder
	* items are named 0 to u16::MAX (their asset ID)
* when exported for submission, compress as a .zip file, but name it .Bingo