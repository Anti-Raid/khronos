#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ColumnType {
    /// A single valued column (scalar)
    Scalar {
        /// The value type
        #[serde(flatten)]
        inner: InnerColumnType,
    },
    /// An array column
    Array {
        /// The inner type of the array
        #[serde(flatten)]
        inner: InnerColumnType,
    },
}

impl From<ColumnType> for crate::traits::ir::ColumnType {
    fn from(v: ColumnType) -> crate::traits::ir::ColumnType {
        match v {
            ColumnType::Scalar { inner } => crate::traits::ir::ColumnType::Scalar {
                inner: inner.into(),
            },
            ColumnType::Array { inner } => crate::traits::ir::ColumnType::Array {
                inner: inner.into(),
            },
        }
    }
}

impl From<crate::traits::ir::ColumnType> for ColumnType {
    fn from(v: crate::traits::ir::ColumnType) -> ColumnType {
        match v {
            crate::traits::ir::ColumnType::Scalar { inner } => ColumnType::Scalar {
                inner: inner.into(),
            },
            crate::traits::ir::ColumnType::Array { inner } => ColumnType::Array {
                inner: inner.into(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "inner")]
pub enum InnerColumnType {
    String {
        min_length: Option<usize>,
        max_length: Option<usize>,
        allowed_values: Vec<String>, // If empty, all values are allowed
        kind: String, // e.g. uuid, textarea, channel, user, role, interval, timestamp etc.
    },
    Integer {},
    Float {},
    BitFlag {
        /// The bit flag values
        values: indexmap::IndexMap<String, i64>,
    },
    Boolean {},
    Json {
        max_bytes: Option<usize>,
    },
}

impl From<InnerColumnType> for crate::traits::ir::InnerColumnType {
    fn from(v: InnerColumnType) -> crate::traits::ir::InnerColumnType {
        match v {
            InnerColumnType::String {
                min_length,
                max_length,
                allowed_values,
                kind,
            } => crate::traits::ir::InnerColumnType::String {
                min_length,
                max_length,
                allowed_values,
                kind,
            },
            InnerColumnType::Integer {} => crate::traits::ir::InnerColumnType::Integer {},
            InnerColumnType::Float {} => crate::traits::ir::InnerColumnType::Float {},
            InnerColumnType::BitFlag { values } => {
                crate::traits::ir::InnerColumnType::BitFlag { values }
            }
            InnerColumnType::Boolean {} => crate::traits::ir::InnerColumnType::Boolean {},
            InnerColumnType::Json { max_bytes } => {
                crate::traits::ir::InnerColumnType::Json { max_bytes }
            }
        }
    }
}

impl From<crate::traits::ir::InnerColumnType> for InnerColumnType {
    fn from(v: crate::traits::ir::InnerColumnType) -> InnerColumnType {
        match v {
            crate::traits::ir::InnerColumnType::String {
                min_length,
                max_length,
                allowed_values,
                kind,
            } => InnerColumnType::String {
                min_length,
                max_length,
                allowed_values,
                kind,
            },
            crate::traits::ir::InnerColumnType::Integer {} => InnerColumnType::Integer {},
            crate::traits::ir::InnerColumnType::Float {} => InnerColumnType::Float {},
            crate::traits::ir::InnerColumnType::BitFlag { values } => {
                InnerColumnType::BitFlag { values }
            }
            crate::traits::ir::InnerColumnType::Boolean {} => InnerColumnType::Boolean {},
            crate::traits::ir::InnerColumnType::Json { max_bytes } => {
                InnerColumnType::Json { max_bytes }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ColumnSuggestion {
    Static { suggestions: Vec<String> },
    None {},
}

impl From<ColumnSuggestion> for crate::traits::ir::ColumnSuggestion {
    fn from(v: ColumnSuggestion) -> crate::traits::ir::ColumnSuggestion {
        match v {
            ColumnSuggestion::Static { suggestions } => {
                crate::traits::ir::ColumnSuggestion::Static { suggestions }
            }
            ColumnSuggestion::None {} => crate::traits::ir::ColumnSuggestion::None {},
        }
    }
}

impl From<crate::traits::ir::ColumnSuggestion> for ColumnSuggestion {
    fn from(v: crate::traits::ir::ColumnSuggestion) -> ColumnSuggestion {
        match v {
            crate::traits::ir::ColumnSuggestion::Static { suggestions } => {
                ColumnSuggestion::Static { suggestions }
            }
            crate::traits::ir::ColumnSuggestion::None {} => ColumnSuggestion::None {},
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Column {
    /// The ID of the column on the database
    pub id: String,

    /// The friendly name of the column
    pub name: String,

    /// The description of the column
    pub description: String,

    /// The type of the column
    pub column_type: ColumnType,

    /// Whether or not the column is nullable
    ///
    /// Note that the point where nullability is checked may vary but will occur after pre_checks are executed
    pub nullable: bool,

    /// Suggestions to display
    pub suggestions: ColumnSuggestion,

    /// A secret field that is not shown to the user
    pub secret: bool,

    /// For which operations should the field be ignored for (essentially, read only)
    ///
    /// Semantics are defined by the Executor
    pub ignored_for: Vec<OperationType>,
}

impl From<Column> for crate::traits::ir::Column {
    fn from(v: Column) -> crate::traits::ir::Column {
        crate::traits::ir::Column {
            id: v.id,
            name: v.name,
            description: v.description,
            column_type: v.column_type.into(),
            nullable: v.nullable,
            suggestions: v.suggestions.into(),
            secret: v.secret,
            ignored_for: v.ignored_for.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<crate::traits::ir::Column> for Column {
    fn from(v: crate::traits::ir::Column) -> Column {
        Column {
            id: v.id,
            name: v.name,
            description: v.description,
            column_type: v.column_type.into(),
            nullable: v.nullable,
            suggestions: v.suggestions.into(),
            secret: v.secret,
            ignored_for: v.ignored_for.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub enum OperationType {
    View,
    Create,
    Update,
    Delete,
}

impl From<OperationType> for crate::traits::ir::OperationType {
    fn from(v: OperationType) -> crate::traits::ir::OperationType {
        match v {
            OperationType::View => crate::traits::ir::OperationType::View,
            OperationType::Create => crate::traits::ir::OperationType::Create,
            OperationType::Update => crate::traits::ir::OperationType::Update,
            OperationType::Delete => crate::traits::ir::OperationType::Delete,
        }
    }
}

impl From<crate::traits::ir::OperationType> for OperationType {
    fn from(v: crate::traits::ir::OperationType) -> OperationType {
        match v {
            crate::traits::ir::OperationType::View => OperationType::View,
            crate::traits::ir::OperationType::Create => OperationType::Create,
            crate::traits::ir::OperationType::Update => OperationType::Update,
            crate::traits::ir::OperationType::Delete => OperationType::Delete,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Setting {
    /// The ID of the option
    pub id: String,

    /// The name of the option
    pub name: String,

    /// The description of the option
    pub description: String,

    /// The primary key of the table. Should be present in ID
    pub primary_key: String,

    /// Title template, used for the title of the embed
    pub title_template: String,

    /// The columns for this option
    pub columns: Vec<Column>,

    /// The supported operations for this option
    pub supported_operations: SettingOperations,
}

impl From<Setting> for crate::traits::ir::Setting {
    fn from(v: Setting) -> crate::traits::ir::Setting {
        crate::traits::ir::Setting {
            id: v.id,
            name: v.name,
            description: v.description,
            primary_key: v.primary_key,
            title_template: v.title_template,
            columns: v.columns.into_iter().map(|e| e.into()).collect(),
            supported_operations: v.supported_operations.into(),
        }
    }
}

impl From<crate::traits::ir::Setting> for Setting {
    fn from(v: crate::traits::ir::Setting) -> Setting {
        Setting {
            id: v.id,
            name: v.name,
            description: v.description,
            primary_key: v.primary_key,
            title_template: v.title_template,
            columns: v.columns.into_iter().map(|e| e.into()).collect(),
            supported_operations: v.supported_operations.into(),
        }
    }
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SettingOperations {
    /// How to view this setting
    pub view: bool,

    /// How to create this setting
    pub create: bool,

    /// How to update this setting
    pub update: bool,

    /// How to delete this setting
    pub delete: bool,
}

impl From<SettingOperations> for crate::traits::ir::SettingOperations {
    fn from(v: SettingOperations) -> crate::traits::ir::SettingOperations {
        crate::traits::ir::SettingOperations {
            view: v.view,
            create: v.create,
            update: v.update,
            delete: v.delete,
        }
    }
}

impl From<crate::traits::ir::SettingOperations> for SettingOperations {
    fn from(v: crate::traits::ir::SettingOperations) -> SettingOperations {
        SettingOperations {
            view: v.view,
            create: v.create,
            update: v.update,
            delete: v.delete,
        }
    }
}
