""src/css_helper.py - Utilidades para manejar CSS en ReactPyx"""

def combine_classes(*args):
    """
    Combina múltiples strings de clases CSS, ignorando valores None o vacíos.
    """
    return " ".join(filter(None, args))

def use_styles(styles_dict):
    """
    Prepara un diccionario de estilos para ser usado como prop 'style'.
    """
    return styles_dict # Devuelve el diccionario tal cual por ahora
