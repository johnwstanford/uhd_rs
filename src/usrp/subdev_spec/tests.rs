use crate::usrp::subdev_spec::SubdevSpec;

#[test]
fn subdev_spec_basic() -> Result<(), &'static str> {

    let mut s = SubdevSpec::new("A:0")?;
    assert_eq!(1, s.len()?);

    Ok(())
}