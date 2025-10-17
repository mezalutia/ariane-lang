# ArianeVM Specification

The Ariane Virtual Machine (ArianeVM) reads bytecode from files with the extension `.arianeb`.

## `.arianeb` File Format
A binary file containing the following
```
.arianeb {
    u64 magic_number;

    u16 constant_pool_count;
    constant_info constant_pool[];
    
    u16 functions_count;
    function_info functions[]
}
```
```
constant_info {
    u8 tag;
    u8 data[];
}
```
```
function_info {
    u16 name; //index into constant pool
    
    u16 arity;
    type_info arg_types[];
    
    u16 local_count;
    type_info local_types[];
    
    u32 bytecode_count;
    u8 bytecode[]
}
```
```
type_info {
    u8 type;
}
```