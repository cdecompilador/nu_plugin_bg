## HOW TO USE
Just do `cargo install --path .`, restart your nushell and now you can 
use it, for example to open vscode without blocking the terminal `bg "code" "."`.
Also I suggest to use this tool with aliases, for example if your nushell config file is like
this:
```toml
startup = [
	"alias code = bg \"code\" \".\""
]
```
Now you can use just code to start vscode without blocking terminal.
