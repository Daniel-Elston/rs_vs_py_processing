NUM_RUNS = 10

def save_results(results: str, filename: str):
    """Save the given results string to a file."""
    with open(filename, "w") as f:
        f.write(results)

def generate_results(avg_creation, avg_processing, total_runtime) -> str:
    """Format the results output string."""
    result_string = (
        f"\n=== Average Function Times over {NUM_RUNS} Runs ===\n"
        f"Average Creation Time: {avg_creation:.3f} ms\n"
        f"Average Processing Time: {avg_processing:.3f} ms\n"
        f"Total Python main() time: {total_runtime:.3f} ms\n"
    )
    print(result_string)
    return result_string