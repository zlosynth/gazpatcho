# Changelog

## Main

* Allow control of the graph state from the user code.
* Deprecate `run` in favor of `run_with_callback` and `run_with_mpsc`.

## 1.0.1

* Bump imgui version to 0.6.
* Drop unused dependencies.

## 1.0.0

* Added save/load support.

## 0.1.0

* Allows adding and removal of nodes and patches.
* Supports widgets: Multiline input, slider, trigger, switch and dropdown.
* Accepts user-defined node templates as arguments.
* Informs the caller about the current state via a callback function.
