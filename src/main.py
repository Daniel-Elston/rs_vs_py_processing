import time

def tester():
    print('Hello, World!')


if __name__ == '__main__':
    start_t = time.time()
    tester()
    end_t = time.time()
    print(end_t - start_t)