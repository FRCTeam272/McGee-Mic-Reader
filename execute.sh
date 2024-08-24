echo "entering dir... $0"
cd "$(dirname "$0")"

if [ -d "env" ]; then
    echo "env folder is present"
else
    echo "env folder is not present"
    echo "generating env folder..."
    python -m venv env
fi

echo "activating env..."
source env/bin/activate

echo "Checking internet connectivity..."
if ping -q -c 1 -W 1 google.com >/dev/null; then
    echo "Internet is available."
    echo "installing requirments..."
    pip install -r requirments.txt
else
    echo "Internet is not available."
fi

echo "running main.py..."
python main.py