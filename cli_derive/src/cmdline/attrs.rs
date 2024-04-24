use syn::{Result, *};

use crate::err::*;
// use crate::utils::*;

use super::flag::*;


pub (super) enum FieldAttr {
    Command,
    Flag(FlagMods),
}

enum AttrKind {
    Command,
    Flag,
}

pub (super) fn parse_attr(attr: Attribute) -> Result<Option<FieldAttr>> {
    let kind = match get_attr_kind(&attr)? {
        Some(kind) => kind,
        None => return Ok(None),
    };
    let attr = match kind {
        AttrKind::Command => FieldAttr::Command,
        AttrKind::Flag => {
            let mods = FlagMods::from_attr(&attr)?;
            if mods.names.is_empty() {
                return syn_err(attr, "Missing any way to call option");
            }
            FieldAttr::Flag(mods)
        },
    };
    return Ok(Some(attr));
}

fn get_attr_kind(attr: &Attribute) -> Result<Option<AttrKind>> {
    let meta = &attr.meta;
    if meta.path().is_ident("command") {
        meta.require_path_only()?;
        Ok(Some(AttrKind::Command))
    } else if meta.path().is_ident("flag") {
        meta.require_list()?;
        Ok(Some(AttrKind::Flag))
    } else {
        // println!("[DEBUG] {meta:?}");
        Ok(None)
    }
}
