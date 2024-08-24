if [ -d "env" ]; then
    echo "env folder already exists."
else
    echo "env folder does not exist. generating..."
    python -m venv env
fi

echo "activating virtual environment..."
source env/bin/activate

echo "installing requirements..."
pip install -r $(pwd)/requirments.txt

echo "running the script..."
python $(pwd)/main.py