use crate::usrp::subdev_spec::SubdevSpec;

#[test]
fn subdev_spec_basic() -> Result<(), &'static str> {

    let mut s = SubdevSpec::new("A0")?;
    assert_eq!(1, s.len()?);

    s.push_back("A1")?;
    assert_eq!(2, s.len()?);

    Ok(())
}