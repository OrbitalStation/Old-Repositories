from os import listdir
from os.path import isdir


# A raw_code estimate of the work done
def count_lines():
    def in_dir(name):
        total = 0
        for f in listdir(name):
            fn = name + '/' + f
            if isdir(fn):
                total += in_dir(fn)
                continue
            if not fn.endswith('.rs'):
                continue
            with open(fn, 'r') as file:
                for count, _ in enumerate(file):
                    pass
                total += count + 1
        return total
    print('line count =', in_dir('oko-proc-macro') + in_dir('src'))


if __name__ == '__main__':
    count_lines()
