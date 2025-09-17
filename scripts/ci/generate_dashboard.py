#!/usr/bin/env python3
"""
Generate benchmark dashboard for GitHub Pages.

FULLY AUTO-DISCOVERS all benchmarks from all platforms:
- Rust (Criterion) - HTML reports with detailed charts
- Python (MathHook PyO3 bindings)
- Node.js (MathHook NAPI bindings)

NO HARDCODED BENCHMARK IDs - everything is discovered from JSON files.
Human-readable names are auto-generated from benchmark IDs.

Last Updated: 2025-11-30T0430
"""

import json
import os
import shutil
import sys
from datetime import datetime, timezone
from pathlib import Path


# =============================================================================
# AUTO-DISCOVERY: No hardcoded benchmark IDs
# =============================================================================

def benchmark_id_to_display(bench_id: str) -> tuple[str, str]:
    """Auto-generate human-readable name and description from benchmark ID.

    Examples:
        parse_simple -> ("Parse Simple", "Benchmark: parse_simple")
        gcd_large -> ("GCD Large", "Benchmark: gcd_large")
        mul_sparse -> ("Mul Sparse", "Benchmark: mul_sparse")
    """
    parts = bench_id.split("_")

    # First part is often an abbreviation - expand common ones
    abbreviations = {
        "gcd": "GCD",
        "mul": "Multiply",
        "div": "Divide",
        "sqrt": "Square Root",
        "pow": "Power",
        "ln": "Natural Log",
    }

    display_parts = []
    for i, part in enumerate(parts):
        if i == 0 and part.lower() in abbreviations:
            display_parts.append(abbreviations[part.lower()])
        else:
            display_parts.append(part.capitalize())

    name = " ".join(display_parts)
    desc = f"Benchmark: {bench_id}"
    return (name, desc)


def get_category(bench_id: str) -> str:
    """Auto-detect category from benchmark ID prefix."""
    # Common prefixes mapped to categories
    prefix_map = {
        "parse": "Parsing",
        "gcd": "GCD",
        "mul": "Multiplication",
        "div": "Division",
        "expand": "Expansion",
        "simplify": "Simplification",
        "factor": "Factorization",
        "derive": "Calculus",
        "deriv": "Calculus",
        "integrate": "Calculus",
        "integ": "Calculus",
        "solve": "Solving",
        "matrix": "Matrix",
        "eval": "Evaluation",
    }

    lower_id = bench_id.lower()
    for prefix, category in prefix_map.items():
        if lower_id.startswith(prefix):
            return category
    return "Other"


def get_all_categories(benchmarks: dict) -> list[str]:
    """Get all unique categories from benchmarks, sorted logically."""
    # Priority order for display
    priority = [
        "Parsing", "GCD", "Multiplication", "Division",
        "Expansion", "Simplification", "Factorization",
        "Calculus", "Solving", "Matrix", "Evaluation", "Other"
    ]

    found = set()
    for bench_id in benchmarks.keys():
        found.add(get_category(bench_id))

    # Return in priority order, then any new ones alphabetically
    result = [c for c in priority if c in found]
    extras = sorted(found - set(priority))
    return result + extras


# =============================================================================
# DATA LOADING
# =============================================================================

def load_json_safe(path: str) -> dict:
    """Load JSON with error handling."""
    try:
        data = json.loads(Path(path).read_text())
        return data if isinstance(data, dict) else {"status": "invalid", "benchmarks": {}}
    except Exception as e:
        return {"status": f"error: {e}", "benchmarks": {}}


