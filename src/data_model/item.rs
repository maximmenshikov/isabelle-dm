use yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::util::accessor::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Properties)]
pub struct Item {
    #[serde(default = "unset_id")]
    pub id: u64,

    #[serde(default = "unset_str_map")]
    pub strs: HashMap<String, String>,

    #[serde(default = "unset_strstr_map")]
    pub strstrs: HashMap<String, HashMap<String, String>>,

    #[serde(default = "unset_bool_map")]
    pub bools: HashMap<String, bool>,

    #[serde(default = "unset_u64_map")]
    pub u64s: HashMap<String, u64>,

    #[serde(default = "unset_u64_map")]
    pub ids: HashMap<String, u64>,
}

unsafe impl Send for Item {}

impl Item {
    pub fn new() -> Self {
        Self {
            id: u64::MAX,
            strs: HashMap::new(),
            strstrs: HashMap::new(),
            bools: HashMap::new(),
            u64s: HashMap::new(),
            ids: HashMap::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn safe_str(&self, name: &str, def: &str) -> String {
        if self.strs.contains_key(name) {
            self.strs[name].clone()
        }
        else {
            def.to_string()
        }
    }

    pub fn set_str(&mut self, name: &str, val: &str) {
        if self.strs.contains_key(name) {
            let v = self.strs.get_mut(name).unwrap();
            *v = val.to_string();
        } else {
            self.strs.insert(name.to_string(), val.to_string());
        }
    }

    pub fn safe_strstr(&self, name: &str, def: &HashMap<String, String>) -> HashMap<String, String> {
        if self.strstrs.contains_key(name) {
            self.strstrs[name].clone()
        }
        else {
            def.clone()
        }
    }

    pub fn set_strstr(&mut self, name: &str, val: &HashMap<String, String>) {
        if self.strstrs.contains_key(name) {
            let v = self.strstrs.get_mut(name).unwrap();
            *v = val.clone();
        } else {
            self.strstrs.insert(name.to_string(), val.clone());
        }
    }

    pub fn safe_bool(&self, name: &str, def: bool) -> bool {
        if self.bools.contains_key(name) {
            self.bools[name].clone()
        }
        else {
            def
        }
    }

    pub fn set_bool(&mut self, name: &str, val: bool) {
        if self.bools.contains_key(name) {
            let v = self.bools.get_mut(name).unwrap();
            *v = val;
        } else {
            self.bools.insert(name.to_string(), val);
        }
    }

    pub fn safe_u64(&self, name: &str, def: u64) -> u64 {
        if self.ids.contains_key(name) {
            self.ids[name].clone()
        }
        else {
            def
        }
    }

    pub fn set_u64(&mut self, name: &str, val: u64) {
        if self.u64s.contains_key(name) {
            let v = self.u64s.get_mut(name).unwrap();
            *v = val;
        } else {
            self.u64s.insert(name.to_string(), val);
        }
    }

    pub fn safe_id(&self, name: &str, def: u64) -> u64 {
        if self.ids.contains_key(name) {
            self.ids[name].clone()
        }
        else {
            def
        }
    }

    pub fn set_id(&mut self, name: &str, val: u64) {
        if self.ids.contains_key(name) {
            let v = self.ids.get_mut(name).unwrap();
            *v = val;
        } else {
            self.ids.insert(name.to_string(), val);
        }
    }

    pub fn merge(&mut self, itm: &Item) {
        for obj in &itm.strs {
            self.set_str(obj.0, obj.1);
        }

        for obj in &itm.strstrs {
            self.set_strstr(obj.0, obj.1);
        }

        for obj in &itm.bools {
            self.set_bool(obj.0, *obj.1);
        }

        for obj in &itm.u64s {
            self.set_u64(obj.0, *obj.1);
        }

        for obj in &itm.ids {
            self.set_id(obj.0, *obj.1);
        }
    }
}

