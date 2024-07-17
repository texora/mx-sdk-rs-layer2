#!/bin/bash

## Prerequisites: `sudo npm install -g json-fmt`

DENALI_SCEN_FILES=$(find . -name "*.scen.json")
DENALI_STEP_FILES=$(find . -name "*.step.json")
DENALI_STEPS_FILES=$(find . -name "*.steps.json")
DENALI_ALL_FILES="$DENALI_SCEN_FILES $DENALI_STEP_FILES $DENALI_STEPS_FILES"

TEMP_FILE=denali-fmt-temp.scen.json
for DENALI_FILE in $DENALI_ALL_FILES
do
    echo $DENALI_FILE
    json-fmt $DENALI_FILE --indent "    " --prettify --output $TEMP_FILE || exit 1
    echo >> $TEMP_FILE # adds missing newline
    mv $TEMP_FILE $DENALI_FILE
done