def discover_criterion_benchmarks(criterion_dir: Path) -> dict:
    """Auto-discover ALL Criterion benchmarks from estimates.json files.

    Criterion stores data at: target/criterion/{group}/{benchmark}/new/estimates.json
    The estimates.json format contains:
    {
        "mean": {"point_estimate": <nanoseconds>, ...},
        "std_dev": {"point_estimate": <nanoseconds>, ...},
        "median": {"point_estimate": <nanoseconds>, ...},
        ...
    }

    Returns:
        dict with structure:
        {
            "groups": ["group1", "group2", ...],
            "benchmarks": {
                "group1/bench1": {"mean_ns": ..., "stdev_ns": ..., ...},
                "group1/bench2": {...},
                ...
            }
        }
    """
    result = {
        "groups": [],
        "benchmarks": {},
    }

    if not criterion_dir.exists():
        return result

    # Find all groups (directories in criterion_dir, excluding "report")
    for group_dir in sorted(criterion_dir.iterdir()):
        if not group_dir.is_dir() or group_dir.name == "report":
            continue

        group_name = group_dir.name
        has_benchmarks = False

        # Find all benchmarks within this group
        for bench_dir in sorted(group_dir.iterdir()):
            if not bench_dir.is_dir() or bench_dir.name == "report":
                continue

            # Check for estimates.json in the "new" subdirectory
            estimates_path = bench_dir / "new" / "estimates.json"
            if not estimates_path.exists():
                continue

            try:
                estimates = json.loads(estimates_path.read_text())

                # Extract timing data from Criterion format
                mean_ns = estimates.get("mean", {}).get("point_estimate", 0)
                stdev_ns = estimates.get("std_dev", {}).get("point_estimate", 0)
                median_ns = estimates.get("median", {}).get("point_estimate", 0)

                bench_id = f"{group_name}/{bench_dir.name}"
                result["benchmarks"][bench_id] = {
                    "mean_ns": mean_ns,
                    "stdev_ns": stdev_ns,
                    "median_ns": median_ns,
                    "group": group_name,
                    "name": bench_dir.name,
                }
                has_benchmarks = True

            except Exception as e:
                print(f"Warning: Failed to parse {estimates_path}: {e}")

        if has_benchmarks:
            result["groups"].append(group_name)

    return result


def discover_all_benchmarks(artifacts_dir: str) -> dict:
    """Auto-discover all benchmarks from all platform JSON files."""
    result = {
        "python": {},
        "node": {},
        "rust": {},
    }

    artifacts = Path(artifacts_dir)

    # Load Python benchmarks (MathHook via PyO3)
    python_path = artifacts / "python-benchmark-results" / "python.json"
    if python_path.exists():
        data = load_json_safe(str(python_path))
        result["python"] = data.get("benchmarks", {})
        result["python_meta"] = {
            "version": data.get("python_version", "unknown"),
            "binding": data.get("binding", "PyO3"),
        }

    # Load Node benchmarks (MathHook via NAPI)
    node_path = artifacts / "node-benchmark-results" / "node.json"
    if node_path.exists():
        data = load_json_safe(str(node_path))
        result["node"] = data.get("benchmarks", {})
        result["node_meta"] = {
            "version": data.get("node_version", "unknown"),
            "binding": data.get("binding", "NAPI"),
        }



    # Load Rust status
    rust_path = artifacts / "rust-benchmark-results" / "rust.json"
    if rust_path.exists():
        result["rust_meta"] = load_json_safe(str(rust_path))

    return result


# =============================================================================
# MAIN DASHBOARD
# =============================================================================

def generate_benchmark_cards(benchmarks: dict) -> str:
    """Generate HTML cards for each benchmark category."""
    if not benchmarks:
        return '<div class="no-data">No benchmark data available</div>'

    # Group by category (auto-discovered)
    categories = {}
    for bench_id, data in benchmarks.items():
        if not isinstance(data, dict) or "error" in data:
            continue
        category = get_category(bench_id)
        if category not in categories:
            categories[category] = []

        name, desc = benchmark_id_to_display(bench_id)
        mean_ns = data.get("mean_ns", 0)
        mean_us = mean_ns / 1000

        categories[category].append({
            "id": bench_id,
            "name": name,
            "desc": desc,
            "mean_us": mean_us,
            "stdev_us": data.get("stdev_ns", 0) / 1000,
        })

    # Build HTML using discovered categories
    html = ""
    category_order = get_all_categories(benchmarks)

    for category in category_order:
        if category not in categories:
            continue

        items = categories[category]
        items_html = ""
        for item in sorted(items, key=lambda x: x["id"]):
            items_html += f'''
                <div class="bench-item" title="{item['desc']}">
                    <span class="bench-name">{item['name']}</span>
                    <span class="bench-time">{item['mean_us']:.1f} us</span>
                </div>'''

        html += f'''
            <div class="category-card">
                <h4>{category}</h4>
                <div class="bench-items">{items_html}</div>
            </div>'''

    return html


