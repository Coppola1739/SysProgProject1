Both versions of the project do not use command line args as I could not get that to work so instead it prompts for a folder as input and runs from there.
Once "data" is input, version 1 recursively loops through the folders and adds the values from each folder and writes it to the output file. Version 2 accepts the folder in the same way version 1 does. For version 2,
I had already spent so much time using the Arc/Mutex functionality for the threads that I did not use channels. Either way, the program still safely uses threads to
handle each folder which showed an increase in speed. Version 2 took 4774292 nano seconds and Version 1 took 14321417.
