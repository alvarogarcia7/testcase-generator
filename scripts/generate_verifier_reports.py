#!/usr/bin/env python3
"""
Generate PDF reports for verifier scenarios.

This script:
1. Runs test-executor (runner) on test case YAML files to generate execution logs
2. Runs verifier on the execution logs to create verification reports
3. Generates PDF reports from the verification results using reportlab
"""

import os
import sys
import json
import subprocess
from pathlib import Path
from datetime import datetime

try:
    from reportlab.lib.pagesizes import letter, A4
    from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
    from reportlab.lib.units import inch
    from reportlab.lib.enums import TA_CENTER, TA_LEFT, TA_RIGHT
    from reportlab.lib import colors
    from reportlab.platypus import SimpleDocTemplate, Table, TableStyle, Paragraph, Spacer, PageBreak
    from reportlab.platypus import Image as RLImage
    REPORTLAB_AVAILABLE = True
except ImportError:
    REPORTLAB_AVAILABLE = False
    print("Warning: reportlab not installed. Will generate HTML reports instead.")
    print("To generate PDF reports, install reportlab: pip3 install reportlab")

try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False
    print("Warning: PyYAML not installed. Validation features will be limited.")
    print("To use validation, install PyYAML: pip3 install pyyaml")

# Define the 7 scenarios to generate reports for
SCENARIOS = [
    {
        'name': 'successful',
        'test_case_id': 'TEST_SUCCESS_001',
        'yaml': 'testcases/verifier_scenarios/successful/TEST_SUCCESS_001.yml',
        'execution_log': 'testcases/verifier_scenarios/successful/TEST_SUCCESS_001_execution_log.json',
        'description': 'Successful execution with all steps passing'
    },
    {
        'name': 'failed_first',
        'test_case_id': 'TEST_FAILED_FIRST_001',
        'yaml': 'testcases/verifier_scenarios/failed_first/TEST_FAILED_FIRST_001.yml',
        'execution_log': 'testcases/verifier_scenarios/failed_first/TEST_FAILED_FIRST_001_execution_log.json',
        'description': 'First step failure preventing subsequent execution'
    },
    {
        'name': 'failed_intermediate',
        'test_case_id': 'TEST_FAILED_INTERMEDIATE_001',
        'yaml': 'testcases/verifier_scenarios/failed_intermediate/TEST_FAILED_INTERMEDIATE_001.yml',
        'execution_log': 'testcases/verifier_scenarios/failed_intermediate/TEST_FAILED_INTERMEDIATE_001_execution_log.json',
        'description': 'Intermediate step failure with partial execution'
    },
    {
        'name': 'failed_last',
        'test_case_id': 'TEST_FAILED_LAST_001',
        'yaml': 'testcases/verifier_scenarios/failed_last/TEST_FAILED_LAST_001.yml',
        'execution_log': 'testcases/verifier_scenarios/failed_last/TEST_FAILED_LAST_001_execution_log.json',
        'description': 'Last step failure with output mismatch'
    },
    {
        'name': 'interrupted',
        'test_case_id': 'TEST_INTERRUPTED_001',
        'yaml': 'testcases/verifier_scenarios/interrupted/TEST_INTERRUPTED_001.yml',
        'execution_log': 'testcases/verifier_scenarios/interrupted/TEST_INTERRUPTED_001_execution_log.json',
        'description': 'Interrupted execution with incomplete sequences'
    },
    {
        'name': 'multiple_sequences',
        'test_case_id': 'TEST_MULTI_SEQ_001',
        'yaml': 'testcases/verifier_scenarios/multiple_sequences/TEST_MULTI_SEQ_001.yml',
        'execution_log': 'testcases/verifier_scenarios/multiple_sequences/TEST_MULTI_SEQ_001_execution_log.json',
        'description': 'Multiple sequences with mixed results'
    },
    {
        'name': 'hook_script_start',
        'test_case_id': 'TEST_HOOK_SCRIPT_START_001',
        'yaml': 'testcases/verifier_scenarios/hooks/TEST_HOOK_SCRIPT_START_001.yml',
        'execution_log': 'testcases/verifier_scenarios/hooks/TEST_HOOK_SCRIPT_START_001_execution_log.json',
        'description': 'Hook failure at script start'
    }
]


def run_command(cmd, cwd=None):
    """Run a shell command and return the result."""
    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Error: {result.stderr}")
    return result


def build_binaries():
    """Build the test-executor and verifier binaries."""
    print("\n=== Building binaries ===")
    result = run_command(['cargo', 'build', '--release', '--bin', 'test-executor', '--bin', 'verifier'])
    if result.returncode != 0:
        print("Failed to build binaries")
        return False
    print("✓ Binaries built successfully")
    return True