def generate_rust_benchmark_cards(criterion_benchmarks: dict) -> str:
    """Generate HTML cards for Rust Criterion benchmarks, grouped by group name.

    Args:
        criterion_benchmarks: Dict with structure:
            {"group/bench_name": {"mean_ns": ..., "stdev_ns": ..., "group": ..., "name": ...}, ...}

    Returns:
        HTML string with benchmark cards grouped by Criterion group.
    """
    if not criterion_benchmarks:
        return '<div class="no-data">No Rust benchmark data available. Run <code>cargo bench</code> first.</div>'

    # Group by Criterion group
    groups = {}
    for bench_id, data in criterion_benchmarks.items():
        if not isinstance(data, dict):
            continue
        group_name = data.get("group", "unknown")
        bench_name = data.get("name", bench_id)

        if group_name not in groups:
            groups[group_name] = []

        mean_ns = data.get("mean_ns", 0)
        stdev_ns = data.get("stdev_ns", 0)

        # Format time nicely (ns, us, or ms)
        if mean_ns >= 1_000_000:
            time_str = f"{mean_ns / 1_000_000:.2f} ms"
        elif mean_ns >= 1000:
            time_str = f"{mean_ns / 1000:.2f} us"
        else:
            time_str = f"{mean_ns:.0f} ns"

        groups[group_name].append({
            "id": bench_id,
            "name": bench_name,
            "mean_ns": mean_ns,
            "time_str": time_str,
        })

    # Build HTML
    html = ""
    for group_name in sorted(groups.keys()):
        items = groups[group_name]
        items_html = ""
        for item in sorted(items, key=lambda x: x["name"]):
            # Make benchmark name more readable
            display_name = item["name"].replace("_", " ").title()
            items_html += f'''
                <div class="bench-item" title="Benchmark: {item['id']}">
                    <span class="bench-name">{display_name}</span>
                    <span class="bench-time">{item['time_str']}</span>
                </div>'''

        # Make group name more readable
        display_group = group_name.replace("_", " ").title()
        html += f'''
            <div class="category-card">
                <h4>{display_group}</h4>
                <div class="bench-items">{items_html}</div>
            </div>'''

    return html


def generate_chart_data(benchmarks: dict) -> tuple[list, list, list]:
    """Extract labels, values, and colors for Chart.js."""
    labels = []
    values = []
    colors = []

    # Auto-assign colors by category
    color_palette = [
        "rgba(88, 166, 255, 0.8)",   # blue
        "rgba(63, 185, 80, 0.8)",    # green
        "rgba(210, 153, 34, 0.8)",   # yellow
        "rgba(248, 81, 73, 0.8)",    # red
        "rgba(163, 113, 247, 0.8)",  # purple
        "rgba(219, 109, 163, 0.8)",  # pink
        "rgba(121, 192, 255, 0.8)",  # light blue
        "rgba(139, 148, 158, 0.8)",  # gray
    ]

    # Map categories to colors dynamically
    all_categories = get_all_categories(benchmarks)
    category_colors = {cat: color_palette[i % len(color_palette)]
                       for i, cat in enumerate(all_categories)}

    for bench_id, data in sorted(benchmarks.items()):
        if not isinstance(data, dict) or "error" in data:
            continue
        name, _ = benchmark_id_to_display(bench_id)
        labels.append(name)
        values.append(round(data.get("mean_ns", 0) / 1000, 2))
        colors.append(category_colors.get(get_category(bench_id), color_palette[-1]))

    return labels, values, colors


