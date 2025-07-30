# @antiraid/kv

Utilities for key-value operations.

## Scopes

Scopes are used to organize keys in the key-value store. Each key can belong to multiple scopes, allowing for flexible organization and performant retrieval of keys. When setting or getting a key, you can specify one or more scopes. Unscoped keys are not allowed due to security and organizational reasons. The ``kv:{action}` and `kv[scope]:{action}`` capabilities are used to control access to keys with granular control over which actions can be performed on which scopes.

## Basic Operations:

### List Scopes

The ``list_scopes`` method can be used to list all scopes available in the key-value store. This is useful for debugging purposes. This requires the `kv.meta:list_scopes` capability. Returns a list of strings where each string is a scope name.

### List Keys within a Scope

The ``keys`` method can be used to list all keys within a specific scope. This is useful for debugging purposes. This requires the `kv.meta:keys` capability (all scopes) or the `kv.meta[scope]:keys` capability (specific scope only). Returns a list of strings where each string is a key name.

### Find Key-Value

The ``find`` method allows you to search for keys that match a specific pattern within a scope. This is useful for finding keys that match a certain naming convention or prefix. This requires the `kv:find` capability (all scopes) or the `kv[scope]:find` capability (specific scope only). Returns a list of `KVRecord` objects that match the pattern.

Within patterns, the following wildcards are supported:
- `_`: Matches a single character.
- `%`: Matches zero or more characters.

To match a literal underscore or percent sign, you can escape them with a backslash (`\_` or `\%`).

As a useful fastpath, you can use the ``%%`` pattern to match all keys in a scope. This is equivalent to listing all keys in the scope using the ``keys`` method but returns `KVRecord` objects instead of just key names.

### Key Existence

The ``exists`` method checks if a key exists within a specific scope. This requires the `kv:exists` capability (all scopes) or the `kv[scope]:exists` capability (specific scope only). Returns a boolean indicating whether the key exists.

### Get a Key

The ``get`` method retrieves the value of a key within a specific scope. If the key does not exist, it returns `nil`. This requires the `kv:get` capability (all scopes) or the `kv[scope]:get` capability for a scope the key is in. Returns the value of the key or `nil` if the key does not exist. 

If a ``id`` is known, the ``getbyid`` method can be used to retrieve the value of a key by its ID. This is useful for performance reasons, as it avoids the need to look up the key by name and scope. This requires the `kv.meta:id_ops` capability.

Lastly, the ``getrecord`` method retrieves the value of a key and its metadata, including the scopes it belongs to. ``getrecord`` has the same capability requirements as ``get``. Returns a ``KVRecord``. Similar to ``get``, a ``getrecordbyid`` method exists to retrieve the value and metadata of a key by its ID, requiring the `kv.meta:id_ops` capability (just like ``getbyid``).

### Set a Key

The ``set`` method allows you to set the value of a key within a specific scope along with special properties like ``expires_at`` and ``resume``. If the key already exists, it will be updated. If it does not exist, it will be created. This requires the `kv:set` capability (all scopes) or the `kv[scope]:set` capability for a scope the key is in. Returns a ``SetResult`` object.

Similar to ``get``, a ``setbyid`` method exists to set the value of a key by its ID, requiring the `kv.meta:id_ops` capability. This is useful for performance reasons, as it avoids the need to look up the key by name and scope. Returns a ``SetResult`` object.

### Set Expiry on Key

The ``setexpiry`` method allows you to set an expiry time on a key. If the key does not exist, it will be created with the specified expiry time. If the key already exists, its expiry time will be updated. This requires the `kv:set_expiry` capability (all scopes) or the `kv[scope]:set_expiry` capability for a scope the key is in. Returns a boolean indicating whether the expiry was set successfully.

Similar to ``get``, a ``setexpirybyid`` method exists to set the expiry time of a key by its ID, requiring the `kv.meta:id_ops` capability. This is useful for performance reasons, as it avoids the need to look up the key by name and scope. Returns a boolean indicating whether the expiry was set successfully.

### Remove/Delete a Key

The ``delete`` method allows you to remove a key from the key-value store. If the key does not exist, it will do nothing. This requires the `kv:delete` capability (all scopes) or the `kv[scope]:delete` capability for a scope the key is in. Returns a boolean indicating whether the key was deleted successfully.

Similar to ``get``, a ``deletebyid`` method exists to remove a key by its ID, requiring the `kv.meta:id_ops` capability. This is useful for performance reasons, as it avoids the need to look up the key by name and scope. Returns a boolean indicating whether the key was deleted successfully.

## Expiry and Resume

A key on the key value pair can have 2 special properties attached to them: ``expires_at`` and ``resume``. If ``expires_at`` is set, the key will be automatically removed from the key-value store after the specified time with a ``KeyExpiry`` event emitted on removal. If ``resume`` is set to ``true``, any reload of the worker process or reloads of the template will emit a ``KeyResume`` event for the key, allowing you to restart a ongoing operation that was interrupted by a worker process restart.

Note that the ``resume`` property can only be set using the ``set`` and ``setbyid`` methods. Of note, there is no ``setresume`` or ``setresumebyid`` method, as whether or not a key is resumable should be known in advance.

## Objects

### KVRecord

A ``KVRecord`` is a structure that contains the value of a key and its metadata. It includes the following fields:

- ``id``: The unique identifier of the key-value pair.
- ``key``: The name of the key.
- ``value``: The value of the key.
- ``scopes``: A list of scopes the key belongs to.
- ``exists``: A boolean indicating whether the key exists.
- ``created_at``: The timestamp when the key was created. This is a DateTime object.
- ``last_updated_at``: The timestamp when the key was last updated. This is a DateTime object.
- ``expires_at``: The timestamp when the key will expire, if set. This is either a DateTime object or `nil` if the key does not expire.
- ``resume``: A boolean indicating whether the key is set to resume

### SetResult

A ``SetResult`` is a structure that contains the result of a key set operation. It includes the following fields:

- ``id``: The unique identifier of the key-value pair.
- ``exists``: A boolean indicating whether the key already existed before the set operation.