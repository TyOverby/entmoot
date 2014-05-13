struct Entity {
    id: uint,
    c_offsets: Vec<uint>,
}

impl Entity {
    fn new(id: uint, prealloc: uint) -> Entity {
        let ret = Entity {id: id, c_offsets: Vec::new()};
        ret.c_offsets.reserve(prealloc);
        ret
    }
}
