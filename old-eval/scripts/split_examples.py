#!/usr/bin/env python3
"""Split exNN_compat.rs into per-approach files for non-spec variants."""

import re
import os

from typing import Optional

DIR = os.path.join(os.path.dirname(__file__),
                   "..", "benches-crate", "src", "examples")
DIR = os.path.abspath(DIR)

ACTIVE = [3, 4] + list(range(7, 42))  # ex03, ex04, ex07-ex41

# Section header regex: matches both "// --- dyn dispatch ---" and
# "// --- Approach A: dyn dispatch ---"
SECTION_RE = re.compile(
    r'^//\s*---+\s*(?:Approach\s+\w+:\s*)?(.+?)\s*---+\s*$'
)

def classify(section_name: str) -> Optional[str]:
    """Map a section header to one of our 4 non-spec approaches.

    Returns None for sections that should be skipped (e.g. Bulk functions).
    """
    s = section_name.lower()
    # Skip bulk/benchmarkable aggregate sections (they cross-reference approaches)
    if 'bulk' in s or 'benchmarkable' in s:
        return None
    if 'dyn' in s:
        return 'dyn'
    if 'enum' in s:
        return 'enum'
    if 'runtime' in s or 'typeid' in s:
        return 'runtime_typeid'
    # generic, manual, fallback, stdlib, counting, clone, memcpy, memset, etc.
    return 'generic'


BULK_SUBCOMMENT_RE = re.compile(
    r'^\s*//\s*(?:Benchmarkable\s+)?[Bb]ulk\b', re.IGNORECASE
)

def _strip_bulk_subblocks(sect_lines: list[str], ex_num: int) -> list[str]:
    """Remove inline 'Benchmarkable bulk' sub-blocks from a section.

    Some sections have a sub-comment like '// Benchmarkable bulk functions'
    followed by aggregation functions that cross-reference other sections.
    We strip everything from that sub-comment to the end of the section.
    """
    for i, line in enumerate(sect_lines):
        if BULK_SUBCOMMENT_RE.match(line):
            # Found a bulk sub-block: keep everything before it
            return sect_lines[:i]
    return sect_lines


def split_compat(ex_num: int) -> dict[str, bool]:
    """Split exNN_compat.rs into up to 4 approach files.

Returns dict of which approach files were created.
    """
    path = os.path.join(DIR, f"ex{ex_num:02d}_compat.rs")
    with open(path, encoding="utf-8") as f:
        lines = f.readlines()

    # Parse into header + sections
    header_lines = []   # type: list
    sections = []       # type: list  # (approach, lines)
    cur_approach: Optional[str] = None
    cur_lines = []  # type: list

    for line in lines:
        m = SECTION_RE.match(line)
        if m:
            if cur_approach is not None:
                sections.append((cur_approach, cur_lines))
            else:
                header_lines = cur_lines[:]
            cur_approach = classify(m.group(1))
            cur_lines = [line]
        else:
            cur_lines.append(line)

    if cur_approach is not None:
        sections.append((cur_approach, cur_lines))

    # Group by approach (skip None = bulk sections)
    grouped = {}  # type: dict
    for approach, sect_lines in sections:
        if approach is None:
            continue
        # Strip inline cross-referencing blocks marked with sub-comments like
        # "// Benchmarkable bulk functions" or "// Bulk functions"
        clean_lines = _strip_bulk_subblocks(sect_lines, ex_num)
        grouped.setdefault(approach, []).extend(clean_lines)

    created = {}
    for approach, body_lines in grouped.items():
        body = "".join(body_lines)

        # Figure out which header imports are needed
        filtered_header = []
        for hl in header_lines:
            stripped = hl.strip()
            if stripped.startswith("use "):
                # Include import only if some symbol from it appears in body
                # Extract imported names from `use foo::bar::{X, Y};`
                after_colon = stripped.split("::")[-1].rstrip(";").strip()
                names = re.findall(r'\w+', after_colon)
                if any(n in body for n in names if len(n) > 1):
                    filtered_header.append(hl)
            elif stripped.startswith("//"):
                # Rewrite the file-level comment
                new_comment = stripped.replace(
                    "WITHOUT macros", approach.upper().replace("_", " ")
                )
                filtered_header.append(new_comment + "\n")
            elif stripped == "":
                filtered_header.append(hl)

        out_path = os.path.join(DIR, f"ex{ex_num:02d}_{approach}.rs")
        with open(out_path, "w", encoding="utf-8") as f:
            for hl in filtered_header:
                f.write(hl)
            if filtered_header and not filtered_header[-1].endswith("\n\n"):
                f.write("\n")
            f.write(body)
            # Ensure trailing newline
            if not body.endswith("\n"):
                f.write("\n")

        created[approach] = True
        print(f"  Created ex{ex_num:02d}_{approach}.rs")

    return created


def main():
    all_created = {}  # type: dict
    for ex in ACTIVE:
        print(f"\n=== Example {ex:02d} ===")
        # spec approach is maintained in exNN_spec.rs
        created = split_compat(ex)
        all_created[ex] = created

    # Summary
    print("\n=== Summary ===")
    approaches = sorted(
        {a for d in all_created.values() for a in d},
        key=lambda x: ["generic", "dyn", "enum", "runtime_typeid"].index(x)
        if x in ["generic", "dyn", "enum", "runtime_typeid"] else 99
    )
    print(f"{'ex':<6} | {'spec':<5} | " + " | ".join(f"{a:<16}" for a in approaches))
    print("-" * 80)
    for ex in ACTIVE:
        parts = [f"{'Y':<5}"]
        for a in approaches:
            parts.append(f"{'Y' if a in all_created[ex] else '-':<16}")
        print(f"ex{ex:02d}   | " + " | ".join(parts))


if __name__ == "__main__":
    main()
