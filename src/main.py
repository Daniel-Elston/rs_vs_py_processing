import time
from src.data_creation.create import run_create
from src.data_processing.process import run_process


def main():
    creation_t_store = []
    process_t_store = []
    
    for i in range(10):
        print(f"\n--- Run {i+1} ---")
        
        start_create = time.perf_counter()
        run_create()
        end_create = time.perf_counter()
        creation_duration = (end_create - start_create)*1000
        
        start_process = time.perf_counter()
        run_process()
        end_process = time.perf_counter()
        process_duration = (end_process - start_process)*1000
        
        creation_t_store.append(creation_duration)
        process_t_store.append(process_duration)
    
    avg_creation = sum(creation_t_store) / len(creation_t_store)
    avg_processing = sum(process_t_store) / len(process_t_store)

    print("\n=== Average Execution Times over 10 Runs ===")
    print(f"Average Creation Time: {avg_creation:.3f} ms")
    print(f"Average Processing Time: {avg_processing:.3f} ms")


if __name__ == '__main__':
    main()