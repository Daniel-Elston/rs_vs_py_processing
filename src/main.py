import time
from src.data_creation.create import run_create
from src.data_processing.process import run_process
from src.utils.pipe_utils import generate_results, save_results

NUM_RUNS = 10


def main():
    creation_t_store = []
    process_t_store = []
    
    for i in range(NUM_RUNS):
        print(f"\n--- Run {i+1} ---")
        
        start_create = time.time()
        run_create()
        end_create = time.time()
        creation_duration = (end_create - start_create)*1000
        
        start_process = time.time()
        run_process()
        end_process = time.time()
        process_duration = (end_process - start_process)*1000
        
        creation_t_store.append(creation_duration)
        process_t_store.append(process_duration)
    
    avg_creation = sum(creation_t_store) / len(creation_t_store)
    avg_processing = sum(process_t_store) / len(process_t_store)
    return avg_creation, avg_processing


if __name__ == '__main__':
    start = time.time()
    avg_creation, avg_processing = main()
    end = time.time()
    total_runtime = (end - start) * 1000

    results_str = generate_results(avg_creation, avg_processing, total_runtime)
    save_results(results_str, "results/python_results.txt")
