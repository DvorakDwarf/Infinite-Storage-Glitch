use crate::{args::DislodgeParams, etcher};

pub async fn run_dislodge(args: DislodgeParams) -> anyhow::Result<()> {
    let out_data = etcher::read(&args.in_path.expect("no in path at run_dislodge"), 1)?;
    etcher::write_bytes(
        &args.out_path.expect("no out path at run_dislodge"),
        out_data,
    )?;
    Ok(())
}
