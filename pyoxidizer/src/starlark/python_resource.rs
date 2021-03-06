// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use {
    crate::py_packaging::resource::{
        BytecodeModule, BytecodeOptimizationLevel, ExtensionModuleData, PythonResource,
        ResourceData, SourceModule,
    },
    crate::py_packaging::standalone_distribution::ExtensionModule,
    starlark::environment::Environment,
    starlark::values::{default_compare, TypedValue, Value, ValueError, ValueResult},
    starlark::{any, immutable, not_supported},
    std::any::Any,
    std::cmp::Ordering,
    std::collections::HashMap,
};

#[derive(Debug, Clone)]
pub struct PythonSourceModule {
    pub module: SourceModule,
}

impl TypedValue for PythonSourceModule {
    immutable!();
    any!();
    not_supported!(
        binop, dir_attr, function, get_hash, indexable, iterable, sequence, set_attr, to_int
    );

    fn to_str(&self) -> String {
        format!("PythonSourceModule<name={}>", self.module.name)
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "PythonSourceModule"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }

    fn get_attr(&self, attribute: &str) -> ValueResult {
        let v = match attribute {
            "name" => Value::new(self.module.name.clone()),
            // TODO expose source
            // "source" => Value::new(self.module.source),
            "is_package" => Value::new(self.module.is_package),
            attr => {
                return Err(ValueError::OperationNotSupported {
                    op: format!(".{}", attr),
                    left: "PythonSourceModule".to_string(),
                    right: None,
                })
            }
        };

        Ok(v)
    }

