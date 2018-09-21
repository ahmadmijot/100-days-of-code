"""
This is extension from frame_and_button.py
- add title to windows
- set coordinate for button placement
- set frame size
set self_init function
"""


from tkinter import *

class Window(Frame):

    def __init__(self, master=None):
        Frame.__init__(self,master)
        self.master = master
        self.init_window()

    #Creation of init_window
    def init_window(self):

        #changing the title of master widget
        self.master.title("GUI")

        #allow widget to take full space of root window
        self.pack(fill=BOTH, expand=1)

        #create button instance
        quitButton = Button(self, text = "Quit")

        #placing button
        quitButton.place(relx=0.5, rely=0.5)


root = Tk()

#size of window
root.geometry("400x400")

app = Window(root)
root.mainloop()
print("\nprogram closed!")

        

# import tkinter
# from tkinter import ttk

# def main():
#     root = tkinter.Tk()

#     frame1 = ttk.Frame(root, padding=25)
#     frame1.grid()

#     go_forward_button = ttk.Button(frame1, text='Forward')
#     go_forward_button.grid()

#     go_backward_button = ttk.Button(frame1, text='Backward')
#     go_backward_button.grid()

#     root.mainloop()

#     print('\n Program closed!')


# #----
# # Call main to start the ball rollin
# #----

# main()