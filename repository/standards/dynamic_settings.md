# v1 standard of the dynamic settings

Dynamic settings is a standard that allows games and components
integrations to create their own settings UI in the launcher
from the lua side.

## Settings integration format

```ts
type DynamicSettingsManifest = {
    standard: 1,

    // List of the settings groups.
    settings: SettingsGroup[]
};

type SettingsGroup = {
    // Title of the settings group.
    title?: Localizable,

    // Optional description added close to the title.
    description?: Localizable,

    // Entries of the settings group.
    entries: SettingsEnty[]
};

type SettingsEnty = {
    // Unique name of the settings entry.
    // Will be used by the launcher to keep track of the value.
    name: string,

    // Title of the setting.
    title: Localizable,

    // Optional description of the setting.
    description?: Localizable,

    // Information about the settings entry.
    entry: SettingsEntrySwitch | SettingsEntryText | SettingsEntryEnum
};

// Switch which can be enabled or disabled.
type SettingsEntrySwitch = {
    format: 'switch',
    default: boolean
};

// Input text row where user writes a string.
type SettingsEntryText = {
    format: 'text',
    default: string
};

// List with different values from which user makes a choice.
type SettingsEntryEnum = {
    format: 'enum',

    // Table of list values.
    values: [key: string]: Localizable,

    // Key of a value chosen by default.
    default: string
};
```

## Returned settings object format

```ts
type DynamicSettingsValues = {
    standard: 1,

    // Table of settings specified by the user.
    settings: [name: string]: any
};
```
