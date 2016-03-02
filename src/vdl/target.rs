use std::io;
use vdl;

pub trait Target {
	// FromBool converts from the src bool to the target, where tt represents the
	// concrete type of bool.
	fn from_bool(&self, src: bool, tt: *mut vdl::Type) -> Option<io::Error>;
	// FromUint converts from the src uint to the target, where tt represents the
	// concrete type of uint.
	fn from_uint(&self, src: u64, tt: *mut vdl::Type) -> Option<io::Error>;
	// FromInt converts from the src int to the target, where tt represents the
	// concrete type of int.
	fn from_int(&self, src: i64, tt: *mut vdl::Type) -> Option<io::Error>;
	// FromFloat converts from the src float to the target, where tt represents
	// the concrete type of float.
	fn from_float(&self, src: f64, tt: *mut vdl::Type) -> Option<io::Error>;
	// FromComplex converts from the src complex to the target, where tt
	// represents the concrete type of complex.
//	fn from_complex(&self, src: complex128, tt: *mut vdl::Type) -> Option<io::Error>;
	// FromBytes converts from the src bytes to the target, where tt represents
	// the concrete type of bytes.
	fn from_bytes(&self, src: &[u8], tt: *mut vdl::Type) -> Option<io::Error>;
	// FromString converts from the src string to the target, where tt represents
	// the concrete type of string.
	fn from_string(&self, src: &str, tt: *mut vdl::Type) -> Option<io::Error>;
	// FromEnumLabel converts from the src enum label to the target, where tt
	// represents the concrete type of enum.
	fn from_enum_label(&self, src: &str, tt: *mut vdl::Type) -> Option<io::Error>;
	// FromTypeObject converts from the src type to the target.
	fn from_type_object(&self, src: *mut vdl::Type) -> Option<io::Error>;
	// FromNil converts from a nil (nonexistent) value of type tt, where tt must
	// be of kind Optional or Any.
	fn from_nil(&self, tt: *mut vdl::Type) -> Option<io::Error>;

	// StartList prepares conversion from a list or array of type tt, with the
	// given len.  FinishList must be called to finish the list.
	fn start_list(&self, tt: *mut vdl::Type, len: usize) -> Result<ListTarget, io::Error>;
	// FinishList finishes a prior StartList call.
	fn finish_list(&self, x: ListTarget) -> Option<io::Error>;

	// StartSet prepares conversion from a set of type tt, with the given len.
	// FinishSet must be called to finish the set.
	fn start_set(&self, tt: *mut vdl::Type, len: usize) -> Result<SetTarget, io::Error>;
	// FinishSet finishes a prior StartSet call.
	fn finish_set(&self, x: SetTarget) -> Option<io::Error>;

	// StartMap prepares conversion from a map of type tt, with the given len.
	// FinishMap must be called to finish the map.
	fn start_map(&self, tt: *mut vdl::Type, len: usize) -> Result<MapTarget, io::Error>;
	// FinishMap finishes a prior StartMap call.
	fn finish_map(&self, x: MapTarget) -> Option<io::Error>;

	// StartFields prepares conversion from a struct or union of type tt.
	// FinishFields must be called to finish the fields.
	fn start_fields(&self, tt: *mut vdl::Type) -> Result<FieldsTarget, io::Error>;
	// FinishFields finishes a prior StartFields call.
	fn finish_fields(&self, x: FieldsTarget) -> Option<io::Error>;
}

// ListTarget represents conversion from a list or array.
pub trait ListTarget {
	// StartElem prepares conversion of the next list elem.  The given index must
	// start at 0, and be incremented by one by each successive StartElem call.
	// FinishElem must be called to finish the elem.
	//
	// TODO(toddw): Remove index?
	fn start_elem(&self, index: usize) -> Result<Target, io::Error>;
	// FinishElem finishes a prior StartElem call.
	fn finish_elem(&self, elem: Target) -> Option<io::Error>;
}

// SetTarget represents conversion from a set.
pub trait SetTarget {
	// StartKey prepares conversion of the next set key.  FinishKey must be called
	// to finish the key.
	fn start_key(&self, ) -> Result<Target, io::Error>;
	// FinishKey finishes a prior StartKey call.  ErrFieldNoExist indicates the
	// key doesn't exist on the target.
	fn finish_key(&self, key: Target) ->Option<io::Error>;
}

// MapTarget represents conversion from a map.
pub trait MapTarget {
	// StartKey prepares conversion of the next map key.  FinishKeyStartField must
	// be called to finish the key.
	fn start_key(&self, ) -> Result<Target, io::Error>;
	// FinishKeyStartField finishes a prior StartKey call, and starts the
	// associated field.  ErrFieldNoExist indicates the key doesn't exist on the
	// target.
	fn finish_key_start_field(&self, key: Target) -> Result<Target, io::Error>;
	// FinishField finishes a prior FinishKeyStartField call.
	fn finish_field(&self, key: Target, field: Target) -> Option<io::Error>;
}

// FieldsTarget represents conversion from struct or union fields.
pub trait FieldsTarget {
	// StartField prepares conversion of the field with the given name.
	// FinishField must be called to finish the field.  ErrFieldNoExist indicates
	// the field name doesn't exist on the target.
	fn start_field(&self, name: &str) -> Result<(Target, Target), io::Error>;
	// FinishField finishes a prior StartField call.
	fn finish_field(&self, key: Target, field: Target) -> Option<io::Error>;
}