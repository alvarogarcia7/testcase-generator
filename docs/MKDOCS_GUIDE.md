# MkDocs Documentation Guide

This guide explains how to work with the MkDocs documentation system for the testcase-generator project.

## Overview

This project uses [MkDocs](https://www.mkdocs.org/) with the [Material theme](https://squidfunk.github.io/mkdocs-material/) to generate both HTML and PDF documentation. The documentation is automatically deployed to GitLab Pages and GitHub Pages on every push to the main branch.

## Setup and Local Development

### Installation

Install MkDocs and all required dependencies:

```bash
make docs-install
```

This creates a Python virtual environment (`mkdocs-venv/`) and installs:
- MkDocs (documentation generator)
- Material for MkDocs (theme)
- mkdocs-with-pdf (PDF export plugin)
- pymdown-extensions (markdown extensions)

### Serving Locally

Start a local development server to preview documentation changes:

```bash
make docs-serve
```

The documentation will be available at `http://127.0.0.1:8000`. The server automatically reloads when you edit markdown files, allowing you to see changes in real-time.

### Building HTML

Build static HTML documentation:

```bash
make docs-build
```

The generated site will be in the `site/` directory. Open `site/index.html` in your browser to view the built documentation.

### Generating PDF

Build documentation with PDF export:

```bash
make docs-build-pdf
```

The PDF will be generated at `site/pdf/testcase-manager-documentation.pdf`. This command sets the `ENABLE_PDF_EXPORT=1` environment variable to enable PDF generation.

### Cleaning Build Artifacts

Remove generated files:

```bash
make docs-clean
```

This removes the `site/` and `pdf/` directories.

## Documentation Structure

The documentation is organized into logical sections:

```
docs/
├── index.md                    # Homepage (from README.md content)
├── getting-started/
│   ├── index.md               # Quick start guide
│   └── installation.md        # Installation instructions
├── user-guide/
│   ├── index.md               # User guide overview
│   ├── basic-usage.md         # Basic usage examples
│   └── advanced-usage.md      # Advanced features
├── cli-tools/
│   ├── index.md               # CLI tools overview
│   ├── testcase-manager.md    # Main CLI tool
│   └── json-escape.md         # JSON escape utility
├── features/
│   ├── index.md               # Features overview
│   ├── templating.md          # Template system
│   └── validation.md          # Validation features
├── development/
│   ├── index.md               # Development overview
│   ├── coverage.md            # Coverage testing
│   ├── contributing.md        # Contribution guidelines
│   └── architecture.md        # System architecture
├── examples/
│   ├── index.md               # Examples overview
│   └── tutorials/             # Step-by-step tutorials
└── MKDOCS_GUIDE.md            # This file
```

### File Organization

- **Section directories**: Group related documentation
- **index.md files**: Serve as section landing pages
- **Descriptive filenames**: Use lowercase with hyphens (e.g., `basic-usage.md`)
- **Logical hierarchy**: Keep nesting depth to 2-3 levels maximum

## Adding New Pages

### 1. Create the Markdown File

Create a new `.md` file in the appropriate section directory:

```bash
touch docs/user-guide/my-new-page.md
```

### 2. Add Content

Write your documentation using standard markdown syntax (see [Markdown Syntax](#markdown-syntax-and-extensions) below).

### 3. Update Navigation

Edit `mkdocs.yml` and add your page to the `nav` section:

```yaml
nav:
  - Home: index.md
  - User Guide:
      - Overview: user-guide/index.md
      - Basic Usage: user-guide/basic-usage.md
      - My New Page: user-guide/my-new-page.md  # Add here
```

### 4. Preview Changes

Run `make docs-serve` to preview your changes locally before committing.

## Markdown Syntax and Extensions

MkDocs supports standard markdown plus several extensions for enhanced formatting.

### Code Blocks with Syntax Highlighting

Use triple backticks with a language identifier:

````markdown
```python
def hello_world():
    print("Hello, World!")
```

```bash
cargo build --release
```

```rust
fn main() {
    println!("Hello, World!");
}
```
````

### Admonitions

Create callout boxes for notes, warnings, and tips:

```markdown
!!! note
    This is a note admonition.

!!! warning
    This is a warning admonition.

!!! tip
    This is a tip admonition.

!!! danger
    This is a danger admonition.

!!! info
    This is an info admonition.

!!! example
    This is an example admonition.
```

Collapsible admonitions:

```markdown
??? note "Click to expand"
    This content is hidden by default.
```

### Tables

Create tables using standard markdown syntax:

```markdown
| Command | Description |
|---------|-------------|
| `make docs-install` | Install MkDocs dependencies |
| `make docs-serve` | Start local development server |
| `make docs-build` | Build HTML documentation |
```

### Internal Links

Link to other documentation pages:

```markdown
[See the User Guide](../user-guide/index.md)
[Installation Instructions](../getting-started/installation.md)
[Coverage Testing](../development/coverage.md)
```

**Important**: Use relative paths from the current file's location.

### Inline Code

Use single backticks for inline code:

```markdown
The `make test` command runs all tests.
```

### Lists

Unordered lists:

```markdown
- Item 1
- Item 2
  - Nested item
  - Another nested item
- Item 3
```

Ordered lists:

```markdown
1. First step
2. Second step
3. Third step
```

### Headings

Use hierarchical headings:

```markdown
# H1 - Page Title (use once per page)
## H2 - Major Section
### H3 - Subsection
#### H4 - Sub-subsection
```

## Deployment

### GitLab Pages

Documentation is automatically deployed to GitLab Pages when you push to the `main` branch.

**URL**: `https://<username>.gitlab.io/<repository>/`

**CI/CD Pipeline** (`.gitlab-ci.yml`):
- Triggered on push to `main` branch
- Installs Python dependencies
- Builds documentation with PDF export
- Publishes to GitLab Pages
- Artifacts expire after 30 days

### GitHub Pages

Documentation is automatically deployed to GitHub Pages when you push to the `main` branch.

**URL**: `https://<username>.github.io/<repository>/`

**GitHub Actions Workflow** (`.github/workflows/docs.yml`):
- Triggered on push to `main` branch or manual workflow dispatch
- Installs Python dependencies
- Builds HTML documentation
- Deploys to GitHub Pages
- Generates PDF and uploads as artifact

### Manual Deployment

Deploy to GitHub Pages manually:

```bash
make docs-deploy-github
```

This runs `mkdocs gh-deploy`, which builds the documentation and pushes it to the `gh-pages` branch.

## PDF Generation

### Automatic Generation (CI/CD)

PDF documentation is automatically generated during CI/CD builds on both GitLab and GitHub:

- **GitLab CI**: PDF available in pipeline artifacts under `public/pdf/`
- **GitHub Actions**: PDF uploaded as workflow artifact and attached to releases

### Manual Generation

Generate PDF locally:

```bash
make docs-build-pdf
```

The PDF will be created at `site/pdf/testcase-manager-documentation.pdf`.

### PDF Configuration

PDF export is configured in `mkdocs.yml`:

```yaml
plugins:
  - search
  - with-pdf:
      enabled_if_env: ENABLE_PDF_EXPORT
      author: Your Name
      copyright: Copyright © 2024
      output_path: pdf/testcase-manager-documentation.pdf
      cover_title: Testcase Manager Documentation
      cover_subtitle: Comprehensive Guide
      toc_title: Table of Contents
      heading_shift: false
      toc_level: 3
      ordered_chapter_level: 2
```

### Excluding Pages from PDF

Some pages are excluded from PDF generation (e.g., implementation notes):

```yaml
plugins:
  - with-pdf:
      exclude_pages:
        - 'development/IMPLEMENTATION_*'
        - 'development/*_SUMMARY'
```

## Troubleshooting

### WeasyPrint Dependencies

If PDF generation fails with WeasyPrint errors, install system dependencies:

**Ubuntu/Debian**:
```bash
sudo apt-get install -y libpango-1.0-0 libpangoft2-1.0-0 libharfbuzz-subset0
```

**macOS**:
```bash
brew install pango gdk-pixbuf libffi
```

**Fedora/RHEL**:
```bash
sudo dnf install -y pango gdk-pixbuf2 libffi
```

### Broken Links

Check for broken internal links:

```bash
# Build and check for warnings
make docs-build
```

MkDocs will warn about broken links during the build process. Fix them by:
1. Verifying the target file exists
2. Checking the relative path is correct
3. Ensuring the file is included in `mkdocs.yml` navigation

### Theme Customization Issues

If custom CSS or theme features aren't working:

1. **Check Material version compatibility**: Ensure `mkdocs-material>=9.5.0`
2. **Verify configuration syntax**: Review `mkdocs.yml` for YAML errors
3. **Clear cache**: Remove `site/` directory and rebuild

### Port Already in Use

If `make docs-serve` fails because port 8000 is in use:

```bash
# Use a different port
mkdocs-venv/bin/mkdocs serve -a localhost:8001
```

Or kill the process using port 8000:

```bash
# Find process
lsof -i :8000

# Kill process
kill -9 <PID>
```

### Missing Python Dependencies

If you see import errors:

```bash
# Reinstall dependencies
make docs-clean
rm -rf mkdocs-venv/
make docs-install
```

### PDF Generation Hangs

If PDF generation hangs or takes too long:

1. **Check for large images**: Compress images before adding to docs
2. **Reduce complexity**: Simplify complex CSS or embedded content
3. **Exclude problematic pages**: Add to `exclude_pages` in `mkdocs.yml`

### Virtual Environment Issues

If the virtual environment is corrupted:

```bash
# Remove and recreate
rm -rf mkdocs-venv/
make docs-install
```

## Best Practices

### Writing Documentation

1. **Be concise**: Keep explanations clear and to the point
2. **Use examples**: Show code examples for complex concepts
3. **Add context**: Explain why, not just how
4. **Test locally**: Always preview with `make docs-serve` before committing
5. **Update navigation**: Don't forget to add new pages to `mkdocs.yml`

### Markdown Conventions

1. **One sentence per line**: Makes diffs cleaner
2. **Use descriptive link text**: Avoid "click here"
3. **Add alt text to images**: `![Description](image.png)`
4. **Consistent heading levels**: Don't skip levels (H1 → H3)
5. **Code blocks**: Always specify the language for syntax highlighting

### Maintenance

1. **Keep dependencies updated**: Regularly update `requirements.txt`
2. **Review broken links**: Check CI/CD logs for warnings
3. **Monitor PDF generation**: Ensure PDFs generate correctly
4. **Test both themes**: Preview in light and dark mode

## Additional Resources

- [MkDocs Documentation](https://www.mkdocs.org/)
- [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/)
- [mkdocs-with-pdf Plugin](https://github.com/orzih/mkdocs-with-pdf)
- [PyMdown Extensions](https://facelessuser.github.io/pymdown-extensions/)
- [Markdown Guide](https://www.markdownguide.org/)
