# -------------------------------------------------------------
# Por favor no modifiques este código, si no la prueba fallara.
# -------------------------------------------------------------

import sys
import os
import io

# Force UTF-8 encoding for stdout
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

# Add the directory containing this script to the Python path
script_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, script_dir)

# Now import should work
try:
    import caras
except ImportError as e:
    print(f"Import error: {e}")
    sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit(1)

    function_name = sys.argv[1]
    argument = sys.argv[2] if len(sys.argv) > 2 else ""

    try:
        if function_name == "conversion":
            result = caras.conversion(argument)
            print(result, flush=True)
    except Exception as e:
        print(f"Error executing {function_name}: {e}")
        sys.exit(1)