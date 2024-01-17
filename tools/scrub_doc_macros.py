import os
import sys

def delete_lines_starting_with(file_path, prefix):
    # Read the content of the file
    with open(file_path, 'r') as file:
        lines = file.readlines()

    # Filter lines that do not start with the specified prefix
    lines = [line for line in lines if not line.startswith(prefix)]

    # Write the modified content back to the file
    with open(file_path, 'w') as file:
        file.writelines(lines)

def filter_lines_containing(file_path, substring):
    # Read the content of the file
    with open(file_path, 'r') as file:
        lines = file.readlines()

    # Filter lines that do not contain the specified substring
    lines = [line for line in lines if substring not in line]

    # Write the modified content back to the file
    with open(file_path, 'w') as file:
        file.writelines(lines)


def scrub_doc_macros(root_directory):
    for root, dirs, files in os.walk(root_directory):
        for file in files:
            file_path = os.path.join(root, file)
            if file_path.endswith(".rs"):
                filter_lines_containing(file_path, "#[doc" )
                filter_lines_containing(file_path, "#![doc" )

scrub_doc_macros(sys.argv[1])
