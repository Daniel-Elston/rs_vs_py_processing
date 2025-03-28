import polars as pl
import pandas as pd


def load_frame(path: str) -> pd.DataFrame:
    return pd.read_csv(path)

def group_by_category(df: pd.DataFrame) -> pd.DataFrame:
    grouped = df.groupby("category", as_index=False)["value"].mean()
    grouped.rename(columns={"value": "mean_values"}, inplace=True)
    return grouped

def run_process():
    df = load_frame("data/data.csv")
    grouped = group_by_category(df)
    # print(f"Grouped means: {grouped}")


# def load_frame(path: str):
#     return pl.read_csv(path, has_header=True)

# def group_by_category(df):
#     grouped = df.lazy().group_by("category").agg(
#         [pl.col("value").mean().alias("mean_values")]
#     ).collect()
#     return grouped

# def run_process():
#     df = load_frame("data/data.csv")
#     grouped = group_by_category(df)
#     print("Grouped means:")
#     print(grouped)
