# src/components/Header.pyx

def Header(props):
    title = props.get('title', 'Default Title')
    subtitle = props.get('subtitle', '')

    return (
        <header className="header">
            <h1>{title}</h1>
            {subtitle and <p>{subtitle}</p>}
            {/* Scoped styles for the header */}
            <style>
              .header {{ margin-bottom: 2rem; padding-bottom: 1rem; border-bottom: 1px solid #e2e8f0; }}
              .header h1 {{ margin: 0 0 0.25rem 0; color: #2d3748; font-size: 2em; }}
              .header p {{ margin: 0; color: #718096; font-size: 1.1em; }}
            </style>
        </header>
    )
