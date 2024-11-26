#!/usr/bin/env python3

import sys

# Parse the file into dictionary
def parse_file(file_path):
    data = {}
    try:
        with open(file_path, 'r') as file:
            for line in file:
                try:
                    id_str, group_id_str = line.strip().split()
                    id, groud_id = int(id_str), int(group_id_str)
                    data[id] = group_id_str
                except ValueError:
                    raise ValueError(f"Invalid line format in file {file_path}: '{line.strip()}'")
    except FileNotFoundError:
        raise FileNotFoundError(f"File not found: {file_path}")
    return data

# Check the correctness between the two files
def check_correctness_wcc(file1_path, file2_path, mode):
    file1_data = parse_file(file1_path)
    file2_data = parse_file(file2_path)

    if set(file1_data.keys()) != set(file2_data.keys()):
        raise ValueError("The two files do not have the same ids")
    
    # Group ids by group_id in both files
    def group_by_group_id(data):
        grouped = {}
        for id, group_id in data.items():
            if group_id not in grouped:
                grouped[group_id] = set()
            grouped[group_id].add(id)
        return grouped

    file1_groups = group_by_group_id(file1_data)
    file2_groups = group_by_group_id(file2_data)

    # Transform group mappings into sets of id sets
    file1_id_sets = set(frozenset(group) for group in file1_groups.values())
    file2_id_sets = set(frozenset(group) for group in file2_groups.values())

    # Check correctness by comparing id sets
    return file1_id_sets == file2_id_sets

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Incorrect useage, please pass expected_file_path, input_file_path, and algo_mode")
        sys.exit(1)

    file1 = sys.argv[1]
    file2 = sys.argv[2]
    mode = sys.argv[3]  # Placeholder for future implementation, currently only tests wcc

    try:
        result = check_correctness_wcc(file1, file2, mode)
        if result:
            print("The files are correct.")
        else:
            print("The files are incorrect.")
    except Exception as e:
        print(f"Error: {e}")