use std::any::Any;

#[repr(C)]
union DataUnion {
    s_num: Box<i128>,
    u_num: Box<u128>,
    str: Box<string>,
    list: Box<Vec<string>>,
}


enum AcceptedDataTypeEnum {

}

struct DatabaseValue {
    data: DataUnion,
    data_type: AcceptedDataTypeEnum,
}