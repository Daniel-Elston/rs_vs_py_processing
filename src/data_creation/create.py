import numpy as np
import polars as pl
import pyarrow.parquet as pq
import pandas as pd

import numpy as np
import pandas as pd


### Pandas version
# def create_frame(num_rows: int):
#     df = pd.DataFrame({
#         'id': np.arange(1, num_rows + 1),
#         'value': np.random.uniform(0, 100, num_rows),
#         'category': np.random.choice(['A', 'B', 'C', 'D'], num_rows)
#     })
#     return df

# def save_csv(df: pd.DataFrame, path: str):
#     df.to_csv(path, index=False)

# def save_parquet(df: pd.DataFrame, path: str):
#     df.to_parquet(path, index=False)

# def run_create():
#     df = create_frame(num_rows=1_000_000)
#     save_csv(df, "data/data.csv")
#     save_parquet(df, "data/data.parquet")


### Polars version
def create_frame(num_rows: int):
    df = pl.DataFrame({
        'id': np.arange(1, num_rows + 1),
        'value': np.random.uniform(0, 100, num_rows),
        'category': np.random.choice(['A', 'B', 'C', 'D'], num_rows)
    })
    return df

def save_csv(df, path: str):
    df.write_csv(path)

def save_parquet(df, path: str):
    pq.write_table(df.to_arrow(), path)

def run_create():
    df = create_frame(num_rows=1_000_000)
    # save_csv(df, "data/data.csv")
    save_parquet(df, "data/data.parquet")
