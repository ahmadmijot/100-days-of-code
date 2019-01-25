"""
Example showing for tkinter and ttk:
-- How to CONSTRUCT and DISPLAY a WIDGET, in this case:
   >in this case, ttk.Button

-- How to associate a widget (here a ttk.Button)
    with a CALLBACK function that is a LAMBDA function.

From github/CSSE120StartingCode/TKinterPractice

"""

import tkinter
from tkinter import ttk
import random

def main():
    root = tkinter.Tk()

    frame1 = ttk.Frame(root, padding = 15)
    frame1.grid()
    frame2 = ttk.Frame(root, padding = 10)
    frame2.grid()

    print_stuff_button = ttk.Button(frame1, text='Print random sequence of letters')
    print_stuff_button['command'] = (lambda: do_stuff())
    print_stuff_button.grid()

    go_button_test = ttk.Button(frame2, text='test')
    go_button_test['command'] = (lambda: print_here())
    go_button_test.grid()

    root.mainloop()
    print('\n Program closed!')

def do_stuff():
    """
    Print onto the console a random 10-letter string.

    in this example, it is used as the function that is "CALLED BACK"
    when an event (namely, the pressing of certain Button) occurs.
    """
    letters = ('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
               'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
               'w', 'x', 'y', 'z')
    rand_word = ''
    for _ in range(10):
        letter = letters[random.randrange(26)]
        rand_word = rand_word + letter
    
    print('\n', rand_word)

def print_here():
    print('here!\n')

main()

########################################################################
#
# EXPLANATION of the above:
#
# This example is the same as the previous example except for:
#   -- There is a function called   do_stuff   that prints random stuff.
#   -- The Button responds to a button-press
#        by calling the  do_stuff   function.
#
# To make a Button respond to a button-press:
#
#   1. Setting a value to the Button's 'command' attribute
#      tells the Button to respond to a button press.  For example:
#
#           button1['command'] = ...
#
#      tells the Button named  button1  to do the   ...   stuff
#      when the button is pressed.
#
#      This is called "dictionary-like" notation.  Tkinter knows that
#      when you put the special string    'command'    inside the
#      square brackets, then you are telling the Button what to do
#      when the Button is pressed.
#
#   2. To express WHAT the Button should do when pressed,
#      use a  LAMBDA  expression, like this:
#
#           button1['command'] = (lambda:
#                                 foo())
#
#      The LEFT-hand-side of the assignment tells the Button that
#      it should execute the function defined by the RIGHT-hand-side
#      of the assignment, when the Button is pressed.
#
#      The right-hand-side of the assignment DEFINES a function,
#      just like a DEF expression DEFINES a function.  The difference
#      is that a function defined by a DEF has a name, e.g.
#           def blah():
#               ...
#               ...
#      and can contain multiple statements, but a LAMBDA function
#      is ANONYMOUS -- it has no name -- and (in Python) can contain
#      only a single statement as its body.  So the expression:
#              lambda: foo()
#      defines an anonymous (no-name) function that, when executed,
#      simply calls the function named  foo  that is defined elsewhere.
#
#      Note that a lambda expression DEFINES a function that is EXECUTED
#      LATER (here, when the Button is pressed).  It serves as a way
#      to DEFINE a function INSIDE an assignment statement.
#
#      You don't have to break a lambda expression over two lines.
#      For example:
#           button1['command'] = lambda: foo()
#
#      is equivalent to:
#           button1['command'] = (lambda:
#                                 foo())
#
#      Here we choose to use the two-line form because it makes it look
#      more like a DEF and gives a bit more space on a line
#      for the body of the lambda expression.
#
########################################################################