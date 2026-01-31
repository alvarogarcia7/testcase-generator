#!/usr/bin/env python3
"""
Perform basic security scanning on Docker image metadata.
"""

import sys
import json


def analyze_docker_image(inspect_data: dict) -> dict:
    """Analyze Docker image for basic security concerns."""
    
    results = {
        'warnings': [],
        'info': [],
        'passed_checks': []
    }
    
    config = inspect_data[0].get('Config', {}) if isinstance(inspect_data, list) else inspect_data.get('Config', {})
    
    if config.get('User') == '' or config.get('User') == 'root':
        results['warnings'].append({
            'severity': 'medium',
            'message': 'Container runs as root user',
            'recommendation': 'Create and use a non-root user in Dockerfile'
        })
    else:
        results['passed_checks'].append('Running as non-root user')
    
    exposed_ports = config.get('ExposedPorts', {})
    if exposed_ports:
        results['info'].append({
            'message': f'Exposed ports: {", ".join(exposed_ports.keys())}'
        })
    
    env_vars = config.get('Env', [])
    sensitive_patterns = ['PASSWORD', 'SECRET', 'TOKEN', 'KEY', 'API_KEY']
    
    for env in env_vars:
        var_name = env.split('=')[0]
        if any(pattern in var_name.upper() for pattern in sensitive_patterns):
            results['warnings'].append({
                'severity': 'high',
                'message': f'Potentially sensitive environment variable: {var_name}',
                'recommendation': 'Avoid hardcoding secrets in images'
            })
    
    if not any(pattern in var_name.upper() for env in env_vars for pattern in sensitive_patterns if '=' in env):
        results['passed_checks'].append('No obvious secrets in environment variables')
    
    volumes = config.get('Volumes', {})
    if volumes:
        results['info'].append({
            'message': f'Defined volumes: {", ".join(volumes.keys())}'
        })
    
    healthcheck = config.get('Healthcheck')
    if not healthcheck:
        results['info'].append({
            'message': 'No healthcheck defined',
            'recommendation': 'Consider adding HEALTHCHECK instruction'
        })
    else:
        results['passed_checks'].append('Healthcheck configured')
    
    image_size = inspect_data[0].get('Size', 0) if isinstance(inspect_data, list) else inspect_data.get('Size', 0)
    size_mb = image_size / (1024 * 1024)
    
    if size_mb > 500:
        results['warnings'].append({
            'severity': 'low',
            'message': f'Large image size: {size_mb:.1f} MB',
            'recommendation': 'Consider using multi-stage builds to reduce size'
        })
    else:
        results['passed_checks'].append(f'Reasonable image size: {size_mb:.1f} MB')
    
    results['summary'] = {
        'total_warnings': len(results['warnings']),
        'total_info': len(results['info']),
        'total_passed': len(results['passed_checks']),
        'image_size_mb': round(size_mb, 1)
    }
    
    return results


def main():
    if len(sys.argv) < 2:
        print("Usage: docker_security_scan.py <docker_inspect.json>", file=sys.stderr)
        sys.exit(1)
    
    inspect_file = sys.argv[1]
    
    try:
        with open(inspect_file, 'r') as f:
            inspect_data = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"Error reading inspect file: {e}", file=sys.stderr)
        sys.exit(1)
    
    results = analyze_docker_image(inspect_data)
    
    print(json.dumps(results, indent=2))


if __name__ == '__main__':
    main()