def generate_dashboard(artifacts_dir: str, output_dir: str, criterion_dir: str = None):
    """Generate dashboard from benchmark artifacts."""
    out = Path(output_dir)
    out.mkdir(parents=True, exist_ok=True)

    # Copy Criterion reports if available
    criterion_src = Path(criterion_dir) if criterion_dir else Path("target/criterion")
    criterion_available = False
    criterion_groups = []
    criterion_benchmarks = {}

    if criterion_src.exists():
        criterion_dst = out / "criterion"
        if criterion_dst.exists():
            shutil.rmtree(criterion_dst)
        shutil.copytree(criterion_src, criterion_dst)
        criterion_available = True

        # Auto-discover ALL Criterion benchmarks (groups AND individual benchmarks)
        criterion_data = discover_criterion_benchmarks(criterion_dst)
        criterion_groups = criterion_data["groups"]
        criterion_benchmarks = criterion_data["benchmarks"]

        print(f"Copied Criterion reports: {len(criterion_groups)} groups, {len(criterion_benchmarks)} benchmarks")

    # Auto-discover all benchmarks from all platforms
    all_benchmarks = discover_all_benchmarks(artifacts_dir)

    # Generate chart data
    py_labels, py_values, py_colors = generate_chart_data(all_benchmarks.get("python", {}))
    node_labels, node_values, node_colors = generate_chart_data(all_benchmarks.get("node", {}))

    # Metadata
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
    commit = os.environ.get("GITHUB_SHA", "local")[:7]
    repo = os.environ.get("GITHUB_REPOSITORY", "mathhook/mathhook")

    # Status (auto-detected from data)
    def get_status(benchmarks):
        if benchmarks and len(benchmarks) > 0:
            return ("ok", f"{len(benchmarks)} benchmarks")
        return ("skip", "Not available")

    # Rust status includes both groups and individual benchmarks
    if criterion_benchmarks:
        rust_status = ("ok", f"{len(criterion_groups)} groups, {len(criterion_benchmarks)} benchmarks")
    elif criterion_groups:
        rust_status = ("ok", f"{len(criterion_groups)} groups")
    else:
        rust_status = ("skip", "Not available")
    python_status = get_status(all_benchmarks.get("python", {}))
    node_status = get_status(all_benchmarks.get("node", {}))

    # Build Criterion links
    criterion_links = ""
    if criterion_groups:
        links = []
        for group in criterion_groups:
            links.append(f'<a href="criterion/{group}/report/index.html" class="bench-link">{group}</a>')
        criterion_links = "\n                ".join(links)

    # Generate Rust benchmark cards from Criterion data
    rust_cards = generate_rust_benchmark_cards(criterion_benchmarks)

    # Generate benchmark cards
    python_cards = generate_benchmark_cards(all_benchmarks.get("python", {}))
    node_cards = generate_benchmark_cards(all_benchmarks.get("node", {}))

    html = generate_html(
        timestamp=timestamp,
        commit=commit,
        repo=repo,
        rust_status=rust_status,
        python_status=python_status,
        node_status=node_status,
        criterion_available=criterion_available,
        criterion_links=criterion_links,
        rust_cards=rust_cards,
        python_cards=python_cards,
        node_cards=node_cards,
        py_labels=py_labels,
        py_values=py_values,
        py_colors=py_colors,
        node_labels=node_labels,
        node_values=node_values,
        node_colors=node_colors,
    )

    (out / "index.html").write_text(html)

    # Write JSON API
    combined = {
        "generated_at": timestamp,
        "commit": commit,
        "repository": repo,
        "criterion_available": criterion_available,
        "criterion_groups": criterion_groups,
        "platforms": {
            "rust": {
                "status": "ok" if criterion_benchmarks else ("ok" if criterion_groups else "not_available"),
                "groups": criterion_groups,
                "benchmarks": criterion_benchmarks,  # Now includes ALL individual benchmarks!
                "total_benchmarks": len(criterion_benchmarks),
            },
            "python": all_benchmarks.get("python", {}),
            "node": all_benchmarks.get("node", {}),
        },
        "metadata": {
            "python": all_benchmarks.get("python_meta", {}),
            "node": all_benchmarks.get("node_meta", {})
        },
    }
    (out / "results.json").write_text(json.dumps(combined, indent=2))

    print(f"Dashboard generated: {output_dir}")
    print(f"  Rust Criterion: {len(criterion_groups)} groups, {len(criterion_benchmarks)} benchmarks (auto-discovered!)")
    print(f"  Python: {len(all_benchmarks.get('python', {}))} benchmarks")
    print(f"  Node: {len(all_benchmarks.get('node', {}))} benchmarks")


