I've been eyeing [TinySearch](https://github.com/tinysearch/tinysearch) for a long while, and now is a great chance while things are super simple to try to get it working.

[Matthias Endler](https://endler.dev/2019/tinysearch/) has a great post on experimenting with building a _really_ tiny site-wide search utalizing Rust & WASM, and ever since I read it I've been wanting to try it out.

What we'll need up front is a content index–a JSON file that contains all of the content on the site that we want to index for search. This will include things like the title, description, and URL of each page. We'll want to include the body as well, but there is probably some magic we can do to reduce the amount of text we bring in from the body to keep things as small as possible.

We need an output that looks something like [this](https://github.com/tinysearch/tinysearch/blob/master/fixtures/index.json), so let's get to work on that.

## Creating the Index

After playing with some `ts-node` nonsense, I realized that python is probably the easiest tool for the job here.

This little script will walk through the `src/content` directory and generate a JSON file that contains all of the metadata for each markdown document.

```python
import json
import os
import re
from typing import Dict

def create_slug_from_filename(filename: str) -> str:
    return re.sub(r'^\d+[-_]', '', filename).rsplit('.', 1)[0]

def extract_metadata(content: str) -> Dict[str, str]:
    metadata = {}
    match = re.search(r'^---\s+(.*?)\s+---', content, re.DOTALL)
    if match:
        front_matter = match.group(1)
        for line in front_matter.splitlines():
            key, value = line.split(': ', 1)
            metadata[key.strip()] = value.strip().strip('"').strip("'")
    return metadata

def create_content_index(src_folder: str) -> None:
    index = {}
    for root, dirs, files in os.walk(src_folder):
        folder = os.path.basename(root)
        if folder == 'content':
            continue
        index[folder] = {}
        for file in files:
            if file.endswith('.md'):
                filepath = os.path.join(root, file)
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                    metadata = extract_metadata(content)
                    slug = create_slug_from_filename(file)
                    index[folder][slug] = {
                        'slug': slug,
                        'date_published': metadata.get('date_published', 'No Date Published'),
                        'title': metadata.get('title', 'No Title'),
                        'preview': metadata.get('preview', 'No Preview')
                    }

    with open('index/content.json', 'w', encoding='utf-8') as f:
        json.dump(index, f, indent=4)

create_content_index('src/content')
```

Running it outputs some json that looks like this:

```json
{
  "fragment": {
    "00": {
      "slug": "00",
      "date_published": "2024-02-27",
      "title": "Building a new site with Astro",
      "preview": "It's that time of year again \u2013 time to build a new site. This time, I'm using Astro. Here's how it's going."
    },
    "01": {
      "slug": "01",
      "date_published": "2024-02-27",
      "title": "Generating a content index",
      "preview": "Ahead of some schemes of building site-wide content search, and perhaps an RSS feed, let's build a content index."
    }
  }
}
```

This looks pretty good for our needs. I'm not sure yet if we need to match the exact structure of the TinySearch index – we'll look into that next. But now we have a handy JSON index of all of our content that we can use for whatever we need.
