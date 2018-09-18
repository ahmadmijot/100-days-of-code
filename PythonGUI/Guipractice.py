"""
Example showing for tkinter and ttk:
-- tkinter.Tk - the main (root) window
-- The root wondow's mainloop - the event loop

A window should pop up. That's all for this demo.

Authors: David Mutchler, Mark Hays, Michael Wollowswki, Matt Boutell,
         Chandan Rupakheti, Claude Anderson and their colleagues
         at Rose-Hulman Institute of Technology.

         Edited by Ahmad Max, UPM
"""

import tkinter
from tkinter import ttk #necessary  in all but this trivial example.

def main():
    root = tkinter.Tk()
    root.mainloop()

    print('done with the Envent Loop') #Note when this line runs.


#------------------
#Calls main to start the ball rollin
#------------------

main()


