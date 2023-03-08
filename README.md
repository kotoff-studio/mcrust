# mcrust
Minecraft plugin example using Rust

## How to use
1. Compile using `cargo build --release`
2. Copy file **libmcrust.rlib** to *Your Minecraft Server Dir*\**libs**
3. Create file **libconf.json**
4. Add the content to this file as shown below:
```json
{
    "libs": {
        "libmcrust.rlib": {
            "name": "MCRust",
            "execute-class": "WelcomeMessage",
            "main-func": true,
            "imports-list": 0
        }
    }
}
```
5. Place **libconf.json** in the **libmcrust.rlib** dir
