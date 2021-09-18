# Changelog

* Fix mistake
* Comment all relevant `unwrap`s

  I'm basically treating them like `unsafe`.
* Refactor confirmation prompt
* Use const for probability
* Remove `clone`s
* Remove unnecessary `pub`s
* Show exit confirmation prompt if game was played for > 1 minute
* Minor improvements
* Polish grid resize feature further
* Remove some `pub`s
* Fix color not being reset sometimes
* Remove useless annotation
* Add grid resizing

  By dragging the arrow in the bottom right you can now easily generate a new grid with a new size.
* Remove some `crate::`s
* Better debugging tools
* Amend README.md
* Many refactors
* Avoid unnecessary flushes in some cases
* Fix clear delay, level loading and refactor
* Import std::cmp
* Add FILE_EXTENSION
* Ability to load grids ingame
* New idea

## 0.8.0 (2021-06-27)

* Remove LICENSE

  `license = "MIT"` in Cargo.toml suffices
* Fix overlapping texts once more
* Undo CHANGELOG rename
* Update showcase image
* Update README
* Update terminal
* Fix a comment
* Add LICENSE
* Update comments
* Optimize alert clearing
* Refactor placing of measured cells
* Remove menu.old for being too old
* Refactor events
* Fix solved screen texts sometimes overlapping
* Refactor alerts
* Refactoring + ability to place cells only with the keyboard

  Implements #2.
* Improve example
* Rename CHANGELOG.md
* Use Rust 1.53.0

## 0.7.1 (2021-06-11)

* Fix top text y
* Refactor
* Fix accidental use of color
* Fix top text y-axis
* Properly clear window size message
* Doc adjustments
* Clear before awaiting fitting window size
* Capitalize errors
* Clean up
* Improve await_fitting_window_size
* Update README.md

## 0.7.0 (2021-06-08)

* Fix alert alignment
* Change percentage into progress bar

  The problem with the percentage was that it wasn't centered with even grid heights.
* Add hovered cell point drawing

  This is unused for now.
* Show "Grid saved as {}" message as an alert
* Lock stdout permanently and use BufWriter

  This will make the game run a lot smoother.
* Add percentage showing progress
* Fix alert overdrawing picture
* Small reorder
* Minor optimization
* Draw the whole grid picture in the top left
* Refactor into grid/cell.rs
* Remove a rather useless variable
* Add the Very Fast build mode
* Draw optimizations by drawing clues only if necessary
* Refactor highlighted clue background color
* Change grid background colors
* Previous_point_point -> previous_point
* Add more tests
* Add more comments to the cell representations
* Draw_dash_line -> write_dash_line
* Remove a few set_cursor calls
* "previous_cursor" -> "previous_point"
* Rename variables in centered_point
* Remove Cursor

  I didn't really see the point in this type.
* Remove self.cursor.update
* Abstract cell drawing and so display index on highlighted measured cells
* Reorder imports

  `mod`s should always come first because you might want to use `use` some of it afterwards.
* Fill measured cells regardless of their index
* Remove some NOTEs

  I now think that it's fine as is because if I forgot to use the previous_x variable, I'd get a warning.
* "notification" -> "alert"
* Add fill tool

## 0.6.0 (2021-06-03)

* "darkening" is now "highlighting"
* Fixup
* Make get_cell_point_from_cursor_point a const fn
* Second part of darkening cells in all directions constantly

  This simplifies a lot of things.
* Remove redundant clone
* Remap keys

  I hope that this is going to be the final revision.
* Use Grid::clear
* Refactor event.rs
* Random grid reset mystery solved

  It turns out the reason middle-click in VS Code can cause grid resets is because middle-click makes VS Code paste and if that data contained an r, it would reset the grid.
* First part of darkening cells in all directions constantly
* List more game names
* Change keyword order
* Remove unneeded Clone constraint
* Add --version and -V support

## 0.5.0 (2021-05-30)

* Shrink showcase image
* Document versions in CHANGELOG.md
* Remove old TODO comment
* Mention new controls
* Update showcase.png
* Update README.md
* Fix outdated help message
* Fix time formatting
* Remove redundant flushes
* Remap some keys
* Change measurement tool help text at the bottom
* Fix `unreachable` being reached when unknown key is pressed
* Basic implementation of arrow keys for darkening all cells in that direction

  This should be very useful for big grids. With this it is no longer possible to undo and redo with arrow key left and right respectively.
* Remove enter key for saving
* Rearrange match arms
* Add keyboard shortcut to window size message

  This won't work everywhere but in many cases.
* Replace x, y parameters with point
* Move Grid::from_lines to tests and simplify input

## 4.0.0 (2021-05-29)

* Make undo redo buffer keep track of measured cells
* Format
* No longer clear .yaya files for the period of the session
* Correct a comment
* Measurement tool
* Small refactor
* Use MAX_GRID_SIZE
* Improve window size info message
* Mention img2yaya

## 3.0.1 (2021-05-28)

* Make use of Cell::default
* Make it possible to pass both a width and height from the command line
* Make non-squared grids possible

  Fixes #1
* Add some more ideas

  I came up with this when I thought how to make yayagram more accessible to the average user.
* Add a basis for possibly a Ruler tool

  For measuring blocks.
* Remove 2 `unwrap`s

## 0.2.5 (2021-05-24)

* Small nits
* Move some functions to util.rs
* Fix dark cell color sometimes not being drawn
* Add help
* Lower default grid size
* A few nits & improvements
* Use new terminal crate version
* Improve README
* Initial commit
