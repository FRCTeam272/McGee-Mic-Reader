import yaml

with open("config.yaml", 'r') as config_file:
    config = yaml.safe_load(config_file)

metawear_address = config['metawear_address']
