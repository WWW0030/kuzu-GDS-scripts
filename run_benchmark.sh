#!/bin/bash

# GDS Algorithm to test, currently only able to test for wcc
# Change this field to the algorithm you wish to test
GDS_ALGO="weakly_connected_component"

# Change this to path to the node and edge csvs
NODE_FILE="/home/www0030/Testing/datagen8/datagen-8_0-fbv.csv"
EDGE_FILE="/home/www0030/Testing/datagen8/datagen-8_0-fbe.csv"

# Change this to the expected file's path
EXPECTED_FILE="./datagen-8_0-fb-WCC"

# Temp variables & files
OUTPUT_FILE="./temp.cypher"
OUTPUT_DATABSE="./temp"

# Result csv's name
RESULT_FILE="result.csv"


### Generate cypher file to pass into kuzu
echo "CREATE NODE TABLE Node(id INT64 PRIMARY KEY);" >> "$OUTPUT_FILE"
echo "CREATE REL TABLE Edge(FROM Node TO Node, weight FLOAT);" >> "$OUTPUT_FILE"
echo "COPY Node FROM \"$NODE_FILE\";" >> "$OUTPUT_FILE"
echo "COPY Edge FROM \"$EDGE_FILE\" (delimiter=' ');" >> "$OUTPUT_FILE"
# Do case switching for different algorithms
if [[ "$GDS_ALGO" == "weakly_connected_component" ]]; then
    echo "COPY (PROJECT GRAPH PK (Node, Edge) CALL $GDS_ALGO(PK) RETURN _node.id, group_id) TO \"$RESULT_FILE\"(delimiter=' ');" >> "$OUTPUT_FILE"
else
    echo "Unimplemented return for $GDS_ALGO testing"
fi

# Call kuzu with cypher file, result csv is generated
kuzu ./$OUTPUT_DATABSE < $OUTPUT_FILE

# Removes temp files
rm -r $OUTPUT_DATABSE
rm $OUTPUT_FILE
rm history.txt

# Run python script to check for correctness
python3 run_correctness.py $RESULT_FILE $EXPECTED_FILE $GDS_ALGO

