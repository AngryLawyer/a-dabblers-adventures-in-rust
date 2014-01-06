#!/usr/bin/env python

def import_line(line):
    name = line[3:]
    with open("snippets/{0}".format(name)) as snippet:
        snippet_content = snippet.read().splitlines()
        # We need to find the start and end slice points
        start = snippet_content.index("//---START");
        end = snippet_content.index("//---END");
        indented_content = ["    {0}".format(line) for line in snippet_content[start + 1:end]]
        return "    !rust\n{0}".format("\n".join(indented_content))


with open("slides.md") as f:
    content = f.read().splitlines()
    output = []
    for line in content:
        if line.find('!!!') == 0:
            output.append(import_line(line))
        else:
           output.append(line)
    print "\n".join(output)