def ensure_execution_log_exists(scenario):
    """
    Check if execution log exists. If not, run test-executor to generate it.
    For verifier scenarios, execution logs should already exist as they simulate specific outcomes.
    """
    log_path = Path(scenario['execution_log'])
    if log_path.exists():
        print(f"✓ Execution log exists: {log_path}")
        return True
    
    print(f"⚠ Execution log missing: {log_path}")
    print("  Note: Verifier scenarios use pre-generated execution logs to simulate specific outcomes")
    print("  Skipping test-executor run for this scenario")
    return False


def run_verifier(scenario, output_dir):
    """Run verifier on the scenario and generate verification report."""
    print(f"\n=== Running verifier for {scenario['test_case_id']} ===")
    
    # Check if execution log exists
    if not Path(scenario['execution_log']).exists():
        print(f"⚠ Skipping {scenario['test_case_id']}: execution log not found")
        return None
    
    # Create output filename for verification result
    verification_output = output_dir / f"{scenario['test_case_id']}_verification.json"
    
    # Run verifier
    cmd = [
        'cargo', 'run', '--release', '--bin', 'verifier', '--',
        '--log', scenario['execution_log'],
        '--test-case', scenario['test_case_id'],
        '--format', 'json',
        '--output', str(verification_output)
    ]
    
    result = run_command(cmd)
    if result.returncode != 0 and result.returncode != 1:  # Exit code 1 is expected for failed tests
        print(f"✗ Verifier failed for {scenario['test_case_id']}")
        return None
    
    if verification_output.exists():
        print(f"✓ Verification report generated: {verification_output}")
        return verification_output
    else:
        print(f"✗ Verification report not created: {verification_output}")
        return None


