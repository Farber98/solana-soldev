1. Create project

    ```
    anchor init anchor-counter


2. Build project

    ```
    anchor-build

3. See anchor keys and look for the programID output

    ```
    anchor keys list

4. Update declare_id! in lib.rs

    ```
    declare_id!("key");

5. Update Anchor.toml

    ```
    [programs.localnet]
    anchor_counter = "key"

7. Delete default code in lib.rs and let the following

    ```
    use anchor_lang::prelude::*;

    declare_id!("key");

    #[program]
    pub mod project_name {
        use super::*;

    }
