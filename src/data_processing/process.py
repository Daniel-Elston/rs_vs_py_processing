import polars as pl


def load_frame(path: str):
    return pl.read_csv(path, has_header=True)

def group_by_category(df):
    grouped = df.lazy().group_by("category").agg(
        [pl.col("value").mean().alias("mean_values")]
    ).collect()
    return grouped

def run_process():
    df = load_frame("data/data.csv")
    grouped = group_by_category(df)
    print("Grouped means:")
    print(grouped)