def generate_pdf_report(scenario, verification_file, output_dir):
    """Generate a PDF report from the verification result."""
    print(f"\n=== Generating PDF report for {scenario['test_case_id']} ===")
    
    # Read verification result
    with open(verification_file, 'r') as f:
        verification_data = json.load(f)
    
    # Get the test case result (should be the first one in the report)
    if 'test_cases' in verification_data and len(verification_data['test_cases']) > 0:
        test_case = verification_data['test_cases'][0]
    else:
        test_case = verification_data
    
    # Create PDF filename
    pdf_output = output_dir / f"{scenario['test_case_id']}_report.pdf"
    
    # Create PDF document
    doc = SimpleDocTemplate(
        str(pdf_output),
        pagesize=letter,
        rightMargin=72,
        leftMargin=72,
        topMargin=72,
        bottomMargin=18
    )
    
    # Container for PDF elements
    story = []
    
    # Styles
    styles = getSampleStyleSheet()
    title_style = ParagraphStyle(
        'CustomTitle',
        parent=styles['Heading1'],
        fontSize=24,
        textColor=colors.HexColor('#1a1a1a'),
        spaceAfter=30,
        alignment=TA_CENTER
    )
    heading_style = ParagraphStyle(
        'CustomHeading',
        parent=styles['Heading2'],
        fontSize=16,
        textColor=colors.HexColor('#2c3e50'),
        spaceAfter=12,
        spaceBefore=12
    )
    normal_style = styles['Normal']
    
    # Title
    title = Paragraph(f"Test Verification Report<br/>{scenario['test_case_id']}", title_style)
    story.append(title)
    story.append(Spacer(1, 12))
    
    # Summary section
    story.append(Paragraph("Test Summary", heading_style))
    
    overall_pass = test_case.get('overall_pass', False)
    status_color = colors.green if overall_pass else colors.red
    status_text = "PASS" if overall_pass else "FAIL"
    
    summary_data = [
        ['Test Case ID:', scenario['test_case_id']],
        ['Description:', scenario['description']],
        ['Overall Status:', status_text],
        ['Total Steps:', str(test_case.get('total_steps', 0))],
        ['Passed Steps:', str(test_case.get('passed_steps', 0))],
        ['Failed Steps:', str(test_case.get('failed_steps', 0))],
        ['Not Executed:', str(test_case.get('not_executed_steps', 0))],
        ['Report Date:', datetime.now().strftime('%Y-%m-%d %H:%M:%S')]
    ]
    
    summary_table = Table(summary_data, colWidths=[2*inch, 4*inch])
    summary_table.setStyle(TableStyle([
        ('BACKGROUND', (0, 0), (0, -1), colors.HexColor('#ecf0f1')),
        ('TEXTCOLOR', (0, 0), (-1, -1), colors.black),
        ('ALIGN', (0, 0), (-1, -1), 'LEFT'),
        ('FONTNAME', (0, 0), (0, -1), 'Helvetica-Bold'),
        ('FONTNAME', (1, 0), (1, -1), 'Helvetica'),
        ('FONTSIZE', (0, 0), (-1, -1), 10),
        ('GRID', (0, 0), (-1, -1), 1, colors.grey),
        ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
        ('BACKGROUND', (1, 2), (1, 2), status_color),
        ('TEXTCOLOR', (1, 2), (1, 2), colors.white),
    ]))
    
    story.append(summary_table)
    story.append(Spacer(1, 20))
    
    # Sequences section
    if 'sequences' in test_case:
        story.append(Paragraph("Test Sequences", heading_style))
        
        for seq in test_case['sequences']:
            # Sequence header
            seq_title = f"Sequence {seq['sequence_id']}: {seq['name']}"
            story.append(Paragraph(seq_title, styles['Heading3']))
            story.append(Spacer(1, 6))
            
            # Sequence steps table
            step_data = [['Step', 'Description', 'Status', 'Reason']]
            
            for step_result in seq.get('step_results', []):
                status = step_result.get('status', 'unknown')
                step_num = step_result.get('step', '?')
                description = step_result.get('description', 'N/A')
                reason = step_result.get('reason', '-')
                
                # Truncate long descriptions
                if len(description) > 60:
                    description = description[:57] + '...'
                if len(reason) > 40:
                    reason = reason[:37] + '...'
                
                step_data.append([
                    str(step_num),
                    description,
                    status,
                    reason
                ])
            
            if len(step_data) > 1:  # Has steps beyond header
                step_table = Table(step_data, colWidths=[0.5*inch, 2.5*inch, 0.8*inch, 2.2*inch])
                step_table.setStyle(TableStyle([
                    ('BACKGROUND', (0, 0), (-1, 0), colors.HexColor('#34495e')),
                    ('TEXTCOLOR', (0, 0), (-1, 0), colors.white),
                    ('ALIGN', (0, 0), (-1, -1), 'LEFT'),
                    ('FONTNAME', (0, 0), (-1, 0), 'Helvetica-Bold'),
                    ('FONTSIZE', (0, 0), (-1, 0), 10),
                    ('FONTSIZE', (0, 1), (-1, -1), 9),
                    ('GRID', (0, 0), (-1, -1), 0.5, colors.grey),
                    ('VALIGN', (0, 0), (-1, -1), 'MIDDLE'),
                ]))
                
                # Color code status column
                for i in range(1, len(step_data)):
                    status = step_data[i][2]
                    if status == 'pass':
                        color = colors.green
                    elif status == 'fail':
                        color = colors.red
                    elif status == 'not_executed':
                        color = colors.grey
                    else:
                        color = colors.orange
                    step_table.setStyle(TableStyle([
                        ('BACKGROUND', (2, i), (2, i), color),
                        ('TEXTCOLOR', (2, i), (2, i), colors.white),
                    ]))
                
                story.append(step_table)
                story.append(Spacer(1, 12))
    
    # Build PDF
    doc.build(story)
    print(f"✓ PDF report generated: {pdf_output}")
    return pdf_output


