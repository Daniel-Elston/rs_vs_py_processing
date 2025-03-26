import numpy as np
import polars as pl


def create_frame(num_rows: int):
    df = pl.DataFrame({
        'id': np.arange(1, num_rows + 1),
        'value': np.random.uniform(0, 100, num_rows),
        'category': np.random.choice(['A', 'B', 'C', 'D'], num_rows)
    })
    return df

def save_frame(df, path: str):
    df.write_csv(path)

def run_create():
    df = create_frame(num_rows=100)
    save_frame(df, "data/data.csv")
