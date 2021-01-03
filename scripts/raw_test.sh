#!/usr/bin/env bash

set -x
set -e

echo "*** PL007 image ok ***"
mkdir -p test_images/pl007/ok
echo "some data" > test_images/pl007/ok/test
cat > test_images/pl007/ok/Dockerfile <<EOF
FROM scratch
COPY test /app/file
USER someuser
ENTRYPOINT echo "test"
EOF
cd test_images/pl007/ok/
docker build -t "pl007ok:latest" -f Dockerfile .
cd ../../..
docker save "pl007ok:latest" -o testPL007ok.tar
cargo run -- -f testPL007ok.tar -o report-testPL007ok.sarif || true
# check result of the rule
RES=$( cat report-testPL007ok.sarif | jq -r '.runs[0].results[] | select(.ruleId | contains("PL007")) | .kind ' )
[ "pass" == "$RES" ]

echo "*** PL007 image vulnerable ***"
mkdir -p test_images/pl007/vulnerable
echo "some data" > test_images/pl007/vulnerable/test
cat > test_images/pl007/vulnerable/Dockerfile <<EOF
FROM scratch
COPY test /app/file
ENTRYPOINT echo "test"
EOF
cd test_images/pl007/vulnerable/
docker build -t "pl007vulnerable:latest" -f Dockerfile .
cd ../../..
docker save "pl007vulnerable:latest" -o testPL007vulnerable.tar
cargo run -- -f testPL007vulnerable.tar -o report-testPL007vulnerable.sarif || true
# check result of the rule
RES=$( cat report-testPL007vulnerable.sarif | jq -r '.runs[0].results[] | select(.ruleId | contains("PL007")) | .kind ' )
[ "fail" == "$RES" ]


echo "*** PL001 image vulnerable ***"
mkdir -p test_images/pl001/vulnerable
echo "some data" > test_images/pl001/vulnerable/test
cat > test_images/pl001/vulnerable/Dockerfile <<EOF
FROM scratch
ENV token=vEryS3cr3t
COPY test /app/file
ENTRYPOINT echo "test"
EOF
cd test_images/pl001/vulnerable/
docker build -t "pl001vulnerable:latest" -f Dockerfile .
cd ../../..
docker save "pl001vulnerable:latest" -o testPL001vulnerable.tar
cargo run -- -f testPL007vulnerable.tar -o report-testPL001vulnerable.sarif || true
# check result of the rule
RES=$( cat report-testPL001vulnerable.sarif | jq -r '.runs[0].results[] | select(.ruleId | contains("PL001")) | .kind ' )
[ "fail" == "$RES" ]
