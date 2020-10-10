# twitter-tui

lightweight in-terminal user interface for twitter [written in Rust]

# recommended terminal size:
Width: 117
Height: 30

## File Structure <a name="structure"></a>
    twitter-tui
    ├── README 
    ├── Cargo.toml
    ├── Cargo.lock
    └── src
        ├── main.rs
        |
        ├── config                     
        |   ├── config.rs      
        |   ├── mod.rs    
        |   └── settings
        |
        ├── ui                      
        |   ├── ui.rs         
        |   └── mod.rs
        |
        └── util 
            ├── misc.rs    
            ├── tweet.rs  
            ├── user.rs     
            └── mod.rs
