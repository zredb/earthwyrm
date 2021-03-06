// one_tile.rs
//
// Copyright (c) 2019-2020  Minnesota Department of Transportation
//
use earthwyrm::{TileId, Wyrm, WyrmCfg};
use postgres::{self, Client, NoTls};
use std::env;
use std::fs::File;

const MUON: &str = &r#"
bind_address:
document_root:
db_conn_string: postgres://Meteodyn:Meteodyn@localhost:2345/earthwyrm
tile_extent: 256
edge_extent: 6
query_limit: 500000
table: polygon
  db_table: planet_osm_polygon
  id_column: osm_id
  geom_column: way
  geom_type: polygon
layer_group: tile
  layer: city
    table: polygon
    zoom: 1+
    tags: boundary=administrative admin_level=8 ?population
"#;

fn write_tile(
    x: u32,
    y: u32,
    z: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let wyrm_cfg: WyrmCfg = muon_rs::from_str(MUON)?;
    let wyrm = Wyrm::from_cfg(&wyrm_cfg)?;
    // let username = whoami::username();
    // let uds = format!("postgres://{:}@%2Frun%2Fpostgresql/earthwyrm", username);
    //let uds = wyrm_cfg.db_conn_string.parse()?;
    let mut file = File::create("./one_tile.mvt")?;
    let mut conn = Client::connect(&wyrm_cfg.db_conn_string, NoTls)?;
    let tid = TileId::new(x, y, z)?;
    wyrm.fetch_tile(&mut file, &mut conn, "tile", tid)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    args.next().unwrap();
    let x = args.next().expect("missing x").parse()?;
    let y = args.next().expect("missing y").parse()?;
    let z = args.next().expect("missing z").parse()?;
    write_tile(x, y, z)?;
    Ok(())
}
