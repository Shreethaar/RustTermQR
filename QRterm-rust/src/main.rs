use image;
use rqrr;

static img = image::open("tests/data/github.gif")?.to_luma();
let mut img = rqrr::PreparedImage::prepare(img);
let grids = img.detect_grids();
assert_eq!(grids.len(), 1);

let (meta, content) = grids[0].decode()?;
assert_eq!(meta.ecc_level, 0);
assert_eq!(content, "https://github.com/WanzenBug/rqrr");

