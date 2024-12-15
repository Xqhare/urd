# Urd
A journal App written in rust

## Name
Urd is named after one of the Norns from Norse mythology, responsible for shaping destiny and is responsible for the past.

## Issues preventing a stable release

- Creating a new viewport will crash the app if it's not supported by the device

## Password protection

Urd supports password protection, which can be used to prevent unauthorized access to the journal.
This protection is not designed to be secure, and should not be used for sensitive data.
It is only intended to protect against a child, spouse or friend from accessing the journal.

There are major issues with password protection:
- Journal data is not encrypted.
- The password is stored as is, only lightly obfuscated, and could be recovered by an attacker.
    - This makes it also possible to retrieve the password should it be lost.

### Recovery of the password

The password is stored inside the `settings.xff` file, located in the `urd_state` directory in the same directory as the Urd executable.
Open this file in a binary or hex editor. Most editors will display a parsed readout of the file somewhere, and the password should be visible within this readout.
Switch to `ASCII` or `utf-8` encoding if needed.
Look for the `password` key, and the following clear text value is the password.

Please note that you can not change the password inside the file, as it will break the file.

### Changing the password

You can change the password inside Urd, inside the `Settings` menu.
