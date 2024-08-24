if [ -d "$(pwd)env" ]; then
    echo "env folder already exists."
else
    echo "env folder does not exist. generating..."
    python -m venv $(pwd)env
fi

echo "activating virtual environment..."
source env/bin/activate

echo "checking internet access..."
if ping -q -c 1 -W 1 8.8.8.8 >/dev/null; then
    echo "Internet access available."
    echo "installing requirements..."
    pip install -r $(pwd)/requirments.txt
else
    echo "No internet access."
    echo "ignoring requirments.txt"
fi


sudo apt-get remove libportaudio2
sudo apt-get install libasound2-dev
git clone -b alsapatch https://github.com/gglockner/portaudio
cd portaudio
./configure && make
sudo make install
sudo ldconfig
cd ..