def generate_html(
    timestamp: str,
    commit: str,
    repo: str,
    rust_status: tuple,
    python_status: tuple,
    node_status: tuple,
    criterion_available: bool,
    criterion_links: str,
    rust_cards: str,
    python_cards: str,
    node_cards: str,
    py_labels: list,
    py_values: list,
    py_colors: list,
    node_labels: list,
    node_values: list,
    node_colors: list,
) -> str:
    """Generate the main dashboard HTML."""

    return f'''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MathHook Benchmarks</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        :root {{
            --bg: #0d1117;
            --card: #161b22;
            --border: #30363d;
            --text: #c9d1d9;
            --text-muted: #8b949e;
            --accent: #58a6ff;
            --green: #3fb950;
            --yellow: #d29922;
            --red: #f85149;
            --purple: #a371f7;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.5;
        }}
        .container {{ max-width: 1400px; margin: 0 auto; padding: 2rem; }}

        header {{
            text-align: center;
            margin-bottom: 2rem;
            padding-bottom: 1.5rem;
            border-bottom: 1px solid var(--border);
        }}
        h1 {{
            font-size: 2.5rem;
            font-weight: 600;
            color: var(--accent);
            margin-bottom: 0.5rem;
        }}
        .meta {{ color: var(--text-muted); font-size: 0.9rem; }}
        .meta code {{
            background: var(--card);
            padding: 0.2rem 0.5rem;
            border-radius: 4px;
            font-family: 'SF Mono', Consolas, monospace;
        }}
        .meta a {{ color: var(--accent); text-decoration: none; }}
        .meta a:hover {{ text-decoration: underline; }}

        .status-grid {{
            display: grid;
            grid-template-columns: repeat(5, 1fr);
            gap: 1rem;
            margin-bottom: 2rem;
        }}
        .status-card {{
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 1rem 1.2rem;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }}
        .status-dot {{
            width: 12px;
            height: 12px;
            border-radius: 50%;
            flex-shrink: 0;
        }}
        .status-dot.ok {{ background: var(--green); }}
        .status-dot.skip {{ background: var(--yellow); }}
        .status-dot.fail {{ background: var(--red); }}
        .status-info h3 {{ font-size: 0.9rem; font-weight: 600; }}
        .status-info p {{ font-size: 0.75rem; color: var(--text-muted); }}

        section {{
            background: var(--card);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 1.5rem;
        }}
        section h2 {{
            font-size: 1.25rem;
            font-weight: 600;
            margin-bottom: 1rem;
            color: var(--accent);
        }}
        section > p {{ color: var(--text-muted); margin-bottom: 1rem; font-size: 0.9rem; }}

        .bench-links {{
            display: flex;
            flex-wrap: wrap;
            gap: 0.75rem;
        }}
        .bench-link {{
            display: inline-block;
            background: var(--bg);
            border: 1px solid var(--border);
            padding: 0.5rem 1rem;
            border-radius: 6px;
            color: var(--text);
            text-decoration: none;
            font-size: 0.9rem;
            transition: all 0.2s;
        }}
        .bench-link:hover {{
            border-color: var(--accent);
            color: var(--accent);
        }}
        .bench-link.primary {{
            background: var(--accent);
            color: white;
            border-color: var(--accent);
        }}

        .categories-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
            gap: 1rem;
            margin-top: 1rem;
        }}
        .category-card {{
            background: var(--bg);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 1rem;
        }}
        .category-card h4 {{
            font-size: 0.9rem;
            font-weight: 600;
            margin-bottom: 0.75rem;
            color: var(--accent);
        }}
        .bench-items {{
            display: flex;
            flex-direction: column;
            gap: 0.5rem;
        }}
        .bench-item {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 0.4rem 0.6rem;
            background: var(--card);
            border-radius: 4px;
            font-size: 0.85rem;
        }}
        .bench-name {{ color: var(--text); }}
        .bench-time {{
            font-family: 'SF Mono', Consolas, monospace;
            color: var(--green);
            font-size: 0.8rem;
        }}

        .chart-container {{
            position: relative;
            height: 400px;
            margin-top: 1rem;
        }}

        .no-data {{
            text-align: center;
            padding: 2rem;
            color: var(--text-muted);
        }}

        .tabs {{
            display: flex;
            gap: 0.5rem;
            margin-bottom: 1rem;
        }}
        .tab {{
            padding: 0.5rem 1rem;
            background: var(--bg);
            border: 1px solid var(--border);
            border-radius: 6px;
            color: var(--text-muted);
            cursor: pointer;
            font-size: 0.9rem;
        }}
        .tab:hover {{ color: var(--text); }}
        .tab.active {{
            background: var(--accent);
            color: white;
            border-color: var(--accent);
        }}
        .tab-content {{ display: none; }}
        .tab-content.active {{ display: block; }}

        footer {{
            text-align: center;
            padding-top: 2rem;
            border-top: 1px solid var(--border);
            margin-top: 2rem;
            color: var(--text-muted);
            font-size: 0.85rem;
        }}
        footer a {{ color: var(--accent); text-decoration: none; }}
        footer a:hover {{ text-decoration: underline; }}

        @media (max-width: 1100px) {{
            .status-grid {{ grid-template-columns: repeat(3, 1fr); }}
        }}
        @media (max-width: 700px) {{
            .status-grid {{ grid-template-columns: repeat(2, 1fr); }}
            .chart-container {{ height: 300px; }}
        }}
        @media (max-width: 500px) {{
            .status-grid {{ grid-template-columns: 1fr; }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>MathHook Benchmarks</h1>
            <p class="meta">
                Last updated: {timestamp}<br>
                Commit: <code>{commit}</code> |
                <a href="https://github.com/{repo}">Repository</a> |
                <a href="https://github.com/{repo}/actions">CI Runs</a>
            </p>
        </header>

        <div class="status-grid">
            <div class="status-card">
                <div class="status-dot {rust_status[0]}"></div>
                <div class="status-info">
                    <h3>Rust (Criterion)</h3>
                    <p>{rust_status[1]}</p>
                </div>
            </div>
            <div class="status-card">
                <div class="status-dot {python_status[0]}"></div>
                <div class="status-info">
                    <h3>Python (PyO3)</h3>
                    <p>{python_status[1]}</p>
                </div>
            </div>
            <div class="status-card">
                <div class="status-dot {node_status[0]}"></div>
                <div class="status-info">
                    <h3>Node.js (NAPI)</h3>
                    <p>{node_status[1]}</p>
                </div>
            </div>
        </div>

        {"" if not criterion_available else f'''
        <section>
            <h2>Rust Criterion Reports</h2>
            <p>Detailed benchmarks with histograms, violin plots, and regression analysis.</p>
            <div class="bench-links">
                <a href="criterion/report/index.html" class="bench-link primary">All Benchmarks Index</a>
                {criterion_links}
            </div>
        </section>
        '''}

        <section>
            <h2>Rust Benchmarks (Native Performance)</h2>
            <p>Auto-discovered from Criterion. All individual benchmark timings shown below.</p>
            <div class="categories-grid">{rust_cards}</div>
        </section>

        <section>
            <h2>Python Benchmarks (MathHook via PyO3)</h2>
            <div class="tabs">
                <button class="tab active" onclick="showTab('py-cards', this)">Cards</button>
                <button class="tab" onclick="showTab('py-chart', this)">Chart</button>
            </div>
            <div id="py-cards" class="tab-content active">
                <div class="categories-grid">{python_cards}</div>
            </div>
            <div id="py-chart" class="tab-content">
                {"<div class='no-data'>No data</div>" if not py_labels else '''
                <div class="chart-container">
                    <canvas id="pythonChart"></canvas>
                </div>
                '''}
            </div>
        </section>

        <section>
            <h2>Node.js Benchmarks (MathHook via NAPI)</h2>
            <div class="tabs">
                <button class="tab active" onclick="showTab('node-cards', this)">Cards</button>
                <button class="tab" onclick="showTab('node-chart', this)">Chart</button>
            </div>
            <div id="node-cards" class="tab-content active">
                <div class="categories-grid">{node_cards}</div>
            </div>
            <div id="node-chart" class="tab-content">
                {"<div class='no-data'>No data</div>" if not node_labels else '''
                <div class="chart-container">
                    <canvas id="nodeChart"></canvas>
                </div>
                '''}
            </div>
        </section>

        <footer>
            <p>
                Built with <a href="https://github.com/bheisler/criterion.rs">Criterion.rs</a>,
                <a href="https://pyo3.rs">PyO3</a>, and
                <a href="https://napi.rs">NAPI-RS</a>
            </p>
        </footer>
    </div>

    <script>
        function showTab(tabId, btn) {{
            const section = btn.closest('section');
            section.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            section.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.getElementById(tabId).classList.add('active');
            btn.classList.add('active');
        }}

        const chartConfig = {{
            type: 'bar',
            options: {{
                indexAxis: 'y',
                responsive: true,
                maintainAspectRatio: false,
                plugins: {{
                    legend: {{ display: false }},
                    tooltip: {{
                        callbacks: {{
                            label: (ctx) => ctx.raw.toLocaleString() + ' us'
                        }}
                    }}
                }},
                scales: {{
                    x: {{
                        title: {{ display: true, text: 'Time (microseconds)', color: '#8b949e' }},
                        grid: {{ color: '#30363d' }},
                        ticks: {{ color: '#8b949e' }}
                    }},
                    y: {{
                        grid: {{ display: false }},
                        ticks: {{ color: '#c9d1d9', font: {{ size: 11 }} }}
                    }}
                }}
            }}
        }};

        {f'''
        new Chart(document.getElementById('pythonChart'), {{
            ...chartConfig,
            data: {{
                labels: {json.dumps(py_labels)},
                datasets: [{{
                    data: {json.dumps(py_values)},
                    backgroundColor: {json.dumps(py_colors)},
                    borderWidth: 0
                }}]
            }}
        }});
        ''' if py_labels else ''}

        {f'''
        new Chart(document.getElementById('nodeChart'), {{
            ...chartConfig,
            data: {{
                labels: {json.dumps(node_labels)},
                datasets: [{{
                    data: {json.dumps(node_values)},
                    backgroundColor: {json.dumps(node_colors)},
                    borderWidth: 0
                }}]
            }}
        }});
        ''' if node_labels else ''}
    </script>
</body>
</html>'''


if __name__ == "__main__":
    artifacts = sys.argv[1] if len(sys.argv) > 1 else "artifacts"
    output = sys.argv[2] if len(sys.argv) > 2 else "gh-pages"
    criterion = sys.argv[3] if len(sys.argv) > 3 else "target/criterion"
    generate_dashboard(artifacts, output, criterion)
