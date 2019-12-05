#[derive(Clone)]
pub struct Project {
    pub name: String,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: String::from("rustbigtable"),
        }
    }
}

#[derive(Clone)]
pub struct Instance {
    pub project: Project,
    pub name: String,
}

impl Default for Instance {
    fn default() -> Self {
        Instance {
            name: String::from("test-inst"),
            project: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Table {
    pub instance: Instance,
    pub name: String,
}

impl Default for Table {
    fn default() -> Self {
        Table {
            name: String::from("my-table"),
            instance: Default::default(),
        }
    }
}
