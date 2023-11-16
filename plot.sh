#!/bin/bash

ID=$(ls log | tail -n 1)

if [ ! -z "$1" ]
    then
    ID=$1
fi

LOG=log/$ID

# Create python virtual environment and locally install required packages
if [[ ! -d plot/venv ]]
then
    echo "Creating python virtual environment"
    python3 -m venv plot/venv
    source plot/venv/bin/activate
    pip3 install --upgrade pip
    pip3 install -r plot/requirements.txt
fi

# Activate virtual environment
source plot/venv/bin/activate

# python3 plot/plot_parametrized.py $LOG/log_seq.json $LOG/log_cpp.json $LOG/log_rust.json --titles seq,cpp,rust -o plot/parametrized.png
python3 plot/plot_parametrized.py $LOG/log_rust.json --titles heft -o plot/parametrized.png

python3 plot/plot_speedup.py $LOG/log_rust.json --titles heft -o plot/speedup.png