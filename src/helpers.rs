pub fn grow_capacity(old_capacity: i16) -> i16 {
    if old_capacity < 8 {
        return 8;
    }
    return old_capacity * 2;
}
