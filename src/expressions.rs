use polars::{chunked_array::float, prelude::*};
use pyo3_polars::derive::polars_expr;
use s2::{cellid::CellID, latlng::LatLng};
use serde::Deserialize;

pub struct LatLngCellKwargs {
    lat: f64,
    lon: f64,
    level: Option<u64>,
}

#[derive(Deserialize)]
pub struct AddS2CellKwargs {
    level: Option<u64>,
}

fn lat_lng_to_s2_cell_id(args: LatLngCellKwargs) -> CellID {
    let latlng: LatLng = LatLng::from_degrees(args.lat, args.lon);
    let cell_id: CellID = CellID::from(latlng);
    args.level.map_or(cell_id, |lvl: u64| cell_id.parent(lvl))
}

#[polars_expr(output_type=UInt64)]
fn add_s2_cell(inputs: &[Series], kwargs: AddS2CellKwargs) -> PolarsResult<Series> {
    let lat_series: &ChunkedArray<Float64Type> = inputs[0].f64()?;
    let lon_series: &ChunkedArray<Float64Type> = inputs[1].f64()?;
    let level: Option<u64> = kwargs.level;
    let cell_ids: Series = lat_series
        .into_iter()
        .zip(lon_series)
        .map(|(lat, lon)| match (lat, lon) {
            (Some(lat), Some(lon)) => {
                let args: LatLngCellKwargs = LatLngCellKwargs { lat, lon, level };
                Some(lat_lng_to_s2_cell_id(args).0)
            }
            _ => None,
        })
        .collect::<UInt64Chunked>()
        .into_series();

    Ok(cell_ids.with_name("cell"))
}
