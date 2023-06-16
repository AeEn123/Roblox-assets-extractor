ver = 0.02
print(f"Roblox assets extractor v{ver}")
print("✓ Byfron friendly")
print("Please report bugs to https://alfies-funtime.rf.gd")
print("-----{Now loading}-----")
# Import modules
import os
import shutil
import tempfile
import winreg
try:
    import tkinter as tk
except ImportError:
    os.system("pip install tkinter")
    import tkinter as tk
from tkinter import ttk, messagebox, filedialog

def delete_directory_contents(directory):
    try:
        for filename in os.listdir(directory):
            file_path = os.path.join(directory, filename)
            if os.path.isfile(file_path):
                os.remove(file_path)
        messagebox.showinfo("Success", "All files and folders deleted successfully.")
    except Exception as e:
        print(e)
        messagebox.showerror("Error", f"Failed to delete files and folders: {str(e)}")

def list_files(directory):
    file_list.delete(0, tk.END)  # Clear the current file list
    for file_name in os.listdir(directory):
        file_path = os.path.join(directory, file_name)
        if os.path.isfile(file_path):
            file_list.insert(tk.END, file_name)

def on_file_double_click(event):
    selected_file = file_list.get(file_list.curselection())
    source_path = os.path.join(current_directory.get(), selected_file)
    dest_path = os.path.join(temporary_directory, selected_file + '.ogg')
    shutil.copy2(source_path, dest_path)
    os.system("start " + dest_path)

def on_tab_change(event):
    selected_tab = event.widget.select()
    selected_tab_id = event.widget.index(selected_tab)
    selected_tab_idx = list(tab_directory_map.keys())[selected_tab_id]
    
    current_directory.set(tab_directory_map[selected_tab_idx])
    list_files(current_directory.get())

def add_tab(directory):
    tab_name = os.path.basename(directory)
    if os.path.basename(directory) == "sounds":
        tab_name = "Music"
    tab = ttk.Frame(tab_control)
    tab_control.add(tab, text=tab_name)
    tab_control.pack(expand=1, fill=tk.BOTH)
    tab_directory_map[tab] = directory

def delete_all_files(event=None):
    if messagebox.askyesno("Deleting...", "Are you sure you want to delete all files?"):
        delete_directory_contents(current_directory.get())
        list_files(current_directory.get())

def refresh(event=None):
    list_files(current_directory.get())

def extract_all_from_directory(event=None):
    destination = filedialog.askdirectory()
    directory = current_directory.get()
    try:
        for filename in os.listdir(directory):
            file_path = os.path.join(directory, filename)
            if os.path.isfile(file_path):
                source_path = os.path.join(directory, os.path.basename(file_path))
                dest_path = os.path.join(destination, os.path.basename(file_path) + '.ogg')
                shutil.copy2(source_path, dest_path)
        messagebox.showinfo("Success", "All files were copied successfully.")
    except Exception as e:
        print(e)
        messagebox.showerror("Error", f"Failed to copy files: {str(e)}")
    


root = tk.Tk()
root.title(f"Roblox assets extractor v{ver}")

# Create temporary directory
temporary_directory_object = tempfile.TemporaryDirectory()
temporary_directory = temporary_directory_object.name
print("Temp dir: " + temporary_directory)

# Create tab control
tab_frame = tk.Frame(root)
tab_frame.pack(fill=tk.BOTH, expand=False)
tab_control = ttk.Notebook(tab_frame)
tab_directory_map = {}

# Add initial tab for main directory
home_directory = os.path.expandvars("%Temp%\\Roblox\\sounds")
print("Roblox sounds: " + home_directory)
current_directory = tk.StringVar(value=home_directory)
add_tab(home_directory)

# Create buttons
button_frame = tk.Frame(root)
button_frame.pack()

delete_button = tk.Button(button_frame, text="Delete All Files", command=delete_all_files)
delete_button.pack(side=tk.LEFT)

refresh_button = tk.Button(button_frame, text="Refresh <F5>", command=refresh)
refresh_button.pack(side=tk.RIGHT)
root.bind('<F5>', refresh)

extract_button = tk.Button(button_frame, text="Extract all from this directory", command=extract_all_from_directory)
extract_button.pack(side=tk.RIGHT)

# Create file list
file_list = tk.Listbox(root, width=50)
file_list.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

# Set up double-click event on file list
file_list.bind("<Double-Button-1>", on_file_double_click)

# Set up tab change event
tab_control.bind("<<NotebookTabChanged>>", on_tab_change)

# Start GUI event loop
root.mainloop()
temporary_directory_object.cleanup()