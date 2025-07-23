# @antiraid/discord

This plugin allows for templates to interact with the Discord API. Types are as defined by Discord if not explicitly documented.

## Bulk operations

When performing bulk operations, AntiRaid's standard GCRA based ratelimits might not work so well. For this, AntiRaid provides a ``antiraid_bulk_op`` which will return a discord ``Plugin`` that allows performing bulk operations. To do a bulk operation, your code must perform one operation at a time, and then call ``antiraid_bulk_op_wait`` to wait for the enforced wait period.