import sys

from mypy import api

result = api.run(sys.argv[1:])

if result[0]:
    sys.stdout.write(result[0])

if result[1]:
    sys.stderr.write(result[1])

sys.exit(result[2])
