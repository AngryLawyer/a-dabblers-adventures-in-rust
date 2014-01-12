#!/usr/bin/env python

def import_line(line):
    (name, _separator, subgroup) = line[3:].partition(":")

    with open("snippets/{0}".format(name)) as snippet:
        snippet_content = snippet.read().splitlines()
        # We need to find the start and end slice points
        try:
            if subgroup:
                start = snippet_content.index("//---START:{0}".format(subgroup));
                end = snippet_content.index("//---END:{0}".format(subgroup));
            else:
                start = snippet_content.index("//---START");
                end = snippet_content.index("//---END");
        except ValueError:
            raise Exception("Could not find snip tags for {0}".format(line[3:]))
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
