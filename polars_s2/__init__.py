import polars as pl
from polars.type_aliases import IntoExpr
from polars.utils.udfs import _get_shared_lib_location

# boilerplate needed to inform polars of the location of binary wheel.
lib = _get_shared_lib_location(__file__)

@pl.api.register_expr_namespace("s2")
class S2Cell:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def add_s2_cell(self, lon: pl.Expr, level: int = 30) -> pl.Expr:
        return self._expr._register_plugin(
            lib=lib,
            args=[lon],
            kwargs={"level": level},
            symbol="add_s2_cell",
            is_elementwise=True,
        )