fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = vergen_gix::BuildBuilder::all_build()?;
    vergen_gix::Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&vergen_gix::GixBuilder::all_git()?)?
        .emit()?;
    Ok(())
}
