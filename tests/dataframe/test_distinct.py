from __future__ import annotations

import pyarrow as pa
import pytest

from daft.datatype import DataType
from tests.utils import sort_arrow_table


@pytest.mark.parametrize("repartition_nparts", [1, 2, 5])
def test_distinct_with_nulls(make_df, repartition_nparts, with_morsel_size):
    daft_df = make_df(
        {
            "id": [1, None, None, None],
            "values": ["a1", "b1", "b1", "c1"],
        },
        repartition=repartition_nparts,
    )
    result_df = daft_df.distinct()

    expected = {
        "id": [1, None, None],
        "values": ["a1", "b1", "c1"],
    }
    assert sort_arrow_table(pa.Table.from_pydict(result_df.to_pydict()), "values") == sort_arrow_table(
        pa.Table.from_pydict(expected), "values"
    )

    # Test unique alias.
    result_df = daft_df.unique()
    assert sort_arrow_table(pa.Table.from_pydict(result_df.to_pydict()), "values") == sort_arrow_table(
        pa.Table.from_pydict(expected), "values"
    )


@pytest.mark.parametrize("repartition_nparts", [1, 2, 5])
def test_distinct_with_all_nulls(make_df, repartition_nparts, with_morsel_size):
    daft_df = make_df(
        {
            "id": [None, None, None, None],
            "values": ["a1", "b1", "b1", "c1"],
        },
        repartition=repartition_nparts,
    )
    result_df = daft_df.select(daft_df["id"].cast(DataType.int64()), daft_df["values"]).distinct()

    expected = {
        "id": [None, None, None],
        "values": ["a1", "b1", "c1"],
    }
    assert sort_arrow_table(pa.Table.from_pydict(result_df.to_pydict()), "values") == sort_arrow_table(
        pa.Table.from_pydict(expected), "values"
    )

    # Test unique alias.
    result_df = daft_df.select(daft_df["id"].cast(DataType.int64()), daft_df["values"]).unique()
    assert sort_arrow_table(pa.Table.from_pydict(result_df.to_pydict()), "values") == sort_arrow_table(
        pa.Table.from_pydict(expected), "values"
    )


@pytest.mark.parametrize("repartition_nparts", [1, 2])
def test_distinct_with_empty(make_df, repartition_nparts, with_morsel_size):
    daft_df = make_df(
        {
            "id": [1],
            "values": ["a1"],
        },
        repartition=repartition_nparts,
    )
    result_df = daft_df.where(daft_df["id"] != 1).distinct()
    result_df.collect()

    resultset = result_df.to_pydict()
    assert len(resultset["id"]) == 0
    assert len(resultset["values"]) == 0

    # Test unique alias.
    result_df = daft_df.where(daft_df["id"] != 1).unique()
    result_df.collect()
    resultset = result_df.to_pydict()
    assert len(resultset["id"]) == 0
    assert len(resultset["values"]) == 0
