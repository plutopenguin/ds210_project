# ds210_project
(The dataset used is "\ds210_project\twitch\twitch\DE\musae_DE_edges.csv")

This is a fairly simple program that returns information about a dataset containing pairs of Twitch users where at least one follows the other. The Twitch users are represented with integers starting from 0.

The program relies on user inputs. 

The first input must be an integer representing one Twitch user.
  
The second input is an integer 1~4 that either calls a separate function or exits the program.
  
- If the second input is 1, then a third input representing another Twitch user is taken. Then, the number of edges (i.e. distance) between the two users is returned.

- If the second input is 2, then the number of direct connections that the Twitch user has, as well as a list of those direct connections, is returned.
 
- If the second input is 3, then the mean distance from that Twitch user to another user is returned.

- If the second input is 4, then the program says "Goodbye".

If an invalid integer is entered at any point, then the program displays a message and exits.
 
If a non-integer is entered at any point, then the program panics.
