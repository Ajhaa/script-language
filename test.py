import subprocess


def run_and_compare(file, res_should):
    res_should.append('')
    result = subprocess.run(['cargo', 'run', './scripts/' + file + '.script'], stdout=subprocess.PIPE, encoding="UTF8", stderr=subprocess.PIPE)
    split = str(result.stdout).split('\n')
    try:
        assert split == res_should
        return True
    except:
        print('%s test failed: %r != %r' % (file, split, res_should))
        return False


tests = [
    ['exp', ['4294967296', '256']],
    ['fibonacci', ['55']],
    ['counter', ['-3', '201']],
    ['object', ['o1', 'o2', 'o3', 'o2']]
]

successes = 0
for test in tests:
    if run_and_compare(test[0], test[1]):
        successes += 1

print("\n%d / %d tests succeeded" % (successes, len(tests)))