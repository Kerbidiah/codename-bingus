# AI GENERATED:
# https://chatgpt.com/c/6a67633c-c214-83ea-992d-5fba926662d4

import re
from pathlib import Path

input_file = "budapest.txt"
output_dir = Path("budapest_items")
output_dir.mkdir(exist_ok=True)

# Characters not allowed in filenames on Windows
INVALID_CHARS = r'[<>:"/\\|?*]'

with open(input_file, "r", encoding="utf-8") as f:
    for line in f:
        line = line.strip()

        if not line:
            continue

        # Extract the title
        match = re.search(r'title:"([^"]+)"', line)
        if not match:
            print(f"Skipping line (no title found): {line}")
            continue

        title = match.group(1)

        # Sanitize filename
        filename = re.sub(INVALID_CHARS, "_", title).strip()
        filepath = output_dir / f"{filename}.BingoItem"

        with open(filepath, "w", encoding="utf-8") as out:
            out.write(line + "\n")

        print(f"Created {filepath}")