def generate_html_report(scenario, verification_file, output_dir):
    """Generate an HTML report as fallback when reportlab is not available."""
    print(f"\n=== Generating HTML report for {scenario['test_case_id']} ===")
    
    # Read verification result
    with open(verification_file, 'r') as f:
        verification_data = json.load(f)
    
    # Get the test case result
    if 'test_cases' in verification_data and len(verification_data['test_cases']) > 0:
        test_case = verification_data['test_cases'][0]
    else:
        test_case = verification_data
    
    # Create HTML filename
    html_output = output_dir / f"{scenario['test_case_id']}_report.html"
    
    overall_pass = test_case.get('overall_pass', False)
    status_color = 'green' if overall_pass else 'red'
    status_text = "PASS" if overall_pass else "FAIL"
    
    html_content = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Test Verification Report - {scenario['test_case_id']}</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 40px;
            background-color: #f5f5f5;
        }}
        .container {{
            background-color: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #1a1a1a;
            text-align: center;
            border-bottom: 3px solid #3498db;
            padding-bottom: 10px;
        }}
        h2 {{
            color: #2c3e50;
            margin-top: 30px;
        }}
        h3 {{
            color: #34495e;
            margin-top: 20px;
        }}
        .summary-table {{
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
        }}
        .summary-table th {{
            background-color: #ecf0f1;
            padding: 10px;
            text-align: left;
            width: 200px;
        }}
        .summary-table td {{
            padding: 10px;
            border: 1px solid #ddd;
        }}
        .status-pass {{
            background-color: green;
            color: white;
            padding: 5px 10px;
            border-radius: 4px;
            font-weight: bold;
        }}
        .status-fail {{
            background-color: red;
            color: white;
            padding: 5px 10px;
            border-radius: 4px;
            font-weight: bold;
        }}
        .step-table {{
            width: 100%;
            border-collapse: collapse;
            margin: 15px 0;
        }}
        .step-table th {{
            background-color: #34495e;
            color: white;
            padding: 10px;
            text-align: left;
        }}
        .step-table td {{
            padding: 8px;
            border: 1px solid #ddd;
        }}
        .step-pass {{
            background-color: #d4edda;
        }}
        .step-fail {{
            background-color: #f8d7da;
        }}
        .step-not-executed {{
            background-color: #f0f0f0;
        }}
        .footer {{
            margin-top: 30px;
            text-align: center;
            color: #7f8c8d;
            font-size: 0.9em;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Test Verification Report<br/>{scenario['test_case_id']}</h1>
        
        <h2>Test Summary</h2>
        <table class="summary-table">
            <tr>
                <th>Test Case ID:</th>
                <td>{scenario['test_case_id']}</td>
            </tr>
            <tr>
                <th>Description:</th>
                <td>{scenario['description']}</td>
            </tr>
            <tr>
                <th>Overall Status:</th>
                <td><span class="status-{status_text.lower()}">{status_text}</span></td>
            </tr>
            <tr>
                <th>Total Steps:</th>
                <td>{test_case.get('total_steps', 0)}</td>
            </tr>
            <tr>
                <th>Passed Steps:</th>
                <td>{test_case.get('passed_steps', 0)}</td>
            </tr>
            <tr>
                <th>Failed Steps:</th>
                <td>{test_case.get('failed_steps', 0)}</td>
            </tr>
            <tr>
                <th>Not Executed:</th>
                <td>{test_case.get('not_executed_steps', 0)}</td>
            </tr>
            <tr>
                <th>Report Date:</th>
                <td>{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</td>
            </tr>
        </table>
"""
    
    # Add sequences
    if 'sequences' in test_case:
        html_content += "        <h2>Test Sequences</h2>\n"
        
        for seq in test_case['sequences']:
            html_content += f"""
        <h3>Sequence {seq['sequence_id']}: {seq['name']}</h3>
        <table class="step-table">
            <tr>
                <th>Step</th>
                <th>Description</th>
                <th>Status</th>
                <th>Reason</th>
            </tr>
"""
            
            for step_result in seq.get('step_results', []):
                status = step_result.get('status', 'unknown')
                step_num = step_result.get('step', '?')
                description = step_result.get('description', 'N/A')
                reason = step_result.get('reason', '-')
                
                row_class = f"step-{status.replace('_', '-')}"
                
                html_content += f"""
            <tr class="{row_class}">
                <td>{step_num}</td>
                <td>{description}</td>
                <td>{status}</td>
                <td>{reason}</td>
            </tr>
"""
            
            html_content += "        </table>\n"
    
    html_content += """
        <div class="footer">
            <p>Generated by Test Plan Documentation Generator</p>
        </div>
    </div>
</body>
</html>
"""
    
    with open(html_output, 'w') as f:
        f.write(html_content)
    
    print(f"✓ HTML report generated: {html_output}")
    return html_output


def validate_verification_data(yaml_path, verification_json_path):
    """
    Validate verification data against test case YAML.
    
    This function:
    1. Loads the YAML test case and verification JSON
    2. Compares test sequence counts
    3. Compares step counts per sequence
    4. Validates that no invalid values exist in critical fields
    
    Args:
        yaml_path: Path to the YAML test case file (string or Path)
        verification_json_path: Path to the verification JSON file (string or Path)
    
    Returns:
        List of validation error messages (empty list if validation passes)
    """
    errors = []
    
    # Check if YAML library is available
    if not YAML_AVAILABLE:
        errors.append("PyYAML library not available - cannot validate YAML test case")
        return errors
    
    # Convert to Path objects
    yaml_path = Path(yaml_path)
    verification_json_path = Path(verification_json_path)
    
    # Check if files exist
    if not yaml_path.exists():
        errors.append(f"YAML test case file not found: {yaml_path}")
        return errors
    
    if not verification_json_path.exists():
        errors.append(f"Verification JSON file not found: {verification_json_path}")
        return errors
    
    # Load YAML test case
    try:
        with open(yaml_path, 'r') as f:
            test_case = yaml.safe_load(f)
    except Exception as e:
        errors.append(f"Failed to load YAML test case: {e}")
        return errors
    
    # Load verification JSON
    try:
        with open(verification_json_path, 'r') as f:
            verification_data = json.load(f)
    except Exception as e:
        errors.append(f"Failed to load verification JSON: {e}")
        return errors
    
    # Extract test case from verification data
    # The verification JSON can have a 'test_cases' array or be a single test case
    if 'test_cases' in verification_data and len(verification_data['test_cases']) > 0:
        test_case_result = verification_data['test_cases'][0]
    else:
        test_case_result = verification_data
    
    # Get test sequences from YAML
    yaml_sequences = test_case.get('test_sequences', [])
    yaml_sequence_count = len(yaml_sequences)
    
    # Get sequences from verification JSON
    verification_sequences = test_case_result.get('sequences', [])
    verification_sequence_count = len(verification_sequences)
    
    # Validate sequence count
    if yaml_sequence_count != verification_sequence_count:
        errors.append(
            f"Sequence count mismatch: YAML has {yaml_sequence_count} sequences, "
            f"verification JSON has {verification_sequence_count} sequences"
        )
    
    # Validate step counts per sequence
    # Build a map of sequence ID to step count from YAML
    yaml_step_counts = {}
    for seq in yaml_sequences:
        seq_id = seq.get('id')
        steps = seq.get('steps', [])
        yaml_step_counts[seq_id] = len(steps)
    
    # Build a map of sequence ID to step count from verification JSON
    verification_step_counts = {}
    for seq in verification_sequences:
        seq_id = seq.get('sequence_id')
        step_results = seq.get('step_results', [])
        verification_step_counts[seq_id] = len(step_results)
    
    # Compare step counts
    for seq_id, yaml_count in yaml_step_counts.items():
        verification_count = verification_step_counts.get(seq_id)
        if verification_count is None:
            errors.append(
                f"Sequence {seq_id} exists in YAML but not in verification JSON"
            )
        elif yaml_count != verification_count:
            errors.append(
                f"Sequence {seq_id} step count mismatch: YAML has {yaml_count} steps, "
                f"verification JSON has {verification_count} steps"
            )
    
    # Check for sequences in verification JSON that don't exist in YAML
    for seq_id in verification_step_counts.keys():
        if seq_id not in yaml_step_counts:
            errors.append(
                f"Sequence {seq_id} exists in verification JSON but not in YAML"
            )
    
    # Validate that no invalid values exist in critical fields
    # Invalid values are: 'N/A', 'unknown', '?', or empty strings
    invalid_values = ['N/A', 'unknown', '?', '']
    
    for seq in verification_sequences:
        seq_id = seq.get('sequence_id', 'unknown')
        
        for step_result in seq.get('step_results', []):
            # Check status field
            status = step_result.get('status')
            if status in invalid_values:
                errors.append(
                    f"Sequence {seq_id}: Invalid status value '{status}' found"
                )
            
            # Check description field
            description = step_result.get('description')
            if description in invalid_values:
                errors.append(
                    f"Sequence {seq_id}: Invalid description value '{description}' found"
                )
            
            # Check step field (could be int or string)
            step = step_result.get('step')
            # Convert to string for comparison if it's not None
            step_str = str(step) if step is not None else step
            if step is None or step_str in invalid_values:
                errors.append(
                    f"Sequence {seq_id}: Invalid step value '{step}' found"
                )
            
            # Check reason field (only present in Fail variants)
            # Note: reason is only required in Fail variants, so we only check if it exists
            if 'reason' in step_result:
                reason = step_result.get('reason')
                if reason in invalid_values:
                    errors.append(
                        f"Sequence {seq_id}, Step {step}: Invalid reason value '{reason}' found"
                    )
    
    return errors


def generate_html_report_validated(scenario, verification_file, output_dir):
    """
    Generate an HTML report with validation and enhanced styling.
    
    This function:
    1. Validates the verification data against the test case YAML
    2. Exits with error code 1 if validation fails
    3. Generates an HTML report with enhanced styling and validation status indicators
    
    Args:
        scenario: Dictionary containing scenario information (name, test_case_id, yaml, etc.)
        verification_file: Path to the verification JSON file
        output_dir: Path to the output directory for the HTML report
    
    Returns:
        Path to the generated HTML report, or exits with code 1 if validation fails
    """
    print(f"\n=== Generating validated HTML report for {scenario['test_case_id']} ===")
    
    # Step 1: Validate the verification data
    print("Validating verification data...")
    validation_errors = validate_verification_data(scenario['yaml'], verification_file)
    
    if validation_errors:
        print("✗ Validation failed with the following errors:")
        for error in validation_errors:
            print(f"  • {error}")
        sys.exit(1)
    
    print("✓ Validation passed")
    
    # Step 2: Read verification result
    with open(verification_file, 'r') as f:
        verification_data = json.load(f)
    
    # Get the test case result
    if 'test_cases' in verification_data and len(verification_data['test_cases']) > 0:
        test_case = verification_data['test_cases'][0]
    else:
        test_case = verification_data
    
    # Create HTML filename
    html_output = output_dir / f"{scenario['test_case_id']}_report_validated.html"
    
    overall_pass = test_case.get('overall_pass', False)
    status_color = '#28a745' if overall_pass else '#dc3545'
    status_text = "PASS" if overall_pass else "FAIL"
    status_icon = "✓" if overall_pass else "✗"
    
    # Step 3: Generate enhanced HTML report
    html_content = f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Validated Test Verification Report - {scenario['test_case_id']}</title>
    <style>
        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 20px;
            min-height: 100vh;
        }}
        
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
            overflow: hidden;
        }}
        
        .header {{
            background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
            color: white;
            padding: 40px;
            text-align: center;
            position: relative;
        }}
        
        .validation-badge {{
            position: absolute;
            top: 20px;
            right: 20px;
            background-color: #28a745;
            color: white;
            padding: 8px 16px;
            border-radius: 20px;
            font-size: 0.85em;
            font-weight: 600;
            box-shadow: 0 2px 8px rgba(0,0,0,0.2);
        }}
        
        .validation-badge::before {{
            content: "✓ ";
            font-weight: bold;
        }}
        
        h1 {{
            font-size: 2.2em;
            margin-bottom: 10px;
            font-weight: 700;
        }}
        
        .test-id {{
            font-size: 1.1em;
            opacity: 0.9;
            font-weight: 400;
            letter-spacing: 1px;
        }}
        
        .content {{
            padding: 40px;
        }}
        
        .status-banner {{
            background-color: {status_color};
            color: white;
            padding: 20px;
            border-radius: 8px;
            text-align: center;
            font-size: 1.5em;
            font-weight: 700;
            margin-bottom: 30px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        }}
        
        .status-banner::before {{
            content: "{status_icon} ";
            font-size: 1.2em;
        }}
        
        h2 {{
            color: #2c3e50;
            margin-top: 40px;
            margin-bottom: 20px;
            font-size: 1.8em;
            border-bottom: 3px solid #3498db;
            padding-bottom: 10px;
        }}
        
        h2:first-child {{
            margin-top: 0;
        }}
        
        h3 {{
            color: #34495e;
            margin-top: 30px;
            margin-bottom: 15px;
            font-size: 1.3em;
            display: flex;
            align-items: center;
        }}
        
        h3::before {{
            content: "▸";
            color: #3498db;
            margin-right: 10px;
            font-weight: bold;
        }}
        
        .summary-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin: 20px 0;
        }}
        
        .summary-card {{
            background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            transition: transform 0.2s, box-shadow 0.2s;
        }}
        
        .summary-card:hover {{
            transform: translateY(-2px);
            box-shadow: 0 4px 16px rgba(0,0,0,0.15);
        }}
        
        .summary-card-label {{
            font-size: 0.85em;
            color: #5a6c7d;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            font-weight: 600;
            margin-bottom: 8px;
        }}
        
        .summary-card-value {{
            font-size: 1.8em;
            font-weight: 700;
            color: #2c3e50;
        }}
        
        .summary-table {{
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
            background: white;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        }}
        
        .summary-table th {{
            background: linear-gradient(135deg, #34495e 0%, #2c3e50 100%);
            color: white;
            padding: 15px;
            text-align: left;
            font-weight: 600;
            width: 220px;
        }}
        
        .summary-table td {{
            padding: 15px;
            border-bottom: 1px solid #ecf0f1;
        }}
        
        .summary-table tr:last-child td {{
            border-bottom: none;
        }}
        
        .summary-table tr:nth-child(even) {{
            background-color: #f8f9fa;
        }}
        
        .status-badge-inline {{
            display: inline-block;
            padding: 6px 16px;
            border-radius: 20px;
            font-weight: 700;
            font-size: 0.95em;
            letter-spacing: 0.5px;
            box-shadow: 0 2px 6px rgba(0,0,0,0.15);
        }}
        
        .status-pass {{
            background-color: #28a745;
            color: white;
        }}
        
        .status-fail {{
            background-color: #dc3545;
            color: white;
        }}
        
        .step-table {{
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
            background: white;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        }}
        
        .step-table th {{
            background: linear-gradient(135deg, #34495e 0%, #2c3e50 100%);
            color: white;
            padding: 15px 12px;
            text-align: left;
            font-weight: 600;
            font-size: 0.95em;
        }}
        
        .step-table td {{
            padding: 12px;
            border-bottom: 1px solid #ecf0f1;
        }}
        
        .step-table tr:last-child td {{
            border-bottom: none;
        }}
        
        .step-table tbody tr {{
            transition: background-color 0.2s;
        }}
        
        .step-table tbody tr:hover {{
            background-color: #f8f9fa;
        }}
        
        .step-pass {{
            background-color: #d4edda;
            border-left: 4px solid #28a745;
        }}
        
        .step-fail {{
            background-color: #f8d7da;
            border-left: 4px solid #dc3545;
        }}
        
        .step-not-executed {{
            background-color: #f0f0f0;
            border-left: 4px solid #6c757d;
        }}
        
        .step-status {{
            display: inline-block;
            padding: 4px 12px;
            border-radius: 12px;
            font-size: 0.85em;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.3px;
        }}
        
        .step-status-pass {{
            background-color: #28a745;
            color: white;
        }}
        
        .step-status-fail {{
            background-color: #dc3545;
            color: white;
        }}
        
        .step-status-not-executed {{
            background-color: #6c757d;
            color: white;
        }}
        
        .step-number {{
            font-weight: 700;
            color: #2c3e50;
            font-size: 1.1em;
        }}
        
        .step-description {{
            color: #555;
        }}
        
        .step-reason {{
            color: #666;
            font-style: italic;
        }}
        
        .footer {{
            background: linear-gradient(135deg, #ecf0f1 0%, #bdc3c7 100%);
            padding: 30px;
            text-align: center;
            color: #5a6c7d;
            font-size: 0.9em;
            border-top: 3px solid #3498db;
        }}
        
        .footer-content {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        
        .footer-title {{
            font-weight: 700;
            margin-bottom: 5px;
            color: #2c3e50;
        }}
        
        .stats-row {{
            display: flex;
            justify-content: space-around;
            flex-wrap: wrap;
            margin: 30px 0;
        }}
        
        .stat-box {{
            background: white;
            border-radius: 8px;
            padding: 20px 30px;
            min-width: 150px;
            text-align: center;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            margin: 10px;
        }}
        
        .stat-label {{
            font-size: 0.85em;
            color: #7f8c8d;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 8px;
        }}
        
        .stat-value {{
            font-size: 2.5em;
            font-weight: 700;
            color: #2c3e50;
        }}
        
        .stat-value.passed {{
            color: #28a745;
        }}
        
        .stat-value.failed {{
            color: #dc3545;
        }}
        
        .stat-value.not-executed {{
            color: #6c757d;
        }}
        
        @media print {{
            body {{
                background: white;
                padding: 0;
            }}
            
            .container {{
                box-shadow: none;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="validation-badge">VALIDATED</div>
            <h1>Test Verification Report</h1>
            <div class="test-id">{scenario['test_case_id']}</div>
        </div>
        
        <div class="content">
            <div class="status-banner">{status_text}</div>
            
            <h2>Test Summary</h2>
            
            <div class="stats-row">
                <div class="stat-box">
                    <div class="stat-label">Total Steps</div>
                    <div class="stat-value">{test_case.get('total_steps', 0)}</div>
                </div>
                <div class="stat-box">
                    <div class="stat-label">Passed</div>
                    <div class="stat-value passed">{test_case.get('passed_steps', 0)}</div>
                </div>
                <div class="stat-box">
                    <div class="stat-label">Failed</div>
                    <div class="stat-value failed">{test_case.get('failed_steps', 0)}</div>
                </div>
                <div class="stat-box">
                    <div class="stat-label">Not Executed</div>
                    <div class="stat-value not-executed">{test_case.get('not_executed_steps', 0)}</div>
                </div>
            </div>
            
            <table class="summary-table">
                <tr>
                    <th>Test Case ID</th>
                    <td>{scenario['test_case_id']}</td>
                </tr>
                <tr>
                    <th>Description</th>
                    <td>{scenario['description']}</td>
                </tr>
                <tr>
                    <th>Overall Status</th>
                    <td><span class="status-badge-inline status-{status_text.lower()}">{status_text}</span></td>
                </tr>
                <tr>
                    <th>Validation Status</th>
                    <td><span class="status-badge-inline status-pass">VALIDATED ✓</span></td>
                </tr>
                <tr>
                    <th>Report Date</th>
                    <td>{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</td>
                </tr>
            </table>
"""
    
    # Add sequences
    if 'sequences' in test_case and test_case['sequences']:
        html_content += """
            <h2>Test Sequences</h2>
"""
        
        for seq in test_case['sequences']:
            seq_id = seq.get('sequence_id', 'unknown')
            seq_name = seq.get('name', 'Unknown Sequence')
            
            html_content += f"""
            <h3>Sequence {seq_id}: {seq_name}</h3>
            <table class="step-table">
                <thead>
                    <tr>
                        <th style="width: 80px;">Step</th>
                        <th>Description</th>
                        <th style="width: 140px;">Status</th>
                        <th>Reason</th>
                    </tr>
                </thead>
                <tbody>
"""
            
            for step_result in seq.get('step_results', []):
                status = step_result.get('status', 'unknown')
                step_num = step_result.get('step', '?')
                description = step_result.get('description', 'N/A')
                reason = step_result.get('reason', '-')
                
                row_class = f"step-{status.replace('_', '-')}"
                status_class = f"step-status-{status.replace('_', '-')}"
                
                html_content += f"""
                    <tr class="{row_class}">
                        <td class="step-number">{step_num}</td>
                        <td class="step-description">{description}</td>
                        <td><span class="step-status {status_class}">{status}</span></td>
                        <td class="step-reason">{reason}</td>
                    </tr>
"""
            
            html_content += """
                </tbody>
            </table>
"""
    
    html_content += """
        </div>
        
        <div class="footer">
            <div class="footer-content">
                <div class="footer-title">Test Plan Documentation Generator</div>
                <div>Generated with validation and enhanced reporting</div>
            </div>
        </div>
    </div>
</body>
</html>
"""
    
    with open(html_output, 'w') as f:
        f.write(html_content)
    
    print(f"✓ Validated HTML report generated: {html_output}")
    return html_output


def main():
    """Main execution function."""
    print("=" * 70)
    print("Test Verifier Report Generator")
    print("=" * 70)
    
    # Check for reportlab availability and install if missing
    global REPORTLAB_AVAILABLE
    if not REPORTLAB_AVAILABLE:
        print("\n⚠ reportlab not found. Installing...")
        try:
            result = subprocess.run(
                [sys.executable, '-m', 'pip', 'install', 'reportlab'],
                capture_output=True,
                text=True
            )
            if result.returncode == 0:
                print("✓ reportlab installed successfully")
                # Re-import reportlab after installation
                try:
                    from reportlab.lib.pagesizes import letter, A4
                    from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
                    from reportlab.lib.units import inch
                    from reportlab.lib.enums import TA_CENTER, TA_LEFT, TA_RIGHT
                    from reportlab.lib import colors
                    from reportlab.platypus import SimpleDocTemplate, Table, TableStyle, Paragraph, Spacer, PageBreak
                    from reportlab.platypus import Image as RLImage
                    REPORTLAB_AVAILABLE = True
                    print("✓ reportlab loaded successfully")
                except ImportError as e:
                    print(f"✗ Failed to import reportlab after installation: {e}")
                    print("Continuing with HTML reports only")
            else:
                print(f"✗ Failed to install reportlab: {result.stderr}")
                print("Continuing with HTML reports only")
        except Exception as e:
            print(f"✗ Error installing reportlab: {e}")
            print("Continuing with HTML reports only")
    
    # Create output directory
    output_dir = Path('reports/verifier_scenarios')
    output_dir.mkdir(parents=True, exist_ok=True)
    print(f"\nOutput directory: {output_dir.absolute()}")
    
    # Build binaries
    if not build_binaries():
        print("\n✗ Failed to build binaries. Exiting.")
        return 1
    
    # Process each scenario
    generated_pdf_reports = []
    generated_html_reports = []
    
    for scenario in SCENARIOS:
        print("\n" + "=" * 70)
        print(f"Processing: {scenario['test_case_id']}")
        print("=" * 70)
        
        # Ensure execution log exists
        ensure_execution_log_exists(scenario)
        
        # Run verifier
        verification_file = run_verifier(scenario, output_dir)
        if verification_file is None:
            print(f"⚠ Skipping report generation for {scenario['test_case_id']}")
            continue
        
        # Generate both PDF and HTML reports (both are mandatory)
        if REPORTLAB_AVAILABLE:
            pdf_report = generate_pdf_report(scenario, verification_file, output_dir)
            if pdf_report:
                generated_pdf_reports.append(pdf_report)
        else:
            print(f"⚠ reportlab not available, skipping PDF generation for {scenario['test_case_id']}")
        
        html_report = generate_html_report_validated(scenario, verification_file, output_dir)
        if html_report:
            generated_html_reports.append(html_report)
    
    # Summary
    print("\n" + "=" * 70)
    print("Report Generation Complete")
    print("=" * 70)
    print(f"\nGenerated {len(generated_pdf_reports)} PDF reports:")
    for report_path in generated_pdf_reports:
        print(f"  • {report_path.absolute()}")
    print(f"\nGenerated {len(generated_html_reports)} HTML reports:")
    for report_path in generated_html_reports:
        print(f"  • {report_path.absolute()}")
    
    return 0


if __name__ == '__main__':
    sys.exit(main())
