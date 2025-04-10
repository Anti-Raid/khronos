#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ColumnType {
    /// A single valued column (scalar)
    Scalar {
        /// The value type
        inner: InnerColumnType,
    },
    /// An array column
    Array {
        /// The inner type of the array
        inner: InnerColumnType,
    },
}

impl ColumnType {
    /// Returns whether the column type is an array
    #[allow(dead_code)]
    pub fn is_array(&self) -> bool {
        matches!(self, ColumnType::Array { .. })
    }

    /// Returns whether the column type is a scalar
    #[allow(dead_code)]
    pub fn is_scalar(&self) -> bool {
        matches!(self, ColumnType::Scalar { .. })
    }

    pub fn new_scalar(inner: InnerColumnType) -> Self {
        ColumnType::Scalar { inner }
    }

    pub fn new_array(inner: InnerColumnType) -> Self {
        ColumnType::Array { inner }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
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
        kind: String, // e.g. templateref etc.
        max_bytes: Option<usize>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnSuggestion {
    Static { suggestions: Vec<String> },
    None {},
}

#[derive(Debug, Clone)]
pub struct Column {
    /// The ID of the column on the database
    pub id: String,

    /// The friendly name of the column
    pub name: String,

    /// The description of the column
    pub description: String,

    /// The type of the column
    pub column_type: ColumnType,

    /// Whether or not the column is a primary key
    pub primary_key: bool,

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

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
#[allow(dead_code)]
pub enum OperationType {
    View,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone)]
pub struct Setting {
    /// The ID of the option
    pub id: String,

    /// The name of the option
    pub name: String,

    /// The description of the option
    pub description: String,

    /// Title template, used for the title of the embed
    pub title_template: String,

    /// The columns for this option
    pub columns: Vec<Column>,

    /// The supported operations for this option
    pub supported_operations: SettingOperations,
}

#[derive(Clone, Debug, Default)]
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

pub struct Page {
    pub title: String,
    pub description: String,
    pub settings: Vec<Setting>,
}