    fn has_attr(&self, attribute: &str) -> Result<bool, ValueError> {
        Ok(match attribute {
            "name" => true,
            // TODO expose source
            // "source" => true,
            "is_package" => true,
            _ => false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PythonBytecodeModule {
    pub module: BytecodeModule,
}

impl TypedValue for PythonBytecodeModule {
    immutable!();
    any!();
    not_supported!(
        binop, dir_attr, function, get_hash, indexable, iterable, sequence, set_attr, to_int
    );

    fn to_str(&self) -> String {
        format!(
            "PythonBytecodeModule<name={}; level={:?}>",
            self.module.name, self.module.optimize_level
        )
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "PythonBytecodeModule"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }

    fn get_attr(&self, attribute: &str) -> ValueResult {
        let v = match attribute {
            "name" => Value::new(self.module.name.clone()),
            // TODO expose source
            // "source" => Value::new(self.module.source),
            "optimize_level" => Value::new(match self.module.optimize_level {
                BytecodeOptimizationLevel::Zero => 0,
                BytecodeOptimizationLevel::One => 1,
                BytecodeOptimizationLevel::Two => 2,
            }),
            "is_package" => Value::new(self.module.is_package),
            attr => {
                return Err(ValueError::OperationNotSupported {
                    op: format!(".{}", attr),
                    left: "PythonBytecodeModule".to_string(),
                    right: None,
                })
            }
        };

        Ok(v)
    }

    fn has_attr(&self, attribute: &str) -> Result<bool, ValueError> {
        Ok(match attribute {
            "name" => true,
            // TODO expose source
            // "source" => true,
            "optimize_level" => true,
            "is_package" => true,
            _ => false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PythonResourceData {
    pub data: ResourceData,
}

impl TypedValue for PythonResourceData {
    immutable!();
    any!();
    not_supported!(
        binop, dir_attr, function, get_hash, indexable, iterable, sequence, set_attr, to_int
    );

    fn to_str(&self) -> String {
        format!(
            "PythonResourceData<package={}, name={}>",
            self.data.package, self.data.name
        )
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "PythonResourceData"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }

    fn get_attr(&self, attribute: &str) -> ValueResult {
        let v = match attribute {
            "package" => Value::new(self.data.package.clone()),
            "name" => Value::new(self.data.name.clone()),
            // TODO expose raw data
            attr => {
                return Err(ValueError::OperationNotSupported {
                    op: format!(".{}", attr),
                    left: "PythonResourceData".to_string(),
                    right: None,
                })
            }
        };

        Ok(v)
    }

    fn has_attr(&self, attribute: &str) -> Result<bool, ValueError> {
        Ok(match attribute {
            "package" => true,
            "name" => true,
            // TODO expose raw data
            _ => false,
        })
    }
}

/// Represents an extension module flavor.
#[derive(Debug, Clone)]
pub enum PythonExtensionModuleFlavor {
    /// An extension module from a Python distribution.
    Distribution(ExtensionModule),

    /// An extension module that can be statically linked.
    StaticallyLinked(ExtensionModuleData),

    /// An extension module that exists as a dynamic library.
    DynamicLibrary(ExtensionModuleData),
}

impl PythonExtensionModuleFlavor {
    pub fn name(&self) -> String {
        match self {
            PythonExtensionModuleFlavor::Distribution(m) => m.module.clone(),
            PythonExtensionModuleFlavor::StaticallyLinked(m) => m.name.clone(),
            PythonExtensionModuleFlavor::DynamicLibrary(m) => m.name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PythonExtensionModule {
    pub em: PythonExtensionModuleFlavor,
}

impl TypedValue for PythonExtensionModule {
    immutable!();
    any!();
    not_supported!(
        binop, dir_attr, function, get_hash, indexable, iterable, sequence, set_attr, to_int
    );

    fn to_str(&self) -> String {
        format!("PythonExtensionModule<name={}>", self.em.name())
    }

    fn to_repr(&self) -> String {
        self.to_str()
    }

    fn get_type(&self) -> &'static str {
        "PythonExtensionModule"
    }

    fn to_bool(&self) -> bool {
        true
    }

    fn compare(&self, other: &dyn TypedValue, _recursion: u32) -> Result<Ordering, ValueError> {
        default_compare(self, other)
    }

    fn get_attr(&self, attribute: &str) -> ValueResult {
        let v = match attribute {
            "name" => Value::new(self.em.name()),
            attr => {
                return Err(ValueError::OperationNotSupported {
                    op: format!(".{}", attr),
                    left: "PythonExtensionModule".to_string(),
                    right: None,
                })
            }
        };

        Ok(v)
    }

    fn has_attr(&self, attribute: &str) -> Result<bool, ValueError> {
        Ok(match attribute {
            "name" => true,
            _ => false,
        })
    }
}

impl<'a> From<&'a PythonResource> for Value {
    fn from(resource: &'a PythonResource) -> Value {
        match resource {
            PythonResource::ModuleSource {
                name,
                source,
                is_package,
            } => Value::new(PythonSourceModule {
                module: SourceModule {
                    name: name.clone(),
                    source: source.clone(),
                    is_package: *is_package,
                },
            }),

            PythonResource::ModuleBytecodeRequest {
                name,
                source,
                optimize_level,
                is_package,
            } => Value::new(PythonBytecodeModule {
                module: BytecodeModule {
                    name: name.clone(),
                    source: source.clone(),
                    optimize_level: BytecodeOptimizationLevel::from(*optimize_level),
                    is_package: *is_package,
                },
            }),

            PythonResource::ModuleBytecode { .. } => {
                panic!("not yet implemented");
            }

            PythonResource::Resource {
                package,
                name,
                data,
            } => Value::new(PythonResourceData {
                data: ResourceData {
                    package: package.clone(),
                    name: name.clone(),
                    data: data.clone(),
                },
            }),

            PythonResource::ExtensionModuleDynamicLibrary(em) => {
                Value::new(PythonExtensionModule {
                    em: PythonExtensionModuleFlavor::DynamicLibrary(em.clone()),
                })
            }

            PythonResource::ExtensionModuleStaticallyLinked(em) => {
                Value::new(PythonExtensionModule {
                    em: PythonExtensionModuleFlavor::StaticallyLinked(em.clone()),
                })
            }
        }
    }
}
