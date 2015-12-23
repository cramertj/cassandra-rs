use cql_bindgen::cass_tuple_new;
use cql_bindgen::cass_tuple_new_from_data_type;
use cql_bindgen::cass_tuple_free;
use cql_bindgen::cass_tuple_data_type;
use cql_bindgen::cass_tuple_set_null;
use cql_bindgen::cass_tuple_set_int32;
use cql_bindgen::cass_tuple_set_int64;
use cql_bindgen::cass_tuple_set_float;
use cql_bindgen::cass_tuple_set_double;
use cql_bindgen::cass_tuple_set_bool;
use cql_bindgen::cass_tuple_set_string;
use cql_bindgen::cass_tuple_set_bytes;
use cql_bindgen::cass_tuple_set_uuid;
use cql_bindgen::cass_tuple_set_inet;
use cql_bindgen::cass_tuple_set_collection;
use cql_bindgen::cass_tuple_set_user_type;
use cql_bindgen::cass_tuple_set_uint32;
use cql_bindgen::cass_tuple_set_tuple;
use cql_bindgen::cass_tuple_set_int8;
use cql_bindgen::cass_tuple_set_int16;
#[allow(unused_imports)]
use cql_bindgen::cass_tuple_set_decimal;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_value;

use std::ffi::CString;

use cql_ffi::inet::AsInet;
use std::net::SocketAddr;
use cql_bindgen::CassTuple as _Tuple;
use cql_ffi::uuid::Uuid;
use cql_ffi::data_type::DataType;
use cql_ffi::data_type::ConstDataType;
use cql_ffi::error::CassError;
use cql_bindgen::CassIterator as _CassIterator;
use cql_ffi::value::Value;
use cql_ffi::user_type::UserType;
use cql_ffi::collection::Set;

pub struct Tuple(pub *mut _Tuple);
pub struct TupleIterator(pub *mut _CassIterator);

impl Drop for TupleIterator {
    fn drop(&mut self) { unsafe { cass_iterator_free(self.0) } }
}

impl Iterator for TupleIterator {
    type Item = Value;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(self.get_value()),
            }
        }
    }
}

impl TupleIterator {
    pub fn get_value(&mut self) -> Value { unsafe { Value::new(cass_iterator_get_value(self.0)) } }
}


impl Tuple {
    ///Creates a new tuple.
    pub fn new(item_count: u64) -> Self { unsafe { Tuple(cass_tuple_new(item_count)) } }

    ///Creates a new tuple from an existing data type.
    pub fn new_from_data_type(data_type: DataType) -> Tuple {
        unsafe { Tuple(cass_tuple_new_from_data_type(data_type.0)) }
    }

    ///Gets the data type of a tuple.
    pub fn data_type(&mut self) -> ConstDataType { unsafe { ConstDataType(cass_tuple_data_type(self.0)) } }

    ///Sets an null in a tuple at the specified index.
    pub fn set_null(&mut self, index: u64) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_null(self.0, index)).wrap(()) }
    }

    ///Sets a "tinyint" in a tuple at the specified index.
    pub fn set_int8(&mut self, index: u64, value: i8) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_int8(self.0, index, value)).wrap(()) }
    }

    ///Sets an "smallint" in a tuple at the specified index.
    pub fn set_int16(&mut self, index: u64, value: i16) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_int16(self.0, index, value)).wrap(()) }
    }

    ///Sets an "int" in a tuple at the specified index.
    pub fn set_int32(&mut self, index: u64, value: i32) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_int32(self.0, index, value)).wrap(()) }
    }

    ///Sets a "date" in a tuple at the specified index.
    pub fn set_uint32(&mut self, index: u64, value: u32) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_uint32(self.0, index, value)).wrap(()) }
    }

    ///Sets a "bigint", "counter", "timestamp" or "time" in a tuple at the
    ///specified index.
    pub fn set_int64(&mut self, index: u64, value: i64) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_int64(self.0, index, value)).wrap(()) }
    }

    ///Sets a "float" in a tuple at the specified index.
    pub fn set_float(&mut self, index: u64, value: f32) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_float(self.0, index, value)).wrap(()) }
    }

    ///Sets a "double" in a tuple at the specified index.
    pub fn set_double(&mut self, index: u64, value: f64) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_double(self.0, index, value)).wrap(()) }
    }

    ///Sets a "boolean" in a tuple at the specified index.
    pub fn set_bool(&mut self, index: u64, value: bool) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_bool(self.0, index, if value { 1 } else { 0 })).wrap(()) }
    }

    ///Sets an "ascii", "text" or "varchar" in a tuple at the specified index.
    pub fn set_string<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<String>
    {
        unsafe {
            let value = CString::new(value.into()).unwrap();
            CassError::build(cass_tuple_set_string(self.0, index, value.as_ptr())).wrap(())
        }
    }

    ///Sets a "blob", "varint" or "custom" in a tuple at the specified index.
    pub fn set_bytes(&mut self, index: u64, value: Vec<u8>) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_bytes(self.0, index, value.as_ptr(), value.len() as u64)).wrap(()) }
    }

    ///Sets a "uuid" or "timeuuid" in a tuple at the specified index.
    pub fn set_uuid<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<Uuid>
    {
        unsafe { CassError::build(cass_tuple_set_uuid(self.0, index, value.into().0)).wrap(()) }
    }

    ///Sets an "inet" in a tuple at the specified index.
    pub fn set_inet(&mut self, index: u64, value: SocketAddr) -> Result<(), CassError> {
        let inet = AsInet::as_cass_inet(&value);
        unsafe { CassError::build(cass_tuple_set_inet(self.0, index, inet.0)).wrap(()) }
    }

    ///Sets a "list", "map" or "set" in a tuple at the specified index.
    pub fn set_collection<S>(&mut self, index: u64, value: S) -> Result<(), CassError>
        where S: Into<Set>
    {
        unsafe { CassError::build(cass_tuple_set_collection(self.0, index, value.into().0)).wrap(()) }
    }

    ///Sets a "tuple" in a tuple at the specified index.
    pub fn set_tuple(&mut self, index: u64, value: Tuple) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_tuple(self.0, index, value.0)).wrap(()) }
    }

    ///Sets a "udt" in a tuple at the specified index.
    pub fn set_user_type(&mut self, index: u64, value: UserType) -> Result<(), CassError> {
        unsafe { CassError::build(cass_tuple_set_user_type(self.0, index, value.0)).wrap(()) }
    }
}

impl Drop for Tuple {
    fn drop(&mut self) { unsafe { cass_tuple_free(self.0) } }
}
