* The programm takes all dotfiles as argument;
* Copies that files to a `target`.

TODO:
- Add error handling [V]
- Pull out all to separate functions, structures etc [V]
- Move all stuff to lib.rs [V]
- Unbind args from hardcoded args[1] position [V]
- Add a target as command line option and create a new field
  in the Config struct [V]
- Implement tests [V]
- If there is no command line args produce stderr [V]

- Add command line args:
    1) arg should take a list of dotfiles like in .toml config
        but it has to have higher precendence;

    2) ...

