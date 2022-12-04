def read_file_to_lines(file_name):
    with open(file_name, "r") as fh:
        return list(line.strip() for line in fh.readlines())
