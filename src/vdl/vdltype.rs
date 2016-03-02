pub enum Kind {
    // Variant kinds
	Any,      // any type
	Optional,             // value might not exist
	// Scalar kinds
	Bool,       // boolean
	Byte,       // 8 bit unsigned integer
	Uint16,     // 16 bit unsigned integer
	Uint32,     // 32 bit unsigned integer
	Uint64,     // 64 bit unsigned integer
	Int8,       // 8 bit signed integer
	Int16,      // 16 bit signed integer
	Int32,      // 32 bit signed integer
	Int64,      // 64 bit signed integer
	Float32,    // 32 bit IEEE 754 floating point
	Float64,    // 64 bit IEEE 754 floating point
	Complex64,  // {real,imag} each 32 bit IEEE 754 floating point
	Complex128, // {real,imag} each 64 bit IEEE 754 floating point
	String,     // unicode string (encoded as UTF-8 in memory)
	Enum,       // one of a set of labels
	TypeObject, // type represented as a value
	// Composite kinds
	Array,  // fixed-length ordered sequence of elements
	List,   // variable-length ordered sequence of elements
	Set,    // unordered collection of distinct keys
	Map,    // unordered association between distinct keys and values
	Struct, // conjunction of an ordered sequence of (name,type) fields
	Union,  // disjunction of an ordered sequence of (name,type) fields
}

pub struct Type {
    kind: Kind,
    name: *const str,
	labels:       * const[*const str],    // used by Enum
	len:          usize,         // used by Array
	elem:         *mut Type,       // used by Optional, Array, List, Map
	key:          *mut Type,       // used by Set, Map
	fields:       *const [Field],     // used by Struct, Union
}

pub struct Field {
    name: *const str,
    t: *mut Type,
}
