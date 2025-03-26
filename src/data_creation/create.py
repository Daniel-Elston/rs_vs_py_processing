import pandas as pd
import numpy as np


def run(num_rows: int):
    # Define the number of rows
    num_rows = 10_000_000

    # Create the DataFrame
    df = pd.DataFrame({
        'id': np.arange(1, num_rows + 1),
        'value': np.random.uniform(0, 100, num_rows),
        'category': np.random.choice(['A', 'B', 'C', 'D'], num_rows)
    })
    return df