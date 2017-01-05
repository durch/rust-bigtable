macro_rules! gen_get_name {
    () => {
        pub fn get_name(&self) -> &str {
            &self.name
        }
    };
}

pub struct Project {
    name: String
}

impl Project {
    gen_get_name!();
}

impl Default for Project {
    fn default() -> Self {
        Project { name: String::from("rustbigtable") }
    }
}

pub struct Instance {
    project: Project,
    name: String
}

impl Instance {
    gen_get_name!();
    pub fn get_project(&self) -> &Project {
        &self.project
    }
}

impl Default for Instance {
    fn default() -> Self {
        Instance { name: String::from("test-inst"), project: Default::default() }
    }
}

pub struct Table {
    instance: Instance,
    name: String
}

impl Table {
    gen_get_name!();
    pub fn get_instance(&self) -> &Instance {
        &self.instance
    }
}

impl Default for Table {
    fn default() -> Self {
        Table { name: String::from("my-table"), instance: Default::default() }
    }
}
