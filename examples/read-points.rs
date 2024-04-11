use pdal::Pipeline;
use pdal_sys::core::DimTypeId;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // get filename from args
    let filename = std::env::args().nth(1).expect("missing filename argument");
    let pipeline_json = format!(
        r#"
        {{
            "pipeline": [
                {{
                    "type": "readers.las",
                    "filename": "{filename}"
                }},
                {{
                    "type": "writers.null"
                }}
            ]
        }}
    "#
    );

    let pipeline = Pipeline::new(pipeline_json)?;
    let results = pipeline.execute()?;

    let views = results.point_views()?;
    let view = views.first().ok_or("no point view")?;
    for pid in view.point_ids().take(3) {
        let x = view.point_value(DimTypeId::X, pid)?;
        let y = view.point_value(DimTypeId::Y, pid)?;
        let z = view.point_value(DimTypeId::Z, pid)?;
        println!("{}: ({}, {}, {})", pid, x.to_f64(), y.to_f64(), z.to_f64());
    }

    Ok(())
}
