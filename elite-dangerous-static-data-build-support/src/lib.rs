pub mod types;
pub mod build;

pub trait CodeGeneratorName {

    fn name_pascal_case(&self) -> String;
    fn name_snake_case(&self) -> String;
}