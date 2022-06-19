pub fn align_up(addr: usize, align: usize) -> usize {
    let reminder = addr % align;
    if reminder == 0 {
        addr
    } else {
        addr - reminder + align
    }
}
