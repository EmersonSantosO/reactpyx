"""CSS compilation and processing module for ReactPyx"""

import os
import re


def extract_css_from_pyx(pyx_content):
    """Extract CSS from PyX component using style tags"""
    css_pattern = re.compile(r'<style>\s*(.*?)\s*</style>', re.DOTALL)
    css_blocks = css_pattern.findall(pyx_content)
    
    return '\n\n'.join(css_blocks)


def compile_css_modules(component_dir, output_dir):
    """Compile CSS modules from component directory"""
    compiled_css = []
    
    # Collect all CSS files
    for root, _, files in os.walk(component_dir):
        for file in files:
            if file.endswith('.css'):
                css_path = os.path.join(root, file)
                with open(css_path, 'r') as f:
                    css_content = f.read()
                compiled_css.append(f"/* From {os.path.relpath(css_path)} */\n{css_content}")
    
    # Combine into single CSS file
    combined_css = "\n\n".join(compiled_css)
    
    # Ensure output directory exists
    os.makedirs(output_dir, exist_ok=True)
    
    # Write combined CSS
    output_path = os.path.join(output_dir, 'styles.css')
    with open(output_path, 'w') as f:
        f.write(combined_css)
    
    return output_path


def process_tailwind_classes(css_content):
    """
    Transform Tailwind class lists into processable format
    Note: This is a placeholder. Real implementation would use a proper
    Tailwind processor
    """
    return css_content


def integrate_framework_styles(framework, output_dir):
    """
    Integrate framework styles into the project
    This would normally compile framework-specific CSS
    """
    os.makedirs(output_dir, exist_ok=True)
    
    if framework == "tailwind":
        # In a real implementation, this would run tailwind CLI
        # For now, we'll just create a placeholder file
        with open(os.path.join(output_dir, 'tailwind.css'), 'w') as f:
            f.write('/* Tailwind styles would be generated here */')
            
    elif framework == "bootstrap":
        # For bootstrap, we'd normally copy the CSS file
        # For now, we'll just create a placeholder file
        with open(os.path.join(output_dir, 'bootstrap.css'), 'w') as f:
            f.write('/* Bootstrap styles would be included here */')
    
    return os.path.join(output_dir, f"{framework}.css")
