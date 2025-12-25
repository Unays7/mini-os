use x86_64::{VirtAddr, structures::paging::PageTable};

/// # Safety
pub unsafe fn active_lvl_4_pt(offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (frame, _) = Cr3::read();
    let phys = frame.start_address();
    let virt = offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    unsafe { &mut *page_table_ptr }
}

/// # Safety
pub unsafe fn next_lvl(pt: &PageTable, offset: VirtAddr) -> Option<&'static mut PageTable> {
    if !pt.is_empty() {
        let level_entry = &pt[0];
        if level_entry.is_unused() {
            return None;
        }

        let lower_lvl_frame = level_entry.frame().unwrap();
        let lower_lvl_phys = lower_lvl_frame.start_address();
        let lower_lvl_virt = offset + lower_lvl_phys.as_u64();
        let page_table_ptr: *mut PageTable = lower_lvl_virt.as_mut_ptr();

        return Some(unsafe { &mut *page_table_ptr });
    }
    None
}
