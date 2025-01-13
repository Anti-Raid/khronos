# @antiraid/permissions

Utilities for handling permission checks.

## Types

<div id="type.Permission" />

### Permission

Permission is the primitive permission type used by AntiRaid. See https://github.com/InfinityBotList/kittycat for more information

```json
{
  "namespace": "moderation",
  "perm": "ban",
  "negator": false
}
```

#### Fields

- `namespace` ([string](#type.string)): The namespace of the permission.
- `perm` ([string](#type.string)): The permission bit on the namespace.
- `negator` ([bool](#type.bool)): Whether the permission is a negator permission or not


<div id="type.StaffPermissions" />

### StaffPermissions

StaffPermissions as per kittycat terminology.

```json
{
  "user_positions": [
    {
      "id": "1234567890",
      "index": 1,
      "perms": [
        {
          "namespace": "moderation",
          "perm": "ban",
          "negator": false
        },
        {
          "namespace": "moderation",
          "perm": "kick",
          "negator": false
        }
      ]
    },
    {
      "id": "0987654321",
      "index": 2,
      "perms": [
        {
          "namespace": "moderation",
          "perm": "ban",
          "negator": false
        },
        {
          "namespace": "moderation",
          "perm": "kick",
          "negator": false
        }
      ]
    }
  ],
  "perm_overrides": [
    {
      "namespace": "moderation",
      "perm": "ban",
      "negator": true
    },
    {
      "namespace": "moderation",
      "perm": "kick",
      "negator": true
    }
  ]
}
```

#### Fields

- `perm_overrides` ([{Permission}](#type.Permission)): Permission overrides on the member.
- `user_positions` ([{PartialStaffPosition}](#type.PartialStaffPosition)): The staff positions of the user.


<div id="type.PartialStaffPosition" />

### PartialStaffPosition

PartialStaffPosition as per kittycat terminology.

```json
{
  "id": "1234567890",
  "index": 1,
  "perms": [
    {
      "namespace": "moderation",
      "perm": "ban",
      "negator": false
    },
    {
      "namespace": "moderation",
      "perm": "kick",
      "negator": false
    }
  ]
}
```

#### Fields

- `id` ([string](#type.string)): The ID of the staff member.
- `index` ([number](#type.number)): The index of the staff member.
- `perms` ([{Permission}](#type.Permission)): The permissions of the staff member.


## Methods

### permission_from_string

```lua
function permission_from_string(perm_string: string): Permission
```

Returns a Permission object from a string.

#### Parameters

- `perm_string` ([string](#type.string)): The string to parse into a Permission object.


#### Returns

- `permission` ([Permission](#type.Permission)): The parsed Permission object.

### permission_to_string

```lua
function permission_to_string(permission: Permission): string
```

Returns a string from a Permission object.

#### Parameters

- `permission` ([Permission](#type.Permission)): The Permission object to parse into a string.


#### Returns

- `perm_string` ([string](#type.string)): The parsed string.

### has_perm

```lua
function has_perm(permissions: {Permission}, permission: Permission): bool
```

Checks if a list of permissions in Permission object form contains a specific permission.

#### Parameters

- `permissions` ([{Permission}](#type.Permission)): The list of permissions
- `permission` ([Permission](#type.Permission)): The permission to check for.


#### Returns

- `has_perm` ([bool](#type.bool)): Whether the permission is present in the list of permissions as per kittycat rules.

### has_perm_str

```lua
function has_perm_str(permissions: {string}, permission: string): bool
```

Checks if a list of permissions in canonical string form contains a specific permission.

#### Parameters

- `permissions` ([{string}](#type.string)): The list of permissions
- `permission` ([string](#type.string)): The permission to check for.


#### Returns

- `has_perm` ([bool](#type.bool)): Whether the permission is present in the list of permissions as per kittycat rules.

### staff_permissions_resolve

```lua
function staff_permissions_resolve(sp: StaffPermissions): {Permission}
```

Resolves a StaffPermissions object into a list of Permission objects. See https://github.com/InfinityBotList/kittycat for more details

#### Parameters

- `sp` ([StaffPermissions](#type.StaffPermissions)): The StaffPermissions object to resolve.


#### Returns

- `permissions` ([{Permission}](#type.Permission)): The resolved list of Permission objects.

### check_patch_changes

```lua
function check_patch_changes(manager_perms: {Permission}, current_perms: {Permission}, new_perms: {Permission})
```

Checks if a list of permissions can be patched to another list of permissions.

#### Parameters

- `manager_perms` ([{Permission}](#type.Permission)): The permissions of the manager.
- `current_perms` ([{Permission}](#type.Permission)): The current permissions of the user.
- `new_perms` ([{Permission}](#type.Permission)): The new permissions of the user.

#### Returns

- `can_patch` ([bool](#type.bool)): Whether the permissions can be patched.- `error` ([any](#type.any)): The error if the permissions cannot be patched. Will contain ``type`` field with the error type and additional fields depending on the error type.