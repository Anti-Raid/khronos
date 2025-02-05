# Khronos

Khronos is the runtime powering AntiRaid. For AntiRaid, AntiRaid templating services (right now, ``template-worker``) can simply implement ``KhronosContext`` and the many ``Provider`` traits to provide the necessary data for the runtime to function. For users, a CLI is being developed to allow for local development of templates by implementing the same ``KhronosContext`` and ``Provider`` traits but in a way designed for local use.


## Local Development

Note: Khronos CLI is not yet implemented. This section will be updated when it is.