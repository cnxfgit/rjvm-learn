use rjvm_reader::class_file_method::ClassFileMethod;

use crate::class::ClassRef;

#[derive(Debug, Clone)]
pub struct ClassAndMethod<'a> {
    pub class: ClassRef<'a>,
    pub method: &'a ClassFileMethod,
}

impl<'a> ClassAndMethod<'a> {

    pub fn num_arguments(&self) -> usize {
        self.method.parsed_type_descriptor.num_arguments()
    }
    
}
