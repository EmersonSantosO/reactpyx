import sys
import os

sys.path.append(os.path.abspath("python"))
try:
    import reactpyx

    print("Success")
except Exception as e:
    print(f"Error: {e}")
    import traceback

    traceback.print_exc()
