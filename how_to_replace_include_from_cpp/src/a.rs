// Mark Stankus (c) Wed Aug  3 13:37:54 PDT 2022
// File: a.rs

// The function bar is defined in the file b.rs.
//
// The pub keyword means that code outside of this
// file can call the function.

pub fn ace() { crate::b::bar() }
