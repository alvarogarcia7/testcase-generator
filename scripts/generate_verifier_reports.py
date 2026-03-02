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


def main():
    """Main execution function."""
    print("=" * 70)
    print("Test Verifier Report Generator")
    print("=" * 70)
    
    # Create output directory
    output_dir = Path('reports/verifier_scenarios')
    output_dir.mkdir(parents=True, exist_ok=True)
    print(f"\nOutput directory: {output_dir.absolute()}")
    
    # Build binaries
    if not build_binaries():
        print("\n✗ Failed to build binaries. Exiting.")
        return 1
    
    # Process each scenario
    generated_reports = []
    
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
        
        # Generate report
        if REPORTLAB_AVAILABLE:
            report_file = generate_pdf_report(scenario, verification_file, output_dir)
        else:
            report_file = generate_html_report(scenario, verification_file, output_dir)
        
        if report_file:
            generated_reports.append(report_file)
    
    # Summary
    print("\n" + "=" * 70)
    print("Report Generation Complete")
    print("=" * 70)
    print(f"\nGenerated {len(generated_reports)} reports:")
    for report_path in generated_reports:
        print(f"  • {report_path.absolute()}")
    
    return 0


if __name__ == '__main__':
    sys.exit(main())
