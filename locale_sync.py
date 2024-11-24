# Simple script to sync en-GB.ftl to other locales for easier locale development
# Must be ran after updates to en-GB.ftl
import os

LOCALE_DIRECTORY = "./locales"
MAIN_LOCALE_FILE = "en-GB.ftl"

def parse_locale(raw_text):
    messages = {}
    for line in raw_text.splitlines():
        line = line.split("#")[0] # Ignore comments
        if len(line) > 0:
            key, value = line.split(" = ")
            messages[key] = value

    return messages
    



with open(os.path.join(LOCALE_DIRECTORY, MAIN_LOCALE_FILE)) as f:
    text = f.read()

refrence_messages = parse_locale(text)


# Iterate through all locale files to sync them
for file in os.listdir(LOCALE_DIRECTORY):
    if file != MAIN_LOCALE_FILE:
        with open(os.path.join(LOCALE_DIRECTORY, file)) as f:
            text = f.read()

        new_messages = parse_locale(text)
        diff = set(refrence_messages) - set(new_messages) # Subtract the new messages from the refrence messages


        if diff != {}:
            with open(os.path.join(LOCALE_DIRECTORY, file), "a") as f:
                f.write("\n\n# TODO: Translate these values:\n")
                for value in diff:
                    f.write(f"{value} = {refrence_messages[value]}\n")