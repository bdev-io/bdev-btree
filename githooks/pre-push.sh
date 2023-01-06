#!/bin/sh

#
# Coded by: KuuwangE < root@ql.gl > 
#

PROJECT_DIR=$(dirname `dirname $(`dirname $(realpath $0)`)`)
# HACK : This is Hacky Way to Get Project ABSOLUTE PATH Based On CURRENT FILE

# $PROJECT_DIR/bin/test_script.sh

$PROJECT_DIR/bin/test_script.sh # This is the way to run script from another directory
