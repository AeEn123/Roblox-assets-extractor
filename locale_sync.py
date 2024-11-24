# Simple script to sync en-GB.ftl to other locales for easier locale development
# Must be ran after updates to en-GB.ftl
import os

LOCALE_DIRECTORY = "./locales"
MAIN_LOCALE_FILE = "en-GB.ftl"

def parse_locale(raw_text):
    messages = {}
    current_heading = ""
    for line in raw_text.splitlines():
        if len(line) > 0:
            if line[0] == "#": # Treat individual comments as headings
                current_heading = line
            else:
                line = line.split("#")[0] # Ignore comments
                key, value = line.split(" = ")
                messages[key] = [value, current_heading]

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


        if len(diff) != 0:
            for value in diff:
                header = refrence_messages[value][1]
                header_pos = text.find(header)

                if header_pos == -1:
                    header_pos = len(text)
                else:
                    header_pos = header_pos + len(header)
                
                new_value = f"\n{value} = {refrence_messages[value][0]} # TODO: Translate"

                text = text[:header_pos] + new_value + text[header_pos:]

            print(f"{file} +{len(diff)}")
            with open(os.path.join(LOCALE_DIRECTORY, file), "w") as f:
                f.write(text)