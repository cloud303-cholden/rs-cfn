fn main() {
    let template_str = include_str!("template.yaml");
    let template: cfn::Template = serde_yaml::from_str(template_str).unwrap();
    dbg!(template);
}
