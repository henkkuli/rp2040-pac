impl<REG: Readable + Writable<Ux = u32> + crate::markers::AtomicMarker> Reg<REG> {
    /// Set high every bit in the register that was set in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn set_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut REG::Writer,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: Default::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        let alias = (self.register.as_ptr() as usize + 0x2000) as *mut u32;
        alias.write_volatile(bits);
    }

    /// Clear every bit in the register that was cleared in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn clear_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut REG::Writer,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: !REG::Ux::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        let alias = (self.register.as_ptr() as usize + 0x3000) as *mut u32;
        alias.write_volatile(bits);
    }

    /// Toggle every bit in the register that was set in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn toggle_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut REG::Writer,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: Default::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        let alias = (self.register.as_ptr() as usize + 0x1000) as *mut u32;
        alias.write_volatile(bits);
    }
}
