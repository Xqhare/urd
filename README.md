# `Urd`
A journaling app written in rust.

## Name
`Urd` is named after one of the Norns from Norse mythology spinning the threads of fate at the foot of Yggdrasil. `Urd` is responsible for shaping destiny and is responsible for the past.

## Features

- Lockable screen
    - Password protection
- If you write a journal entry every day, completed months or years will be marked
    - This can be customised in the `Settings` menu
- Backups
- Export your journal
- Daily mood tracker
    - To add new moods or edit existing moods, use the `Settings` menu
- Important day marker
- Years aspirations
    1. Year Theme
        - Give your year a theme, e.g. `Learning` or `Growth` will be displayed as `Year of Learning` or `Year of Growth`
    2. Year Pledge
        - Give yourself a pledge for the year, e.g. `learn more about Rust` will be displayed as `This year I pledge to learn more about Rust`
    3. Years Resolutions
        - The classic year resolutions
        - As many as you want!
        - No you cannot mark them completed, you won't finish them anyway!

## Roadmap

## Issues preventing a stable release

- Creating a new viewport will crash the app if it's not supported by the device
- max tag length is 46 characters, more starts to break the Metadata UI, below the entry text field
- Better user onboarding
    - On hover text for everything
    - Detection of first run is implemented
        - Opens `Help` window only currently
    - Open Modal `Welcome`
        - Extremely short explanation
            - All your entries are displayed in the side-bar
            - You can click on them to edit them
                - That will open them in the central panel
        - Direct them to the `Settings` menu
        - Button `Take me to the settings menu`
        - Button `Got it - don't show again`
    - Onboarding in the `Settings` menu
        - Instead of main window display a panel with text and button for `next`
        - explain settings page in detail



## Password protection

`Urd` supports password protection, which can be used to prevent unauthorized access to the journal.
This protection is not designed to be secure, and should not be used for sensitive data.
It is only intended to protect against a child, spouse or friend from accessing the journal.

There are major issues with password protection:
- Journal data is not encrypted.
- The password is stored as is, only lightly obfuscated, and could be recovered by an attacker.
    - This makes it also possible to retrieve the password should it be lost.

### Help, I forgot the password!

There are two ways to deal with a forgotten password:

#### Removing the password protection
This is the easiest way to deal with a forgotten password.
Please note that this will not only remove the password protection, but reset all settings to their default values.
No Journal data will be lost.

1. You simply delete the `settings.xff` file from the `urd_state` directory in the same directory as the `Urd` executable.
2. Start `Urd`

#### Recovery of the password
This is the only 'non-destructive' way to recover the password. Please read [Removing the password protection](#removing-the-password-protection) first, and use this method only if you are sure about it.

The password is stored inside the `settings.xff` file, located in the `urd_state` directory in the same directory as the `Urd` executable.

1. Open this file in a binary or hex editor. Most editors will display a parsed readout of the file somewhere, and the password should be visible within this readout.
2. Switch to `ASCII` or `utf-8` encoding if needed.
3. Look for the `password` key, and the following clear text value is the password.

Please note that you can not change the password inside the file, as it will break the file.

### Changing the password

You can change the password inside `Urd`, using the `Settings` menu.

## Technical details

### Dependencies

#### `eframe`
The only outside dependency is [`eframe`](https://github.com/emilk/egui).

### All other dependencies
Written by me, you can find them on my [GitHub](https://github.com/xqhare).

