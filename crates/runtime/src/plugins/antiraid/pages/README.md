# @antiraid/pages

Make and modify AntiRaid template pages

## Enums

<div id="type.Setting.Column.InnerColumnType" />

### Setting.Column.InnerColumnType

The inner column type of the value. See examples for full typings until we have a full spec.

<div id="type.Setting.Column.ColumnType" />

### Setting.Column.ColumnType

The type of a setting column

#### Variants

##### Setting.Column.ColumnType::Scalar
A scalar column type.

**Fields**
- `inner` ([Setting.Column.InnerColumnType](#type.Setting.Column.InnerColumnType)): The inner type of the column.

##### Setting.Column.ColumnType::Array
An array column type.

**Fields**
- `inner` ([Setting.Column.InnerColumnType](#type.Setting.Column.InnerColumnType)): The array type of the column.

## Types

<div id="type.Setting.Column" />

### Setting.Column

A setting column

```json
{
  "id": "created_at",
  "name": "Created At",
  "description": "The time the record was created.",
  "column_type": {
    "type": "Scalar",
    "inner": "String",
    "min_length": 120,
    "max_length": 120,
    "allowed_values": [
      "allowed_value"
    ],
    "kind": "timestamp"
  },
  "nullable": false,
  "suggestions": {
    "type": "None"
  },
  "secret": false,
  "ignored_for": [
    "Create",
    "Update"
  ]
}
```

#### Fields
- `id` ([string](#type.string)): The ID of the column.
- `name` ([string](#type.string)): The name of the column.
- `description` ([string](#type.string)): The description of the column.
- `column_type` ([Setting.Column.ColumnType](#type.Setting.Column.ColumnType)): The type of the column.
- `nullable` ([bool](#type.bool)): Whether the column can be null.
- `suggestions` ([Setting.Column.ColumnSuggestion](#type.Setting.Column.ColumnSuggestion)): The suggestions for the column.
- `secret` ([bool](#type.bool)): Whether the column is secret.
- `ignored_for` ([{OperationType}](#type.OperationType)): The operations that the column is ignored for [read-only]. It is *not guaranteed* that ignored field are sent to the template.

<div id="type.Setting" />

### Setting

A setting

```json
{
  "id": "setting_id",
  "name": "Setting Name",
  "description": "Setting Description",
  "primary_key": "id",
  "title_template": "{col1} - {col2}",
  "columns": [
    {
      "id": "col1",
      "name": "Column 1",
      "description": "Column 1 desc",
      "column_type": {
        "type": "Scalar",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "Normal"
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Create"
      ]
    },
    {
      "id": "col2",
      "name": "Column 2",
      "description": "Column 2 desc",
      "column_type": {
        "type": "Array",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "something cool?" // Is ignored...
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "View"
      ]
    },
    {
      "id": "col3",
      "name": "Column 3",
      "description": "Column 3 desc",
      "column_type": {
        "type": "Array",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "textarea",
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Create"
      ]
    },
    {
      "id": "col4",
      "name": "Column 4",
      "description": "Column 4 desc",
      "column_type": {
        "type": "Array",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "templateref"
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Create"
      ]
    },
    {
      "id": "col5",
      "name": "Column 5",
      "description": "Column 5 desc",
      "column_type": {
        "type": "Array",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "kittycat-permission"
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Create"
      ]
    },
    {
      "id": "col6",
      "name": "Column 6",
      "description": "Column 6 desc",
      "column_type": {
        "type": "Array",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "user"
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Create"
      ]
    },
    {
      "id": "col7",
      "name": "Column 7",
      "description": "Column 7 desc",
      "column_type": {
        "type": "Array",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "role"
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Create"
      ]
    },
    {
      "id": "col8",
      "name": "Column 8",
      "description": "Column 8 desc",
      "column_type": {
        "type": "Array",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "channel"
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Update"
      ]
    },
    {
      "id": "col9",
      "name": "Column 9",
      "description": "Column 9 desc",
      "column_type": {
        "type": "Scalar",
        "inner": "Integer"
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Update"
      ]
    },
    {
      "id": "col10",
      "name": "Column 10",
      "description": "Column 10 desc",
      "column_type": {
        "type": "Scalar",
        "inner": "Boolean"
      },
      "nullable": false,
      "suggestions": {
        "type": "Static",
        "suggestions": [
          "suggestion"
        ]
      },
      "secret": false,
      "ignored_for": [
        "Update"
      ]
    },
    {
      "id": "created_at",
      "name": "Created At",
      "description": "The time the record was created.",
      "column_type": {
        "type": "Scalar",
        "inner": "String",
        "min_length": 120,
        "max_length": 120,
        "allowed_values": [
          "allowed_value"
        ],
        "kind": "timestamp"
      },
      "nullable": false,
      "suggestions": {
        "type": "None"
      },
      "secret": false,
      "ignored_for": [
        "Create",
        "Update"
      ]
    }
  ],
  "operations": {
    "view": true,
    "create": true,
    "update": false,
    "delete": true
  },
}

```
#### Fields
- `id` ([string](#type.string)): The ID of the setting.
- `name` ([string](#type.string)): The name of the setting.
- `description` ([string](#type.string)): The description of the setting.
- `operations` ([{OperationType}](#type.OperationType)): The operations that can be performed on the setting. 
- `primary_key` ([string](#type.string)): The primary key of the setting that UNIQUELY identifies the row. When ``Delete`` is called, the value of this is what will be sent in the event. On ``Update``, this key MUST also exist (otherwise, the template MUST error out)
- `title_template` ([string](#type.string)): The template for the title of each row for the setting. This is a string that can contain placeholders for columns. The placeholders are in the form of ``{column_id}``. For example, if you have a column with ID ``col1`` and another with ID ``col2``, you can have a title template of ``{col1} - {col2}`` etc..
- `columns` ([{Setting.Column}](#type.Setting.Column)): The columns of the setting.

<div id="type.Page" />

### Page

An AntiRaid template page

#### Fields

- `title` ([string](#type.string)): The title of the page.
- `description` ([string](#type.string)): The description of the page.
- `settings` ([{Setting}](#type.Setting)): The settings of the page.

### PageExecutor

A page executor. This is used to manipulate the page.

#### Methods

##### PageExecutor:get

```luau
function get(): LuaPromise<Page?>
```

Returns the page associated with the template, if any. Using the token from another template to create a PageExecutor and then calling get can be used to get that templates' page.

##### PageExecutor:set

```luau
function set(page: Page): LuaPromise<void>
```

Sets a page to be the templates page. This will overwrite any existing page if one exists.

##### PageExecutor:delete

```luau
function delete(): LuaPromise<void>
```

Deletes the templates page. This will not delete the page itself, but will remove it from the server's list of custom pages.

---

## Methods

### new

```luau
function new(token: TemplateContext): LuaPromise<PageExecutor>
```

Returns a page executor associated with the template which can then be used to manipulate the templates page.